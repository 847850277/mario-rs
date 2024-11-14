use std::task::{Context, Poll};

mod route;
mod empty_router;

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

// 为 Router 实现 Service
impl<S, R> Service<R> for Router<S>
    where
        S: Service<R>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.svc.poll_ready(cx)
    }

    #[inline]
    fn call(&mut self, req: R) -> Self::Future {
        self.svc.call(req)
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

}
