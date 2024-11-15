use std::future::Future;
use std::net::SocketAddr;

use axum_like::{handler::get,handler::post, Router};
use axum_like::handler::put;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/post",post(post_handler))
        .route("/put",put(put_handler))
        ;

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum_like::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "<h1>Hello, World!</h1>"
}
async fn post_handler() -> &'static str {
    "<h1> Post Hello, World!</h1>"
}

async fn put_handler() -> &'static str {
    "<h1> Put Hello, World!</h1>"
}
