use actix_web::web::{get, resource, ServiceConfig};

mod get;

pub fn configure_routes(config: &mut ServiceConfig) {
    config.service(resource("/").route(get().to(get::handle)));
}
