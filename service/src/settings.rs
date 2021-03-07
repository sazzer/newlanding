use config::{Config, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub port: u16,
}

pub fn load() -> Settings {
    let mut s = Config::new();
    s.set_default("port", 8000)
        .expect("Failed to set default value for 'port'");

    s.merge(Environment::default())
        .expect("Failed to load environment properties");

    s.try_into().expect("Failed to build settings from config")
}
