use std::sync::Arc;

use http::Response;
use sync_core::route::{BoxBody, Handler, IntoResponse};
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

    let route =
        sync_core::route::Route::new("GET".to_string(), "/hello".to_string(), Arc::new(Test1));
    let route2 =
        sync_core::route::Route::new("GET".to_string(), "/hello2".to_string(), Arc::new(Test2));

    // push route to server
    server.service.routes.push(route);
    server.service.routes.push(route2);

    server.start();
}
