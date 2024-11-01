use crate::route::error::Error;
use crate::route::request::Request;
use crate::route::response::Response;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;

pub trait Endpoint: Debug + Sync + Send {
    //fn handle(&self, req: Request) -> Result<Response, Error>;
    //fn call(&self, req: &Request) -> impl Future<Output=Result<Response<String>, Error>> + Send;
    fn call(
        &self,
        req: &Request,
    ) -> Pin<Box<dyn Future<Output = Result<Response<String>, Error>> + Send>>;
}

#[derive(Debug)]
pub struct MyHandler;

impl MyHandler {
    pub fn new() -> Self {
        Self
    }
}

// impl Endpoint for MyHandler {
// fn call(&self, req: &Request) -> impl Future<Output=Result<Response<String>, Error>> + Send {
//     async move {
//         Ok(Response::new("run my handler".to_string()))
//     }
// }
// fn call(&self, req: &Request) -> Result<Response<String>, Error> {
//     // Your implementation here
//     Ok(Response::new("run my handler".to_string()))
// }
// }

impl Endpoint for MyHandler {
    fn call(
        &self,
        req: &Request,
    ) -> Pin<Box<dyn Future<Output = Result<Response<String>, Error>> + Send>> {
        Box::pin(async move { Ok(Response::new("run my handler".to_string())) })
    }
}
