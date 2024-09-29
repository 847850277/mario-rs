use http::{Method, Request};
use log::info;
use mario_core::mario_server::MarioServer;
use mario_core::route::handler::{Handler, MyHandler};
use mario_core::route::response::Response;
use mario_core::route::route::Route;
use std::sync::Arc;

use mario_macro::route;

#[route("/test", method = "GET")]
async fn example() -> Response {
    Response::new("run example")
}

#[tokio::main]
pub async fn main() {
    // init trace log
    tracing_subscriber::fmt::init();
    let response = example().await;
    info!("{:?}", response);
    let mut server = MarioServer::new();
    let handler = Arc::new(Box::new(MyHandler::new()) as Box<dyn Handler>);
    let route = Route::new(http::Method::GET, "/hello_world".to_string(), handler);
    server.server.bind_route(route);
    server.start().await;
}
