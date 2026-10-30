use reqwest::header::{ACCEPT, HeaderValue};
use reqwest::{Method, RequestBuilder, Url};
use serde::de::DeserializeOwned;
use tower_service::Service;
use tracing::Instrument;

use crate::client::ImmichClient;
use crate::error::{Error, Result};

/// A single URL path segment used to construct endpoint URLs safely without path traversal.
#[derive(Debug, Clone, Copy)]
pub(crate) enum PathSegment<'a> {
    /// Static literal segment from the `OpenAPI` route template or a custom route.
    Static(&'a str),
    /// Dynamic user-provided path parameter segment.
    Param(&'a str),
}

pub(crate) fn build_endpoint_url(base_url: &Url, segments: &[PathSegment<'_>]) -> Result<Url> {
    let mut url = base_url.clone();
    {
        let mut path_segments = url
            .path_segments_mut()
            .map_err(|()| Error::Validation(format!("base_url cannot be a base URL: {base_url}")))?;
        path_segments.pop_if_empty();

        for seg in segments {
            match *seg {
                PathSegment::Static(s) => {
                    for part in s.split('/').filter(|p| !p.is_empty()) {
                        path_segments.push(part);
                    }
                }
                PathSegment::Param(p) => {
                    if p.is_empty() || p == "." || p == ".." {
                        return Err(Error::Validation(format!("invalid URL path parameter segment: {p:?}")));
                    }
                    path_segments.push(p);
                }
            }
        }
    }
    Ok(url)
}

impl ImmichClient {
    fn attach_auth(&self, mut req: RequestBuilder) -> RequestBuilder {
        let auth = self.inner.auth.load();
        match auth.as_ref() {
            crate::client::Auth::ApiKey(k) => req = req.header("x-api-key", k.clone()),
            crate::client::Auth::Bearer(t) => {
                req = req.header(reqwest::header::AUTHORIZATION, t.clone());
            }
            crate::client::Auth::None => {}
        }
        req
    }

    /// Prepares a new HTTP request from a path.
    pub(crate) fn prepare_request(&self, method: Method, path: &str) -> Result<RequestBuilder> {
        let url = build_endpoint_url(&self.inner.base_url, &[PathSegment::Static(path)])?;
        Ok(self.attach_auth(self.inner.raw_http.request(method, url)))
    }

    /// Prepares a new HTTP request from structured static and parameter path segments.
    pub(crate) fn prepare_request_segments(
        &self,
        method: Method,
        segments: &[PathSegment<'_>],
    ) -> Result<RequestBuilder> {
        let url = build_endpoint_url(&self.inner.base_url, segments)?;
        Ok(self.attach_auth(self.inner.raw_http.request(method, url)))
    }

    /// Executes an HTTP request through the Tower execution stack with `OpenTelemetry` semantic spans.
    pub(crate) async fn execute_stream(
        &self,
        module: &'static str,
        builder: RequestBuilder,
    ) -> Result<reqwest::Response> {
        let request = builder.build().map_err(Error::from)?;

        let method = request.method().as_str().to_string();
        let url = request.url();
        let host = url.host_str().unwrap_or_default().to_string();
        let port = url.port_or_known_default().unwrap_or(80);
        let mut clean_url = url.clone();
        clean_url.set_query(None);
        let url_str = clean_url.to_string();

        let span = tracing::info_span!(
            "HTTP request",
            "http.request.method" = %method,
            "server.address" = %host,
            "server.port" = port,
            "url.full" = %url_str,
            "rpc.service" = module,
            "http.response.status_code" = tracing::field::Empty,
            "error.type" = tracing::field::Empty,
        );

        let mut service = self.inner.http.clone();

        async move {
            #[cfg(feature = "opentelemetry")]
            let start_time = crate::core::time::Instant::now();

            let result = service.call(request).await;

            #[cfg(feature = "opentelemetry")]
            {
                let elapsed = start_time.elapsed().as_secs_f64();
                let status = result.as_ref().map(|r| r.status().as_u16()).ok();
                crate::core::metrics::record_http_request(&method, status, elapsed);
            }

            let current_span = tracing::Span::current();
            match result {
                Ok(resp) => {
                    let status_code = resp.status().as_u16();
                    current_span.record("http.response.status_code", status_code);

                    if resp.status().is_success() {
                        Ok(resp)
                    } else {
                        current_span.record("error.type", "http_status_error");
                        Err(Error::from_api_response(module, resp).await)
                    }
                }
                Err(e) => {
                    current_span.record("error.type", "transport_error");
                    Err(e)
                }
            }
        }
        .instrument(span)
        .await
    }

    /// Executes an HTTP request and deserializes the JSON response.
    pub(crate) async fn execute<T: DeserializeOwned>(
        &self,
        module: &'static str,
        builder: RequestBuilder,
    ) -> Result<T> {
        let builder = builder.header(ACCEPT, HeaderValue::from_static("application/json"));
        let resp = self.execute_stream(module, builder).await?;
        let bytes = resp.bytes().await?;
        if bytes.is_empty() {
            return serde_json::from_value(serde_json::Value::Null).map_err(Error::Serialization);
        }
        serde_json::from_slice(&bytes).map_err(Error::Serialization)
    }

    /// Executes an HTTP request and discards the response body.
    pub(crate) async fn execute_unit(&self, module: &'static str, builder: RequestBuilder) -> Result<()> {
        let _ = self.execute_stream(module, builder).await?;
        Ok(())
    }
}
