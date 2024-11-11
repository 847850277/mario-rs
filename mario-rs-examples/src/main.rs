use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use log::info;

use mario_core::error::Error;
use mario_core::handler::Endpoint;
use mario_core::response::Response;
use mario_core::route::Route;
use mario_core::server::Server;
use mario_macro::handler;

async fn example() -> Response<String> {
    Response::new("run example".to_string())
}
async fn example_1() -> String {
    "run example_1".to_string()
}

#[derive(Debug, Default)]
pub struct ExampleHandler;

impl Endpoint for ExampleHandler {
    fn call(
        &self,
        _req: &mario_core::request::Request,
    ) -> Pin<Box<dyn Future<Output = Result<Response<String>, Error>> + Send>> {
        Box::pin(async move {
            let response = example_1().await;
            Ok(Response::new(response.to_string()))
        })
    }
}

#[handler]
async fn hello() -> i32 {
    2
}

#[handler]
async fn world() -> String {
    "example_3".to_string()
}

#[tokio::main]
pub async fn main() {
    // init trace log
    tracing_subscriber::fmt::init();
    let response = example().await;
    info!("Response: {:?}", response);
    let mut server = Server::new();
    let handler = Arc::new(ExampleHandler);
    //let handler = create_handler!(ExampleHandler);
    let route = Route::new(http::Method::GET, "/hello_world".to_string(), handler);

    let handler_1 = Arc::new(hello);
    let route_1 = Route::new(http::Method::GET, "/hello_world_2".to_string(), handler_1);

    let handler_2 = Arc::new(world);
    let route_2 = Route::new(http::Method::GET, "/hello_world_3".to_string(), handler_2);

    server.bind_route(route);
    // server.bind_route(route_1);
    // server.bind_route(route_2);

    server.start_server().await;
}
