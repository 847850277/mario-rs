use crate::error::Error;
use crate::request::Request;
use crate::response::Response;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;

pub trait Endpoint: Debug + Sync + Send {
    fn call(&self, req: &Request) -> Result<Response<String>, Error>;
}
