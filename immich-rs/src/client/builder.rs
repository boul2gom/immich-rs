use std::time::Duration;

use reqwest::Url;
use tower_layer::Layer;

use super::auth::Auth;
#[cfg(not(target_arch = "wasm32"))]
use super::tls::{ClientIdentity, RootCertInput};
use super::{BoxService, ClientState, ImmichClient};
use crate::core::RetryPolicy;
use crate::error::{Error, Result};

type LayerFn = Box<dyn FnOnce(BoxService) -> BoxService + Send + 'static>;

/// Builder for configuring and constructing an [`ImmichClient`].
///
/// Provides options for setting connection timeouts, retries, rate limiting, custom headers,
/// TLS certificate validation, mutual TLS (mTLS) client certificates, custom CA roots,
/// authentication credentials, and custom Tower layers.
#[must_use = "builder methods must be chained or stored"]
pub struct ImmichClientBuilder {
    base_url: Option<String>,
    auth: std::result::Result<Auth, String>,
    accept_invalid_certs: bool,
    #[cfg(not(target_arch = "wasm32"))]
    client_identity: std::result::Result<Option<ClientIdentity>, String>,
    #[cfg(not(target_arch = "wasm32"))]
    root_certificates: std::result::Result<Vec<RootCertInput>, String>,
    max_retries: Option<u32>,
    connect_timeout: Option<Duration>,
    global_timeout: Option<Duration>,
    user_agent: Option<String>,
    default_headers: Option<reqwest::header::HeaderMap>,
    layers: Vec<LayerFn>,
    rate_limit_per_second: Option<u32>,
    connector: Option<BoxService>,
}

impl Default for ImmichClientBuilder {
    fn default() -> Self {
        Self {
            base_url: None,
            auth: Ok(Auth::None),
            accept_invalid_certs: false,
            #[cfg(not(target_arch = "wasm32"))]
            client_identity: Ok(None),
            #[cfg(not(target_arch = "wasm32"))]
            root_certificates: Ok(Vec::new()),
            max_retries: None,
            connect_timeout: None,
            global_timeout: None,
            user_agent: None,
            default_headers: None,
            layers: Vec::new(),
            rate_limit_per_second: None,
            connector: None,
        }
    }
}

impl std::fmt::Debug for ImmichClientBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dbg = f.debug_struct("ImmichClientBuilder");
        dbg.field("base_url", &self.base_url)
            .field("auth", &self.auth)
            .field("accept_invalid_certs", &self.accept_invalid_certs);
        #[cfg(not(target_arch = "wasm32"))]
        dbg.field("client_identity", &self.client_identity)
            .field("root_certificates", &self.root_certificates);
        dbg.field("max_retries", &self.max_retries)
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
    /// - Base URL: reads `IMMICH_INSTANCE_URL`, `IMMICH_BASE_URL`, `IMMICH_URL`, or `IMMICH_SERVER_URL`.
    /// - API Key: reads `IMMICH_API_KEY`.
    /// - mTLS Client Identity (when the `fs` feature is enabled on native targets):
    ///   reads `IMMICH_CLIENT_CERT_PATH` and `IMMICH_CLIENT_KEY_PATH`.
    /// - Custom Root CA Certificate (when the `fs` feature is enabled on native targets):
    ///   reads `IMMICH_CA_CERT_PATH`.
    ///
    /// Empty or whitespace-only environment variables are ignored.
    ///
    /// # Errors
    ///
    /// Returns an error if no non-empty base URL environment variable is defined, or if only one
    /// of `IMMICH_CLIENT_CERT_PATH` or `IMMICH_CLIENT_KEY_PATH` is set.
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
    /// Returns an error if no non-empty base URL can be found from the lookup provider, or if
    /// only one of `IMMICH_CLIENT_CERT_PATH` or `IMMICH_CLIENT_KEY_PATH` is provided.
    pub fn from_env_map(mut self, mut lookup: impl FnMut(&str) -> Option<String>) -> Result<Self> {
        let non_empty = |v: String| (!v.trim().is_empty()).then_some(v);

        if self.base_url.is_none() {
            let url = lookup("IMMICH_INSTANCE_URL")
                .and_then(non_empty)
                .or_else(|| lookup("IMMICH_BASE_URL").and_then(non_empty))
                .or_else(|| lookup("IMMICH_URL").and_then(non_empty))
                .or_else(|| lookup("IMMICH_SERVER_URL").and_then(non_empty))
                .ok_or_else(|| {
                    Error::Validation(
                        "missing IMMICH_INSTANCE_URL, IMMICH_BASE_URL, IMMICH_URL, or IMMICH_SERVER_URL environment variable"
                            .into(),
                    )
                })?;
            self.base_url = Some(url.trim().to_string());
        }
        if matches!(self.auth, Ok(Auth::None))
            && let Some(key) = lookup("IMMICH_API_KEY").and_then(non_empty)
        {
            self = self.api_key(key.trim());
        }

        self = self.apply_tls_env_from_map(lookup)?;

        Ok(self)
    }

    #[cfg_attr(
        not(all(feature = "fs", not(target_arch = "wasm32"))),
        allow(unused_mut, clippy::unnecessary_wraps)
    )]
    pub(crate) fn apply_tls_env_from_map(mut self, mut lookup: impl FnMut(&str) -> Option<String>) -> Result<Self> {
        #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
        {
            let non_empty = |v: String| (!v.trim().is_empty()).then_some(v);

            if matches!(self.client_identity, Ok(None)) {
                let cert_path = lookup("IMMICH_CLIENT_CERT_PATH").and_then(non_empty);
                let key_path = lookup("IMMICH_CLIENT_KEY_PATH").and_then(non_empty);
                match (cert_path, key_path) {
                    (Some(cert), Some(key)) => {
                        self = self.client_identity_from_files(cert.trim(), key.trim());
                    }
                    (Some(_), None) | (None, Some(_)) => {
                        return Err(Error::Validation(
                            "both IMMICH_CLIENT_CERT_PATH and IMMICH_CLIENT_KEY_PATH must be set for mTLS".into(),
                        ));
                    }
                    (None, None) => {}
                }
            }

            if self.root_certificates.as_ref().is_ok_and(Vec::is_empty)
                && let Some(ca_path) = lookup("IMMICH_CA_CERT_PATH").and_then(non_empty)
            {
                self = self.add_root_certificate_from_file(ca_path.trim());
            }
        }
        #[cfg(not(all(feature = "fs", not(target_arch = "wasm32"))))]
        let _ = &mut lookup;

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
        self.auth = Auth::from_api_key(&key_str);
        self
    }

    /// Sets a bearer token for authentication.
    pub fn bearer_token(mut self, token: impl Into<String>) -> Self {
        let token_str = token.into();
        self.auth = Auth::from_bearer_token(&token_str);
        self
    }

    /// Allows accepting invalid TLS certificates (insecure).
    pub fn danger_accept_invalid_certs(mut self, accept: bool) -> Self {
        self.accept_invalid_certs = accept;
        self
    }

    /// Sets a client certificate chain and private key (both PEM-encoded) for mutual TLS (mTLS) authentication.
    ///
    /// Validation and parsing errors are deferred and reported when [`Self::build`] is called.
    ///
    /// - Under the `rustls` feature (default), RSA (`PKCS#1`), EC (`SEC1`), and `PKCS#8` PEM private keys are supported.
    /// - Under the `native-tls` feature (without `rustls`), the private key must be an unencrypted `PKCS#8` PEM key
    ///   (`-----BEGIN PRIVATE KEY-----`).
    ///
    /// Note: Real-time Socket.IO event streams (`subscribe_events`) use a separate WebSocket transport that
    /// does not currently inherit custom TLS identities.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use immich_rs::ImmichClient;
    /// # fn example(cert_pem: &[u8], key_pem: &[u8]) -> Result<(), immich_rs::Error> {
    /// let client = ImmichClient::builder()
    ///     .base_url("https://photos.example.com/api")
    ///     .api_key("your-api-key")
    ///     .client_identity_pem(cert_pem, key_pem)
    ///     .build()?;
    /// # let _ = client;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg_attr(docsrs, doc(cfg(not(target_arch = "wasm32"))))]
    pub fn client_identity_pem(mut self, cert_pem: impl AsRef<[u8]>, key_pem: impl AsRef<[u8]>) -> Self {
        self.client_identity = ClientIdentity::from_pem_pair(cert_pem.as_ref(), key_pem.as_ref()).map(Some);
        self
    }

    /// Sets a client certificate and private key from a single combined PEM bundle for mutual TLS (mTLS) authentication.
    ///
    /// The bundle must contain at least one PEM-encoded X.509 certificate and one PEM-encoded private key.
    /// Validation and parsing errors are deferred and reported when [`Self::build`] is called.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use immich_rs::ImmichClient;
    /// # fn example(bundle_pem: &[u8]) -> Result<(), immich_rs::Error> {
    /// let client = ImmichClient::builder()
    ///     .base_url("https://photos.example.com/api")
    ///     .api_key("your-api-key")
    ///     .client_identity_pem_bundle(bundle_pem)
    ///     .build()?;
    /// # let _ = client;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg_attr(docsrs, doc(cfg(not(target_arch = "wasm32"))))]
    pub fn client_identity_pem_bundle(mut self, pem_bundle: impl AsRef<[u8]>) -> Self {
        self.client_identity = ClientIdentity::from_pem_bundle(pem_bundle.as_ref()).map(Some);
        self
    }

    /// Loads a PEM-encoded client certificate chain and private key from the filesystem for mutual TLS (mTLS) authentication.
    ///
    /// File read or validation errors are deferred and reported when [`Self::build`] is called.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use immich_rs::ImmichClient;
    /// # fn example() -> Result<(), immich_rs::Error> {
    /// let client = ImmichClient::builder()
    ///     .base_url("https://photos.example.com/api")
    ///     .api_key("your-api-key")
    ///     .client_identity_from_files("/etc/certs/client.crt", "/etc/certs/client.key")
    ///     .build()?;
    /// # let _ = client;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    #[cfg_attr(docsrs, doc(cfg(all(feature = "fs", not(target_arch = "wasm32")))))]
    pub fn client_identity_from_files(
        mut self,
        cert_path: impl AsRef<std::path::Path>,
        key_path: impl AsRef<std::path::Path>,
    ) -> Self {
        self.client_identity = ClientIdentity::from_files(cert_path.as_ref(), key_path.as_ref()).map(Some);
        self
    }

    /// Sets a pre-constructed [`reqwest::Identity`] for mutual TLS (mTLS) authentication (e.g., from a PKCS#12 archive).
    ///
    /// Note: The supplied [`reqwest::Identity`] must match the active TLS backend (`reqwest::Identity::from_pem`
    /// when the `rustls` feature is enabled; `from_pkcs8_pem` or `from_pkcs12_der` when only `native-tls` is enabled).
    #[cfg(all(not(target_arch = "wasm32"), any(feature = "rustls", feature = "native-tls")))]
    #[cfg_attr(
        docsrs,
        doc(cfg(all(not(target_arch = "wasm32"), any(feature = "rustls", feature = "native-tls"))))
    )]
    pub fn client_identity(mut self, identity: reqwest::Identity) -> Self {
        self.client_identity = Ok(Some(ClientIdentity::Raw(identity)));
        self
    }

    /// Adds a custom root CA certificate (or PEM bundle of multiple CA certificates) to trust alongside system roots.
    ///
    /// Useful when connecting to a self-hosted Immich instance or reverse proxy signed by a private CA.
    /// Validation and parsing errors are deferred and reported when [`Self::build`] is called.
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg_attr(docsrs, doc(cfg(not(target_arch = "wasm32"))))]
    pub fn add_root_certificate_pem(mut self, ca_pem: impl AsRef<[u8]>) -> Self {
        if let Ok(ref mut certs) = self.root_certificates {
            match RootCertInput::from_pem(ca_pem.as_ref()) {
                Ok(cert) => certs.push(cert),
                Err(err) => self.root_certificates = Err(err),
            }
        }
        self
    }

    /// Adds a binary DER-encoded root CA certificate to trust alongside system roots.
    ///
    /// Validation and parsing errors are deferred and reported when [`Self::build`] is called.
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg_attr(docsrs, doc(cfg(not(target_arch = "wasm32"))))]
    pub fn add_root_certificate_der(mut self, ca_der: impl AsRef<[u8]>) -> Self {
        if let Ok(ref mut certs) = self.root_certificates {
            match RootCertInput::from_der(ca_der.as_ref()) {
                Ok(cert) => certs.push(cert),
                Err(err) => self.root_certificates = Err(err),
            }
        }
        self
    }

    /// Loads one or more custom root CA certificates from a file (PEM bundle or `.der`/`.cer` file) to trust alongside system roots.
    ///
    /// File read or parsing errors are deferred and reported when [`Self::build`] is called.
    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    #[cfg_attr(docsrs, doc(cfg(all(feature = "fs", not(target_arch = "wasm32")))))]
    pub fn add_root_certificate_from_file(mut self, ca_path: impl AsRef<std::path::Path>) -> Self {
        if let Ok(ref mut certs) = self.root_certificates {
            match RootCertInput::from_file(ca_path.as_ref()) {
                Ok(cert) => certs.push(cert),
                Err(err) => self.root_certificates = Err(err),
            }
        }
        self
    }

    /// Adds a pre-constructed [`reqwest::Certificate`] to trust alongside system roots.
    #[cfg(all(not(target_arch = "wasm32"), any(feature = "rustls", feature = "native-tls")))]
    #[cfg_attr(
        docsrs,
        doc(cfg(all(not(target_arch = "wasm32"), any(feature = "rustls", feature = "native-tls"))))
    )]
    pub fn add_root_certificate(mut self, cert: reqwest::Certificate) -> Self {
        if let Ok(ref mut certs) = self.root_certificates {
            certs.push(RootCertInput::Raw(cert));
        }
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
    /// Returns an error if the base URL, credentials, or TLS certificates are invalid, or if the HTTP client fails to build.
    pub fn build(self) -> Result<ImmichClient> {
        let raw_base_url = self
            .base_url
            .ok_or_else(|| Error::Validation("missing base_url".into()))?;

        let mut base_url = raw_base_url.trim().to_string();
        if !base_url.ends_with('/') {
            base_url.push('/');
        }
        let parsed_url = Url::parse(&base_url)?;
        if parsed_url.cannot_be_a_base() {
            return Err(Error::Validation(format!(
                "base_url cannot be a base URL: {parsed_url}"
            )));
        }

        let auth = self.auth.map_err(Error::Validation)?;
        #[cfg(not(target_arch = "wasm32"))]
        let client_identity = self.client_identity.map_err(Error::Validation)?;
        #[cfg(not(target_arch = "wasm32"))]
        let root_certificates = self.root_certificates.map_err(Error::Validation)?;

        let user_agent =
            self.user_agent
                .as_deref()
                .unwrap_or(concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")));

        let mut cb = reqwest::Client::builder().user_agent(user_agent);

        #[cfg(not(target_arch = "wasm32"))]
        {
            cb = cb.connect_timeout(self.connect_timeout.unwrap_or_else(|| Duration::from_secs(10)));
        }

        #[cfg(all(not(target_arch = "wasm32"), any(feature = "rustls", feature = "native-tls")))]
        {
            // Pin the TLS backend explicitly so additive `--all-features` builds keep `rustls`
            // as the primary backend instead of reqwest's `NativeTls` fallback.
            #[cfg(feature = "rustls")]
            {
                cb = cb.tls_backend_rustls();
            }
            #[cfg(all(feature = "native-tls", not(feature = "rustls")))]
            {
                cb = cb.tls_backend_native();
            }

            cb = cb.tls_danger_accept_invalid_certs(self.accept_invalid_certs);

            if !root_certificates.is_empty() {
                let certs = RootCertInput::resolve_all(root_certificates)?;
                cb = cb.tls_certs_merge(certs);
            }

            if let Some(identity) = client_identity {
                cb = identity.apply_to_builder(cb)?;
            }
        }
        #[cfg(target_arch = "wasm32")]
        let _ = (self.accept_invalid_certs, self.connect_timeout, self.global_timeout);

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

        let mut boxed = base_transport;
        for layer_fn in self.layers {
            boxed = layer_fn(boxed);
        }

        let service = match (self.rate_limit_per_second, retry_policy.max_attempts > 0) {
            (None, false) => boxed,
            (None, true) => {
                crate::core::BoxCloneSyncService::new(crate::core::RetryLayer::new(retry_policy.clone()).layer(boxed))
            }
            (Some(rps), false) => {
                crate::core::BoxCloneSyncService::new(crate::middleware::RateLimitLayer::new(rps).layer(boxed))
            }
            (Some(rps), true) => {
                let rate_limited = crate::middleware::RateLimitLayer::new(rps).layer(boxed);
                crate::core::BoxCloneSyncService::new(
                    crate::core::RetryLayer::new(retry_policy.clone()).layer(rate_limited),
                )
            }
        };

        Ok(ImmichClient {
            inner: std::sync::Arc::new(ClientState {
                http: service,
                raw_http,
                base_url: parsed_url,
                auth: std::sync::Arc::new(arc_swap::ArcSwap::from_pointee(auth)),
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
