use std::time::Duration;

use reqwest::Url;
use reqwest::header::HeaderValue;
use tower_layer::Layer;

use crate::core::RetryPolicy;
use crate::error::{Error, Result};

pub mod pagination_stream;
pub mod raw_request;

pub use pagination_stream::PaginationStream;
pub use raw_request::RawRequestBuilder;

#[derive(Clone, Default)]
pub(crate) enum Auth {
    ApiKey(HeaderValue),
    Bearer(HeaderValue),
    #[default]
    None,
}

impl Auth {
    fn from_api_key(key: &str) -> std::result::Result<Self, String> {
        if key.trim().is_empty() {
            return Err("API key cannot be empty".into());
        }
        let mut val = HeaderValue::try_from(key).map_err(|e| format!("invalid API key format: {e}"))?;
        val.set_sensitive(true);
        Ok(Self::ApiKey(val))
    }

    fn from_bearer_token(token: &str) -> std::result::Result<Self, String> {
        let trimmed = token.trim();
        if trimmed.is_empty() {
            return Err("Bearer token cannot be empty".into());
        }
        let raw_token = trimmed.strip_prefix("Bearer ").unwrap_or(trimmed);
        if raw_token.trim().is_empty() {
            return Err("Bearer token cannot be empty".into());
        }
        let mut val = HeaderValue::try_from(format!("Bearer {raw_token}"))
            .map_err(|e| format!("invalid Bearer token format: {e}"))?;
        val.set_sensitive(true);
        Ok(Self::Bearer(val))
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
    pub(crate) auth: std::sync::Arc<arc_swap::ArcSwap<Auth>>,
    pub(crate) retry_policy: RetryPolicy,
}

impl std::fmt::Debug for ClientState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientState")
            .field("raw_http", &self.raw_http)
            .field("base_url", &self.base_url)
            .field("auth", &**self.auth.load())
            .field("retry_policy", &self.retry_policy)
            .finish_non_exhaustive()
    }
}

/// An asynchronous, type-safe client for the Immich self-hosted photo and video management platform.
///
/// Constructed via [`ImmichClient::new`], [`ImmichClient::from_env`], or [`ImmichClient::builder`].
///
/// Cloning an `ImmichClient` is cheap (`O(1)`) and thread-safe as internal state is shared via [`Arc`](std::sync::Arc).
#[derive(Debug, Clone)]
pub struct ImmichClient {
    pub(crate) inner: std::sync::Arc<ClientState>,
}

impl ImmichClient {
    /// Creates a new `ImmichClient` with the given base URL and API key.
    ///
    /// For advanced configuration (timeouts, retries, rate limiting, headers, bearer token),
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
    /// Inspects the following variables (ignoring empty or whitespace-only values):
    /// - Base URL: `IMMICH_INSTANCE_URL`, `IMMICH_BASE_URL`, or `IMMICH_URL`.
    /// - API Key: `IMMICH_API_KEY`.
    ///
    /// # Errors
    ///
    /// Returns an error if no non-empty base URL environment variable is set, or if
    /// the client configuration fails validation.
    pub fn from_env() -> Result<Self> {
        Self::builder().from_env()?.build()
    }

    /// Creates a new builder for `ImmichClient`.
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
    pub fn api_key(&self) -> Option<String> {
        let auth = self.inner.auth.load();
        match auth.as_ref() {
            Auth::ApiKey(val) => val.to_str().ok().map(ToOwned::to_owned),
            Auth::Bearer(_) | Auth::None => None,
        }
    }

    /// Returns the configured raw Bearer token (without the `"Bearer "` prefix), if present.
    #[must_use]
    pub fn bearer_token(&self) -> Option<String> {
        let auth = self.inner.auth.load();
        match auth.as_ref() {
            Auth::Bearer(val) => val
                .to_str()
                .ok()
                .map(|s| s.strip_prefix("Bearer ").unwrap_or(s).to_string()),
            Auth::ApiKey(_) | Auth::None => None,
        }
    }

    /// Dynamically updates the Bearer token used for all subsequent requests on this client.
    ///
    /// Reads and swaps are atomic and lock-free across all threads.
    ///
    /// # Errors
    ///
    /// Returns an error if the token is empty or contains invalid header characters.
    pub fn set_bearer_token(&self, token: impl AsRef<str>) -> Result<()> {
        let auth = Auth::from_bearer_token(token.as_ref()).map_err(Error::Validation)?;
        self.inner.auth.store(std::sync::Arc::new(auth));
        Ok(())
    }

    /// Dynamically updates the API key used for all subsequent requests on this client.
    ///
    /// Reads and swaps are atomic and lock-free across all threads.
    ///
    /// # Errors
    ///
    /// Returns an error if the key is empty or contains invalid header characters.
    pub fn set_api_key(&self, key: impl AsRef<str>) -> Result<()> {
        let auth = Auth::from_api_key(key.as_ref()).map_err(Error::Validation)?;
        self.inner.auth.store(std::sync::Arc::new(auth));
        Ok(())
    }

    /// Authenticates with the Immich server using email and password, setting the resulting
    /// Bearer access token on this client instance in-place.
    ///
    /// Returns the server's [`LoginResponseDTO`](immich_models::authentication::LoginResponseDTO)
    /// containing user account information and session details.
    ///
    /// # Errors
    ///
    /// Returns an error if the login request fails (e.g. invalid credentials) or updating the token fails.
    pub async fn login(
        &self,
        email: impl Into<String>,
        password: impl Into<String>,
    ) -> Result<immich_models::authentication::LoginResponseDTO> {
        let creds = immich_models::authentication::LoginCredentialDTO::new(email, password);
        let resp = self.authentication().login(creds).send().await?;
        self.set_bearer_token(&resp.access_token)?;
        Ok(resp)
    }

    /// Creates and authenticates a new `ImmichClient` instance using email and password.
    ///
    /// # Errors
    ///
    /// Returns an error if client construction fails or authentication fails.
    pub async fn login_to(
        base_url: impl Into<String>,
        email: impl Into<String>,
        password: impl Into<String>,
    ) -> Result<Self> {
        let client = Self::builder().base_url(base_url).build()?;
        client.login(email, password).await?;
        Ok(client)
    }

    /// Creates a new `ImmichClient` by loading configuration from the official `@immich/cli` auth file.
    ///
    /// Reads `$IMMICH_CONFIG_DIR/auth.yml` or `~/.config/immich/auth.yml`.
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration file is missing, invalid YAML, or contains empty credentials.
    #[cfg(all(feature = "cli-config", not(target_arch = "wasm32")))]
    pub fn from_immich_cli() -> Result<Self> {
        let (url, key) = crate::config::cli::resolve_from_immich_cli()?;
        Self::new(url, key)
    }

    /// Creates a new `ImmichClient` by loading configuration from `immich-go` files.
    ///
    /// Checks `./immich-go.toml`, `./immich-go.yaml`, `./immich-go.json`, and `~/.config/immich-go/`.
    ///
    /// # Errors
    ///
    /// Returns an error if no `immich-go` configuration file is found or if parsing fails.
    #[cfg(all(feature = "cli-config", not(target_arch = "wasm32")))]
    pub fn from_immich_go() -> Result<Self> {
        let (url, key) = crate::config::cli::resolve_from_immich_go()?;
        Self::new(url, key)
    }

    /// Creates a new `ImmichClient` by cascading through environment variables and CLI configurations.
    ///
    /// Discovery priority:
    /// 1. Environment variables (`IMMICH_INSTANCE_URL` / `IMMICH_API_KEY`)
    /// 2. Local `immich-go` file in current working directory (`./immich-go.toml`, etc.)
    /// 3. Global `@immich/cli` configuration (`~/.config/immich/auth.yml`)
    /// 4. Global `immich-go` configuration (`~/.config/immich-go/`)
    ///
    /// # Errors
    ///
    /// Returns an error if no valid Immich instance URL and API key could be discovered from any source.
    #[cfg(all(feature = "cli-config", not(target_arch = "wasm32")))]
    pub fn from_env_or_config() -> Result<Self> {
        let (url, key) = crate::config::cli::resolve_env_or_config()?;
        Self::new(url, key)
    }

    /// Starts a low-level HTTP request against the Immich instance at the specified path.
    ///
    /// Dispatched requests traverse the client's full Tower middleware stack
    /// (authentication, tracing, retries, rate limiting).
    pub fn request(&self, method: reqwest::Method, path: impl Into<String>) -> RawRequestBuilder<'_> {
        RawRequestBuilder::new(self, method, path)
    }

    /// Shortcut to execute a low-level HTTP `GET` request.
    pub fn get(&self, path: impl Into<String>) -> RawRequestBuilder<'_> {
        self.request(reqwest::Method::GET, path)
    }

    /// Shortcut to execute a low-level HTTP `POST` request.
    pub fn post(&self, path: impl Into<String>) -> RawRequestBuilder<'_> {
        self.request(reqwest::Method::POST, path)
    }

    /// Shortcut to execute a low-level HTTP `PUT` request.
    pub fn put(&self, path: impl Into<String>) -> RawRequestBuilder<'_> {
        self.request(reqwest::Method::PUT, path)
    }

    /// Shortcut to execute a low-level HTTP `PATCH` request.
    pub fn patch(&self, path: impl Into<String>) -> RawRequestBuilder<'_> {
        self.request(reqwest::Method::PATCH, path)
    }

    /// Shortcut to execute a low-level HTTP `DELETE` request.
    pub fn delete(&self, path: impl Into<String>) -> RawRequestBuilder<'_> {
        self.request(reqwest::Method::DELETE, path)
    }
}

type LayerFn = Box<dyn FnOnce(BoxService) -> BoxService + Send + 'static>;

/// Builder for configuring and constructing an [`ImmichClient`].
///
/// Provides options for setting connection timeouts, retries, rate limiting, custom headers,
/// TLS certificate validation, authentication credentials, and custom Tower layers.
#[derive(Default)]
#[must_use = "builder methods must be chained or stored"]
pub struct ImmichClientBuilder {
    base_url: Option<String>,
    auth: Auth,
    auth_error: Option<String>,
    accept_invalid_certs: bool,
    max_retries: Option<u32>,
    connect_timeout: Option<Duration>,
    global_timeout: Option<Duration>,
    user_agent: Option<String>,
    default_headers: Option<reqwest::header::HeaderMap>,
    layers: Vec<LayerFn>,
    rate_limit_per_second: Option<u32>,
    connector: Option<BoxService>,
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
    /// Empty or whitespace-only environment variables are ignored.
    ///
    /// # Errors
    ///
    /// Returns an error if no non-empty base URL environment variable is defined.
    pub fn from_env(self) -> Result<Self> {
        self.from_env_map(|key| std::env::var(key).ok())
    }

    /// Populates unset builder options by querying a custom lookup function.
    ///
    /// Useful for testing or integrating with non-standard configuration stores
    /// without mutating the process environment. Empty or whitespace-only values are ignored.
    ///
    /// # Errors
    ///
    /// Returns an error if no non-empty base URL can be found from the lookup provider.
    pub fn from_env_map(mut self, mut lookup: impl FnMut(&str) -> Option<String>) -> Result<Self> {
        let non_empty = |v: String| (!v.trim().is_empty()).then_some(v);

        if self.base_url.is_none() {
            let url = lookup("IMMICH_INSTANCE_URL")
                .and_then(non_empty)
                .or_else(|| lookup("IMMICH_BASE_URL").and_then(non_empty))
                .or_else(|| lookup("IMMICH_URL").and_then(non_empty))
                .ok_or_else(|| {
                    Error::Validation(
                        "missing IMMICH_INSTANCE_URL, IMMICH_BASE_URL, or IMMICH_URL environment variable".into(),
                    )
                })?;
            self.base_url = Some(url);
        }
        if matches!(self.auth, Auth::None)
            && self.auth_error.is_none()
            && let Some(key) = lookup("IMMICH_API_KEY").and_then(non_empty)
        {
            self = self.api_key(key);
        }
        Ok(self)
    }

    /// Sets the base URL of the Immich instance (e.g., `https://photos.example.com/api`).
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Sets the API key for authentication.
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        let key_str = key.into();
        match Auth::from_api_key(&key_str) {
            Ok(auth) => {
                self.auth = auth;
                self.auth_error = None;
            }
            Err(err) => {
                self.auth = Auth::None;
                self.auth_error = Some(err);
            }
        }
        self
    }

    /// Sets a bearer token for authentication.
    pub fn bearer_token(mut self, token: impl Into<String>) -> Self {
        let token_str = token.into();
        match Auth::from_bearer_token(&token_str) {
            Ok(auth) => {
                self.auth = auth;
                self.auth_error = None;
            }
            Err(err) => {
                self.auth = Auth::None;
                self.auth_error = Some(err);
            }
        }
        self
    }

    /// Allows accepting invalid TLS certificates (insecure).
    pub fn danger_accept_invalid_certs(mut self, accept: bool) -> Self {
        self.accept_invalid_certs = accept;
        self
    }

    /// Sets the maximum number of retries for transient errors (defaults to `3`).
    pub fn max_retries(mut self, max: u32) -> Self {
        self.max_retries = Some(max);
        self
    }

    /// Sets the connection timeout for HTTP requests (defaults to `10s` on native targets).
    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = Some(timeout);
        self
    }

    /// Sets the global timeout for HTTP requests.
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

    /// Adds a custom Tower [`Layer`] into the client request execution pipeline.
    pub fn layer<L>(mut self, layer: L) -> Self
    where
        L: Layer<BoxService> + Send + 'static,
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
    /// Requests exceeding the limit asynchronously wait (smooth token-bucket pacing)
    /// before being dispatched to the server.
    pub fn rate_limit_per_second(mut self, requests_per_second: u32) -> Self {
        self.rate_limit_per_second = Some(requests_per_second);
        self
    }

    /// Sets a custom underlying transport service, replacing the default [`reqwest::Client`] network transport.
    ///
    /// This enables running in-memory mock tests or custom protocol bridges without binding network ports.
    pub fn connector<S>(mut self, service: S) -> Self
    where
        S: tower_service::Service<reqwest::Request, Response = reqwest::Response, Error = Error>
            + Clone
            + Send
            + Sync
            + 'static,
        S::Future: crate::MaybeSend + 'static,
    {
        self.connector = Some(crate::core::BoxCloneSyncService::new(service));
        self
    }

    /// Builds the [`ImmichClient`] synchronously without performing any implicit network I/O.
    ///
    /// To immediately verify server version compatibility upon startup, use [`Self::build_checked`].
    ///
    /// # Errors
    ///
    /// Returns an error if the base URL or credentials are invalid, or if the HTTP client fails to build.
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

        if let Some(err) = self.auth_error {
            return Err(Error::Validation(err));
        }

        let user_agent =
            self.user_agent
                .as_deref()
                .unwrap_or(concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")));

        let mut cb = reqwest::Client::builder().user_agent(user_agent);

        #[cfg(not(target_arch = "wasm32"))]
        {
            cb = cb.connect_timeout(self.connect_timeout.unwrap_or_else(|| Duration::from_secs(10)));
        }

        #[cfg(all(not(target_arch = "wasm32"), any(feature = "rustls-tls", feature = "native-tls")))]
        {
            cb = cb.danger_accept_invalid_certs(self.accept_invalid_certs);
        }
        #[cfg(target_arch = "wasm32")]
        let _ = self.accept_invalid_certs;
        #[cfg(target_arch = "wasm32")]
        let _ = self.connect_timeout;
        #[cfg(target_arch = "wasm32")]
        let _ = self.global_timeout;

        #[cfg(not(target_arch = "wasm32"))]
        if let Some(to) = self.global_timeout {
            cb = cb.timeout(to);
        }
        if let Some(headers) = self.default_headers {
            cb = cb.default_headers(headers);
        }

        let raw_http = cb.build()?;
        let retry_policy = RetryPolicy::default_with_max(self.max_retries.unwrap_or(3));
        let base_transport: BoxService = if let Some(custom) = self.connector {
            custom
        } else {
            crate::core::BoxCloneSyncService::new(crate::core::ReqwestTransport::new(raw_http.clone()))
        };

        let service = if self.layers.is_empty() {
            match (self.rate_limit_per_second, retry_policy.max_attempts > 0) {
                (None, false) => crate::core::BoxCloneSyncService::new(base_transport),
                (None, true) => crate::core::BoxCloneSyncService::new(
                    crate::core::RetryLayer::new(retry_policy.clone()).layer(base_transport),
                ),
                (Some(rps), false) => crate::core::BoxCloneSyncService::new(
                    crate::middleware::RateLimitLayer::new(rps).layer(base_transport),
                ),
                (Some(rps), true) => {
                    let rate_limited = crate::middleware::RateLimitLayer::new(rps).layer(base_transport);
                    crate::core::BoxCloneSyncService::new(
                        crate::core::RetryLayer::new(retry_policy.clone()).layer(rate_limited),
                    )
                }
            }
        } else {
            let mut boxed = crate::core::BoxCloneSyncService::new(base_transport);
            for layer_fn in self.layers {
                boxed = layer_fn(boxed);
            }
            match (self.rate_limit_per_second, retry_policy.max_attempts > 0) {
                (None, false) => boxed,
                (None, true) => crate::core::BoxCloneSyncService::new(
                    crate::core::RetryLayer::new(retry_policy.clone()).layer(boxed),
                ),
                (Some(rps), false) => {
                    crate::core::BoxCloneSyncService::new(crate::middleware::RateLimitLayer::new(rps).layer(boxed))
                }
                (Some(rps), true) => {
                    let rate_limited = crate::middleware::RateLimitLayer::new(rps).layer(boxed);
                    crate::core::BoxCloneSyncService::new(
                        crate::core::RetryLayer::new(retry_policy.clone()).layer(rate_limited),
                    )
                }
            }
        };

        Ok(ImmichClient {
            inner: std::sync::Arc::new(ClientState {
                http: service,
                raw_http,
                base_url: parsed_url,
                auth: std::sync::Arc::new(arc_swap::ArcSwap::from_pointee(self.auth)),
                retry_policy,
            }),
        })
    }

    /// Builds the `ImmichClient` and immediately performs an advisory compatibility check against the server.
    ///
    /// If the server version differs from the SDK target version, an advisory [`tracing::warn!`] is logged.
    /// The call succeeds even if the versions do not match, unless the network request itself fails.
    ///
    /// # Errors
    ///
    /// Returns an error if client construction fails or if the network request for the version check fails.
    pub async fn build_checked(self) -> Result<ImmichClient> {
        let client = self.build()?;
        let _ = client.check_compatibility().await?;
        Ok(client)
    }
}
