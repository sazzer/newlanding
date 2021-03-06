use crate::{authorization::Authorization, http::hal::Link};
use async_trait::async_trait;
use std::sync::Arc;

/// Trait for all components that can contribute links to the home document.
#[async_trait]
pub trait LinkContributor: Send + Sync {
    /// Generate the links for this component.
    async fn generate_links(&self, authorization: &Authorization) -> Vec<(String, Link)>;
}

/// Use Case for generating the entire set of links for the home document.
pub struct HomeLinksUseCase {
    pub(super) contributors: Vec<Arc<dyn LinkContributor>>,
}

impl HomeLinksUseCase {
    /// Generate the links for this component.
    pub async fn generate_links(&self, authorization: &Authorization) -> Vec<(String, Link)> {
        let mut result = vec![];

        for c in &self.contributors {
            let mut links = c.generate_links(authorization).await;
            result.append(&mut links);
        }

        result
    }
}

#[async_trait]
impl LinkContributor for Vec<(String, Link)> {
    async fn generate_links(&self, _authorization: &Authorization) -> Vec<(String, Link)> {
        self.clone()
    }
}
