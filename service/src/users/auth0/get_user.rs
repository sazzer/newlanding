use super::UserRepository;
use crate::{
    model::Identity,
    users::{UserData, UserId, UserResource},
};
use chrono::Utc;
use uuid::Uuid;

impl UserRepository {
    /// Get the requested user from Auth0.
    ///
    /// # Parameters
    /// - `id` - The ID of the user, as understood by Auth0.
    ///
    /// # Returns
    /// The user, or `None` if it couldn't be loaded.
    #[tracing::instrument(skip(self))]
    pub async fn get_user_by_id(&self, id: &UserId) -> Option<UserResource> {
        let _access_token = self.access_token_retriever.get_access_token().await;

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
