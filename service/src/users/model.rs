use crate::model::Resource;

pub struct UserId(String);

pub struct UserData {
    pub display_name: String,
    pub email: String,
    pub email_verified: bool,
    pub social_provider: Option<String>,
}

pub type UserResource = Resource<UserId, UserData>;
