use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::{Duration, UNIX_EPOCH};

use reqwest::{Method, StatusCode};
use tower_layer::Layer;
use tower_service::Service;

use crate::BoxFuture;
use crate::error::Error;

/// Default maximum backoff delay between retry attempts (`60s`).
pub(crate) const DEFAULT_MAX_RETRY_DELAY: Duration = Duration::from_secs(60);

/// Exponential backoff retry policy with jitter.
#[derive(Clone, Debug)]
pub(crate) struct RetryPolicy {
    pub(crate) max_attempts: u32,
    base_delay: Duration,
    pub(crate) max_delay: Duration,
}

impl RetryPolicy {
    /// Creates a new `RetryPolicy` with the given maximum number of retry attempts.
    #[must_use]
    pub(crate) const fn default_with_max(max_attempts: u32) -> Self {
        Self {
            max_attempts,
            base_delay: Duration::from_millis(200),
            max_delay: DEFAULT_MAX_RETRY_DELAY,
        }
    }

    /// Exponential backoff + jitter (± 20%), starting at `base_delay` on attempt 1
    /// and clamped to `self.max_delay` (`60s` by default).
    #[must_use]
    pub(crate) fn delay(&self, attempt: u32) -> Duration {
        let exponent = attempt.saturating_sub(1).min(10);
        let base = self.base_delay * (1u32 << exponent);
        let jitter = 0.8 + rand::random::<f64>() * 0.4;
        let delay = base.mul_f64(jitter);
        delay.min(self.max_delay)
    }
}

/// Parses an HTTP `Retry-After` header formatted either as integer delta-seconds
/// or an RFC 9110 HTTP-date, clamped to `max_delay`.
#[must_use]
pub(crate) fn parse_retry_after(headers: &reqwest::header::HeaderMap, max_delay: Duration) -> Option<Duration> {
    let raw = headers.get(reqwest::header::RETRY_AFTER)?.to_str().ok()?.trim();

    if let Ok(secs) = raw.parse::<u64>() {
        return Some(Duration::from_secs(secs).min(max_delay));
    }

    let target_time = httpdate::parse_http_date(raw).ok()?;
    let now_time = UNIX_EPOCH + crate::core::time::unix_now();
    let delta = target_time.duration_since(now_time).unwrap_or(Duration::ZERO);
    Some(delta.min(max_delay))
}

/// Returns `true` if `method` is HTTP-idempotent (`GET`, `HEAD`, `PUT`, `DELETE`, `OPTIONS`, `TRACE`).
#[must_use]
pub(crate) fn is_idempotent_method(method: &Method) -> bool {
    matches!(
        *method,
        Method::GET | Method::HEAD | Method::PUT | Method::DELETE | Method::OPTIONS | Method::TRACE
    )
}

/// Returns `true` if `status` is eligible for automatic retry under `method`.
///
/// - `429 Too Many Requests`, `502 Bad Gateway`, `503 Service Unavailable`, and `504 Gateway Timeout`
///   are retryable for all replayable requests.
/// - `500 Internal Server Error` is retryable **only** when `method` is explicitly known to be idempotent.
#[must_use]
pub(crate) fn is_retryable_status(status: StatusCode, method: Option<&Method>) -> bool {
    match status.as_u16() {
        429 | 502 | 503 | 504 => true,
        500 => method.is_some_and(is_idempotent_method),
        _ => false,
    }
}

/// Returns `true` if `err` is considered transient and safe to retry for `method`.
#[must_use]
pub(crate) fn should_retry_for_method(err: &Error, method: Option<&Method>) -> bool {
    match err {
        #[cfg(not(target_arch = "wasm32"))]
        Error::Transport(e) => e.is_connect() || e.is_timeout(),
        #[cfg(target_arch = "wasm32")]
        Error::Transport(e) => e.is_timeout(),
        e if e.status().is_some_and(|s| is_retryable_status(s, method)) => true,
        e if e.retry_after().is_some() => true,
        _ => false,
    }
}

/// Returns `true` if `err` is unconditionally transient regardless of HTTP method.
#[must_use]
pub(crate) fn should_retry(err: &Error) -> bool {
    should_retry_for_method(err, None)
}

/// A Tower [`Layer`] that automatically retries transient network errors and eligible 5xx/429 responses.
#[derive(Clone, Debug)]
pub(crate) struct RetryLayer {
    policy: Arc<RetryPolicy>,
}

impl RetryLayer {
    /// Creates a new `RetryLayer` with the provided [`RetryPolicy`].
    #[must_use]
    pub(crate) fn new(policy: RetryPolicy) -> Self {
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
pub(crate) struct RetryService<S> {
    inner: S,
    policy: Arc<RetryPolicy>,
}

impl<S> Service<reqwest::Request> for RetryService<S>
where
    S: Service<reqwest::Request, Response = reqwest::Response, Error = Error> + Clone + crate::MaybeSend + 'static,
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
                let method = current.method().clone();
                let saved = if attempt < policy.max_attempts {
                    current.try_clone()
                } else {
                    None
                };

                match inner.call(current).await {
                    Ok(resp) => {
                        let status = resp.status();
                        if is_retryable_status(status, Some(&method))
                            && attempt < policy.max_attempts
                            && let Some(next) = saved
                        {
                            attempt += 1;
                            let retry_after = parse_retry_after(resp.headers(), policy.max_delay);
                            let delay = retry_after.unwrap_or_else(|| policy.delay(attempt));
                            tracing::warn!(
                                attempt = attempt,
                                max_attempts = policy.max_attempts,
                                ?delay,
                                status = %status,
                                method = %method,
                                "Transient HTTP status error, retrying HTTP request"
                            );
                            crate::core::time::sleep(delay).await;
                            current = next;
                            continue;
                        }
                        return Ok(resp);
                    }
                    Err(err) => {
                        if should_retry_for_method(&err, Some(&method))
                            && attempt < policy.max_attempts
                            && let Some(next) = saved
                        {
                            attempt += 1;
                            let delay = err.retry_after().unwrap_or_else(|| policy.delay(attempt));
                            tracing::warn!(
                                attempt = attempt,
                                max_attempts = policy.max_attempts,
                                ?delay,
                                method = %method,
                                error = %err,
                                "Transient transport error, retrying HTTP request"
                            );
                            crate::core::time::sleep(delay).await;
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
#[path = "retry_tests.rs"]
mod tests;
