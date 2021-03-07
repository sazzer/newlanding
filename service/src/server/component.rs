use super::Server;

pub struct Component {
    pub server: Server,
}

pub fn new(prometheus: prometheus::Registry) -> Component {
    tracing::debug!("Building HTTP Server component");
    Component {
        server: Server {
            port: 8000,
            prometheus,
        },
    }
}
