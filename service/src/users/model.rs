mod user_id;

use crate::model::Resource;
pub use user_id::*;

#[derive(Debug)]
pub struct UserData {
    pub display_name: String,
    pub email: String,
    pub email_verified: bool,
    pub social_provider: Option<String>,
}

pub type UserResource = Resource<UserId, UserData>;
