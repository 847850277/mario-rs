use std::collections::HashMap;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::Method;
use route_recognizer::{Match, Params, Router as InternalRouter};

use crate::error::Error;
use crate::handler::Endpoint;

pub struct RouterMatch<'a> {
    pub handler: &'a dyn Endpoint,
    pub params: Params,
}

#[derive(Debug)]
struct NotFound;

impl Endpoint for NotFound {
    fn call(
        &self,
        _req: &crate::request::Request,
    ) -> Pin<Box<dyn Future<Output = Result<crate::response::Response<String>, Error>> + Send>>
    {
        Box::pin(async move {
            let response = "Not Found";
            Ok(crate::response::Response::new(response.to_string()))
        })
    }
}

static NOT_FOUND: NotFound = NotFound;
#[derive(Default)]
pub struct Router {
    method_map: HashMap<Method, InternalRouter<Arc<dyn Endpoint>>>,
}

impl Clone for Router {
    fn clone(&self) -> Self {
        Self {
            method_map: self.method_map.clone(),
        }
    }
}

impl Router {
    pub fn new() -> Self {
        Self {
            method_map: HashMap::default(),
        }
    }

    pub fn get(&mut self, path: &str, handler: Arc<dyn Endpoint>) {
        self.method_map
            .entry(Method::GET)
            .or_insert_with(InternalRouter::new)
            .add(path, handler)
    }

    pub fn post(&mut self, path: &str, handler: Arc<dyn Endpoint>) {
        self.method_map
            .entry(Method::POST)
            .or_insert_with(InternalRouter::new)
            .add(path, handler)
    }
    pub fn route(&self, path: &str, method: &Method) -> RouterMatch<'_> {
        if let Some(Match { handler, params }) = self
            .method_map
            .get(method)
            .and_then(|r| r.recognize(path).ok())
        {
            RouterMatch {
                handler: &**handler,
                params,
            }
        } else {
            RouterMatch {
                handler: &NOT_FOUND,
                params: Params::new(),
            }
        }
    }
}
