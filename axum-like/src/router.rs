use std::{
    borrow::Cow,
    convert::Infallible,
    fmt,
    future::ready,
    marker::PhantomData,
    sync::Arc,
    task::{Context, Poll},
};

use http::{Request, Response, StatusCode};
use tower::util::ServiceExt;
use tower_layer::Layer;
use tower_service::Service;

use crate::body::BoxBody;
use crate::router::empty_router::EmptyRouter;
use crate::router::route::{PathPattern, Route};

pub mod route;
pub mod empty_router;

pub mod future;
pub mod method_filter;

#[derive(Debug, Clone)]
pub struct Router<S>{
    svc: S,
}

impl<E> Router<EmptyRouter<E>> {
    // 创建一个新的路由，默认是 Not Found
    pub fn new() -> Self {
        Self {
            svc: EmptyRouter::not_found(),
        }
    }
}

impl<E> Default for Router<EmptyRouter<E>> {
    fn default() -> Self {
        Self::new()
    }
}


impl<S> Router<S> {
    pub fn route<T>(self, description: &str, svc: T) -> Router<Route<T, S>> {
        self.map(|fallback| Route {
            pattern: PathPattern::new(description),
            svc,
            fallback,
        })
    }

    fn map<F, S2>(self, f: F) -> Router<S2>
        where
            F: FnOnce(S) -> S2,
    {
        Router { svc: f(self.svc) }
    }


    pub fn into_make_service(self) -> IntoMakeService<S>
        where
            S: Clone,
    {
        IntoMakeService::new(self.svc)
    }

}

#[derive(Debug, Clone)]
pub struct IntoMakeService<S> {
    service: S,
}

impl<S> IntoMakeService<S> {
    fn new(service: S) -> Self {
        Self { service }
    }
}


impl<S, T> Service<T> for IntoMakeService<S>
    where
        S: Clone,
{
    type Response = S;
    type Error = Infallible;
    type Future = future::MakeRouteServiceFuture<S>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _target: T) -> Self::Future {
        future::MakeRouteServiceFuture {
            future: ready(Ok(self.service.clone())),
        }
    }
}

