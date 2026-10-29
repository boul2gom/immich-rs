use reqwest::header::{ACCEPT, HeaderValue};
use reqwest::{Method, RequestBuilder};
use serde::de::DeserializeOwned;
use tower_service::Service;

use crate::client::ImmichClient;
use crate::error::{Error, Result};

impl ImmichClient {
    /// Prepares a new HTTP request for the given module.
    pub(crate) fn prepare_request(&self, method: Method, path: &str) -> Result<RequestBuilder> {
        let safe_path = path.trim_start_matches('/');
        let mut req = self
            .inner
            .raw_http
            .request(method, self.inner.base_url.join(safe_path)?);

        match &self.inner.auth {
            crate::client::Auth::ApiKey(k) => req = req.header("x-api-key", k.clone()),
            crate::client::Auth::Bearer(t) => {
                req = req.header(reqwest::header::AUTHORIZATION, t.clone());
            }
            crate::client::Auth::None => {}
        }

        Ok(req)
    }

    /// Executes an HTTP request through the Tower execution stack with OpenTelemetry semantic spans.
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

        let _guard = span.enter();

        #[cfg(feature = "opentelemetry")]
        let start_time = std::time::Instant::now();

        let mut service = self.inner.http.clone();
        let result = service.call(request).await;

        #[cfg(feature = "opentelemetry")]
        {
            let elapsed = start_time.elapsed().as_secs_f64();
            let status = result.as_ref().map(|r| r.status().as_u16()).ok();
            crate::core::metrics::record_http_request(&method, status, elapsed);
        }

        match result {
            Ok(resp) => {
                let status_code = resp.status().as_u16();
                span.record("http.response.status_code", status_code);

                if resp.status().is_success() {
                    Ok(resp)
                } else {
                    span.record("error.type", "http_status_error");
                    Err(Error::from_api_response(module, resp).await)
                }
            }
            Err(e) => {
                span.record("error.type", "transport_error");
                Err(e)
            }
        }
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
