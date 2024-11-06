//use http::Response;

use std::error::Error as StdError;
use std::sync::Arc;

use bytes::Bytes;
use http_body_util::BodyExt;
use http_body_util::combinators::UnsyncBoxBody;

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
        //let body = UnsyncBoxBody::new(crate::Empty::new().map_err(|err| match err {}));
        let body = UnsyncBoxBody::new(
            http_body_util::Full::new(self.into()).map_err(|_| unreachable!()),
        );
        Response::new(ResponseBody(body))
    }
}

impl IntoResponse for i32 {
    fn into_response(self) -> Response {
        Response::new(ResponseBody(Default::default()))
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
