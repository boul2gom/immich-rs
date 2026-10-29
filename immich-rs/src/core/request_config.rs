//! Per-request configuration options (e.g. timeouts and custom headers).

use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

/// Per-request configuration options that override or extend client defaults.
#[derive(Debug, Clone, Default)]
pub struct RequestConfig {
    /// Optional per-request timeout.
    pub timeout: Option<Duration>,
    /// Custom headers added to this specific request.
    pub headers: HeaderMap,
}

impl RequestConfig {
    /// Creates a new, empty request configuration.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets or overrides the timeout for this specific request.
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = Some(timeout);
    }

    /// Adds a custom HTTP header to this specific request.
    pub fn insert_header<K, V>(&mut self, name: K, value: V)
    where
        K: TryInto<HeaderName>,
        V: TryInto<HeaderValue>,
    {
        if let (Ok(k), Ok(v)) = (name.try_into(), value.try_into()) {
            self.headers.insert(k, v);
        }
    }

    /// Applies this configuration to a [`reqwest::RequestBuilder`].
    pub fn apply(&self, mut builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(timeout) = self.timeout {
            builder = builder.timeout(timeout);
        }
        if !self.headers.is_empty() {
            builder = builder.headers(self.headers.clone());
        }
        builder
    }
}
