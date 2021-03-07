use super::Server;

pub struct Component {
    pub server: Server,
}

pub fn new() -> Component {
    tracing::debug!("Building HTTP Server component");
    Component { server: Server {} }
}
