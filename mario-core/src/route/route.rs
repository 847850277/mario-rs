use std::fmt::Debug;
use crate::route::handler::Handler;

pub struct Route{
    pub http_method: HttpMethod,
    pub path: String,
    pub handler: Box<dyn Handler>,
}


impl Debug for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Route: {:?} {}", self.http_method, self.path)
    }
}

#[derive(Debug, Default)]
pub enum HttpMethod {
    #[default]
    All,
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
    Trace,
    Connect,
    Options,
    Before,
    After,
}