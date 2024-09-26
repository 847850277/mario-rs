use std::convert::Infallible;

use bytes::Bytes;
use http::{Method, Request};
use http_body_util::Full;
use hyper::Response;
use hyper::service::service_fn;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto::Builder;
use tokio::net::{TcpListener, TcpStream};
use tracing::info;
use crate::route::handler::MyHandler;
use crate::route::request::Request as MarioRequest;
use crate::route::route::Route;
use crate::route::route_matcher::RouteMatcher;

pub struct Server {

}

macro_rules! route {
    ($method:expr, $path:expr, $handler:expr) => {
        Route {
            http_method: $method,
            path: String::from($path),
            handler: Box::new($handler),
        }
    };
}

impl Server {
    pub fn new() -> Server {
        Server {}
    }

    pub(crate) async fn start_server(&self) {
        //tokio web server bind port
        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

        println!("Server running on {}", listener.local_addr().unwrap());
        //TODO init routes

        loop {
            let (stream, _) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                handle_connection(stream).await;
            });
        }
    }
}


async fn dispatch(request: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    let request = MarioRequest::new(request);

    let matcher = RouteMatcher::new(vec![
        route!(Method::GET, "/hello_world", MyHandler::new()),
    ]);
    let route = matcher.match_route(&request);
    match route {
        Some(route) => {
            info!("Route found: {:?}", route);
            let response = route.handler.handler(&request);
            //info!("Response: {:?}", response);
            match response {
                Ok(response) => {
                    let body = response.body().to_string();
                    return Ok(Response::new(Full::new(Bytes::from(body))))
                },
                Err(_) => {
                    return Ok(Response::new(Full::new(Bytes::from("500 Internal Server Error"))));
                }
            }
        },
        None => {
            return Ok(Response::new(Full::new(Bytes::from("404 Not Found"))));
        }
    }
    Ok(Response::new(Full::new(Bytes::from("Hello World!"))))
}


pub async fn handle_connection(mut stream: TcpStream) {

    let tcp_stream = TokioIo::new(stream);
    tokio::spawn(async move {
        let builder = Builder::new(TokioExecutor::new());
        builder.serve_connection(tcp_stream, service_fn(dispatch)).await.unwrap();
    });

}

