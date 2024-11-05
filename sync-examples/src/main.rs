use std::sync::Arc;
use sync_core::route::{handler, Response};
use sync_core::server::Server;
use sync_core::service::Service;

fn test_1() -> String{
    "Hello World".to_string()
}

fn test_2() -> i32{
    2
}


struct Test1;

impl handler for Test1 {
    fn call(&self) -> Response {
        //call test_1
        let string = test_1();
        //return response
        println!("{}", string);
        //TODO into Response
        Response{}
    }
}

struct Test2;

impl handler for Test2 {
    fn call(&self) -> Response {
        //call test_2
        let int = test_2();
        //return response
        println!("{}", int);
        //TODO into Response
        Response{}
    }
}



fn main() {
    //trace log
    tracing_subscriber::fmt::init();
    let mut server = Server::new(Service::new());

    let route = sync_core::route::Route::new("GET".to_string(), "/hello".to_string(), Arc::new(Test1)
    );
    // hello world 2 return int value
    let route2 = sync_core::route::Route::new("GET".to_string(), "/hello2".to_string(), Arc::new(Test2)
    );

    // push route to server
    server.service.routes.push(route);
    server.service.routes.push(route2);

    server.start();
}
