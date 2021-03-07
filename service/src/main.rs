#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use dotenv::dotenv;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

mod home;
mod http;
#[cfg(test)]
mod integration;
mod model;
mod server;
mod service;
mod settings;
mod users;

#[actix_rt::main]
async fn main() {
    dotenv().ok();

    env_logger::init();

    opentelemetry::global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let (tracer, _uninstall) = opentelemetry_jaeger::new_pipeline()
        .with_service_name(env!("CARGO_PKG_NAME"))
        .from_env()
        .install()
        .unwrap();
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = Registry::default().with(telemetry);
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let service = service::Service::new(settings::load()).await;
    service.start().await;
}
