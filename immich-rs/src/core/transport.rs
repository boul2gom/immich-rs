//! Base HTTP transport and sync service box wrappers.

use std::task::{Context, Poll};

use tower_service::Service;

use crate::BoxFuture;
use crate::error::Error;

/// Base Tower [`Service`] that delivers requests using an underlying [`reqwest::Client`].
#[derive(Clone, Debug)]
pub(crate) struct ReqwestTransport {
    client: reqwest::Client,
}

impl ReqwestTransport {
    /// Creates a new `ReqwestTransport` wrapping the given client.
    #[must_use]
    pub(crate) const fn new(client: reqwest::Client) -> Self {
        Self { client }
    }
}

impl Service<reqwest::Request> for ReqwestTransport {
    type Response = reqwest::Response;
    type Error = Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: reqwest::Request) -> Self::Future {
        let client = self.client.clone();
        Box::pin(async move { client.execute(req).await.map_err(Error::Transport) })
    }
}

trait CloneSyncService<Req>: Service<Req> {
    fn clone_box(
        &self,
    ) -> Box<
        dyn CloneSyncService<Req, Response = Self::Response, Error = Self::Error, Future = Self::Future> + Send + Sync,
    >;
}

impl<Req, S> CloneSyncService<Req> for S
where
    S: Service<Req> + Clone + Send + Sync + 'static,
{
    fn clone_box(
        &self,
    ) -> Box<
        dyn CloneSyncService<Req, Response = Self::Response, Error = Self::Error, Future = Self::Future> + Send + Sync,
    > {
        Box::new(self.clone())
    }
}

/// A boxed, cloneable, thread-safe (`Send + Sync`) Tower service compatible with WebAssembly futures.
pub struct BoxCloneSyncService<Req, Res, Err> {
    inner: Box<
        dyn CloneSyncService<Req, Response = Res, Error = Err, Future = BoxFuture<'static, Result<Res, Err>>>
            + Send
            + Sync,
    >,
}

impl<Req, Res, Err> std::fmt::Debug for BoxCloneSyncService<Req, Res, Err> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BoxCloneSyncService").finish()
    }
}

impl<Req: 'static, Res: 'static, Err: 'static> BoxCloneSyncService<Req, Res, Err> {
    /// Boxes a service into a thread-safe `BoxCloneSyncService`.
    pub fn new<S>(inner: S) -> Self
    where
        S: Service<Req, Response = Res, Error = Err> + Clone + Send + Sync + 'static,
        S::Future: std::future::Future<Output = Result<Res, Err>> + crate::MaybeSend + 'static,
    {
        struct Adapter<S>(S);
        impl<S: Clone> Clone for Adapter<S> {
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }
        impl<Req: 'static, Res: 'static, Err: 'static, S> Service<Req> for Adapter<S>
        where
            S: Service<Req, Response = Res, Error = Err> + Clone + Send + Sync + 'static,
            S::Future: std::future::Future<Output = Result<Res, Err>> + crate::MaybeSend + 'static,
        {
            type Response = Res;
            type Error = Err;
            type Future = BoxFuture<'static, Result<Res, Err>>;

            fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
                self.0.poll_ready(cx)
            }

            fn call(&mut self, req: Req) -> Self::Future {
                Box::pin(self.0.call(req))
            }
        }

        Self {
            inner: Box::new(Adapter(inner)),
        }
    }
}

impl<Req, Res, Err> Clone for BoxCloneSyncService<Req, Res, Err> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone_box(),
        }
    }
}

impl<Req, Res, Err> Service<Req> for BoxCloneSyncService<Req, Res, Err> {
    type Response = Res;
    type Error = Err;
    type Future = BoxFuture<'static, Result<Res, Err>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Req) -> Self::Future {
        self.inner.call(req)
    }
}
