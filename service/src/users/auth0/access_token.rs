use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    cell::RefCell,
    sync::Mutex,
    time::{Duration, SystemTime},
};

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct AccessToken(String);

#[derive(Debug, Serialize)]
pub struct ClientId(String);

#[derive(Debug, Serialize)]
pub struct ClientSecret(String);

#[derive(Debug, Deserialize)]
struct AccessTokenResponse {
    access_token: AccessToken,
    expires_in: u64,
}

#[derive(Debug)]
struct CacheEntry {
    expires: SystemTime,
    token: Option<AccessToken>,
}

impl CacheEntry {
    fn expired(&self) -> bool {
        self.token.is_none() || self.expires < SystemTime::now()
    }
}

pub struct Retriever {
    domain: String,
    client_id: ClientId,
    client_secret: ClientSecret,
    client: Client,
    cache: Mutex<RefCell<CacheEntry>>,
}

impl Retriever {
    pub fn new(domain: String, client_id: ClientId, client_secret: ClientSecret) -> Self {
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

    #[tracing::instrument(skip(self))]
    pub fn clear_cache(&self) {
        tracing::debug!("Clearing cached access token");
        let lock = self.cache.lock().unwrap();
        let mut entry = lock.borrow_mut();
        entry.token = None;
    }

    #[tracing::instrument(skip(self))]
    async fn fetch_access_token(&self) -> Option<(AccessToken, u64)> {
        let request = json!({
          "client_id": self.client_id,
          "client_secret": self.client_secret,
          "audience": format!("{}/api/v2/", self.domain),
          "grant_type": "client_credentials"
        });

        let result = self
            .client
            .post(&format!("{}/oauth/token", self.domain))
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
            mockito::server_url(),
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
            mockito::server_url(),
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
            mockito::server_url(),
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
            mockito::server_url(),
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
            mockito::server_url(),
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
