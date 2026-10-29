//! Client-side rate limiting Tower layer implementing smooth async token-bucket throttling.

use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;

use tokio::sync::Mutex;
use tokio::time::Instant;
use tower_layer::Layer;
use tower_service::Service;

use crate::BoxFuture;

#[derive(Debug)]
struct TokenBucket {
    available: f64,
    last_refill: Instant,
}

/// Token-bucket rate limiter that asynchronously waits until capacity is available.
#[derive(Debug, Clone)]
pub struct RateLimiter {
    state: Arc<Mutex<TokenBucket>>,
    refill_rate: f64,
    capacity: f64,
}

impl RateLimiter {
    /// Creates a new rate limiter with the given requests per second.
    #[must_use]
    pub fn new(requests_per_second: u32) -> Self {
        let rps = f64::from(requests_per_second.max(1));
        Self {
            state: Arc::new(Mutex::new(TokenBucket {
                available: rps,
                last_refill: Instant::now(),
            })),
            refill_rate: rps,
            capacity: rps,
        }
    }

    /// Acquires a token, asynchronously waiting if the rate limit is currently exhausted.
    pub async fn acquire(&self) {
        loop {
            let wait_duration = {
                let mut bucket = self.state.lock().await;
                let now = Instant::now();
                let elapsed = now.duration_since(bucket.last_refill).as_secs_f64();
                bucket.available = (bucket.available + elapsed * self.refill_rate).min(self.capacity);
                bucket.last_refill = now;

                if bucket.available >= 1.0 {
                    bucket.available -= 1.0;
                    return;
                }

                let needed = 1.0 - bucket.available;
                Duration::from_secs_f64(needed / self.refill_rate)
            };

            tokio::time::sleep(wait_duration).await;
        }
    }
}

/// A Tower [`Layer`] that applies client-side token bucket rate limiting.
#[derive(Debug, Clone)]
pub struct RateLimitLayer {
    rate_limiter: Arc<RateLimiter>,
}

impl RateLimitLayer {
    /// Creates a new `RateLimitLayer` enforcing at most `requests_per_second`.
    #[must_use]
    pub fn new(requests_per_second: u32) -> Self {
        Self {
            rate_limiter: Arc::new(RateLimiter::new(requests_per_second)),
        }
    }

    /// Returns a reference to the underlying [`RateLimiter`].
    #[must_use]
    pub fn rate_limiter(&self) -> &RateLimiter {
        &self.rate_limiter
    }
}

impl<S> Layer<S> for RateLimitLayer {
    type Service = RateLimitService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RateLimitService {
            inner,
            rate_limiter: Arc::clone(&self.rate_limiter),
        }
    }
}

/// A Tower [`Service`] that enforces token bucket rate limiting before forwarding requests.
#[derive(Debug, Clone)]
pub struct RateLimitService<S> {
    inner: S,
    rate_limiter: Arc<RateLimiter>,
}

impl<S, Req> Service<Req> for RateLimitService<S>
where
    S: Service<Req> + Clone + Send + 'static,
    S::Future: crate::MaybeSend + 'static,
    Req: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Req) -> Self::Future {
        let mut inner = self.inner.clone();
        let rate_limiter = Arc::clone(&self.rate_limiter);
        Box::pin(async move {
            rate_limiter.acquire().await;
            inner.call(req).await
        })
    }
}
