use std::time::Duration;

use reqwest::{Method, StatusCode};

/// High-level category of an API, HTTP, or real-time transport error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    /// Client error: `400 Bad Request`.
    BadRequest,
    /// Authentication error: `401 Unauthorized`.
    Unauthorized,
    /// Authorization error: `403 Forbidden`.
    Forbidden,
    /// Resource not found: `404 Not Found`.
    NotFound,
    /// Resource conflict: `409 Conflict`.
    Conflict,
    /// Server error: `500`, `502`, `503`, or `504`.
    InternalServerError,
    /// Rate limit exceeded: `429 Too Many Requests`.
    RateLimited,
    /// Real-time Socket.IO connection or protocol error.
    SocketIo,
    /// Any other unclassified status code.
    Unknown,
}

/// Strongly typed domain error codes returned or classified from Immich API responses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ImmichErrorCode {
    /// The uploaded or referenced asset is a duplicate.
    DuplicateAsset,
    /// The specified asset could not be found.
    AssetNotFound,
    /// The specified album could not be found.
    AlbumNotFound,
    /// The specified user could not be found.
    UserNotFound,
    /// Authentication credentials (API key, password, or bearer token) are invalid.
    InvalidCredentials,
    /// Storage quota or disk space has been exceeded.
    InsufficientStorage,
    /// Upload payload size exceeds server configured limits.
    PayloadTooLarge,
    /// Rate limit or concurrency limit exceeded.
    RateLimitExceeded,
    /// Local or remote input validation failed.
    ValidationFailed,
    /// Resource already exists or conflict occurred.
    Conflict,
    /// Server maintenance or initialization in progress.
    ServerUnavailable,
}

impl From<StatusCode> for ErrorKind {
    fn from(status: StatusCode) -> Self {
        match status.as_u16() {
            400 => Self::BadRequest,
            401 => Self::Unauthorized,
            403 => Self::Forbidden,
            404 => Self::NotFound,
            409 => Self::Conflict,
            429 => Self::RateLimited,
            500 | 502 | 503 | 504 => Self::InternalServerError,
            _ => Self::Unknown,
        }
    }
}

/// Structured error details extracted from an Immich server error response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiErrorDetails {
    /// HTTP status code returned by the server.
    pub status: StatusCode,
    /// High-level classified error kind.
    pub kind: ErrorKind,
    /// Classified Immich domain error code, if recognized.
    pub error_code: Option<ImmichErrorCode>,
    /// Human-readable error message returned by the server.
    pub message: String,
    /// Immich correlation ID for tracking requests in server logs.
    pub correlation_id: Option<String>,
    /// Server-requested backoff duration (from `Retry-After` header), if any.
    pub retry_after: Option<Duration>,
}

impl std::fmt::Display for ApiErrorDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({:?}) - {}", self.status.as_u16(), self.kind, self.message)?;
        if let Some(ec) = self.error_code {
            write!(f, " [{ec:?}]")?;
        }
        if let Some(cid) = &self.correlation_id {
            write!(f, " [correlation_id: {cid}]")?;
        }
        Ok(())
    }
}

/// Comprehensive error type for all operations in `immich-rs`.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Error related to authentication or OAuth operations.
    #[error("Authentication error: {0}")]
    Auth(ApiErrorDetails),

    /// Error related to asset operations (upload, download, faces, stacks, metadata).
    #[error("Asset error: {0}")]
    Asset(ApiErrorDetails),

    /// Error related to album operations.
    #[error("Album error: {0}")]
    Album(ApiErrorDetails),

    /// Error related to system configuration operations.
    #[error("Config error: {0}")]
    Config(ApiErrorDetails),

    /// Error related to user administration operations.
    #[error("User error: {0}")]
    User(ApiErrorDetails),

    /// Generic API error in a specific Immich module.
    #[error("API error in {module}: {details}")]
    Api {
        /// Module where the error occurred.
        module: &'static str,
        /// Extracted API error details.
        details: ApiErrorDetails,
    },

    /// Network or HTTP transport error.
    #[error("Network or protocol error: {0}")]
    Transport(#[from] reqwest::Error),

    /// JSON serialization or deserialization failure.
    #[error("Data parsing error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Local filesystem I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Malformed base URL.
    #[error("invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    /// Local client-side parameter validation error.
    #[error("local validation error: {0}")]
    Validation(String),

    /// Real-time Socket.IO connection or protocol error.
    #[error("Socket.IO error: {0}")]
    SocketIo(String),
}

impl From<immich_models::common::error::ConversionError> for Error {
    fn from(err: immich_models::common::error::ConversionError) -> Self {
        Self::Validation(err.to_string())
    }
}

/// Specialized [`Result`](std::result::Result) type for `immich-rs` operations.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(serde::Deserialize, Default)]
#[serde(untagged)]
enum MessageField {
    #[default]
    Empty,
    Single(String),
    Multiple(Vec<String>),
}

#[derive(serde::Deserialize, Default)]
struct ApiErrorBody {
    #[serde(default)]
    message: MessageField,
    #[serde(default)]
    error: Option<String>,
    #[serde(default, rename = "correlationId")]
    correlation_id: Option<String>,
}

/// Classifies HTTP status code and server error message into a structured [`ImmichErrorCode`].
#[must_use]
pub fn classify_error_code(status: StatusCode, message: &str, error_field: Option<&str>) -> Option<ImmichErrorCode> {
    let lower_msg = message.to_ascii_lowercase();
    let lower_err = error_field.map(str::to_ascii_lowercase).unwrap_or_default();

    if lower_msg.contains("duplicate")
        || (status == StatusCode::CONFLICT && (lower_msg.contains("already exist") || lower_msg.contains("asset")))
    {
        return Some(ImmichErrorCode::DuplicateAsset);
    }
    if status == StatusCode::NOT_FOUND {
        if lower_msg.contains("asset") {
            return Some(ImmichErrorCode::AssetNotFound);
        }
        if lower_msg.contains("album") {
            return Some(ImmichErrorCode::AlbumNotFound);
        }
        if lower_msg.contains("user") {
            return Some(ImmichErrorCode::UserNotFound);
        }
    }
    if status == StatusCode::UNAUTHORIZED
        || lower_msg.contains("invalid credential")
        || lower_msg.contains("unauthorized")
    {
        return Some(ImmichErrorCode::InvalidCredentials);
    }
    if status == StatusCode::PAYLOAD_TOO_LARGE
        || lower_msg.contains("payload too large")
        || lower_msg.contains("file too large")
    {
        return Some(ImmichErrorCode::PayloadTooLarge);
    }
    if status == StatusCode::TOO_MANY_REQUESTS || lower_msg.contains("rate limit") {
        return Some(ImmichErrorCode::RateLimitExceeded);
    }
    if status == StatusCode::INSUFFICIENT_STORAGE
        || lower_msg.contains("quota exceeded")
        || lower_msg.contains("storage full")
    {
        return Some(ImmichErrorCode::InsufficientStorage);
    }
    if status == StatusCode::SERVICE_UNAVAILABLE {
        return Some(ImmichErrorCode::ServerUnavailable);
    }
    if status == StatusCode::CONFLICT {
        return Some(ImmichErrorCode::Conflict);
    }
    if status == StatusCode::BAD_REQUEST
        || status == StatusCode::UNPROCESSABLE_ENTITY
        || lower_err.contains("bad request")
    {
        return Some(ImmichErrorCode::ValidationFailed);
    }
    None
}

impl Error {
    pub(crate) async fn from_api_response(module: &'static str, resp: reqwest::Response) -> Self {
        let status = resp.status();
        let retry_after =
            crate::core::retry::parse_retry_after(resp.headers(), crate::core::retry::DEFAULT_MAX_RETRY_DELAY);

        let body_bytes = resp.bytes().await.unwrap_or_default();
        let body: Option<ApiErrorBody> = serde_json::from_slice(&body_bytes).ok();

        let (message, correlation_id, error_field) = if let Some(b) = body {
            let msg = match b.message {
                MessageField::Single(s) => s,
                MessageField::Multiple(v) => v.join(", "),
                MessageField::Empty => {
                    let text = String::from_utf8_lossy(&body_bytes);
                    if text.is_empty() {
                        "unknown error".to_string()
                    } else {
                        text.into_owned()
                    }
                }
            };
            (msg, b.correlation_id, b.error)
        } else {
            let text = String::from_utf8_lossy(&body_bytes);
            let msg = if text.is_empty() {
                "unknown error".to_string()
            } else {
                text.into_owned()
            };
            (msg, None, None)
        };

        let error_code = classify_error_code(status, &message, error_field.as_deref());

        let details = ApiErrorDetails {
            status,
            kind: status.into(),
            error_code,
            message,
            correlation_id,
            retry_after,
        };

        match module {
            "authentication" | "auth" | "oauth" => Self::Auth(details),
            s if s.starts_with("asset") || matches!(s, "download" | "faces" | "stacks") => Self::Asset(details),
            "albums" | "album" => Self::Album(details),
            s if s.starts_with("system/config") || s.starts_with("system_config") || s == "config" => {
                Self::Config(details)
            }
            s if s.starts_with("user") => Self::User(details),
            _ => Self::Api { module, details },
        }
    }

    /// Returns the underlying [`ApiErrorDetails`] if this error originated from an HTTP API response.
    #[must_use]
    pub const fn api_details(&self) -> Option<&ApiErrorDetails> {
        match self {
            Self::Auth(d)
            | Self::Asset(d)
            | Self::Album(d)
            | Self::Config(d)
            | Self::User(d)
            | Self::Api { details: d, .. } => Some(d),
            Self::Transport(_)
            | Self::Serialization(_)
            | Self::Io(_)
            | Self::InvalidUrl(_)
            | Self::Validation(_)
            | Self::SocketIo(_) => None,
        }
    }

    /// Returns the HTTP [`StatusCode`] associated with this error, if any.
    #[must_use]
    pub fn status(&self) -> Option<StatusCode> {
        if let Some(d) = self.api_details() {
            return Some(d.status);
        }
        if let Self::Transport(e) = self {
            return e.status();
        }
        None
    }

    /// Returns the classified [`ErrorKind`] for HTTP API or Socket.IO errors.
    #[must_use]
    pub const fn kind(&self) -> Option<ErrorKind> {
        match self {
            Self::SocketIo(_) => Some(ErrorKind::SocketIo),
            _ => match self.api_details() {
                Some(d) => Some(d.kind),
                None => None,
            },
        }
    }

    /// Returns the classified domain [`ImmichErrorCode`] if recognized.
    #[must_use]
    pub const fn error_code(&self) -> Option<ImmichErrorCode> {
        match self.api_details() {
            Some(d) => d.error_code,
            None => None,
        }
    }

    /// Returns `true` if the error indicates a duplicate asset.
    #[must_use]
    pub fn is_duplicate(&self) -> bool {
        self.error_code() == Some(ImmichErrorCode::DuplicateAsset)
    }

    /// Returns the server-requested `Retry-After` duration, if present.
    #[must_use]
    pub const fn retry_after(&self) -> Option<Duration> {
        match self.api_details() {
            Some(d) => d.retry_after,
            None => None,
        }
    }

    /// Returns the Immich server `correlationId` for this error, if present.
    #[must_use]
    pub fn correlation_id(&self) -> Option<&str> {
        self.api_details()?.correlation_id.as_deref()
    }

    /// Returns `true` if the error corresponds to HTTP `404 Not Found`.
    #[must_use]
    pub fn is_not_found(&self) -> bool {
        self.status() == Some(StatusCode::NOT_FOUND)
    }

    /// Returns `true` if the error corresponds to HTTP `401 Unauthorized`.
    #[must_use]
    pub fn is_unauthorized(&self) -> bool {
        self.status() == Some(StatusCode::UNAUTHORIZED)
    }

    /// Returns `true` if the error corresponds to HTTP `403 Forbidden`.
    #[must_use]
    pub fn is_forbidden(&self) -> bool {
        self.status() == Some(StatusCode::FORBIDDEN)
    }

    /// Returns `true` if the error corresponds to HTTP `429 Too Many Requests`.
    #[must_use]
    pub fn is_rate_limited(&self) -> bool {
        self.status() == Some(StatusCode::TOO_MANY_REQUESTS)
    }

    /// Returns `true` if the error corresponds to HTTP `409 Conflict`.
    #[must_use]
    pub fn is_conflict(&self) -> bool {
        self.status() == Some(StatusCode::CONFLICT)
    }

    /// Returns `true` if the error corresponds to an HTTP `5xx` server error.
    #[must_use]
    pub fn is_server_error(&self) -> bool {
        self.status().is_some_and(|s| s.is_server_error())
    }

    /// Returns `true` if the error was caused by a client or request timeout.
    #[must_use]
    pub fn is_timeout(&self) -> bool {
        match self {
            Self::Transport(e) => e.is_timeout(),
            _ => false,
        }
    }

    /// Returns `true` if the error is unconditionally transient and eligible for automatic retry
    /// regardless of HTTP method (`429`, `502`, `503`, `504`, `Retry-After`, or network timeout/connect).
    #[must_use]
    pub fn is_retryable(&self) -> bool {
        crate::core::retry::should_retry(self)
    }

    /// Returns `true` if the error is eligible for automatic retry under the given HTTP `method`
    /// (treating `500 Internal Server Error` as retryable only on idempotent methods).
    #[must_use]
    pub fn is_retryable_for_method(&self, method: &Method) -> bool {
        crate::core::retry::should_retry_for_method(self, Some(method))
    }
}
