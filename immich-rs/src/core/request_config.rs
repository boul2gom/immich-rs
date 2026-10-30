//! Per-request configuration options (e.g. timeouts and custom headers).

use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

use crate::error::{Error, Result};

/// Per-request configuration options that override or extend client defaults.
#[derive(Debug, Clone, Default)]
pub struct RequestConfig {
    /// Optional per-request timeout.
    pub timeout: Option<Duration>,
    /// Custom headers added to this specific request.
    pub headers: HeaderMap,
    invalid_header: Option<String>,
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
    ///
    /// If either `name` or `value` cannot be converted into a valid HTTP header,
    /// the validation failure is recorded and returned as [`Error::Validation`]
    /// when the request is executed.
    pub fn insert_header<K, V>(&mut self, name: K, value: V)
    where
        K: TryInto<HeaderName>,
        V: TryInto<HeaderValue>,
    {
        match (name.try_into(), value.try_into()) {
            (Ok(k), Ok(v)) => {
                self.headers.insert(k, v);
            }
            (Err(_), _) => {
                if self.invalid_header.is_none() {
                    self.invalid_header = Some("invalid HTTP header name".to_string());
                }
            }
            (_, Err(_)) => {
                if self.invalid_header.is_none() {
                    self.invalid_header = Some("invalid HTTP header value".to_string());
                }
            }
        }
    }

    /// Applies this configuration to a [`reqwest::RequestBuilder`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::Validation`] if an invalid header name or value was supplied via [`Self::insert_header`].
    pub(crate) fn apply(&self, mut builder: reqwest::RequestBuilder) -> Result<reqwest::RequestBuilder> {
        if let Some(reason) = &self.invalid_header {
            return Err(Error::Validation(reason.clone()));
        }
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(timeout) = self.timeout {
            builder = builder.timeout(timeout);
        }
        if !self.headers.is_empty() {
            builder = builder.headers(self.headers.clone());
        }
        Ok(builder)
    }
}
