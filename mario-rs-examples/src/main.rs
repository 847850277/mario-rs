use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use log::info;

use mario_core::error::Error;
use mario_core::handler::Endpoint;
use mario_core::response::Response;
use mario_core::route::Router;
use mario_core::server::Server;
use mario_core::service::Service;
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
    let mut server = Server::new(Service::new());

    let mut router: Router = Router::new();

    router.get("/hello_world", Arc::new(ExampleHandler));

    router.get("/hello_world_2", Arc::new(hello));

    router.get("/hello_world_3", Arc::new(world));

    server.service.set_routes(router);

    server.start_server().await;
}
