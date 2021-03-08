use actix_web::web::{get, resource, ServiceConfig};

mod get;
mod model;

pub fn configure_routes(config: &mut ServiceConfig) {
    config.service(resource("/users/{userId}").route(get().to(get::handle)));
}
