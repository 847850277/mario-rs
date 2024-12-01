use crate::error::Error;
use crate::response::Response;
use http::Request;
use hyper::body::Incoming;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;

pub trait Endpoint: Debug + Sync + Send {
    //fn call(&self, req: Request<Incoming>) -> Result<Response<String>, Error>;

    fn call(
        &self,
        req: Request<Incoming>,
    ) -> Pin<Box<dyn Future<Output = Result<Response<String>, Error>> + Send>>;
}
