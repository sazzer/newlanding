use crate::users::{auth0::UserRepository, UserId, UserResource};

/// Use Case for getting user records.
pub struct GetUserUseCase {
    /// The repository of user data.
    repository: UserRepository,
}

impl GetUserUseCase {
    /// Create a new instance of the use case.
    ///
    /// # Parameters
    /// - `repository` - The repository of user data
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }

    /// Get a user record with the provided ID
    ///
    /// # Parameters
    /// - `id` - The ID of the user
    ///
    /// # Returns
    /// The details of the user, or `None` if the user couldn't be found.
    #[tracing::instrument(skip(self))]
    pub async fn get_user_by_id(&self, id: &UserId) -> Option<UserResource> {
        self.repository.get_user_by_id(id).await
    }
}
