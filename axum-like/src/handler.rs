use std::{
    convert::Infallible,
    future::Future,
    marker::PhantomData,
    task::{Context, Poll},
};

use async_trait::async_trait;
use http::{Request, Response};
use tower::ServiceExt;
use tower_service::Service;

use crate::extract::FromRequest;
use crate::response::IntoResponse;
use crate::router::method_filter::MethodFilter;
use crate::{
    body::{box_body, BoxBody},
    router::empty_router::EmptyRouter,
    util::Either,
};

mod future;

pub struct OnMethod<H, B, T, F> {
    pub(crate) method: MethodFilter,
    pub(crate) handler: H,
    pub(crate) fallback: F,
    pub(crate) _marker: PhantomData<fn() -> (B, T)>,
}

impl<H, B, T, F> Clone for OnMethod<H, B, T, F>
where
    H: Clone,
    F: Clone,
{
    fn clone(&self) -> Self {
        Self {
            method: self.method,
            handler: self.handler.clone(),
            fallback: self.fallback.clone(),
            _marker: PhantomData,
        }
    }
}

// 这个是什么意思?
pub fn on<H, B, T>(method: MethodFilter, handler: H) -> OnMethod<H, B, T, EmptyRouter>
where
    H: Handler<B, T>,
{
    //println!("on, method: {:?}, handler: {:?}", method, handler);
    OnMethod {
        method,
        handler,
        fallback: EmptyRouter::method_not_allowed(),
        _marker: PhantomData,
    }
}

pub fn get<H, B, T>(handler: H) -> OnMethod<H, B, T, EmptyRouter>
where
    H: Handler<B, T>,
{
    on(MethodFilter::GET | MethodFilter::HEAD, handler)
}

pub fn post<H, B, T>(handler: H) -> OnMethod<H, B, T, EmptyRouter>
where
    H: Handler<B, T>,
{
    on(MethodFilter::POST, handler)
}

pub fn put<H, B, T>(handler: H) -> OnMethod<H, B, T, EmptyRouter>
where
    H: Handler<B, T>,
{
    on(MethodFilter::PUT, handler)
}

pub(crate) mod sealed {
    #![allow(unreachable_pub, missing_docs, missing_debug_implementations)]

    pub trait HiddentTrait {}
    pub struct Hidden;
    impl HiddentTrait for Hidden {}
}

#[async_trait]
pub trait Handler<B, T>: Clone + Send + Sized + 'static {
    // This seals the trait. We cannot use the regular "sealed super trait"
    // approach due to coherence.
    #[doc(hidden)]
    type Sealed: sealed::HiddentTrait;

    /// Call the handler with the given request.
    async fn call(self, req: Request<B>) -> Response<BoxBody>;
}

// 异步的方法实现handler，如example里面的handler方法
#[async_trait]
impl<F, Fut, Res, B> Handler<B, ()> for F
where
    F: FnOnce() -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Res> + Send,
    Res: IntoResponse,
    B: Send + 'static,
{
    type Sealed = sealed::Hidden;

    async fn call(self, _req: Request<B>) -> Response<BoxBody> {
        let res = self().await;
        res.into_response().map(box_body)
    }
}

// 异步的方法实现一个参数handler，如example里面的page_handler方法
#[async_trait]
impl<F, Fut, Res, B, T> Handler<B, (T,)> for F
where
    F: FnOnce(T) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Res> + Send,
    Res: IntoResponse,
    T: crate::extract::FromRequest<B> + Send,
    B: Send + 'static,
{
    type Sealed = sealed::Hidden;

    async fn call(self, req: Request<B>) -> Response<BoxBody> {
        //let res = self().await;
        //res.into_response().map(box_body)
        let mut req = crate::extract::RequestParts::new(req);
        let value = match T::from_request(&mut req).await {
            Ok(value) => value,
            Err(rejection) => return rejection.into_response().map(box_body),
        };
        self(value).await.into_response().map(box_body)
    }
}

// 用宏实现多参数的支持
// extract 支持
// macro_rules! impl_handler {
//     () => {
//     };
//
//     ( $head:ident, $($tail:ident),* $(,)? ) => {
//         #[async_trait]
//         #[allow(non_snake_case)]
//         impl<F, Fut, B, Res, $head, $($tail,)*> Handler<B, ($head, $($tail,)*)> for F
//         where
//             F: FnOnce($head, $($tail,)*) -> Fut + Clone + Send + Sync + 'static,
//             Fut: Future<Output = Res> + Send,
//             B: Send + 'static,
//             Res: IntoResponse,
//             B: Send + 'static,
//             $head: FromRequest<B> + Send,
//             $( $tail: FromRequest<B> + Send,)*
//         {
//             type Sealed = sealed::Hidden;
//
//             async fn call(self, req: Request<B>) -> Response<BoxBody> {
//                 let mut req = crate::extract::RequestParts::new(req);
//
//                 let $head = match $head::from_request(&mut req).await {
//                     Ok(value) => value,
//                     Err(rejection) => return rejection.into_response().map(box_body),
//                 };
//
//                 $(
//                     let $tail = match $tail::from_request(&mut req).await {
//                         Ok(value) => value,
//                         Err(rejection) => return rejection.into_response().map(box_body),
//                     };
//                 )*
//
//                 let res = self($head, $($tail,)*).await;
//
//                 res.into_response().map(crate::body::box_body)
//             }
//         }
//
//         impl_handler!($($tail,)*);
//     };
// }
//
// impl_handler!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);

// OnMethod 实现 Service
impl<H, B, T, F> Service<Request<B>> for OnMethod<H, B, T, F>
where
    H: Handler<B, T>,
    F: Service<Request<B>, Response = Response<BoxBody>, Error = Infallible> + Clone,
    B: Send + 'static,
{
    type Response = Response<BoxBody>;
    type Error = Infallible;
    type Future = future::OnMethodFuture<F, B>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let req_method = req.method().clone();

        let fut = if self.method.matches(req.method()) {
            let fut = Handler::call(self.handler.clone(), req);
            Either::A { inner: fut }
        } else {
            let fut = self.fallback.clone().oneshot(req);
            Either::B { inner: fut }
        };

        future::OnMethodFuture {
            inner: fut,
            req_method,
        }
    }
}
