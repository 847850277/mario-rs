use ::hyper::body::Incoming;
use http::Request;
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
use tokio::runtime::Runtime;
use warp::body::bytes;
use warp::hyper;
use warp::hyper::body::HttpBody;
use warp::hyper::body::{to_bytes, Bytes};
use warp::hyper::{body, Body};

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
    fn call(
        &self,
        _req: Request<Incoming>,
    ) -> Pin<Box<dyn Future<Output = Result<Response<String>, Error>> + Send>> {
        Box::pin(async move {
            let response = example_1();
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

#[derive(Debug, Default)]
pub struct ExtraExample;

impl Endpoint for ExtraExample {
    fn call(
        &self,
        req: Request<Incoming>,
    ) -> Pin<Box<dyn Future<Output = Result<Response<String>, Error>> + Send>> {
        Box::pin(async move {
            let query = req.uri().query().unwrap_or_default();
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
        })
    }
}

#[derive(Debug, Default)]
pub struct ExtraMultiExample;

impl Endpoint for ExtraMultiExample {
    fn call(
        &self,
        req: Request<Incoming>,
    ) -> Pin<Box<dyn Future<Output = Result<Response<String>, Error>> + Send>> {
        Box::pin(async move {
            let query = req.uri().query().unwrap_or_default();
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
        })
    }
}

#[derive(Debug, Default)]
pub struct ExtraMulti_2Example;

impl Endpoint for ExtraMulti_2Example {
    fn call(
        &self,
        req: Request<Incoming>,
    ) -> Pin<Box<dyn Future<Output = Result<Response<String>, Error>> + Send>> {
        Box::pin(async move {
            let query = req.uri().query().unwrap_or_default();
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
        })
    }
}

#[derive(Debug, Default)]
pub struct ExtraPostExample;

impl Endpoint for ExtraPostExample {
    fn call(
        &self,
        req: Request<Incoming>,
    ) -> Pin<Box<dyn Future<Output = Result<Response<String>, Error>> + Send>> {
        Box::pin(async move {
            // Incoming to Body
            //let req = convert_request(req);
            let body = req.into_body();
            let collect = body.collect();
            let body = collect.await;
            let body = match body {
                Ok(body) => body,
                Err(_) => return Ok(Response::new("500 Internal Server Error".to_string())),
            };
            //println!("body: {:?}", body);
            let response = format!("body: {:?}", body);
            Ok(Response::new(response))
        })
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
