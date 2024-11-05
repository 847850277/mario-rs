//use http::Response;

use std::fmt;
use std::sync::Arc;

pub struct Response{

}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Customize the output as needed
        write!(f, "Response")
    }
}

pub trait handler {
    fn call(&self) -> Response;
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
