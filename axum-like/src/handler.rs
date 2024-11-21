use std::{
    convert::Infallible,
    fmt,
    future::Future,
    marker::PhantomData,
    task::{Context, Poll},
};

use async_trait::async_trait;
use bytes::Bytes;
use http::{Request, Response};
use tower::util::Oneshot;
use tower::ServiceExt;
use tower_service::Service;

use crate::extract::FromRequest;
use crate::response::IntoResponse;
use crate::router::method_filter::MethodFilter;
use crate::{
    body::{box_body, BoxBody},
    router::empty_router::EmptyRouter,
    util::Either,
    BoxError,
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

pub struct Layered<S, T> {
    svc: S,
    _input: PhantomData<fn() -> T>,
}

impl<S, T> fmt::Debug for Layered<S, T>
where
    S: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Layered").field("svc", &self.svc).finish()
    }
}

impl<S, T> Clone for Layered<S, T>
where
    S: Clone,
{
    fn clone(&self) -> Self {
        Self::new(self.svc.clone())
    }
}

#[async_trait]
impl<S, T, ReqBody, ResBody> Handler<ReqBody, T> for Layered<S, T>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Clone + Send + 'static,
    S::Error: IntoResponse,
    S::Future: Send,
    T: 'static,
    ReqBody: Send + 'static,
    ResBody: http_body::Body<Data = Bytes> + Send + Sync + 'static,
    ResBody::Error: Into<BoxError> + Send + Sync + 'static,
{
    type Sealed = sealed::Hidden;

    async fn call(self, req: Request<ReqBody>) -> Response<BoxBody> {
        match self
            .svc
            .oneshot(req)
            .await
            .map_err(IntoResponse::into_response)
        {
            Ok(res) => res.map(box_body),
            Err(res) => res.map(box_body),
        }
    }
}

impl<S, T> Layered<S, T> {
    pub(crate) fn new(svc: S) -> Self {
        Self {
            svc,
            _input: PhantomData,
        }
    }

    pub fn handle_error<F, ReqBody, ResBody, Res, E>(
        self,
        f: F,
    ) -> Layered<HandleError<S, F, ReqBody>, T>
    where
        S: Service<Request<ReqBody>, Response = Response<ResBody>>,
        F: FnOnce(S::Error) -> Result<Res, E>,
        Res: IntoResponse,
    {
        let svc = HandleError::new(self.svc, f);
        Layered::new(svc)
    }
}

/// A [`Service`] adapter that handles errors with a closure.
///
/// Created with
/// [`handler::Layered::handle_error`](crate::handler::Layered::handle_error) or
/// [`routing::Router::handle_error`](crate::routing::Router::handle_error).
/// See those methods for more details.
pub struct HandleError<S, F, B> {
    inner: S,
    f: F,
    _marker: PhantomData<fn() -> B>,
}

impl<S, F, B> Clone for HandleError<S, F, B>
where
    S: Clone,
    F: Clone,
{
    fn clone(&self) -> Self {
        Self::new(self.inner.clone(), self.f.clone())
    }
}

impl<S, F, B> HandleError<S, F, B> {
    pub(crate) fn new(inner: S, f: F) -> Self {
        Self {
            inner,
            f,
            _marker: PhantomData,
        }
    }
}

impl<S, F, B> fmt::Debug for HandleError<S, F, B>
where
    S: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HandleError")
            .field("inner", &self.inner)
            .field("f", &format_args!("{}", std::any::type_name::<F>()))
            .finish()
    }
}

impl<S, F, ReqBody, ResBody, Res, E> Service<Request<ReqBody>> for HandleError<S, F, ReqBody>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Clone,
    F: FnOnce(S::Error) -> Result<Res, E> + Clone,
    Res: IntoResponse,
    ResBody: http_body::Body<Data = Bytes> + Send + Sync + 'static,
    ResBody::Error: Into<BoxError> + Send + Sync + 'static,
{
    type Response = Response<BoxBody>;
    type Error = E;
    type Future = future::HandleErrorFuture<Oneshot<S, Request<ReqBody>>, F>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        println!("HandleError call");
        future::HandleErrorFuture {
            f: Some(self.f.clone()),
            inner: self.inner.clone().oneshot(req),
        }
    }
}
