use actix_web::web::{get, resource, ServiceConfig};

mod get;
mod model;

/// Configure the HTTP routes for working with users.
///
/// # Parameters
/// - `config` - The HTTP Server configuration to register the routes with.
pub fn configure_routes(config: &mut ServiceConfig) {
    config.service(resource("/users/{userId}").route(get().to(get::handle)));
}
