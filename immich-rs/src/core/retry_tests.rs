use std::time::{Duration, SystemTime};

use pretty_assertions::assert_eq;
use reqwest::header::{HeaderMap, HeaderValue, RETRY_AFTER};
use reqwest::{Method, StatusCode};

use super::*;

#[test]
fn retry_policy_delay_increases_exponentially_within_jitter_bounds() {
    let policy = RetryPolicy::default_with_max(5);

    for _ in 0..20 {
        let d1 = policy.delay(1);
        assert!(
            d1 >= Duration::from_millis(160) && d1 <= Duration::from_millis(240),
            "attempt 1 delay {d1:?} out of [160ms, 240ms]"
        );

        let d2 = policy.delay(2);
        assert!(
            d2 >= Duration::from_millis(320) && d2 <= Duration::from_millis(480),
            "attempt 2 delay {d2:?} out of [320ms, 480ms]"
        );

        let d3 = policy.delay(3);
        assert!(
            d3 >= Duration::from_millis(640) && d3 <= Duration::from_millis(960),
            "attempt 3 delay {d3:?} out of [640ms, 960ms]"
        );
    }
}

#[test]
fn retry_policy_delay_reaches_and_clamps_at_sixty_seconds() {
    let policy = RetryPolicy::default_with_max(12);
    // Attempt 10: 200ms * 512 = 102.4s * [0.8..1.2] = [81.92s..122.88s] -> always clamped to 60s
    for _ in 0..20 {
        assert_eq!(policy.delay(10), Duration::from_secs(60));
        assert_eq!(policy.delay(15), Duration::from_secs(60));
    }
}

#[test]
fn is_retryable_status_enforces_http_method_idempotence_for_500() {
    for status in [
        StatusCode::TOO_MANY_REQUESTS,
        StatusCode::BAD_GATEWAY,
        StatusCode::SERVICE_UNAVAILABLE,
        StatusCode::GATEWAY_TIMEOUT,
    ] {
        assert!(is_retryable_status(status, None));
        assert!(is_retryable_status(status, Some(&Method::POST)));
        assert!(is_retryable_status(status, Some(&Method::GET)));
    }

    // 500 is retryable ONLY for idempotent methods
    assert!(is_retryable_status(
        StatusCode::INTERNAL_SERVER_ERROR,
        Some(&Method::GET)
    ));
    assert!(is_retryable_status(
        StatusCode::INTERNAL_SERVER_ERROR,
        Some(&Method::HEAD)
    ));
    assert!(is_retryable_status(
        StatusCode::INTERNAL_SERVER_ERROR,
        Some(&Method::PUT)
    ));
    assert!(is_retryable_status(
        StatusCode::INTERNAL_SERVER_ERROR,
        Some(&Method::DELETE)
    ));
    assert!(is_retryable_status(
        StatusCode::INTERNAL_SERVER_ERROR,
        Some(&Method::OPTIONS)
    ));
    assert!(!is_retryable_status(
        StatusCode::INTERNAL_SERVER_ERROR,
        Some(&Method::POST)
    ));
    assert!(!is_retryable_status(
        StatusCode::INTERNAL_SERVER_ERROR,
        Some(&Method::PATCH)
    ));
    assert!(!is_retryable_status(StatusCode::INTERNAL_SERVER_ERROR, None));

    for status in [
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
        StatusCode::UNAUTHORIZED,
        StatusCode::FORBIDDEN,
        StatusCode::NOT_FOUND,
        StatusCode::CONFLICT,
    ] {
        assert!(!is_retryable_status(status, Some(&Method::GET)));
        assert!(!is_retryable_status(status, Some(&Method::POST)));
    }
}

#[test]
fn parse_retry_after_handles_seconds_http_dates_and_past_dates() {
    let max_delay = Duration::from_secs(60);

    let mut headers = HeaderMap::new();
    headers.insert(RETRY_AFTER, HeaderValue::from_static("12"));
    assert_eq!(parse_retry_after(&headers, max_delay), Some(Duration::from_secs(12)));

    headers.insert(RETRY_AFTER, HeaderValue::from_static("300"));
    assert_eq!(parse_retry_after(&headers, max_delay), Some(Duration::from_secs(60)));

    let future_date = httpdate::fmt_http_date(SystemTime::now() + Duration::from_secs(25));
    headers.insert(RETRY_AFTER, HeaderValue::from_str(&future_date).unwrap());
    let parsed = parse_retry_after(&headers, max_delay).unwrap();
    assert!(
        parsed >= Duration::from_secs(23) && parsed <= Duration::from_secs(26),
        "expected ~25s from HTTP-date Retry-After, got {parsed:?}"
    );

    let past_date = httpdate::fmt_http_date(SystemTime::now() - Duration::from_secs(30));
    headers.insert(RETRY_AFTER, HeaderValue::from_str(&past_date).unwrap());
    assert_eq!(parse_retry_after(&headers, max_delay), Some(Duration::ZERO));

    headers.insert(RETRY_AFTER, HeaderValue::from_static("not-a-valid-date"));
    assert_eq!(parse_retry_after(&headers, max_delay), None);
}
