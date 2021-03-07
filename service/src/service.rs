#[cfg(test)]
pub mod testing;

use crate::settings::Settings;
use prometheus::Registry;

pub struct Service {
    server: crate::server::Server,
}

impl Service {
    pub async fn new(cfg: Settings) -> Self {
        tracing::debug!("Building New Landing");

        let prometheus = Registry::new();

        let home = crate::home::component::new().build();

        let server = crate::server::component::new()
            .with_routes(home)
            .build(cfg.port, prometheus);

        tracing::debug!("Built New Landing");

        Self {
            server: server.server,
        }
    }

    pub async fn start(self) {
        tracing::info!("Starting New Landing");
        self.server.start().await;
    }
}
