use std::fmt::Debug;
use crate::route::error::Error;
use crate::route::request::Request;
use crate::route::response::Response;

pub trait Handler: Debug + Sync + Send  {
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


impl Handler for MyHandler {
    fn handler(&self, req: &Request) -> Result<Response, Error> {
        // Your implementation here
        Ok(Response::new("run my handler"))
    }

}