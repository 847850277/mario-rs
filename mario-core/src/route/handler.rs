use crate::route::error::Error;
use crate::route::request::Request;
use crate::route::response::Response;

pub trait Handler {
    fn handle(&self, req: Request) -> Result<Response, Error>;
}
