use crate::http::hal::Link;
use std::str::FromStr;
use uritemplate::{IntoTemplateVar, TemplateVar, UriTemplate};

/// Representation of the ID of a user.
#[derive(Debug, PartialEq, Clone)]
pub struct UserId(String);

/// Errors that can occur when parsing a User ID from a string.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ParseUserIdError {
    #[error("The User ID was blank")]
    Blank,
}

impl FromStr for UserId {
    type Err = ParseUserIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            Err(ParseUserIdError::Blank)
        } else {
            Ok(UserId(trimmed.to_owned()))
        }
    }
}

impl From<UserId> for Link {
    fn from(user_id: UserId) -> Self {
        UriTemplate::new("/users/{id}")
            .set("id", user_id.0)
            .build()
            .into()
    }
}

impl IntoTemplateVar for UserId {
    fn into_template_var(self) -> TemplateVar {
        TemplateVar::Scalar(self.0)
    }
}

#[cfg(test)]
impl PartialEq<&str> for UserId {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::{check, let_assert};
    use test_case::test_case;

    #[test_case("",  &ParseUserIdError::Blank  ; "blank string")]
    #[test_case("  ",  &ParseUserIdError::Blank  ; "whitespace only")]
    fn parse_blank_userid(input: &str, err: &ParseUserIdError) {
        let parsed = UserId::from_str(input);

        let_assert!(Err(e) = parsed);
        check!(&e == err);
    }

    #[test_case("testUserId",  "testUserId"  ; "simple string")]
    #[test_case("  testUserId",  "testUserId"  ; "left-padded")]
    #[test_case("testUserId  ",  "testUserId"  ; "right-padded")]
    #[test_case("  testUserId  ",  "testUserId"  ; "both-padded")]
    fn parse_valid_userid(input: &str, output: &str) {
        let parsed = UserId::from_str(input);

        let_assert!(Ok(e) = parsed);
        check!(e.0 == output);
    }
}
