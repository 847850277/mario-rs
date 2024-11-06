//use http::Response;

use std::sync::Arc;
use http_body_util::combinators::UnsyncBoxBody;
use std::error::Error as StdError;
use bytes::Bytes;

pub type Response<T = ResponseBody> = http::Response<T>;

pub type BoxError = Box<dyn StdError + Send + Sync>;

#[derive(Debug)]
pub struct ResponseBody(UnsyncBoxBody<Bytes, BoxError>);

pub trait handler {
    fn call(&self) -> Response;
}

pub trait IntoResponse{
    fn into_response(self) -> Response;
}

//impl String for IntoResponse

impl IntoResponse for String {
    fn into_response(self) -> Response {
        Response::builder()
            .header("Content-Type", "text/plain; charset=utf-8")
            .body(self.into())
            .unwrap()
    }
}

impl IntoResponse for i32 {
    fn into_response(self) -> Response {
        Response::builder()
            .header("Content-Type", "text/plain; charset=utf-8")
            .body(self.into())
            .unwrap()
    }
}


pub struct Route {
    pub method: String,
    pub path: String,
    pub handler: Arc<dyn handler>,
}

impl Route {
    pub fn new(method: String, path: String, handler: Arc<dyn handler>) -> Self {
        Self {
            method,
            path,
            handler,
        }
    }
}
