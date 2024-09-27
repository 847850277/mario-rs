use std::convert::Infallible;
use std::sync::Arc;

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
    pub routes: Vec<Route>,
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
        Server {
            routes: vec![],
        }
    }

    pub fn bind_route(&mut self, route: Route) {
        self.routes.push(route);
    }

    pub fn get_routes(&self) -> Vec<Route> {
        self.routes.clone()
    }

    pub(crate) async fn start_server(&self) {
        //tokio web server bind port
        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
        info!("Server running on {}", listener.local_addr().unwrap());
        let routes = Arc::new(self.get_routes());
        //TODO init routes
        loop {
            let (stream, _) = listener.accept().await.unwrap();
            //let routes = self.get_routes();
            let routes = Arc::clone(&routes);
            tokio::spawn(async move {
                handle_connection(routes,stream).await;
            });
        }
    }
}


async fn dispatch(routes: Arc<Vec<Route>>,request: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    let request = MarioRequest::new(request);

    // let matcher = RouteMatcher::new(vec![
    //     route!(Method::GET, "/hello_world", MyHandler::new()),
    // ]);
    let matcher = RouteMatcher::new(routes);
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


pub async fn handle_connection(routes: Arc<Vec<Route>>,stream: TcpStream) {

    let tcp_stream = TokioIo::new(stream);
    tokio::spawn(async move {
        let builder = Builder::new(TokioExecutor::new());
        builder.serve_connection(tcp_stream, service_fn(|req|dispatch(Arc::clone(&routes),req))).await.unwrap();
    });

}

