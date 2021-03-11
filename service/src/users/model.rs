mod user_id;

use crate::model::Resource;
pub use user_id::*;

/// Definition of the data that makes up a user.
#[derive(Debug)]
pub struct UserData {
    /// The display name of the user.
    pub display_name: String,
    /// The email address of the user.
    pub email: String,
    /// Whether or not the email address is verified.
    pub email_verified: bool,
    /// If the user is registered with a social provider then which one.
    pub social_provider: Option<String>,
}

/// Resource representing a persisted user.
pub type UserResource = Resource<UserId, UserData>;
