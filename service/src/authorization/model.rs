use chrono::{DateTime, Utc};

/// Enumeration of supported Principal IDs
#[derive(Debug, PartialEq)]
pub enum Principal {
    /// The authorized principal is a User.
    User(String),
}

/// Details of a Security Context for a request.
#[derive(Debug)]
pub struct SecurityContext {
    /// The authorized principal.
    pub principal: Principal,
    /// When the security context was issued.
    pub issued: DateTime<Utc>,
    /// When the security context expires.
    pub expires: DateTime<Utc>,
}
