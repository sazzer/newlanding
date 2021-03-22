use super::{keys::Keys, Domain};
use crate::authorization::{Principal, SecurityContext};
use biscuit::{jwk::JWKSet, jws::Compact, ClaimsSet, Validation, ValidationOptions};
use std::ops::Deref;

/// Parser to parse an access token string
pub struct AccessTokenParser {
    /// The keys needed to parse the access token.
    keys: Keys,
    /// The Auth0 domain that the tokens are from.
    domain: Domain,
    /// The Auth0 audience that the tokens must be for.
    audience: String,
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ParseError {
    #[error("The token was malformed")]
    MalformedToken,

    #[error("The token was invalid")]
    InvalidToken,

    #[error("The token was signed with an unknown key")]
    UnknownKey,
}

impl AccessTokenParser {
    /// Create a new instance of the access token parser.
    ///
    /// # Parameters
    /// - `domain` - The Auth0 domain that the tokens are from
    /// - `audience` - The API audience that the tokens are for
    pub fn new<A>(domain: Domain, audience: A) -> Self
    where
        A: Into<String>,
    {
        let keys = Keys::new(&domain);

        Self {
            domain,
            keys,
            audience: audience.into(),
        }
    }

    /// Attempt to parse the provided token.
    ///
    /// # Parameters
    /// - `token` - The token to parse
    ///
    /// # Returns
    /// The parsed token, or an error indicating why it couldn't be parsed.
    pub async fn parse_token<T>(&self, token: T) -> Result<SecurityContext, ParseError>
    where
        T: Into<String>,
    {
        let encoded = Compact::<ClaimsSet<()>, ()>::new_encoded(&token.into());
        let header = encoded
            .unverified_header()
            .map_err(|_| ParseError::MalformedToken)?;
        let kid = header.registered.key_id.ok_or(ParseError::UnknownKey)?;

        let key = self.keys.get(&kid).await.ok_or(ParseError::UnknownKey)?;

        let decoded = encoded
            .decode_with_jwks(&JWKSet { keys: vec![key] }, None)
            .map_err(|_| ParseError::MalformedToken)?;

        decoded
            .validate(ValidationOptions {
                issuer: Validation::Validate(self.domain.build_url("/")),
                audience: Validation::Validate(self.audience.clone()),
                ..ValidationOptions::default()
            })
            .map_err(|e| {
                tracing::warn!(e = ?e, token = ?decoded, "Token validation failed");
                ParseError::InvalidToken
            })?;

        let payload = decoded.payload().map_err(|_| ParseError::MalformedToken)?;

        let sub = payload.registered.subject.clone().ok_or_else(|| {
            tracing::warn!(token = ?decoded, field = "sub", "Missing field");
            ParseError::MalformedToken
        })?;
        let iat = payload.registered.issued_at.ok_or_else(|| {
            tracing::warn!(token = ?decoded, field = "iat", "Missing field");
            ParseError::MalformedToken
        })?;
        let exp = payload.registered.expiry.ok_or_else(|| {
            tracing::warn!(token = ?decoded, field = "exp", "Missing field");
            ParseError::MalformedToken
        })?;

        Ok(SecurityContext {
            principal: Principal::User(sub),
            issued: iat.deref().clone(),
            expires: exp.deref().clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::{check, let_assert};
    use biscuit::{
        jwa::SignatureAlgorithm,
        jwk::{JWKSet, JWK},
        jws::{RegisteredHeader, Secret},
        RegisteredClaims, SingleOrMultiple,
    };
    use chrono::{DateTime, Duration, SubsecRound, Utc};
    use mockito::mock;

    fn load_jwk(kid: &str) -> JWK<()> {
        let jwk_contents = std::fs::read_to_string("./keys/public_key.jwk").unwrap();
        let mut jwk: JWK<()> = serde_json::from_str(&jwk_contents).unwrap();

        jwk.common.key_id = Some(kid.to_owned());

        jwk
    }

    fn load_secret() -> Secret {
        Secret::rsa_keypair_from_file("./keys/private_key.der").unwrap()
    }

    fn build_token(
        kid: Option<&str>,
        iss: Option<&str>,
        sub: Option<&str>,
        aud: Option<&str>,
        iat: Option<DateTime<Utc>>,
        exp: Option<DateTime<Utc>>,
    ) -> String {
        let decoded = Compact::new_decoded(
            RegisteredHeader {
                algorithm: SignatureAlgorithm::RS256,
                key_id: kid.map(|s| s.to_owned()),
                ..Default::default()
            }
            .into(),
            ClaimsSet::<()> {
                registered: RegisteredClaims {
                    issuer: iss.map(|s| s.parse().unwrap()),
                    subject: sub.map(|s| s.parse().unwrap()),
                    audience: aud.map(|s| SingleOrMultiple::Single(s.parse().unwrap())),
                    issued_at: iat.map(|t| t.into()),
                    expiry: exp.map(|t| t.into()),
                    ..Default::default()
                },
                private: (),
            },
        );

        let encoded = decoded.encode(&load_secret()).unwrap();

        let token = encoded.encoded().unwrap().to_string();
        tracing::debug!(token = ?token, "Encoded JWT");

        token
    }

    #[actix_rt::test]
    async fn test_parse_valid_token() {
        let _ = env_logger::try_init();

        let now = Utc::now().round_subsecs(0);

        let m = mock("GET", "/.well-known/jwks.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                serde_json::to_string(&JWKSet {
                    keys: vec![load_jwk("myKeyId")],
                })
                .unwrap(),
            )
            .create();

        let sut = AccessTokenParser::new(
            Domain::new(mockito::server_url()),
            "tag:newlanding,2021:auth0",
        );

        let token = build_token(
            Some("myKeyId"),
            Some(&format!("{}/", mockito::server_url())),
            Some("userId"),
            Some("tag:newlanding,2021:auth0"),
            Some(now - Duration::days(5)),
            Some(now + Duration::days(5)),
        );

        let parsed = sut.parse_token(token).await;

        let_assert!(Ok(security_context) = parsed);
        check!(security_context.principal == Principal::User("userId".to_owned()));
        check!(security_context.issued == now - Duration::days(5));
        check!(security_context.expires == now + Duration::days(5));

        m.assert();
    }

    #[actix_rt::test]
    async fn test_parse_malformed_token() {
        let _ = env_logger::try_init();

        let sut = AccessTokenParser::new(
            Domain::new(mockito::server_url()),
            "tag:newlanding,2021:auth0",
        );

        let parsed = sut.parse_token("malformed").await;

        let_assert!(Err(err) = parsed);
        check!(err == ParseError::MalformedToken);
    }

    #[actix_rt::test]
    async fn test_parse_unknown_key() {
        let _ = env_logger::try_init();

        let now = Utc::now().round_subsecs(0);

        let m = mock("GET", "/.well-known/jwks.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                serde_json::to_string(&JWKSet {
                    keys: vec![load_jwk("myKeyId")],
                })
                .unwrap(),
            )
            .create();

        let sut = AccessTokenParser::new(
            Domain::new(mockito::server_url()),
            "tag:newlanding,2021:auth0",
        );

        let token = build_token(
            Some("unknownKey"),
            Some(&format!("{}/", mockito::server_url())),
            Some("userId"),
            Some("tag:newlanding,2021:auth0"),
            Some(now - Duration::days(5)),
            Some(now + Duration::days(5)),
        );

        let parsed = sut.parse_token(token).await;

        let_assert!(Err(err) = parsed);
        check!(err == ParseError::UnknownKey);

        m.assert();
    }

    #[actix_rt::test]
    async fn test_parse_no_key_id() {
        let _ = env_logger::try_init();

        let now = Utc::now().round_subsecs(0);

        let sut = AccessTokenParser::new(
            Domain::new(mockito::server_url()),
            "tag:newlanding,2021:auth0",
        );

        let token = build_token(
            None,
            Some(&format!("{}/", mockito::server_url())),
            Some("userId"),
            Some("tag:newlanding,2021:auth0"),
            Some(now - Duration::days(5)),
            Some(now + Duration::days(5)),
        );

        let parsed = sut.parse_token(token).await;

        let_assert!(Err(err) = parsed);
        check!(err == ParseError::UnknownKey);
    }

    async fn test_parse_error(
        iss: Option<&str>,
        sub: Option<&str>,
        aud: Option<&str>,
        iat: Option<DateTime<Utc>>,
        exp: Option<DateTime<Utc>>,
    ) -> ParseError {
        let _ = env_logger::try_init();

        let m = mock("GET", "/.well-known/jwks.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                serde_json::to_string(&JWKSet {
                    keys: vec![load_jwk("myKeyId")],
                })
                .unwrap(),
            )
            .create();

        let sut = AccessTokenParser::new(
            Domain::new(mockito::server_url()),
            "tag:newlanding,2021:auth0",
        );

        let token = build_token(Some("myKeyId"), iss, sub, aud, iat, exp);

        let parsed = sut.parse_token(token).await;

        let_assert!(Err(err) = parsed);

        m.assert();

        err
    }

    #[actix_rt::test]
    async fn test_parse_missing_sub_field() {
        let now = Utc::now().round_subsecs(0);

        let err = test_parse_error(
            Some(&format!("{}/", mockito::server_url())),
            None,
            Some("tag:newlanding,2021:auth0"),
            Some(now - Duration::days(5)),
            Some(now + Duration::days(5)),
        )
        .await;

        check!(err == ParseError::MalformedToken);
    }

    #[actix_rt::test]
    async fn test_parse_missing_iat_field() {
        let now = Utc::now().round_subsecs(0);

        let err = test_parse_error(
            Some(&format!("{}/", mockito::server_url())),
            Some("userId"),
            Some("tag:newlanding,2021:auth0"),
            None,
            Some(now + Duration::days(5)),
        )
        .await;

        check!(err == ParseError::MalformedToken);
    }

    #[actix_rt::test]
    async fn test_parse_missing_exp_field() {
        let now = Utc::now().round_subsecs(0);

        let err = test_parse_error(
            Some(&format!("{}/", mockito::server_url())),
            Some("userId"),
            Some("tag:newlanding,2021:auth0"),
            Some(now - Duration::days(5)),
            None,
        )
        .await;

        check!(err == ParseError::MalformedToken);
    }

    #[actix_rt::test]
    async fn test_parse_invalid_iss_field() {
        let now = Utc::now().round_subsecs(0);

        let err = test_parse_error(
            Some("http://other.example.com/"),
            Some("userId"),
            Some("tag:newlanding,2021:auth0"),
            Some(now - Duration::days(5)),
            Some(now + Duration::days(5)),
        )
        .await;

        check!(err == ParseError::InvalidToken);
    }

    #[actix_rt::test]
    async fn test_parse_invalid_exp_field() {
        let now = Utc::now().round_subsecs(0);

        let err = test_parse_error(
            Some(&format!("{}/", mockito::server_url())),
            Some("userId"),
            Some("tag:newlanding,2021:auth0"),
            Some(now - Duration::days(5)),
            Some(now - Duration::days(2)),
        )
        .await;

        check!(err == ParseError::InvalidToken);
    }

    #[actix_rt::test]
    async fn test_parse_invalid_iat_field() {
        let now = Utc::now().round_subsecs(0);

        let err = test_parse_error(
            Some(&format!("{}/", mockito::server_url())),
            Some("userId"),
            Some("tag:newlanding,2021:auth0"),
            Some(now + Duration::days(2)),
            Some(now + Duration::days(5)),
        )
        .await;

        check!(err == ParseError::InvalidToken);
    }

    #[actix_rt::test]
    async fn test_parse_invalid_aud_field() {
        let now = Utc::now().round_subsecs(0);

        let err = test_parse_error(
            Some(&format!("{}/", mockito::server_url())),
            Some("userId"),
            Some("wrong"),
            Some(now - Duration::days(5)),
            Some(now + Duration::days(5)),
        )
        .await;

        check!(err == ParseError::InvalidToken);
    }
}
