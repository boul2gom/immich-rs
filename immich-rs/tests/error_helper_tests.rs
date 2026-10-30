#![cfg(not(target_arch = "wasm32"))]

use std::time::Duration;

use immich_rs::{ApiErrorDetails, Error, ErrorKind, ImmichErrorCode};
use pretty_assertions::assert_eq;
use reqwest::StatusCode;

fn make_api_err(status: StatusCode) -> Error {
    Error::Api {
        module: "test",
        details: ApiErrorDetails {
            status,
            kind: status.into(),
            error_code: None,
            message: "test message".to_string(),
            correlation_id: Some("corr-123".to_string()),
            retry_after: None,
        },
    }
}

fn make_api_err_with_retry(status: StatusCode, retry_after: Option<Duration>) -> Error {
    Error::Api {
        module: "test",
        details: ApiErrorDetails {
            status,
            kind: status.into(),
            error_code: None,
            message: "test message".to_string(),
            correlation_id: Some("corr-123".to_string()),
            retry_after,
        },
    }
}

#[test]
fn error_predicates_correctly_identify_status_codes() {
    let not_found = make_api_err(StatusCode::NOT_FOUND);
    assert!(not_found.is_not_found());
    assert!(!not_found.is_unauthorized());
    assert!(!not_found.is_retryable());
    assert_eq!(not_found.status(), Some(StatusCode::NOT_FOUND));
    assert_eq!(not_found.kind(), Some(ErrorKind::NotFound));
    assert_eq!(not_found.correlation_id(), Some("corr-123"));
    assert_eq!(not_found.retry_after(), None);

    let unauthorized = make_api_err(StatusCode::UNAUTHORIZED);
    assert!(unauthorized.is_unauthorized());
    assert!(!unauthorized.is_not_found());
    assert!(!unauthorized.is_retryable());

    let forbidden = make_api_err(StatusCode::FORBIDDEN);
    assert!(forbidden.is_forbidden());
    assert!(!forbidden.is_retryable());

    let bad_req = make_api_err(StatusCode::BAD_REQUEST);
    assert!(!bad_req.is_retryable());

    let conflict = make_api_err(StatusCode::CONFLICT);
    assert!(conflict.is_conflict());
    assert!(!conflict.is_retryable());

    let rate_limited = make_api_err_with_retry(StatusCode::TOO_MANY_REQUESTS, Some(Duration::from_secs(5)));
    assert!(rate_limited.is_rate_limited());
    assert!(rate_limited.is_retryable());
    assert_eq!(rate_limited.retry_after(), Some(Duration::from_secs(5)));

    let server_error = make_api_err(StatusCode::INTERNAL_SERVER_ERROR);
    assert!(server_error.is_server_error());

    let bad_gateway = make_api_err(StatusCode::BAD_GATEWAY);
    assert!(bad_gateway.is_server_error());
    assert!(bad_gateway.is_retryable());

    let service_unavail = make_api_err(StatusCode::SERVICE_UNAVAILABLE);
    assert!(service_unavail.is_server_error());
    assert!(service_unavail.is_retryable());

    let gateway_timeout = make_api_err(StatusCode::GATEWAY_TIMEOUT);
    assert!(gateway_timeout.is_server_error());
    assert!(gateway_timeout.is_retryable());
}

#[test]
fn non_api_errors_return_none_for_api_details() {
    let io_err = Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"));
    assert_eq!(io_err.status(), None);
    assert_eq!(io_err.kind(), None);
    assert_eq!(io_err.correlation_id(), None);
    assert_eq!(io_err.retry_after(), None);
    assert!(!io_err.is_not_found());
}

#[test]
fn error_code_classification_covers_domain_errors() {
    let check = |status: StatusCode, msg: &str, err_field: Option<&str>| {
        let details = ApiErrorDetails {
            status,
            kind: status.into(),
            error_code: immich_rs::error::classify_error_code(status, msg, err_field),
            message: msg.to_string(),
            correlation_id: None,
            retry_after: None,
        };
        Error::Api {
            module: "test",
            details,
        }
    };

    // Duplicate asset
    let dup_err = check(StatusCode::CONFLICT, "Duplicate asset detected", None);
    assert_eq!(dup_err.error_code(), Some(ImmichErrorCode::DuplicateAsset));
    assert!(dup_err.is_duplicate());

    // Asset not found
    let asset_404 = check(StatusCode::NOT_FOUND, "Asset not found with id 123", None);
    assert_eq!(asset_404.error_code(), Some(ImmichErrorCode::AssetNotFound));

    // Album not found
    let album_404 = check(StatusCode::NOT_FOUND, "Album not found", None);
    assert_eq!(album_404.error_code(), Some(ImmichErrorCode::AlbumNotFound));

    // User not found
    let user_404 = check(StatusCode::NOT_FOUND, "User not found", None);
    assert_eq!(user_404.error_code(), Some(ImmichErrorCode::UserNotFound));

    // Invalid credentials
    let auth_err = check(StatusCode::UNAUTHORIZED, "Invalid credentials provided", None);
    assert_eq!(auth_err.error_code(), Some(ImmichErrorCode::InvalidCredentials));

    // Payload too large
    let large_err = check(StatusCode::PAYLOAD_TOO_LARGE, "File too large", None);
    assert_eq!(large_err.error_code(), Some(ImmichErrorCode::PayloadTooLarge));

    // Rate limit
    let rate_err = check(StatusCode::TOO_MANY_REQUESTS, "Too many requests", None);
    assert_eq!(rate_err.error_code(), Some(ImmichErrorCode::RateLimitExceeded));

    // Storage full
    let storage_err = check(StatusCode::INSUFFICIENT_STORAGE, "Storage quota exceeded", None);
    assert_eq!(storage_err.error_code(), Some(ImmichErrorCode::InsufficientStorage));

    // Validation failed
    let val_err = check(StatusCode::BAD_REQUEST, "email must be an email", Some("Bad Request"));
    assert_eq!(val_err.error_code(), Some(ImmichErrorCode::ValidationFailed));

    // Server unavailable
    let server_unavail = check(StatusCode::SERVICE_UNAVAILABLE, "Server is starting", None);
    assert_eq!(server_unavail.error_code(), Some(ImmichErrorCode::ServerUnavailable));
}
