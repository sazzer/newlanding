use chrono::Utc;
use uuid::Uuid;

use crate::{
    model::Identity,
    users::{UserData, UserId, UserResource},
};

pub struct GetUserUseCase {}

impl GetUserUseCase {
    #[tracing::instrument(skip(self))]
    pub async fn get_user_by_id(&self, id: &UserId) -> Option<UserResource> {
        let user = UserResource {
            identity: Identity {
                id: id.clone(),
                version: Uuid::new_v4().to_string(),
                created: Utc::now(),
                updated: Utc::now(),
            },
            data: UserData {
                display_name: "Graham".to_owned(),
                email: "graham@grahamcox.co.uk".to_owned(),
                email_verified: true,
                social_provider: None,
            },
        };

        Some(user)
    }
}
