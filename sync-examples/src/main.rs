use sync_core::server::Server;
use sync_core::service::Service;

fn main() {
    //trace log
    tracing_subscriber::fmt::init();
    let mut server = Server::new(Service::new());

    let route = sync_core::route::Route::new("GET".to_string(), "/hello".to_string(), || {
        println!("Hello World");
    });

    // push route to server
    server.service.routes.push(route);

    server.start();
}
