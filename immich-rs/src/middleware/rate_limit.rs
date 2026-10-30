//! Client-side rate limiting Tower layer implementing O(1) slot-reservation token-bucket throttling.

use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::time::Duration;

use tower_layer::Layer;
use tower_service::Service;

use crate::BoxFuture;
use crate::core::time::Instant;

#[derive(Debug)]
struct TokenBucket {
    available: f64,
    last_refill: Instant,
}

/// Token-bucket rate limiter that reserves slots in `O(1)` time and asynchronously waits without lock contention.
#[derive(Debug, Clone)]
pub struct RateLimiter {
    state: Arc<Mutex<TokenBucket>>,
    refill_rate: f64,
    capacity: f64,
}

impl RateLimiter {
    /// Creates a new rate limiter enforcing at most `requests_per_second` (clamped to at least `1`).
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

    /// Reserves a single request token and asynchronously waits if the bucket is currently depleted.
    pub async fn acquire(&self) {
        let wait_duration = {
            let mut bucket = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let now = Instant::now();
            let elapsed = now.duration_since(bucket.last_refill).as_secs_f64();
            bucket.available = (bucket.available + elapsed * self.refill_rate).min(self.capacity);
            bucket.last_refill = now;
            bucket.available -= 1.0;

            if bucket.available >= 0.0 {
                Duration::ZERO
            } else {
                Duration::from_secs_f64((-bucket.available) / self.refill_rate)
            }
        };

        if !wait_duration.is_zero() {
            crate::core::time::sleep(wait_duration).await;
        }
    }
}

/// A Tower [`Layer`] that applies client-side token-bucket rate limiting.
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

/// A Tower [`Service`] that enforces token-bucket rate limiting before forwarding requests.
#[derive(Debug, Clone)]
pub struct RateLimitService<S> {
    inner: S,
    rate_limiter: Arc<RateLimiter>,
}

impl<S, Req> Service<Req> for RateLimitService<S>
where
    S: Service<Req> + Clone + crate::MaybeSend + 'static,
    S::Future: crate::MaybeSend + 'static,
    Req: crate::MaybeSend + 'static,
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
