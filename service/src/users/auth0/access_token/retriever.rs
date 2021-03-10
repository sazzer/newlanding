use super::AccessToken;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize)]
pub struct ClientId(String);

#[derive(Debug, Serialize)]
pub struct ClientSecret(String);

#[derive(Debug, Deserialize)]
struct AccessTokenResponse {
    access_token: AccessToken,
    expires_in: u32,
}

pub struct Retriever {
    domain: String,
    client_id: ClientId,
    client_secret: ClientSecret,
    client: Client,
}

impl Retriever {
    pub fn new(domain: String, client_id: ClientId, client_secret: ClientSecret) -> Self {
        Self {
            domain,
            client_id,
            client_secret,
            client: Client::new(),
        }
    }

    pub async fn get_access_token(&self) -> Option<AccessToken> {
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

        Some(body.access_token)
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
}
