use crate::route::error::Error;
use crate::route::request::Request;
use crate::route::response::Response;
use std::fmt::Debug;

pub trait Endpoint: Debug + Sync + Send {
    //fn handle(&self, req: Request) -> Result<Response, Error>;
    fn handler(&self, req: &Request) -> Result<Response, Error>;
}

#[derive(Debug)]
pub struct MyHandler;

impl MyHandler {
    pub fn new() -> Self {
        Self
    }
}

impl Endpoint for MyHandler {
    fn handler(&self, req: &Request) -> Result<Response, Error> {
        // Your implementation here
        Ok(Response::new("run my handler"))
    }
}
