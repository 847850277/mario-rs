use crate::route::error::Error;
use crate::route::request::Request;
use crate::route::response::Response;

pub trait Handler {
    fn handle(&self, req: Request) -> Result<Response, Error>;
}

pub struct MyHandler;

impl MyHandler {
    pub fn new() -> Self {
        Self
    }
}

impl Handler for MyHandler {
    fn handle(&self, req: Request) -> Result<Response, Error> {
        // Your implementation here
        Ok(Response::new("run my handler"))
    }
}