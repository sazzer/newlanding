use super::domain::Domain;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    cell::RefCell,
    sync::Mutex,
    time::{Duration, SystemTime},
};

/// Type-safe representation of the Auth0 Client ID.
#[derive(Debug, Serialize)]
pub struct ClientId(String);

impl ClientId {
    /// Create a new instance of the Auth0 Client ID from the given string
    ///
    /// # Parameters
    /// - `client_id` - The Client ID
    pub fn new<S>(client_id: S) -> Self
    where
        S: Into<String>,
    {
        Self(client_id.into())
    }
}

/// Type-safe representation of the Auth0 Client ID.
#[derive(Debug, Serialize)]
pub struct ClientSecret(String);

impl ClientSecret {
    /// Create a new instance of the Auth0 Client Secret from the given string
    ///
    /// # Parameters
    /// - `client_secret` - The Client Secret
    pub fn new<S>(client_secret: S) -> Self
    where
        S: Into<String>,
    {
        Self(client_secret.into())
    }
}

/// Representation of an actual access token to use with Auth0.
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct AccessToken(String);

/// Mechanism to use to retrieve access tokens to sue with Auth0.
pub struct Retriever {
    domain: Domain,
    client_id: ClientId,
    client_secret: ClientSecret,
    client: Client,
    cache: Mutex<RefCell<CacheEntry>>,
}

impl Retriever {
    /// Create a new instance of the `Retriever`
    ///
    /// # Parameters
    /// - `domain` - the Auth0 domain, including the HTTP scheme. For example "https://example.eu.auth0.com"
    /// - `client_id` - the Auth0 Client ID for this Auth0 M2M Application.
    /// - `client_secret` - the Auth0 Client Secret for this Auth0 M2M Application.
    pub fn new(domain: Domain, client_id: ClientId, client_secret: ClientSecret) -> Self {
        let cache_entry = CacheEntry {
            expires: SystemTime::UNIX_EPOCH,
            token: None,
        };

        Self {
            domain,
            client_id,
            client_secret,
            client: Client::new(),
            cache: Mutex::new(RefCell::new(cache_entry)),
        }
    }

    /// Get an access token to use.
    /// If we have a valid cached version then this will be returned as-is, otherwise a new one will be fetched
    ///
    /// # Returns
    /// The access token to use.
    /// If we fail to fetch a token for any reason then instead `None` is returned.
    #[tracing::instrument(skip(self))]
    pub async fn get_access_token(&self) -> Option<AccessToken> {
        let lock = self.cache.lock().unwrap();

        let mut entry = lock.borrow_mut();

        if entry.expired() {
            tracing::info!("Access token is not cached. Requesting new one");

            let token = self.fetch_access_token().await;
            if let Some((token, expiry)) = token {
                entry.token = Some(token);
                entry.expires = SystemTime::now() + Duration::from_secs(expiry - 10); // Expire 10 seconds earlier than we were told, to be safe.

                tracing::debug!(entry = ?entry, "Caching access token");
            }
        } else {
            tracing::debug!(entry = ?entry, "Using cached access token");
        }

        entry.token.clone()
    }

    /// Clear the cached value.
    /// This allows subsequent calls to `get_access_token()` to instead fetch a new one from Auth0.
    #[tracing::instrument(skip(self))]
    pub fn clear_cache(&self) {
        tracing::debug!("Clearing cached access token");
        let lock = self.cache.lock().unwrap();
        let mut entry = lock.borrow_mut();
        entry.token = None;
    }

    /// Actually call Auth0 to fetch a new access token.
    ///
    /// # Returns
    /// The access token to use.
    /// If we fail to fetch a token for any reason then instead `None` is returned.
    #[tracing::instrument(skip(self))]
    async fn fetch_access_token(&self) -> Option<(AccessToken, u64)> {
        let request = json!({
          "client_id": self.client_id,
          "client_secret": self.client_secret,
          "audience": self.domain.build_url("/api/v2/"),
          "grant_type": "client_credentials"
        });

        tracing::debug!(request = ?request, "Request for access token");

        let result = self
            .client
            .post(&self.domain.build_url("/oauth/token"))
            .json(&request)
            .send()
            .await;

        tracing::debug!(result = ?result, "Access token result");
        let result = match result {
            Ok(r) => {
                if r.status() == StatusCode::OK {
                    Some(r)
                } else {
                    tracing::error!(response = ?r, "Failed to request access token");

                    let body = r.text().await;
                    tracing::error!(response_body = ?body, "Response body from failure");

                    None
                }
            }
            Err(e) => {
                tracing::error!(e = ?e, "Failed to request access token");
                None
            }
        }?;

        let body: AccessTokenResponse = match result.json().await {
            Ok(b) => Some(b),
            Err(e) => {
                tracing::error!(e = ?e, "Failed to parse access token response");
                None
            }
        }?;

        tracing::debug!(result = ?body, "Access token body");

        Some((body.access_token, body.expires_in))
    }
}

/// Representation of the Auth0 response when retrieving an access token.
#[derive(Debug, Deserialize)]
struct AccessTokenResponse {
    access_token: AccessToken,
    expires_in: u64,
}

/// Wrapper around an access token and when it expires, to support caching.
#[derive(Debug)]
struct CacheEntry {
    expires: SystemTime,
    token: Option<AccessToken>,
}

impl CacheEntry {
    /// Check if the cache entry has expired.
    /// Also returns true if we simply don't have an entry cached.
    ///
    /// # Returns
    /// `true` if this cache entry is expired or else doesn't have a value.
    /// `false` if this cache entry has a value that is believed to be valid to use.
    fn expired(&self) -> bool {
        self.token.is_none() || self.expires < SystemTime::now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::{check, let_assert};
    use mockito::{mock, Matcher};

    #[actix_rt::test]
    async fn get_access_token_success() {
        let _ = env_logger::try_init();

        let m = mock("POST", "/oauth/token")
            .match_body(Matcher::Json(json!({
              "client_id": "testClientId",
              "client_secret": "testClientSecret",
              "audience": format!("{}/api/v2/", mockito::server_url()),
              "grant_type": "client_credentials"    
            })))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{
              "access_token":"testAccessToken",
              "scope":"read:users update:users delete:users read:users_app_metadata update:users_app_metadata delete:users_app_metadata",
              "expires_in":86400,
              "token_type":"Bearer"
            }"#)
            .create();

        let sut = Retriever::new(
            Domain::new(mockito::server_url()),
            ClientId("testClientId".to_owned()),
            ClientSecret("testClientSecret".to_owned()),
        );

        let access_token = sut.get_access_token().await;

        let_assert!(Some(access_token) = access_token);
        check!(access_token.0 == "testAccessToken");

        m.assert();
    }

    #[actix_rt::test]
    async fn get_access_token_failure() {
        let _ = env_logger::try_init();

        let m = mock("POST", "/oauth/token")
            .match_body(Matcher::Json(json!({
              "client_id": "testClientId",
              "client_secret": "testClientSecret",
              "audience": format!("{}/api/v2/", mockito::server_url()),
              "grant_type": "client_credentials"
            })))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
              "error":"access_denied",
              "error_description":"Unauthorized"
            }"#,
            )
            .create();

        let sut = Retriever::new(
            Domain::new(mockito::server_url()),
            ClientId("testClientId".to_owned()),
            ClientSecret("testClientSecret".to_owned()),
        );

        let access_token = sut.get_access_token().await;

        check!(access_token == None);

        m.assert();
    }

    #[actix_rt::test]
    async fn get_access_token_cached() {
        let _ = env_logger::try_init();

        let m = mock("POST", "/oauth/token")
            .match_body(Matcher::Json(json!({
              "client_id": "testClientId",
              "client_secret": "testClientSecret",
              "audience": format!("{}/api/v2/", mockito::server_url()),
              "grant_type": "client_credentials"    
            })))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{
              "access_token":"testAccessToken",
              "scope":"read:users update:users delete:users read:users_app_metadata update:users_app_metadata delete:users_app_metadata",
              "expires_in":86400,
              "token_type":"Bearer"
            }"#)
            .create();

        let sut = Retriever::new(
            Domain::new(mockito::server_url()),
            ClientId("testClientId".to_owned()),
            ClientSecret("testClientSecret".to_owned()),
        );

        let access_token_1 = sut.get_access_token().await;
        let_assert!(Some(access_token_1) = access_token_1);
        check!(access_token_1.0 == "testAccessToken");

        let access_token_2 = sut.get_access_token().await;
        let_assert!(Some(access_token_2) = access_token_2);
        check!(access_token_2.0 == "testAccessToken");

        m.assert();
    }

    #[actix_rt::test]
    async fn get_access_token_failure_cached() {
        let _ = env_logger::try_init();

        let m = mock("POST", "/oauth/token")
            .match_body(Matcher::Json(json!({
              "client_id": "testClientId",
              "client_secret": "testClientSecret",
              "audience": format!("{}/api/v2/", mockito::server_url()),
              "grant_type": "client_credentials"
            })))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
              "error":"access_denied",
              "error_description":"Unauthorized"
            }"#,
            )
            .expect(2)
            .create();

        let sut = Retriever::new(
            Domain::new(mockito::server_url()),
            ClientId("testClientId".to_owned()),
            ClientSecret("testClientSecret".to_owned()),
        );

        let access_token_1 = sut.get_access_token().await;
        check!(access_token_1 == None);

        let access_token_2 = sut.get_access_token().await;
        check!(access_token_2 == None);

        m.assert();
    }

    #[actix_rt::test]
    async fn get_access_token_clear_cache() {
        let _ = env_logger::try_init();

        let m = mock("POST", "/oauth/token")
            .match_body(Matcher::Json(json!({
              "client_id": "testClientId",
              "client_secret": "testClientSecret",
              "audience": format!("{}/api/v2/", mockito::server_url()),
              "grant_type": "client_credentials"    
            })))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{
              "access_token":"testAccessToken",
              "scope":"read:users update:users delete:users read:users_app_metadata update:users_app_metadata delete:users_app_metadata",
              "expires_in":86400,
              "token_type":"Bearer"
            }"#)
            .expect(2)
            .create();

        let sut = Retriever::new(
            Domain::new(mockito::server_url()),
            ClientId("testClientId".to_owned()),
            ClientSecret("testClientSecret".to_owned()),
        );

        let access_token_1 = sut.get_access_token().await;
        let_assert!(Some(access_token_1) = access_token_1);
        check!(access_token_1.0 == "testAccessToken");

        sut.clear_cache();

        let access_token_2 = sut.get_access_token().await;
        let_assert!(Some(access_token_2) = access_token_2);
        check!(access_token_2.0 == "testAccessToken");

        m.assert();
    }
}
