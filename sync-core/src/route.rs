use std::collections::HashMap;
use std::sync::Arc;

use bytes::Bytes;
use http::{Method, Response};
use http_body_util::BodyExt;
use route_recognizer::{Match, Params, Router as InternalRouter};

pub type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody {
    http_body_util::Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

#[allow(non_camel_case_types)]
pub trait Handler {
    fn call(&self) -> Response<BoxBody>;
}

struct NotFound;

impl Handler for NotFound {
    fn call(&self) -> Response<BoxBody> {
        let string = "not found".to_string();
        println!("{}", string);
        string.into_response()
    }
}

pub trait IntoResponse {
    fn into_response(self) -> Response<BoxBody>;
}

impl IntoResponse for String {
    fn into_response(self) -> Response<BoxBody> {
        Response::new(full(self))
    }
}

impl IntoResponse for i32 {
    fn into_response(self) -> Response<BoxBody> {
        Response::new(full(self.to_string()))
    }
}

pub struct RouterMatch<'a> {
    pub handler: &'a dyn Handler,
    pub params: Params,
}

#[derive(Default)]
pub struct Router {
    method_map: HashMap<Method, InternalRouter<Arc<dyn Handler>>>,
}

static NOT_FOUND: NotFound = NotFound;

impl Router {
    pub fn new() -> Self {
        Self {
            method_map: HashMap::default(),
        }
    }

    pub fn get(&mut self, path: &str, handler: Arc<dyn Handler>) {
        self.method_map
            .entry(Method::GET)
            .or_insert_with(InternalRouter::new)
            .add(path, handler)
    }

    pub fn post(&mut self, path: &str, handler: Arc<dyn Handler>) {
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
