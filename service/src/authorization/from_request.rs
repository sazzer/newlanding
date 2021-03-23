use super::{auth0::AccessTokenParser, Authorization};
use crate::http::problem::{Problem, UNAUTHORIZED};
use actix_http::Payload;
use actix_web::{http::header, web::Data, FromRequest, HttpRequest};
use futures::future::Future;
use std::{pin::Pin, sync::Arc};

impl FromRequest for Authorization {
    type Error = Problem;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    type Config = ();

    #[tracing::instrument]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let access_token_parser: &Data<Arc<AccessTokenParser>> = req.app_data().unwrap();
        let access_token_parser = access_token_parser.get_ref().clone();

        let authorization = req.headers().get(header::AUTHORIZATION).cloned();
        tracing::debug!("Processing authorization header: {:?}", authorization);

        Box::pin(async move {
            if let Some(authorization) = authorization {
                let header_value = authorization
                    .to_str()
                    .map_err(|_| Problem::from(UNAUTHORIZED))?;

                let token = Some(header_value)
                    .filter(|h| h.starts_with("Bearer "))
                    .map(|h| &h[7..])
                    .ok_or_else(|| Problem::from(UNAUTHORIZED))?;

                let security_context = access_token_parser
                    .parse_token(token)
                    .await
                    .map_err(|_| Problem::from(UNAUTHORIZED))?;

                Ok(Authorization::Authorized(security_context))
            } else {
                Ok(Authorization::Unauthorized)
            }
        })
    }
}
