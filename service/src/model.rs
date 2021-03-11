use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Representation of the identity of some resource
///
/// # Types
/// - `I` - The type to use for the resource ID
#[derive(Debug)]
pub struct Identity<I> {
    pub id: I,
    pub version: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl<I> Default for Identity<I>
where
    I: Default,
{
    fn default() -> Self {
        let id = I::default();
        let now = Utc::now();
        let version = Uuid::new_v4().to_string();

        Self {
            id,
            version,
            created: now,
            updated: now,
        }
    }
}

/// Representation of a persisted resource
///
/// # Types
/// - `I` - The type to use for the resource ID
/// - `D` - The type to use for the resource data.
#[derive(Debug)]
pub struct Resource<I, D> {
    pub identity: Identity<I>,
    pub data: D,
}
