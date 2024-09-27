use log::info;
use mario_core::mario_server::MarioServer;
use mario_core::route::response::Response;

use mario_macro::route;

#[route("/test", method = "GET")]
 async fn example() -> Response {
    //Ok()
    Response::new("run example")
 }

#[tokio::main]
pub async fn main() {
    // init trace log
    tracing_subscriber::fmt::init();
    let response = example().await;
    info!("{:?}", response);
    MarioServer::new()
        .start().await;
}

