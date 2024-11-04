use std::fmt::Debug;
use std::sync::Arc;

use http::Method;

use crate::handler::Endpoint;

pub struct Route {
    pub http_method: Method,
    pub path: String,
    pub handler: Arc<dyn Endpoint>,
}

impl Clone for Route {
    fn clone(&self) -> Self {
        Self {
            http_method: self.http_method.clone(),
            path: self.path.clone(),
            handler: self.handler.clone(),
        }
    }
}

impl Route {
    pub fn new(http_method: Method, path: String, handler: Arc<dyn Endpoint>) -> Self {
        Self {
            http_method,
            path,
            handler,
        }
    }
}

impl Debug for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //write!(f, "Route: {:?} {}", self.http_method, self.path)
        write!(
            f,
            "Route: {:?} {} {:?}",
            self.http_method, self.path, self.handler
        )
    }
}
