use std::sync::Arc;

use http::Response;
use sync_core::route::{BoxBody, Handler, IntoResponse, Router};
use sync_core::server::Server;
use sync_core::service::Service;

fn test_1() -> String {
    "Hello World".to_string()
}

fn test_2() -> i32 {
    2
}

struct Test1;

impl Handler for Test1 {
    fn call(&self) -> Response<BoxBody> {
        let string = test_1();
        println!("{}", string);
        string.into_response()
    }
}

struct Test2;

impl Handler for Test2 {
    fn call(&self) -> Response<BoxBody> {
        let int = test_2();
        println!("{}", int);
        int.into_response()
    }
}

fn main() {
    //trace log
    tracing_subscriber::fmt::init();
    let mut server = Server::new(Service::new());
    let mut router: Router = Router::new();
    router.get("/hello", Arc::new(Test1));
    router.get("/hello2", Arc::new(Test2));
    // push route to server
    server.service.set_routes(router);
    server.start();
}
