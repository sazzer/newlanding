use std::sync::Arc;

use crate::server::RouteConfigurer;
use actix_web::web::ServiceConfig;

use super::{HomeLinksUseCase, LinkContributor};

pub struct Component {
    service: Arc<HomeLinksUseCase>,
}

#[derive(Default)]
pub struct Builder {
    contributors: Vec<Arc<dyn LinkContributor>>,
}

pub fn new() -> Builder {
    Builder::default().with_contributor(Arc::new(vec![("self".to_owned(), "/".into())]))
}

impl Builder {
    /// Add a new contributor of links to the home document.
    #[allow(dead_code)]
    pub fn with_contributor(mut self, contributor: Arc<dyn LinkContributor>) -> Self {
        self.contributors.push(contributor);

        self
    }

    /// Build the actual home document component.
    pub fn build(self) -> Arc<Component> {
        let service = Arc::new(HomeLinksUseCase {
            contributors: self.contributors,
        });

        Arc::new(Component { service })
    }
}

impl RouteConfigurer for Component {
    fn configure_routes(&self, config: &mut ServiceConfig) {
        config.data(self.service.clone());
        super::http::configure_routes(config);
    }
}
