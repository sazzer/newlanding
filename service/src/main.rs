#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use config::{Config, Environment};
use dotenv::dotenv;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

/// Main entry point for the entire application.
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

    let service = newlanding_service_lib::Service::new(load_settings()).await;
    service.start().await;
}

/// Load the application settings from the environment.
///
/// # Returns
/// The loaded settings.
fn load_settings() -> newlanding_service_lib::Settings {
    let mut s = Config::new();
    s.set_default("port", 8000)
        .expect("Failed to set default value for 'port'");

    s.merge(Environment::default())
        .expect("Failed to load environment properties");

    s.try_into().expect("Failed to build settings from config")
}
