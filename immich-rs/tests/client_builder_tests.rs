use std::time::Duration;

use immich_rs::ImmichClient;
use pretty_assertions::assert_eq;
use reqwest::header::{HeaderMap, HeaderValue};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[test]
fn builder_normalizes_base_url_trailing_slash() {
    let client = ImmichClient::builder()
        .base_url("http://example.com/api")
        .api_key("test-key")
        .build()
        .expect("build client");

    assert_eq!(client.base_url().as_str(), "http://example.com/api/");
}

#[test]
fn builder_configures_timeouts_and_headers_without_error() {
    let mut headers = HeaderMap::new();
    headers.insert("cf-access-client-id", HeaderValue::from_static("test-id"));

    let client = ImmichClient::builder()
        .base_url("http://example.com/api/")
        .api_key("test-key")
        .connect_timeout(Duration::from_secs(5))
        .global_timeout(Duration::from_secs(30))
        .user_agent("custom-agent/1.0")
        .default_headers(headers)
        .max_retries(5)
        .build()
        .expect("build client");

    assert_eq!(client.base_url().as_str(), "http://example.com/api/");
}

#[test]
fn builder_rejects_empty_credentials_cleanly() {
    let empty_key = ImmichClient::builder()
        .base_url("http://example.com/api/")
        .api_key("   ")
        .build();
    assert!(empty_key.is_err());

    let empty_bearer = ImmichClient::builder()
        .base_url("http://example.com/api/")
        .bearer_token("   ")
        .build();
    assert!(empty_bearer.is_err());
}

#[test]
fn builder_rejects_malformed_urls_with_validation_error() {
    let invalid = ImmichClient::builder()
        .base_url("not a valid url")
        .api_key("test-key")
        .build();
    assert!(invalid.is_err());

    let cannot_be_base = ImmichClient::builder()
        .base_url("data:text/plain,hello")
        .api_key("test-key")
        .build();
    assert!(cannot_be_base.is_err());
}

#[tokio::test]
async fn builder_sends_custom_user_agent_and_default_headers_to_server() -> anyhow::Result<()> {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/server/ping"))
        .and(header("user-agent", "ImmichCustomApp/3.0.0"))
        .and(header("cf-access-client-id", "test-cf-client-id"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"res": "pong"})))
        .expect(1)
        .mount(&server)
        .await;

    let mut headers = HeaderMap::new();
    headers.insert("cf-access-client-id", HeaderValue::from_static("test-cf-client-id"));

    let client = ImmichClient::builder()
        .base_url(server.uri())
        .api_key("test-key")
        .user_agent("ImmichCustomApp/3.0.0")
        .default_headers(headers)
        .build()?;

    let ping = client.server().ping_server().await?;
    assert_eq!(ping.res, "pong");
    Ok(())
}

#[tokio::test]
async fn check_compatibility_and_build_checked_advisory_flow() -> anyhow::Result<()> {
    use immich_rs::Compatibility;
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/server/version"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "major": 1,
            "minor": 100,
            "patch": 0
        })))
        .mount(&server)
        .await;

    let client = ImmichClient::builder()
        .base_url(server.uri())
        .api_key("test-key")
        .warn_on_version_mismatch(true)
        .build_checked()
        .await?;

    let compat = client.check_compatibility().await?;
    assert!(matches!(compat, Compatibility::ServerOlder { .. }));
    assert!(!compat.is_compatible());

    Ok(())
}

#[tokio::test]
async fn builder_custom_tower_layer_intercepts_requests() -> anyhow::Result<()> {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::task::{Context, Poll};

    use tower_layer::Layer;
    use tower_service::Service;

    #[derive(Clone)]
    struct CountingLayer {
        counter: Arc<AtomicUsize>,
    }

    impl<S> Layer<S> for CountingLayer {
        type Service = CountingService<S>;
        fn layer(&self, inner: S) -> Self::Service {
            CountingService {
                inner,
                counter: Arc::clone(&self.counter),
            }
        }
    }

    #[derive(Clone)]
    struct CountingService<S> {
        inner: S,
        counter: Arc<AtomicUsize>,
    }

    impl<S> Service<reqwest::Request> for CountingService<S>
    where
        S: Service<reqwest::Request, Response = reqwest::Response, Error = immich_rs::Error> + Clone + Send + 'static,
        S::Future: Send + 'static,
    {
        type Response = reqwest::Response;
        type Error = immich_rs::Error;
        type Future = immich_rs::BoxFuture<'static, Result<Self::Response, Self::Error>>;

        fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            self.inner.poll_ready(cx)
        }

        fn call(&mut self, mut req: reqwest::Request) -> Self::Future {
            self.counter.fetch_add(1, Ordering::Relaxed);
            req.headers_mut().insert(
                reqwest::header::HeaderName::from_static("x-custom-layer"),
                reqwest::header::HeaderValue::from_static("injected"),
            );
            let mut inner = self.inner.clone();
            Box::pin(async move { inner.call(req).await })
        }
    }

    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/server/ping"))
        .and(header("x-custom-layer", "injected"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"res": "pong"})))
        .expect(1)
        .mount(&server)
        .await;

    let counter = Arc::new(AtomicUsize::new(0));
    let client = ImmichClient::builder()
        .base_url(server.uri())
        .api_key("test-key")
        .layer(CountingLayer {
            counter: counter.clone(),
        })
        .build()?;

    let res = client.server().ping_server().await?;
    assert_eq!(res.res, "pong");
    assert_eq!(counter.load(Ordering::Relaxed), 1);

    Ok(())
}

#[tokio::test]
async fn builder_rate_limiting_smoothly_throttles_requests() -> anyhow::Result<()> {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/server/ping"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"res": "pong"})))
        .expect(3)
        .mount(&server)
        .await;

    // 2 rps = burst capacity of 2 tokens, 500ms refill per token
    let client = ImmichClient::builder()
        .base_url(server.uri())
        .api_key("test-key")
        .rate_limit_per_second(2)
        .build()?;

    let start = std::time::Instant::now();
    for _ in 0..3 {
        let ping = client.server().ping_server().await?;
        assert_eq!(ping.res, "pong");
    }
    let elapsed = start.elapsed();
    // 3 requests with burst of 2 must take at least ~400ms due to pacing
    assert!(elapsed.as_millis() >= 400, "expected at least 400ms, got {elapsed:?}");

    Ok(())
}
