use serde::Deserialize;

/// The actual settings as loaded from the environment.
#[derive(Debug, Deserialize)]
pub struct Settings {
    pub port: u16,
    pub auth0_domain: String,
    pub auth0_client_id: String,
    pub auth0_client_secret: String,
}
