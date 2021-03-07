pub mod component;

pub struct Server {}

impl Server {
    pub async fn start(&self) {
        tracing::debug!("Starting HTTP server");
    }
}
