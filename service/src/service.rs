#[cfg(test)]
pub mod testing;

use crate::settings::Settings;
use prometheus::Registry;

/// The complete New Landing service.
pub struct Service {
    /// The HTTP Server.
    server: crate::server::Server,
}

impl Service {
    /// Construct a new instance of the service.
    ///
    /// # Parameters
    /// - `cfg` - The configuration settings for the service
    ///
    /// # Returns
    /// The service itself.
    pub async fn new(cfg: Settings) -> Self {
        tracing::debug!("Building New Landing");

        let prometheus = Registry::new();

        let authentication =
            crate::authorization::component::new(&cfg.auth0_domain, &cfg.auth0_audience);
        let users = crate::users::component::new(
            &cfg.auth0_domain,
            &cfg.auth0_client_id,
            &cfg.auth0_client_secret,
        );
        let home = crate::home::component::new().build();

        let server = crate::server::component::new()
            .with_routes(home)
            .with_routes(users)
            .with_routes(authentication)
            .build(cfg.port, prometheus);

        tracing::debug!("Built New Landing");

        Self {
            server: server.server,
        }
    }

    /// Start the service running.
    pub async fn start(self) {
        tracing::info!("Starting New Landing");
        self.server.start().await;
    }
}
