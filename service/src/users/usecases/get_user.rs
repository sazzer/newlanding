use crate::users::{repository::UserRepository, UserId, UserResource};

pub struct GetUserUseCase {
    repository: UserRepository,
}

impl GetUserUseCase {
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }
    #[tracing::instrument(skip(self))]
    pub async fn get_user_by_id(&self, id: &UserId) -> Option<UserResource> {
        self.repository.get_user_by_id(id).await
    }
}
