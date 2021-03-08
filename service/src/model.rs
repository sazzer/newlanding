use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct Identity<I> {
    pub id: I,
    pub version: Uuid,
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
        let version = Uuid::new_v4();

        Self {
            id,
            version,
            created: now,
            updated: now,
        }
    }
}

#[derive(Debug)]
pub struct Resource<I, D> {
    pub identity: Identity<I>,
    pub data: D,
}
