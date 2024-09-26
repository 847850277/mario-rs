use mario_core::mario_server::MarioServer;

#[tokio::main]
pub async fn main() {

    // init trace log
    tracing_subscriber::fmt::init();

    MarioServer::new().start().await;
}

