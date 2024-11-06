
use std::io::Error as IoError;
use std::sync::Arc;

use bytes::Bytes;

pub(crate) type BoxBody = http_body_util::combinators::BoxBody<Bytes, IoError>;

/// A body object for requests and responses.
#[derive(Default, Debug)]
pub struct Body(pub(crate) BoxBody);


#[derive(Debug)]
pub struct Response {
    body: Body,
}

pub struct ResponseBuilder {
}

impl Response{
    pub fn builder() -> ResponseBuilder {
        ResponseBuilder {
        }
    }

    //new
    pub fn new(body: Body) -> Response {
        Response {
            body,
        }
    }


}

impl ResponseBuilder {
    pub fn body(self, body: impl Into<Body>) -> Response {
        Response {
            body: body.into(),
        }
    }
}

pub trait handler {
    fn call(&self) -> Response;
}

pub trait IntoResponse{
    fn into_response(self) -> Response;
}

//impl String for IntoResponse

impl IntoResponse for Body {
    fn into_response(self) -> Response {
        Response::builder().body(self)
    }
}

impl IntoResponse for String {
    fn into_response(self) -> Response {
        Response::builder()
            .body(self)
        //Response::new(1)
    }
}

impl IntoResponse for i32 {
    fn into_response(self) -> Response {
        //Response::new(ResponseBody(Default::default()))
        Response::builder()
            .body(self.to_string())
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
