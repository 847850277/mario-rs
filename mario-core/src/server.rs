use std::convert::Infallible;
use std::sync::Arc;

use crate::route::handler::MyHandler;
use crate::route::request::Request as MarioRequest;
use crate::route::route::Route;
use crate::route::route_matcher::RouteMatcher;
use bytes::Bytes;
use http::{Method, Request};
use http_body_util::Full;
use hyper::service::service_fn;
use hyper::Response;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto::Builder;
use tokio::net::{TcpListener, TcpStream};
use tracing::info;
use crate::route::service::Service;

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
        Server { routes: vec![] }
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
        info!(
            "Server running on http://{}",
            listener.local_addr().unwrap()
        );
        //let routes = Arc::new(self.get_routes());
        loop {
            let (stream, _) = listener.accept().await.unwrap();
            let routers = self.get_routes();
            let mut service = Service::new();
            service.set_router(routers);
            tokio::spawn(async move {
                handle_connection(stream,service).await;
            });
        }
    }
}

async fn dispatch(
    request: Request<hyper::body::Incoming>,
    service: Arc<Service>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let request = MarioRequest::new(request);
    let routes = service.get_router();
    let matcher = RouteMatcher::new(Arc::new(routes));
    let route = matcher.match_route(&request);
    match route {
        Some(route) => {
            info!("Route found: {:?}", route);
            let response = route.handler.handler(&request);
            //info!("Response: {:?}", response);
            match response {
                Ok(response) => {
                    let body = response.get_body().to_string();
                    return Ok(Response::new(Full::new(Bytes::from(body))));
                }
                Err(_) => {
                    return Ok(Response::new(Full::new(Bytes::from(
                        "500 Internal Server Error",
                    ))));
                }
            }
        }
        None => {
            return Ok(Response::new(Full::new(Bytes::from("404 Not Found"))));
        }
    }
}

pub async fn handle_connection(stream: TcpStream,service: Service) {
    let io = TokioIo::new(stream);
    tokio::spawn(async move {
        let builder = Builder::new(TokioExecutor::new());
        let service = Arc::new(service);
        builder
            .serve_connection(io, service_fn(|req| dispatch(req,service.clone())))
            .await
            .unwrap();
    });
}
