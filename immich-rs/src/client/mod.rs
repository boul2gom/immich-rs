use reqwest::Url;

use crate::core::RetryPolicy;
use crate::error::{Error, Result};

mod auth;
mod builder;
pub mod pagination_stream;
pub mod raw_request;
#[cfg(not(target_arch = "wasm32"))]
mod tls;

pub(crate) use auth::Auth;
pub use builder::ImmichClientBuilder;
pub use pagination_stream::PaginationStream;
pub use raw_request::RawRequestBuilder;

#[cfg(all(not(target_arch = "wasm32"), not(any(feature = "rustls", feature = "native-tls"))))]
compile_error!("immich-rs requires a TLS backend feature enabled: choose either `rustls` (default) or `native-tls`.");

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
    /// For advanced configuration (timeouts, retries, rate limiting, headers, bearer token, mTLS),
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
    /// - Base URL: `IMMICH_INSTANCE_URL`, `IMMICH_BASE_URL`, `IMMICH_URL`, or `IMMICH_SERVER_URL`.
    /// - API Key: `IMMICH_API_KEY`.
    /// - mTLS Client Identity (when `fs` feature is enabled): `IMMICH_CLIENT_CERT_PATH` and `IMMICH_CLIENT_KEY_PATH`.
    /// - Custom Root CA Certificate (when `fs` feature is enabled): `IMMICH_CA_CERT_PATH`.
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
    #[cfg_attr(docsrs, doc(cfg(all(feature = "cli-config", not(target_arch = "wasm32")))))]
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
    #[cfg_attr(docsrs, doc(cfg(all(feature = "cli-config", not(target_arch = "wasm32")))))]
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
    /// Optional mTLS and custom CA environment variables (`IMMICH_CLIENT_CERT_PATH`,
    /// `IMMICH_CLIENT_KEY_PATH`, `IMMICH_CA_CERT_PATH`) are also applied when `fs` is enabled.
    ///
    /// # Errors
    ///
    /// Returns an error if no valid Immich instance URL and API key could be discovered from any source.
    #[cfg(all(feature = "cli-config", not(target_arch = "wasm32")))]
    #[cfg_attr(docsrs, doc(cfg(all(feature = "cli-config", not(target_arch = "wasm32")))))]
    pub fn from_env_or_config() -> Result<Self> {
        let (url, key) = crate::config::cli::resolve_env_or_config()?;
        Self::builder()
            .base_url(url)
            .api_key(key)
            .apply_tls_env_from_map(|k| std::env::var(k).ok())?
            .build()
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
