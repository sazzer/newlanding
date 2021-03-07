use std::sync::Arc;

use crate::server::RouteConfigurer;
use actix_web::web::ServiceConfig;

pub struct Component {}

pub fn new() -> Arc<Component> {
    Arc::new(Component {})
}

impl RouteConfigurer for Component {
    fn configure_routes(&self, config: &mut ServiceConfig) {
        super::http::configure_routes(config);
    }
}
