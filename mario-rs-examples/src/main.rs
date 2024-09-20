use mario_core::mario_server::MarioServer;

#[tokio::main]
pub async fn main() {
    MarioServer::new().start().await;
}

