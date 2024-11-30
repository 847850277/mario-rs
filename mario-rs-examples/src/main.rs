use http_body_util::{BodyExt, Full};
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
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;
use warp::hyper;
use warp::hyper::body;
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

fn example_3<T: std::fmt::Display>(param: Query<T>, param_1: Query<T>) -> String {
    let string = format!("run example_2: {},{}", param.0, param_1.0);
    println!("{}", string);
    string
}

fn example_4<T: std::fmt::Display>(
    param: Query<T>,
    param_1: Query<T>,
    param_2: Query<T>,
) -> String {
    let string = format!("run example_2: {},{},{}", param.0, param_1.0, param_2.0);
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
        if result.len() > 0 {
            let response = example_2(Query(result[0].1.clone()));
            return Ok(Response::new(response.to_string()));
        }
        Ok(Response::new("not param".to_string()))
    }
}

#[derive(Debug, Default)]
pub struct ExtraMultiExample;

impl Endpoint for ExtraMultiExample {
    fn call(&self, req: &mario_core::request::Request) -> Result<Response<String>, Error> {
        let copy_req = Arc::new(req);
        let query = copy_req.head.uri.query().unwrap_or_default();
        info!("query: {:?}", query);
        let result = serde_urlencoded::from_str::<Vec<(String, String)>>(query);
        let result = match result {
            Ok(result) => result,
            Err(_) => return Ok(Response::new("500 Internal Server Error".to_string())),
        };
        if result.len() >= 2 {
            let response = example_3(Query(result[0].1.clone()), Query(result[1].1.clone()));
            return Ok(Response::new(response.to_string()));
        }
        Ok(Response::new("not param".to_string()))
    }
}

#[derive(Debug, Default)]
pub struct ExtraMulti_2Example;

impl Endpoint for ExtraMulti_2Example {
    fn call(&self, req: &mario_core::request::Request) -> Result<Response<String>, Error> {
        let copy_req = Arc::new(req);
        let query = copy_req.head.uri.query().unwrap_or_default();
        info!("query: {:?}", query);
        let result = serde_urlencoded::from_str::<Vec<(String, String)>>(query);
        let result = match result {
            Ok(result) => result,
            Err(_) => return Ok(Response::new("500 Internal Server Error".to_string())),
        };
        if result.len() >= 3 {
            let response = example_4(
                Query(result[0].1.clone()),
                Query(result[1].1.clone()),
                Query(result[2].1.clone()),
            );
            return Ok(Response::new(response.to_string()));
        }
        Ok(Response::new("not param".to_string()))
    }
}

#[derive(Debug, Default)]
pub struct ExtraPostExample;

impl Endpoint for ExtraPostExample {
    fn call(&self, req: &mario_core::request::Request) -> Result<Response<String>, Error> {
        dbg!(req);
        let body = &req.body;
        dbg!(body);
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

    router.get("/hello_world_param_multi", Arc::new(ExtraMultiExample));

    router.get("/hello_world_param_multi_2", Arc::new(ExtraMulti_2Example));

    router.get("/hello_world_2", Arc::new(hello));

    router.get("/hello_world_3", Arc::new(world));

    router.post("/hello_world_param", Arc::new(ExtraPostExample));

    server.service.set_routes(router);

    server.start_server().await;
}
