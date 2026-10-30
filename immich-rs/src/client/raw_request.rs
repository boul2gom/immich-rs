use std::future::IntoFuture;
use std::time::Duration;

use reqwest::header::{HeaderName, HeaderValue};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::BoxFuture;
use crate::client::ImmichClient;
use crate::core::{ByteStream, RequestConfig};
use crate::error::{Error, Result};

/// Fluent request builder for executing raw or unmapped HTTP requests through the client middleware pipeline.
///
/// Automatically applies authentication headers, client timeouts, rate limiting, and retry policies.
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct RawRequestBuilder<'a> {
    client: &'a ImmichClient,
    method: reqwest::Method,
    path: String,
    query: Vec<(String, String)>,
    request_config: RequestConfig,
    body: Option<Vec<u8>>,
    body_content_type: Option<&'static str>,
}

impl<'a> RawRequestBuilder<'a> {
    pub(crate) fn new(client: &'a ImmichClient, method: reqwest::Method, path: impl Into<String>) -> Self {
        Self {
            client,
            method,
            path: path.into(),
            query: Vec::new(),
            request_config: RequestConfig::default(),
            body: None,
            body_content_type: None,
        }
    }

    /// Adds a single query parameter pair `(key, value)`.
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.push((key.into(), value.into()));
        self
    }

    /// Serializes and appends query parameters from a serializable struct or map.
    ///
    /// # Errors
    ///
    /// Returns an error if query serialization fails.
    pub fn query<T: Serialize>(mut self, query: &T) -> Result<Self> {
        let query_str = serde_urlencoded::to_string(query).map_err(|e| Error::Validation(e.to_string()))?;
        let pairs: Vec<(String, String)> =
            serde_urlencoded::from_str(&query_str).map_err(|e| Error::Validation(e.to_string()))?;
        self.query.extend(pairs);
        Ok(self)
    }

    /// Sets or overrides the timeout for this specific request.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header for this request.
    pub fn header(mut self, key: impl TryInto<HeaderName>, value: impl TryInto<HeaderValue>) -> Self {
        self.request_config.insert_header(key, value);
        self
    }

    /// Attaches a JSON request body and sets `Content-Type: application/json`.
    ///
    /// # Errors
    ///
    /// Returns an error if JSON serialization fails.
    pub fn json<T: Serialize>(mut self, body: &T) -> Result<Self> {
        let bytes = serde_json::to_vec(body).map_err(Error::Serialization)?;
        self.body = Some(bytes);
        self.body_content_type = Some("application/json");
        Ok(self)
    }

    /// Attaches arbitrary binary bytes to the request body with the specified content type.
    pub fn body(mut self, bytes: impl Into<Vec<u8>>, content_type: Option<&'static str>) -> Self {
        self.body = Some(bytes.into());
        self.body_content_type = content_type;
        self
    }

    /// Dispatches the request through the client's Tower middleware stack and returns the raw [`ByteStream`].
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if preparing or dispatching the request fails, or if the server returns an error code.
    pub async fn send(self) -> Result<ByteStream> {
        let mut builder = self.client.prepare_request(self.method, &self.path)?;
        if !self.query.is_empty() {
            builder = builder.query(&self.query);
        }
        if let Some(body) = self.body {
            builder = builder.body(body);
            if let Some(ct) = self.body_content_type {
                builder = builder.header(reqwest::header::CONTENT_TYPE, ct);
            }
        }
        let builder = self.request_config.apply(builder)?;
        let response = self.client.execute_stream("raw", builder).await?;
        Ok(ByteStream::new(response))
    }

    /// Dispatches the request and deserializes the JSON response body into `T`.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or if deserializing the JSON response fails.
    pub async fn send_json<T: DeserializeOwned>(self) -> Result<T> {
        let stream = self.send().await?;
        let bytes = stream.bytes().await?;
        serde_json::from_slice(&bytes).map_err(Error::Serialization)
    }
}

impl std::fmt::Debug for RawRequestBuilder<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RawRequestBuilder")
            .field("method", &self.method)
            .field("path", &self.path)
            .field("query", &self.query)
            .field("has_body", &self.body.is_some())
            .field("body_content_type", &self.body_content_type)
            .finish_non_exhaustive()
    }
}

impl<'a> IntoFuture for RawRequestBuilder<'a> {
    type Output = Result<ByteStream>;
    type IntoFuture = BoxFuture<'a, Self::Output>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
