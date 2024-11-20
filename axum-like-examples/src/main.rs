use http::header::USER_AGENT;
use http::{HeaderValue, StatusCode};
use std::convert::Infallible;
use std::future::Future;
use std::net::SocketAddr;

use axum_like::extract::{Body, Query, TypedHeader};
use axum_like::handler::put;
use axum_like::{handler::get, handler::post, response::IntoResponse, BoxError, Router};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/post", post(post_handler))
        .route("/put", put(put_handler))
        .route("/page", get(page_handler))
        .layer(SetRequestHeaderLayer::<_, Body>::overriding(
            USER_AGENT,
            HeaderValue::from_static("axum-like demo"),
        ));

    // handler error
    let app = app.handle_error(|error: Infallible| {
        Ok::<_, Infallible>((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unhandled internal error".to_string(),
        ))
    });

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum_like::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// async fn handler() -> &'static str {
//     "<h1>Hello, World!</h1>"
// }

async fn handler(user_agent: Option<TypedHeader<headers::UserAgent>>) -> impl IntoResponse {
    let url = "localhost";
    if let Some(TypedHeader(user_agent)) = user_agent {
        println!(
            "Got a connection! url: {}, content_type: {:?}",
            url,
            user_agent.as_str()
        );
    }

    let res = "<h1>Hello, World!</h1>".into_response();
    println!(
        "Got a response! url: {}, content_type: {:?}",
        url,
        res.headers().get(USER_AGENT)
    );
    res
}

async fn post_handler() -> &'static str {
    // parse int
    // let s = "sss";
    // let i = s.parse::<i32>().unwrap();
    // println!("i: {}", i);
    "<h1> Post Hello, World!</h1>"
}

async fn put_handler() -> &'static str {
    "<h1> Put Hello, World!</h1>"
}

use serde::Deserialize;
use tower_http::set_header::SetRequestHeaderLayer;

#[derive(Deserialize, Debug)]
struct Pagination {
    page: usize,
    per_page: usize,
}

async fn page_handler(pagination: Query<Pagination>) -> &'static str {
    let url = "localhost";
    let pagination: Pagination = pagination.0;

    println!("{:?}", pagination);

    "<h1>Hello, World!</h1>"
}
