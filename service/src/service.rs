use prometheus::Registry;

pub struct Service {
    server: crate::server::Server,
}

impl Service {
    pub async fn new() -> Self {
        tracing::debug!("Building New Landing");

        let prometheus = Registry::new();

        let server = crate::server::component::new().build(prometheus);

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
