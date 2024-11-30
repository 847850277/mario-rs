use crate::error::Error;
use crate::request::Request;
use crate::response::Response;
use std::fmt::Debug;

pub trait Endpoint: Debug + Sync + Send {
    fn call(&self, req: &Request) -> Result<Response<String>, Error>;
}
