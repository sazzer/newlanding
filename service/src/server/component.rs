use std::sync::Arc;

use super::{RouteConfigurer, Server};

pub struct Component {
    pub server: Server,
}

#[derive(Default)]
pub struct Builder {
    routes: Vec<Arc<dyn RouteConfigurer>>,
}

pub fn new() -> Builder {
    Builder::default()
}

impl Builder {
    pub fn with_routes(mut self, routes: Arc<dyn RouteConfigurer>) -> Self {
        self.routes.push(routes);

        self
    }
    pub fn build(self, prometheus: prometheus::Registry) -> Component {
        tracing::debug!("Building HTTP Server component");
        Component {
            server: Server {
                port: 8000,
                prometheus,
                routes: self.routes,
            },
        }
    }
}
