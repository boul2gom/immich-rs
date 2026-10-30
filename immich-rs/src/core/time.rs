//! Cross-platform monotonic and wall-clock time utilities (`tokio::time` on native, `web-time` + `gloo-timers` on WASM).

use std::time::Duration;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) use tokio::time::Instant;
#[cfg(target_arch = "wasm32")]
pub(crate) use web_time::Instant;

/// Sleeps asynchronously for `duration` without panicking on `wasm32-unknown-unknown`.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) async fn sleep(duration: Duration) {
    tokio::time::sleep(duration).await;
}

/// Sleeps asynchronously for `duration` without panicking on `wasm32-unknown-unknown`.
#[cfg(target_arch = "wasm32")]
pub(crate) async fn sleep(duration: Duration) {
    gloo_timers::future::sleep(duration).await;
}

/// Returns the current wall-clock duration since `UNIX_EPOCH` across native and WebAssembly targets.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn unix_now() -> Duration {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
}

/// Returns the current wall-clock duration since `UNIX_EPOCH` across native and WebAssembly targets.
#[cfg(target_arch = "wasm32")]
pub(crate) fn unix_now() -> Duration {
    web_time::SystemTime::now()
        .duration_since(web_time::UNIX_EPOCH)
        .unwrap_or_default()
}

/// Returns the current UTC timestamp without panicking on `wasm32-unknown-unknown`.
pub(crate) fn utc_now() -> chrono::DateTime<chrono::Utc> {
    let dur = unix_now();
    let secs = i64::try_from(dur.as_secs()).unwrap_or(i64::MAX);
    chrono::DateTime::from_timestamp(secs, dur.subsec_nanos()).unwrap_or_default()
}
