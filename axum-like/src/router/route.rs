// 路由匹配： 基于 service
#[derive(Debug, Clone)]
pub struct Route<S, F> {
    pub(crate) pattern: PathPattern,
    pub(crate) svc: S,      // service
    pub(crate) fallback: F, // 回退机制，比如回退到 404，或者支持 SPA应用
}


// B 代表 Body
impl<S, F, B> Service<Request<B>> for Route<S, F>
    where
        S: Service<Request<B>, Response = Response<BoxBody>> + Clone,
        F: Service<Request<B>, Response = Response<BoxBody>, Error = S::Error> + Clone,
        B: Send + Sync + 'static,
{
    type Response = Response<BoxBody>;
    type Error = S::Error;
    type Future = RouteFuture<S, F, B>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: Request<B>) -> Self::Future {
        todo!()
    }
}