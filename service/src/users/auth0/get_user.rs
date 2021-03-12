use super::UserRepository;
use crate::{
    model::Identity,
    users::{UserData, UserId, UserResource},
};
use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use serde::Deserialize;

impl UserRepository {
    /// Get the requested user from Auth0.
    ///
    /// # Parameters
    /// - `id` - The ID of the user, as understood by Auth0.
    ///
    /// # Returns
    /// The user, or `None` if it couldn't be loaded.
    #[tracing::instrument(skip(self))]
    pub async fn get_user_by_id(&self, id: UserId) -> Option<UserResource> {
        let access_token = self.access_token_retriever.get_access_token().await?;

        let url = self
            .domain
            .build_url_template("/api/v2/users/{id}")
            .set("id", id)
            .build();

        let auth0_user = {
            let span = tracing::trace_span!(
                "Auth0 Request",
                http.url = url.as_str(),
                http.status_code = tracing::field::Empty
            );
            let _enter = span.enter();

            let response = self
                .client
                .get(&url)
                .bearer_auth(access_token)
                .send()
                .await
                .unwrap();

            span.record("http.status_code", &response.status().as_u16());

            if response.status() == StatusCode::OK {
                let auth0_user: Auth0User = response.json().await.unwrap();
                Some(auth0_user)
            } else {
                tracing::warn!("Failed to retrieve user from Auth0");
                None
            }
        }?;

        tracing::debug!(auth0_user = ?auth0_user, "Retrieved user details");

        let social_provider = auth0_user.identities.into_iter().find_map(|i| {
            if i.is_social {
                Some(i.provider)
            } else {
                None
            }
        });

        let user = UserResource {
            identity: Identity {
                id: auth0_user.user_id.parse().unwrap(),
                version: base64::encode(format!("{}", auth0_user.updated_at)),
                created: auth0_user.created_at,
                updated: auth0_user.updated_at,
            },
            data: UserData {
                display_name: auth0_user.name,
                email: auth0_user.email,
                email_verified: auth0_user.email_verified,
                social_provider,
            },
        };

        Some(user)
    }
}

/// Representation of a user as retrieved from Auth0.
#[derive(Debug, Deserialize)]
struct Auth0User {
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    pub identities: Vec<Auth0Identity>,
}

/// Representation of a single user identity as retrieved from Auth0.
#[derive(Debug, Deserialize)]
struct Auth0Identity {
    #[serde(rename(deserialize = "isSocial"))]
    pub is_social: bool,
    pub provider: String,
}

#[cfg(test)]
mod tests {
    use crate::users::auth0::{ClientId, ClientSecret, Domain, UserRepository};
    use assert2::{check, let_assert};
    use chrono::DateTime;
    use mockito::mock;

    #[actix_rt::test]
    async fn access_token_failure() {
        let _ = env_logger::try_init();

        let m = mock("POST", "/oauth/token")
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        let sut = UserRepository::new(
            Domain::new(mockito::server_url()),
            ClientId::new("testClientId"),
            ClientSecret::new("testClientSecret"),
        );

        let user = sut.get_user_by_id("userid".parse().unwrap()).await;

        check!(user.is_none());

        m.assert();
    }

    #[actix_rt::test]
    async fn unknown_user() {
        let _ = env_logger::try_init();

        let access_token_mock = mock("POST", "/oauth/token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "access_token":"testAccessToken",
                "scope":"read:users update:users delete:users read:users_app_metadata update:users_app_metadata delete:users_app_metadata",
                "expires_in":86400,
                "token_type":"Bearer"
            }"#)
        .create();
        let users_mock = mock("GET", "/api/v2/users/userid")
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "error": "Not Found",
                "errorCode": "inexistent_user",
                "message": "The user does not exist.",
                "statusCode": 404
            }"#,
            )
            .create();

        let sut = UserRepository::new(
            Domain::new(mockito::server_url()),
            ClientId::new("testClientId"),
            ClientSecret::new("testClientSecret"),
        );

        let user = sut.get_user_by_id("userid".parse().unwrap()).await;

        check!(user.is_none());

        access_token_mock.assert();
        users_mock.assert();
    }

    #[actix_rt::test]
    async fn known_auth0_user() {
        let _ = env_logger::try_init();

        let access_token_mock = mock("POST", "/oauth/token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "access_token":"testAccessToken",
                "scope":"read:users update:users delete:users read:users_app_metadata update:users_app_metadata delete:users_app_metadata",
                "expires_in":86400,
                "token_type":"Bearer"
            }"#)
        .create();
        let users_mock = mock("GET", "/api/v2/users/auth0%7C6044f85d48fea20070575672")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "created_at": "2021-03-07T15:59:25.064Z",
                "email": "testuser@example.com",
                "email_verified": false,
                "identities": [
                    {
                        "connection": "Username-Password-Authentication",
                        "isSocial": false,
                        "provider": "auth0",
                        "user_id": "6044f85d48fea20070575672"
                    }
                ],
                "last_ip": "127.0.0.1",
                "last_login": "2021-03-07T15:59:25.061Z",
                "logins_count": 1,
                "name": "Test User",
                "nickname": "testuser",
                "updated_at": "2021-03-07T16:54:30.826Z",
                "user_id": "auth0|6044f85d48fea20070575672"
            }"#,
            )
            .create();

        let sut = UserRepository::new(
            Domain::new(mockito::server_url()),
            ClientId::new("testClientId"),
            ClientSecret::new("testClientSecret"),
        );

        let user = sut
            .get_user_by_id("auth0|6044f85d48fea20070575672".parse().unwrap())
            .await;

        let_assert!(Some(user) = user);
        check!(user.identity.id == "auth0|6044f85d48fea20070575672");
        check!(user.identity.version == "MjAyMS0wMy0wNyAxNjo1NDozMC44MjYgVVRD");
        check!(
            user.identity.created
                == DateTime::parse_from_rfc3339("2021-03-07T15:59:25.064Z").unwrap()
        );
        check!(
            user.identity.updated
                == DateTime::parse_from_rfc3339("2021-03-07T16:54:30.826Z").unwrap()
        );

        check!(user.data.display_name == "Test User");
        check!(user.data.email == "testuser@example.com");
        check!(user.data.email_verified == false);
        check!(user.data.social_provider == None);

        access_token_mock.assert();
        users_mock.assert();
    }

    #[actix_rt::test]
    async fn known_google_user() {
        let _ = env_logger::try_init();

        let access_token_mock = mock("POST", "/oauth/token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "access_token":"testAccessToken",
                "scope":"read:users update:users delete:users read:users_app_metadata update:users_app_metadata delete:users_app_metadata",
                "expires_in":86400,
                "token_type":"Bearer"
            }"#)
        .create();
        let users_mock = mock("GET", "/api/v2/users/google-oauth2%7C116440097717692497264")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "created_at": "2021-03-07T15:59:25.064Z",
                "email": "testuser@example.com",
                "email_verified": true,
                "identities": [
                    {
                        "connection": "google-oauth2",
                        "isSocial": true,
                        "provider": "google-oauth2",
                        "user_id": "116440097717692497264"
                    }
                ],
                "last_ip": "127.0.0.1",
                "last_login": "2021-03-07T15:59:25.061Z",
                "logins_count": 1,
                "name": "Test User",
                "nickname": "testuser",
                "updated_at": "2021-03-07T16:54:30.826Z",
                "user_id": "google-oauth2|116440097717692497264"
            }"#,
            )
            .create();

        let sut = UserRepository::new(
            Domain::new(mockito::server_url()),
            ClientId::new("testClientId"),
            ClientSecret::new("testClientSecret"),
        );

        let user = sut
            .get_user_by_id("google-oauth2|116440097717692497264".parse().unwrap())
            .await;

        let_assert!(Some(user) = user);
        check!(user.identity.id == "google-oauth2|116440097717692497264");
        check!(user.identity.version == "MjAyMS0wMy0wNyAxNjo1NDozMC44MjYgVVRD");
        check!(
            user.identity.created
                == DateTime::parse_from_rfc3339("2021-03-07T15:59:25.064Z").unwrap()
        );
        check!(
            user.identity.updated
                == DateTime::parse_from_rfc3339("2021-03-07T16:54:30.826Z").unwrap()
        );

        check!(user.data.display_name == "Test User");
        check!(user.data.email == "testuser@example.com");
        check!(user.data.email_verified == true);
        check!(user.data.social_provider == Some("google-oauth2".to_owned()));

        access_token_mock.assert();
        users_mock.assert();
    }
}
