use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;

use tower_layer::Layer;
use tower_service::Service;

use crate::BoxFuture;
use crate::error::Error;

/// Exponential backoff retry policy with jitter.
#[derive(Clone, Debug)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    base_delay: Duration,
}

impl RetryPolicy {
    /// Creates a new `RetryPolicy` with the given maximum number of retry attempts.
    #[must_use]
    pub fn default_with_max(max_attempts: u32) -> Self {
        Self {
            max_attempts,
            base_delay: Duration::from_millis(200),
        }
    }

    /// Exponential backoff + jitter (± 20%), starting at `base_delay` on attempt 1.
    #[must_use]
    pub fn delay(&self, attempt: u32) -> Duration {
        let exponent = attempt.saturating_sub(1).min(6);
        let base = self.base_delay * (1u32 << exponent);
        let jitter = 0.8 + rand::random::<f64>() * 0.4;
        let delay = base.mul_f64(jitter);
        delay.min(Duration::from_secs(60))
    }
}

/// Returns `true` if the status code represents a transient server failure.
#[must_use]
pub fn is_transient(status: reqwest::StatusCode) -> bool {
    matches!(status.as_u16(), 429 | 502 | 503 | 504)
}

/// Returns `true` if the error is considered transient and safe to retry.
#[must_use]
pub fn should_retry(err: &Error) -> bool {
    match err {
        #[cfg(not(target_arch = "wasm32"))]
        Error::Transport(e) => e.is_connect() || e.is_timeout(),
        #[cfg(target_arch = "wasm32")]
        Error::Transport(e) => e.is_timeout(),
        e if e.status().is_some_and(is_transient) => true,
        e if e.retry_after().is_some() => true,
        _ => false,
    }
}

/// A Tower [`Layer`] that automatically retries transient network errors and 5xx/429 server responses.
#[derive(Clone, Debug)]
pub struct RetryLayer {
    policy: Arc<RetryPolicy>,
}

impl RetryLayer {
    /// Creates a new `RetryLayer` with the provided [`RetryPolicy`].
    #[must_use]
    pub fn new(policy: RetryPolicy) -> Self {
        Self {
            policy: Arc::new(policy),
        }
    }
}

impl<S> Layer<S> for RetryLayer {
    type Service = RetryService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RetryService {
            inner,
            policy: Arc::clone(&self.policy),
        }
    }
}

/// A Tower [`Service`] that retries requests upon transient transport or API failures.
#[derive(Clone, Debug)]
pub struct RetryService<S> {
    inner: S,
    policy: Arc<RetryPolicy>,
}

impl<S> Service<reqwest::Request> for RetryService<S>
where
    S: Service<reqwest::Request, Response = reqwest::Response, Error = Error> + Clone + Send + 'static,
    S::Future: crate::MaybeSend + 'static,
{
    type Response = reqwest::Response;
    type Error = Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: reqwest::Request) -> Self::Future {
        let mut inner = self.inner.clone();
        let policy = Arc::clone(&self.policy);

        Box::pin(async move {
            let mut attempt = 0;
            let mut current = req;

            loop {
                let saved = if attempt < policy.max_attempts {
                    current.try_clone()
                } else {
                    None
                };

                match inner.call(current).await {
                    Ok(resp) => {
                        let status = resp.status();
                        if is_transient(status)
                            && attempt < policy.max_attempts
                            && let Some(next) = saved
                        {
                            attempt += 1;
                            let retry_after = resp
                                .headers()
                                .get(reqwest::header::RETRY_AFTER)
                                .and_then(|h| h.to_str().ok())
                                .and_then(|s| s.parse::<u64>().ok())
                                .map(|secs| Duration::from_secs(secs.min(60)));
                            let delay = retry_after.unwrap_or_else(|| policy.delay(attempt));
                            tracing::warn!(
                                attempt = attempt,
                                max_attempts = policy.max_attempts,
                                ?delay,
                                status = %status,
                                "Transient HTTP status error, retrying HTTP request"
                            );
                            tokio::time::sleep(delay).await;
                            current = next;
                            continue;
                        }
                        return Ok(resp);
                    }
                    Err(err) => {
                        if should_retry(&err)
                            && attempt < policy.max_attempts
                            && let Some(next) = saved
                        {
                            attempt += 1;
                            let delay = err.retry_after().unwrap_or_else(|| policy.delay(attempt));
                            tracing::warn!(
                                attempt = attempt,
                                max_attempts = policy.max_attempts,
                                ?delay,
                                error = %err,
                                "Transient transport error, retrying HTTP request"
                            );
                            tokio::time::sleep(delay).await;
                            current = next;
                            continue;
                        }
                        return Err(err);
                    }
                }
            }
        })
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use reqwest::StatusCode;

    use super::*;

    #[test]
    fn retry_policy_delay_increases_exponentially_and_respects_jitter_bounds() {
        let policy = RetryPolicy::default_with_max(5);

        // Attempt 1: base delay is 200ms, jitter is 0.8..1.2 -> 160ms..240ms
        for _ in 0..20 {
            let d1 = policy.delay(1);
            assert!(
                d1 >= Duration::from_millis(160) && d1 <= Duration::from_millis(240),
                "attempt 1 delay {d1:?} out of [160ms, 240ms]"
            );
        }

        // Attempt 2: base delay is 400ms -> 320ms..480ms
        for _ in 0..20 {
            let d2 = policy.delay(2);
            assert!(
                d2 >= Duration::from_millis(320) && d2 <= Duration::from_millis(480),
                "attempt 2 delay {d2:?} out of [320ms, 480ms]"
            );
        }

        // Attempt 3: base delay is 800ms -> 640ms..960ms
        for _ in 0..20 {
            let d3 = policy.delay(3);
            assert!(
                d3 >= Duration::from_millis(640) && d3 <= Duration::from_millis(960),
                "attempt 3 delay {d3:?} out of [640ms, 960ms]"
            );
        }
    }

    #[test]
    fn retry_policy_delay_is_clamped_at_sixty_seconds() {
        let policy = RetryPolicy::default_with_max(10);
        let max_delay = policy.delay(10);
        assert!(max_delay <= Duration::from_secs(60));
    }

    #[test]
    fn is_transient_status_code_coverage() {
        // Transient
        assert!(is_transient(StatusCode::TOO_MANY_REQUESTS));
        assert!(is_transient(StatusCode::BAD_GATEWAY));
        assert!(is_transient(StatusCode::SERVICE_UNAVAILABLE));
        assert!(is_transient(StatusCode::GATEWAY_TIMEOUT));

        // Non-transient
        assert!(!is_transient(StatusCode::OK));
        assert!(!is_transient(StatusCode::BAD_REQUEST));
        assert!(!is_transient(StatusCode::UNAUTHORIZED));
        assert!(!is_transient(StatusCode::FORBIDDEN));
        assert!(!is_transient(StatusCode::NOT_FOUND));
        assert!(!is_transient(StatusCode::CONFLICT));
    }
}
