use crate::server::Server;

pub struct MarioServer {
    pub server: Server,
}

impl MarioServer {
    pub fn new() -> Self {
        Self {
            server: Server::new(),
        }
    }

    pub async fn start(&self) {
        self.server.start_server().await;
    }

}
