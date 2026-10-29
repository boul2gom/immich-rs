//! OpenTelemetry metrics support for `immich-rs`.

#[cfg(feature = "opentelemetry")]
use std::sync::OnceLock;

#[cfg(feature = "opentelemetry")]
use opentelemetry::{
    KeyValue, global,
    metrics::{Counter, Histogram},
};

#[cfg(feature = "opentelemetry")]
struct ClientMetrics {
    requests: Counter<u64>,
    duration: Histogram<f64>,
}

#[cfg(feature = "opentelemetry")]
static METRICS: OnceLock<ClientMetrics> = OnceLock::new();

#[cfg(feature = "opentelemetry")]
fn get_metrics() -> &'static ClientMetrics {
    METRICS.get_or_init(|| {
        let meter = global::meter("immich-rs");
        let requests = meter
            .u64_counter("immich.client.requests")
            .with_description("Total number of HTTP requests issued by the Immich SDK")
            .build();
        let duration = meter
            .f64_histogram("immich.client.request.duration")
            .with_description("Duration of HTTP requests issued by the Immich SDK in seconds")
            .with_unit("s")
            .build();
        ClientMetrics { requests, duration }
    })
}

#[cfg(feature = "opentelemetry")]
pub(crate) fn record_http_request(method: &str, status: Option<u16>, duration_secs: f64) {
    let metrics = get_metrics();
    let status_str = status.map_or_else(|| "error".to_string(), |s| s.to_string());
    let attributes = [
        KeyValue::new("http.request.method", method.to_string()),
        KeyValue::new("http.response.status_code", status_str),
    ];
    metrics.requests.add(1, &attributes);
    metrics.duration.record(duration_secs, &attributes);
}
