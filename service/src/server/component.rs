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
    pub fn build(self, port: u16, prometheus: prometheus::Registry) -> Component {
        tracing::debug!("Building HTTP Server component");
        Component {
            server: Server {
                port,
                prometheus,
                routes: self.routes,
            },
        }
    }
}
