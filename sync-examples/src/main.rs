use sync_core::server::Server;
use sync_core::service::Service;

fn main() {
    //trace log
    tracing_subscriber::fmt::init();
    let mut server = Server::new(Service::new());

    let route = sync_core::route::Route::new("GET".to_string(), "/hello".to_string(), || {
        "Hello World".to_string()
    });

    // hello world 2 return int value
    let route2 = sync_core::route::Route::new("GET".to_string(), "/hello2".to_string(), || {
        //i32 value return
        42.to_string()
    });

    // push route to server
    server.service.routes.push(route);
    server.service.routes.push(route2);

    server.start();
}
