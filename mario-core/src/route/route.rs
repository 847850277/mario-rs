use std::fmt::Debug;
use http::Method;
use crate::route::handler::Handler;

pub struct Route{
    pub http_method: Method,
    pub path: String,
    pub handler: Box<dyn Handler>,
}


impl Debug for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Route: {:?} {}", self.http_method, self.path)
    }
}
