use sync_core::server::Server;
fn main() {
    //trace log
    tracing_subscriber::fmt::init();
    let mut server = Server::new();
    server.start();
}
