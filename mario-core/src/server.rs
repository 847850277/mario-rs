use std::convert::Infallible;
use std::sync::Arc;

use bytes::Bytes;
use http::Request;
use http_body_util::Full;
use hyper::service::service_fn;
use hyper::Response;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto::Builder;
use tokio::net::{TcpListener, TcpStream};
use tracing::info;

use crate::route::Router;
use crate::service::Service;

#[derive(Default)]
pub struct Server {
    pub service: Service,
}

impl Server {
    pub fn new(service: Service) -> Self {
        Self { service }
    }

    pub async fn start_server(&self) {
        //tokio web server bind port
        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
        info!(
            "Server running on http://{}",
            listener.local_addr().unwrap()
        );
        //let routes = Arc::new(self.get_routes());
        loop {
            let (stream, _) = listener.accept().await.unwrap();
            //let routers = self.get_routes();
            //let mut service = Service::new();
            //service.set_router(routers);
            let service1 = self.service.clone();
            tokio::spawn(async move {
                handle_connection(stream, service1).await;
            });
        }
    }
}

async fn dispatch(
    request: Request<hyper::body::Incoming>,
    service: Arc<Service>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let router = service.get_routes();
    let router_match = router.route(request.uri().path(), request.method());
    let response = router_match.handler.call(request).await;
    println!("test");
    match response {
        Ok(response) => {
            let body = response.get_body().to_string();
            Ok(Response::new(Full::new(Bytes::from(body))))
        }
        Err(_) => Ok(Response::new(Full::new(Bytes::from(
            "500 Internal Server Error",
        )))),
    }
}

pub async fn handle_connection(stream: TcpStream, service: Service) {
    let io = TokioIo::new(stream);
    tokio::spawn(async move {
        let builder = Builder::new(TokioExecutor::new());
        let service = Arc::new(service);
        builder
            .serve_connection(io, service_fn(|req| dispatch(req, service.clone())))
            .await
            .unwrap();
    });
}
