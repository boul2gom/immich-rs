use std::time::Duration;

use reqwest::Url;
use reqwest::header::HeaderValue;

use crate::core::RetryPolicy;
use crate::error::{Error, Result};

#[derive(Clone, Default)]
pub(crate) enum Auth {
    ApiKey(HeaderValue),
    Bearer(HeaderValue),
    #[default]
    None,
}

#[derive(Clone, Default)]
enum BuilderAuth {
    ApiKey(String),
    Bearer(String),
    #[default]
    None,
}

impl std::fmt::Debug for BuilderAuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ApiKey(_) => f.write_str("ApiKey(<redacted>)"),
            Self::Bearer(_) => f.write_str("Bearer(<redacted>)"),
            Self::None => f.write_str("None"),
        }
    }
}

impl std::fmt::Debug for Auth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ApiKey(_) => f.write_str("ApiKey(<redacted>)"),
            Self::Bearer(_) => f.write_str("Bearer(<redacted>)"),
            Self::None => f.write_str("None"),
        }
    }
}

#[cfg(all(
    not(target_arch = "wasm32"),
    not(any(feature = "rustls-tls", feature = "native-tls"))
))]
compile_error!(
    "immich-rs requires a TLS backend feature enabled: choose either `rustls-tls` (default) or `native-tls`."
);

/// Type alias for the boxed, cloneable Tower HTTP service pipeline.
pub type BoxService = crate::core::BoxCloneSyncService<reqwest::Request, reqwest::Response, Error>;

#[derive(Clone)]
pub(crate) struct ClientState {
    pub(crate) http: BoxService,
    pub(crate) raw_http: reqwest::Client,
    pub(crate) base_url: Url,
    pub(crate) auth: Auth,
    pub(crate) retry_policy: RetryPolicy,
}

impl std::fmt::Debug for ClientState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientState")
            .field("raw_http", &self.raw_http)
            .field("base_url", &self.base_url)
            .field("auth", &self.auth)
            .field("retry_policy", &self.retry_policy)
            .finish_non_exhaustive()
    }
}

/// An asynchronous, type-safe client for the Immich self-hosted photo backup server.
///
/// Constructed via [`ImmichClient::new`] or [`ImmichClient::builder`].
///
/// Cloning an `ImmichClient` is cheap and thread-safe as internal state is shared via an [`Arc`](std::sync::Arc).
#[derive(Debug, Clone)]
pub struct ImmichClient {
    pub(crate) inner: std::sync::Arc<ClientState>,
}

impl ImmichClient {
    /// Creates a new `ImmichClient` with the given base URL and API key.
    ///
    /// For advanced configuration (timeouts, retries, headers, bearer token),
    /// use [`ImmichClient::builder()`].
    ///
    /// # Errors
    ///
    /// Returns an error if the URL is invalid or credentials are empty.
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>) -> Result<Self> {
        Self::builder().base_url(base_url).api_key(api_key).build()
    }

    /// Creates a new `ImmichClient` initialized from environment variables.
    ///
    /// Inspects the following variables:
    /// - Base URL: `IMMICH_INSTANCE_URL`, `IMMICH_BASE_URL`, or `IMMICH_URL`.
    /// - API Key: `IMMICH_API_KEY`.
    ///
    /// # Errors
    ///
    /// Returns an error if the base URL environment variable is not set, or if
    /// the client configuration fails validation.
    pub fn from_env() -> Result<Self> {
        Self::builder().from_env()?.build()
    }

    /// Creates a new builder for `ImmichClient`.
    ///
    /// # Returns
    ///
    /// A builder for `ImmichClient`.
    pub fn builder() -> ImmichClientBuilder {
        ImmichClientBuilder::default()
    }

    /// Returns the normalized base URL configured for this client.
    #[must_use]
    pub fn base_url(&self) -> &Url {
        &self.inner.base_url
    }

    /// Returns a reference to the underlying raw [`reqwest::Client`].
    #[must_use]
    pub fn http_client(&self) -> &reqwest::Client {
        &self.inner.raw_http
    }

    /// Returns the configured API key, if present.
    #[must_use]
    pub fn api_key(&self) -> Option<&str> {
        match &self.inner.auth {
            Auth::ApiKey(val) => val.to_str().ok(),
            _ => None,
        }
    }

    /// Returns the configured Bearer token, if present.
    #[must_use]
    pub fn bearer_token(&self) -> Option<&str> {
        match &self.inner.auth {
            Auth::Bearer(val) => val.to_str().ok(),
            _ => None,
        }
    }
}

type LayerFn = Box<dyn FnOnce(BoxService) -> BoxService + Send + 'static>;

/// Builder for configuring and constructing an [`ImmichClient`].
///
/// Provides options for setting connection timeouts, retries, custom headers,
/// TLS certificate validation, authentication credentials, and custom Tower layers.
#[derive(Default)]
#[must_use = "builder methods must be chained or stored"]
pub struct ImmichClientBuilder {
    base_url: Option<String>,
    auth: BuilderAuth,
    accept_invalid_certs: bool,
    max_retries: Option<u32>,
    connect_timeout: Option<Duration>,
    global_timeout: Option<Duration>,
    user_agent: Option<String>,
    default_headers: Option<reqwest::header::HeaderMap>,
    warn_on_version_mismatch: bool,
    layers: Vec<LayerFn>,
    rate_limit_per_second: Option<u32>,
}

impl std::fmt::Debug for ImmichClientBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImmichClientBuilder")
            .field("base_url", &self.base_url)
            .field("auth", &self.auth)
            .field("accept_invalid_certs", &self.accept_invalid_certs)
            .field("max_retries", &self.max_retries)
            .field("connect_timeout", &self.connect_timeout)
            .field("global_timeout", &self.global_timeout)
            .field("user_agent", &self.user_agent)
            .field("default_headers", &self.default_headers)
            .field("warn_on_version_mismatch", &self.warn_on_version_mismatch)
            .field("layers_count", &self.layers.len())
            .field("rate_limit_per_second", &self.rate_limit_per_second)
            .finish_non_exhaustive()
    }
}

impl ImmichClientBuilder {
    /// Populates unset builder options from environment variables.
    ///
    /// - Base URL: reads `IMMICH_INSTANCE_URL`, `IMMICH_BASE_URL`, or `IMMICH_URL`.
    /// - API Key: reads `IMMICH_API_KEY`.
    ///
    /// # Errors
    ///
    /// Returns an error if the base URL environment variable is not defined or is not valid UTF-8.
    pub fn from_env(self) -> Result<Self> {
        self.from_env_map(|key| std::env::var(key).ok())
    }

    /// Populates unset builder options by querying a custom lookup function.
    ///
    /// Useful for testing or integrating with non-standard configuration stores
    /// without mutating the process environment.
    ///
    /// # Errors
    ///
    /// Returns an error if no base URL can be found from the lookup provider.
    pub fn from_env_map(mut self, mut lookup: impl FnMut(&str) -> Option<String>) -> Result<Self> {
        if self.base_url.is_none() {
            let url = lookup("IMMICH_INSTANCE_URL")
                .or_else(|| lookup("IMMICH_BASE_URL"))
                .or_else(|| lookup("IMMICH_URL"))
                .ok_or_else(|| {
                    Error::Validation("Missing IMMICH_INSTANCE_URL or IMMICH_BASE_URL environment variable".into())
                })?;
            self.base_url = Some(url);
        }
        if matches!(self.auth, BuilderAuth::None)
            && let Some(key) = lookup("IMMICH_API_KEY")
        {
            self.auth = BuilderAuth::ApiKey(key);
        }
        Ok(self)
    }

    /// Sets the base URL of the Immich instance.
    ///
    /// # Parameters
    ///
    /// - `url` — The base URL of the Immich server API (e.g., `http://localhost:2283/api`).
    ///
    /// # Returns
    ///
    /// The updated builder for chaining.
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Sets the API key for authentication.
    ///
    /// # Parameters
    ///
    /// - `key` — The API key to use for requests.
    ///
    /// # Returns
    ///
    /// The updated builder for chaining.
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.auth = BuilderAuth::ApiKey(key.into());
        self
    }

    /// Sets a bearer token for authentication.
    ///
    /// # Parameters
    ///
    /// - `token` — The bearer token to use for requests.
    ///
    /// # Returns
    ///
    /// The updated builder for chaining.
    pub fn bearer_token(mut self, token: impl Into<String>) -> Self {
        self.auth = BuilderAuth::Bearer(token.into());
        self
    }

    /// Allows accepting invalid TLS certificates (insecure).
    ///
    /// # Parameters
    ///
    /// - `accept` — Whether to accept invalid certificates.
    ///
    /// # Returns
    ///
    /// The updated builder for chaining.
    pub fn danger_accept_invalid_certs(mut self, accept: bool) -> Self {
        self.accept_invalid_certs = accept;
        self
    }

    /// Sets the maximum number of retries for transient errors.
    ///
    /// # Parameters
    ///
    /// - `max` — The maximum number of retry attempts.
    ///
    /// # Returns
    ///
    /// The updated builder for chaining.
    pub fn max_retries(mut self, max: u32) -> Self {
        self.max_retries = Some(max);
        self
    }

    /// Sets the connection timeout for HTTP requests.
    ///
    /// # Parameters
    ///
    /// - `timeout` — The duration to wait before timing out a connection.
    ///
    /// # Returns
    ///
    /// The updated builder for chaining.
    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = Some(timeout);
        self
    }

    /// Sets the global timeout for HTTP requests.
    ///
    /// # Parameters
    ///
    /// - `timeout` — The duration to wait for a request to complete.
    ///
    /// # Returns
    ///
    /// The updated builder for chaining.
    pub fn global_timeout(mut self, timeout: Duration) -> Self {
        self.global_timeout = Some(timeout);
        self
    }

    /// Sets a custom `User-Agent` header for all HTTP requests.
    ///
    /// Defaults to `immich-rs/<version>`.
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Sets default HTTP headers sent with every request (e.g., Cloudflare Access or proxy headers).
    pub fn default_headers(mut self, headers: reqwest::header::HeaderMap) -> Self {
        self.default_headers = Some(headers);
        self
    }

    /// Sets whether to perform an advisory compatibility check against the server version.
    pub fn warn_on_version_mismatch(mut self, enabled: bool) -> Self {
        self.warn_on_version_mismatch = enabled;
        self
    }

    /// Adds a custom Tower [`Layer`](tower_layer::Layer) into the client request execution pipeline.
    pub fn layer<L>(mut self, layer: L) -> Self
    where
        L: tower_layer::Layer<BoxService> + Send + 'static,
        L::Service: tower_service::Service<reqwest::Request, Response = reqwest::Response, Error = Error>
            + Clone
            + Send
            + Sync
            + 'static,
        <L::Service as tower_service::Service<reqwest::Request>>::Future: crate::MaybeSend + 'static,
    {
        self.layers.push(Box::new(move |svc| {
            crate::core::BoxCloneSyncService::new(layer.layer(svc))
        }));
        self
    }

    /// Sets client-side rate limiting to at most `requests_per_second`.
    ///
    /// Requests exceeding the limit will asynchronously wait (smooth throttling / pacing)
    /// before being sent to the server.
    pub fn rate_limit_per_second(mut self, requests_per_second: u32) -> Self {
        self.rate_limit_per_second = Some(requests_per_second);
        self
    }

    /// Builds the `ImmichClient`.
    ///
    /// # Returns
    ///
    /// A configured `ImmichClient`.
    ///
    /// # Errors
    ///
    /// Returns an error if the base URL is missing or invalid, or if the HTTP client fails to build.
    pub fn build(self) -> Result<ImmichClient> {
        let mut base_url = self
            .base_url
            .ok_or_else(|| Error::Validation("missing base_url".into()))?;

        if !base_url.ends_with('/') {
            base_url.push('/');
        }
        let parsed_url = Url::parse(&base_url)?;
        if parsed_url.cannot_be_a_base() {
            return Err(Error::Validation(format!(
                "base_url cannot be a base URL: {parsed_url}"
            )));
        }

        let auth = match self.auth {
            BuilderAuth::ApiKey(key) => {
                if key.trim().is_empty() {
                    return Err(Error::Validation("API key cannot be empty".into()));
                }
                let mut val = HeaderValue::try_from(key)
                    .map_err(|e| Error::Validation(format!("Invalid API key format: {e}")))?;
                val.set_sensitive(true);
                Auth::ApiKey(val)
            }
            BuilderAuth::Bearer(token) => {
                if token.trim().is_empty() {
                    return Err(Error::Validation("Bearer token cannot be empty".into()));
                }
                let mut val = HeaderValue::try_from(format!("Bearer {token}"))
                    .map_err(|e| Error::Validation(format!("Invalid Bearer token format: {e}")))?;
                val.set_sensitive(true);
                Auth::Bearer(val)
            }
            BuilderAuth::None => Auth::None,
        };

        let user_agent =
            self.user_agent
                .as_deref()
                .unwrap_or(concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")));

        let mut cb = reqwest::Client::builder().user_agent(user_agent);

        #[cfg(not(target_arch = "wasm32"))]
        {
            cb = cb.connect_timeout(self.connect_timeout.unwrap_or_else(|| Duration::from_secs(10)));
        }

        #[cfg(any(feature = "rustls-tls", feature = "native-tls"))]
        {
            cb = cb.danger_accept_invalid_certs(self.accept_invalid_certs);
        }

        #[cfg(not(target_arch = "wasm32"))]
        if let Some(to) = self.global_timeout {
            cb = cb.timeout(to);
        }
        if let Some(headers) = self.default_headers {
            cb = cb.default_headers(headers);
        }

        let raw_http = cb.build()?;
        let mut service = crate::core::BoxCloneSyncService::new(crate::core::ReqwestTransport::new(raw_http.clone()));

        for layer_fn in self.layers {
            service = layer_fn(service);
        }

        if let Some(rps) = self.rate_limit_per_second {
            use tower_layer::Layer;
            service = crate::core::BoxCloneSyncService::new(crate::middleware::RateLimitLayer::new(rps).layer(service));
        }

        let retry_policy = RetryPolicy::default_with_max(self.max_retries.unwrap_or(3));
        {
            use tower_layer::Layer;
            service = crate::core::BoxCloneSyncService::new(
                crate::core::RetryLayer::new(retry_policy.clone()).layer(service),
            );
        }

        let client = ImmichClient {
            inner: std::sync::Arc::new(ClientState {
                http: service,
                raw_http,
                base_url: parsed_url,
                auth,
                retry_policy,
            }),
        };

        #[cfg(not(target_arch = "wasm32"))]
        if self.warn_on_version_mismatch {
            let client_clone = client.clone();
            if let Ok(handle) = tokio::runtime::Handle::try_current() {
                handle.spawn(async move {
                    let _ = client_clone.check_compatibility().await;
                });
            }
        }
        #[cfg(target_arch = "wasm32")]
        if self.warn_on_version_mismatch {
            let client_clone = client.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let _ = client_clone.check_compatibility().await;
            });
        }

        Ok(client)
    }

    /// Builds the `ImmichClient` and immediately performs an advisory compatibility check against the server.
    ///
    /// If the server version differs from the SDK target version, an advisory [`tracing::warn!`] is logged.
    /// The call will succeed even if the versions do not match.
    ///
    /// # Errors
    /// Returns an error if client construction fails or if the network request for version check fails.
    pub async fn build_checked(self) -> Result<ImmichClient> {
        let client = self.build()?;
        let _ = client.check_compatibility().await;
        Ok(client)
    }
}
