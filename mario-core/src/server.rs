use std::convert::Infallible;

use bytes::Bytes;
use http::Request;
use http_body_util::Full;
use hyper::Response;
use hyper::service::service_fn;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto::Builder;
use tokio::net::{TcpListener, TcpStream};
use crate::route::request::Request as MarioRequest;
use crate::route::route::Route;
use crate::route::route_matcher::RouteMatcher;

pub struct Server {

}

impl Server {
    pub fn new() -> Server {
        Server {}
    }

    pub(crate) async fn start_server(&self) {
        //tokio web server bind port
        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

        println!("Server running on {}", listener.local_addr().unwrap());
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
    let matcher = RouteMatcher::new(vec![]);
    let route = matcher.match_route(request);
    Ok(Response::new(Full::new(Bytes::from("Hello World!"))))
}


pub async fn handle_connection(mut stream: TcpStream) {

    let tcp_stream = TokioIo::new(stream);
    tokio::spawn(async move {
        let builder = Builder::new(TokioExecutor::new());
        builder.serve_connection(tcp_stream, service_fn(dispatch)).await.unwrap();
    });

}

