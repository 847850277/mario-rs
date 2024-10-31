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

use mario_macro::handler;

macro_rules! create_handler {
    ($handler_type:ty) => {
        Arc::new(<$handler_type>::new())
    };
}

// #[route("/test", method = "GET")]
async fn example() -> Response<String> {
    Response::new("run example".to_string())
}

//#[route("/test", method = "GET")]
// async fn example_1() -> String {
//     //Ok(Response::new("run example_1"))
//     "run example_1".to_string()
// }

//#[route("/test", method = "GET")]
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
    fn handler(
        &self,
        req: &mario_core::route::request::Request,
    ) -> Result<Response<String>, Error> {
        // Your implementation here
        //Ok(Response::new("run example handler"))
        async fn example_1() -> i32 {
            //Ok(Response::new("run example_1"))
            //"run example_1".to_string()
            return 1;
        }
        let fut = example_1();
        let response = executor::block_on(fut);
        Ok(Response::new(response.to_string()))
    }
}

// #[derive(Debug)]
// struct example_2;
//
//
// impl example_2 {
//     pub fn new() -> Self {
//         Self
//     }
// }
//
// impl Endpoint for example_2 {
//     fn handler(&self, req: &mario_core::route::request::Request) -> Result<Response, Error> {
//         // Your implementation here
//         //Ok(Response::new("run example handler"))
//         async fn example_1() -> String {
//             //Ok(Response::new("run example_1"))
//             "run example_2".to_string()
//         }
//         let fut = example_1();
//         let response = executor::block_on(fut);
//         Ok(Response::new(&response))
//     }
// }

#[handler]
async fn example_999() -> i32 {
    //Ok(Response::new("run example_1"))
    // info about the request can be accessed via the `req` parameter
    // info hello world
    let a = 2;
    // info!("hello world");
    // "run example_99999".to_string()
    return a;
}

#[handler]
async fn example_1000() -> String {
    return "example_1000".to_string();
}

#[tokio::main]
pub async fn main() {
    // init trace log
    tracing_subscriber::fmt::init();
    let response = example().await;
    let mut server = MarioServer::new();
    let handler = Arc::new(ExampleHandler::new());
    //let handler = create_handler!(ExampleHandler);
    let route = Route::new(http::Method::GET, "/hello_world".to_string(), handler);

    let handler_1 = create_handler!(example_999);
    let route_1 = Route::new(http::Method::GET, "/hello_world_1".to_string(), handler_1);

    let handler_2 = create_handler!(example_1000);
    let route_2 = Route::new(http::Method::GET, "/hello_world_2".to_string(), handler_2);

    server.server.bind_route(route);
    server.server.bind_route(route_1);
    server.server.bind_route(route_2);
    server.start().await;
}
