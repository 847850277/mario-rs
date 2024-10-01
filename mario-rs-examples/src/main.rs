use futures::executor;
use http::{Method, Request};
use log::info;
use mario_core::mario_server::MarioServer;
use mario_core::route::error::Error;
use mario_core::route::handler::{Endpoint, MyHandler};
use mario_core::route::response::Response;
use mario_core::route::route::Route;
use std::sync::Arc;
use warp::Filter;

use mario_macro::route;

// #[route("/test", method = "GET")]
async fn example() -> Response {
    Response::new("run example")
}

#[route("/test", method = "GET")]
async fn example_1() -> String {
    //Ok(Response::new("run example_1"))
    "run example_1".to_string()
}

#[derive(Debug)]
pub struct ExampleHandler;

impl ExampleHandler {
    pub fn new() -> Self {
        Self
    }
}

impl Endpoint for ExampleHandler {
    fn handler(&self, req: &mario_core::route::request::Request) -> Result<Response, Error> {
        // Your implementation here
        //Ok(Response::new("run example handler"))
        let fut = example_1();
        let response = executor::block_on(fut);
        Ok(Response::new(&response))
    }
}

#[tokio::main]
pub async fn main() {
    // init trace log
    tracing_subscriber::fmt::init();
    let response = example().await;
    info!("{:?}", response);
    let mut server = MarioServer::new();
    let handler = Arc::new(Box::new(ExampleHandler::new()) as Box<dyn Endpoint>);
    let route = Route::new(http::Method::GET, "/hello_world".to_string(), handler);
    server.server.bind_route(route);
    server.start().await;
}
