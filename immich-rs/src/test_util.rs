//! Utilities for in-memory testing and mock clients without real network I/O.

use std::future::Future;
use std::sync::Arc;

use tower_service::Service;

use crate::BoxFuture;
use crate::client::ImmichClient;
use crate::error::Error;

/// A mock Tower service function handler.
pub type MockHandler = Arc<dyn Fn(reqwest::Request) -> BoxFuture<'static, reqwest::Response> + Send + Sync + 'static>;

/// In-memory mock transport service implementing [`tower_service::Service`].
#[derive(Clone)]
pub struct MockService {
    handler: MockHandler,
}

impl MockService {
    /// Creates a new [`MockService`] from a mock handler function.
    #[must_use]
    pub fn new(handler: MockHandler) -> Self {
        Self { handler }
    }
}

impl Service<reqwest::Request> for MockService {
    type Response = reqwest::Response;
    type Error = Error;
    type Future = BoxFuture<'static, std::result::Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::result::Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: reqwest::Request) -> Self::Future {
        let handler = Arc::clone(&self.handler);
        Box::pin(async move {
            let resp = (handler)(req).await;
            Ok(resp)
        })
    }
}

impl std::fmt::Debug for MockService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MockService").finish_non_exhaustive()
    }
}

/// Creates a mocked [`ImmichClient`] using a custom async request handler function.
///
/// Dispatched requests bypass the network layer and execute directly against
/// your closure, enabling fast, isolated unit tests without network ports.
///
/// # Panics
///
/// Panics if the default mock client base configuration is invalid (which is a bug in the SDK).
pub fn mock_client<F, Fut>(handler: F) -> ImmichClient
where
    F: Fn(reqwest::Request) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = reqwest::Response> + Send + 'static,
{
    let handler_arc: MockHandler = Arc::new(move |req| Box::pin(handler(req)));
    let mock_service = MockService::new(handler_arc);

    ImmichClient::builder()
        .base_url("https://mock.immich.local/api")
        .api_key("mock-api-key")
        .connector(mock_service)
        .build()
        .expect("mock client configuration is valid")
}
