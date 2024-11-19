use crate::body::{box_body, BoxBody};
use crate::response::IntoResponse;
use crate::util::{Either, EitherProj};
use crate::BoxError;
use bytes::Bytes;
use futures_util::{
    future::{BoxFuture, Map},
    ready,
};
use http::{Method, Request, Response};
use http_body::Empty;
use pin_project_lite::pin_project;
use std::{
    convert::Infallible,
    fmt,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower::util::Oneshot;
use tower_service::Service;

pin_project! {
    /// Response future for [`HandleError`](super::HandleError).
    #[derive(Debug)]
    pub struct HandleErrorFuture<Fut, F> {
        #[pin]
        pub(super) inner: Fut,
        pub(super) f: Option<F>,
    }
}

impl<Fut, F, E, E2, B, Res> Future for HandleErrorFuture<Fut, F>
where
    Fut: Future<Output = Result<Response<B>, E>>,
    F: FnOnce(E) -> Result<Res, E2>,
    Res: IntoResponse,
    B: http_body::Body<Data = Bytes> + Send + Sync + 'static,
    B::Error: Into<BoxError> + Send + Sync + 'static,
{
    type Output = Result<Response<BoxBody>, E2>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        match ready!(this.inner.poll(cx)) {
            Ok(res) => Ok(res.map(box_body)).into(),
            Err(err) => {
                let f = this.f.take().unwrap();
                match f(err) {
                    Ok(res) => Ok(res.into_response().map(box_body)).into(),
                    Err(err) => Err(err).into(),
                }
            }
        }
    }
}

pin_project! {
    /// The response future for [`OnMethod`](super::OnMethod).
    pub struct OnMethodFuture<F, B>
    where
        F: Service<Request<B>>
    {
        #[pin]
        pub(super) inner: Either<
            BoxFuture<'static, Response<BoxBody>>,
            Oneshot<F, Request<B>>,
        >,
        pub(super) req_method: Method,
    }
}

impl<F, B> Future for OnMethodFuture<F, B>
where
    F: Service<Request<B>, Response = Response<BoxBody>>,
{
    type Output = Result<Response<BoxBody>, F::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let response = match this.inner.project() {
            EitherProj::A { inner } => ready!(inner.poll(cx)),
            EitherProj::B { inner } => ready!(inner.poll(cx))?,
        };

        if this.req_method == &Method::HEAD {
            let response = response.map(|_| box_body(Empty::new()));
            Poll::Ready(Ok(response))
        } else {
            Poll::Ready(Ok(response))
        }
    }
}

impl<F, B> fmt::Debug for OnMethodFuture<F, B>
where
    F: Service<Request<B>>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OnMethodFuture").finish()
    }
}

opaque_future! {
    /// The response future for [`IntoService`](super::IntoService).
    pub type IntoServiceFuture =
        Map<
            BoxFuture<'static, Response<BoxBody>>,
            fn(Response<BoxBody>) -> Result<Response<BoxBody>, Infallible>,
        >;
}
