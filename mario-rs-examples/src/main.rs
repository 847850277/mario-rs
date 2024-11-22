use http_body_util::Full;
use log::info;
use mario_core::error::Error;
use mario_core::extra::{FailedToDeserializeQueryString, Query};
use mario_core::handler::Endpoint;
use mario_core::response::Response;
use mario_core::route::Router;
use mario_core::server::Server;
use mario_core::service::Service;
use mario_macro::handler;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use warp::hyper::body::Bytes;

async fn example() -> Response<String> {
    Response::new("run example".to_string())
}
fn example_1() -> String {
    "run example_1".to_string()
}
fn example_2<T: std::fmt::Display>(param: Query<T>) -> String {
    let string = format!("run example_2: {}", param.0);
    println!("{}", string);
    string
}

#[derive(Debug, Default)]
pub struct ExampleHandler;

impl Endpoint for ExampleHandler {
    fn call(&self, _req: &mario_core::request::Request) -> Result<Response<String>, Error> {
        let response = example_1();
        Ok(Response::new(response.to_string()))
    }
}

#[handler]
fn hello() -> i32 {
    2
}

#[handler]
fn world() -> String {
    "example_3".to_string()
}

#[derive(Debug, Default)]
pub struct ExtraExample;

impl Endpoint for ExtraExample {
    fn call(&self, req: &mario_core::request::Request) -> Result<Response<String>, Error> {
        let copy_req = Arc::new(req);
        let query = copy_req.head.uri.query().unwrap_or_default();
        info!("query: {:?}", query);
        let result = serde_urlencoded::from_str::<Vec<(String, String)>>(query);
        let result = match result {
            Ok(result) => result,
            Err(_) => return Ok(Response::new("500 Internal Server Error".to_string())),
        };
        if (result.len() > 0) {
            let response = example_2(Query(result[0].1.clone()));
            return Ok(Response::new(response.to_string()));
        }
        Ok(Response::new("not param".to_string()))
    }
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

    router.get("/hello_world_param", Arc::new(ExtraExample));

    router.get("/hello_world_2", Arc::new(hello));

    router.get("/hello_world_3", Arc::new(world));

    server.service.set_routes(router);

    server.start_server().await;
}
