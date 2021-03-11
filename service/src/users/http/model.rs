use crate::{
    http::{
        hal::{HalDocument, HalRespondable},
        Response,
    },
    users::UserResource,
};
use actix_http::http::{
    header::{CacheControl, CacheDirective, ETag, EntityTag},
    StatusCode,
};
use serde::Serialize;

/// Representation of a User on the HTTP API.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserModel {
    pub display_name: String,
    pub email: String,
    pub email_verified: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_provider: Option<String>,
}

impl From<UserResource> for Response<HalRespondable> {
    fn from(user: UserResource) -> Self {
        let hal_document = HalDocument::new(UserModel {
            display_name: user.data.display_name,
            email: user.data.email,
            email_verified: user.data.email_verified,
            social_provider: user.data.social_provider,
        })
        .with_link("self", user.identity.id);

        let respondable = HalRespondable::from(hal_document)
            .with_status_code(StatusCode::OK)
            .with_header(CacheControl(vec![
                CacheDirective::Public,
                CacheDirective::MaxAge(3600),
            ]))
            .with_header(ETag(EntityTag::strong(user.identity.version)));

        Response(respondable)
    }
}
