use std::sync::Arc;

use bytes::Bytes;
use http::Response;
use http_body_util::BodyExt;

pub type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody {
    http_body_util::Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

#[allow(non_camel_case_types)]
pub trait Handler {
    fn call(&self) -> Response<BoxBody>;
}

pub trait IntoResponse {
    fn into_response(self) -> Response<BoxBody>;
}

impl IntoResponse for String {
    fn into_response(self) -> Response<BoxBody> {
        Response::new(full(self))
    }
}

impl IntoResponse for i32 {
    fn into_response(self) -> Response<BoxBody> {
        Response::new(full(self.to_string()))
    }
}

pub struct Route {
    pub method: String,
    pub path: String,
    pub handler: Arc<dyn Handler>,
}

impl Route {
    pub fn new(method: String, path: String, handler: Arc<dyn Handler>) -> Self {
        Self {
            method,
            path,
            handler,
        }
    }
}
