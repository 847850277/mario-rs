
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

    pub fn new_empty() -> Response {
        Response {
            body: Default::default(),
        }
    }

    // set body
    pub fn set_body(&mut self, body: Body) {
        self.body = body;
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
        // Response::builder()
        //     .body(Body::from(self))
        //Response::new(1)
        let mut response = Response::new_empty();
        println!("{}", self);
        // sef to body
        let body = Body::new(self);
        response.set_body(body);
        return response;
    }
}

impl IntoResponse for i32 {
    fn into_response(self) -> Response {
        let mut response = Response::new_empty();
        println!("{}", self);
        return response;
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
