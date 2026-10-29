use std::time::Duration;

use immich_rs::{ApiErrorDetails, Error, ErrorKind};
use pretty_assertions::assert_eq;
use reqwest::StatusCode;

#[test]
fn error_predicates_correctly_identify_status_codes() {
    let make_api_err = |status: StatusCode| Error::Api {
        module: "test",
        details: ApiErrorDetails {
            status,
            kind: status.into(),
            error_code: None,
            message: "test message".to_string(),
            correlation_id: Some("corr-123".to_string()),
            retry_after: Some(Duration::from_secs(5)),
        },
    };

    let not_found = make_api_err(StatusCode::NOT_FOUND);
    assert!(not_found.is_not_found());
    assert!(!not_found.is_unauthorized());
    assert_eq!(not_found.status(), Some(StatusCode::NOT_FOUND));
    assert_eq!(not_found.kind(), Some(ErrorKind::NotFound));
    assert_eq!(not_found.correlation_id(), Some("corr-123"));
    assert_eq!(not_found.retry_after(), Some(Duration::from_secs(5)));

    let unauthorized = make_api_err(StatusCode::UNAUTHORIZED);
    assert!(unauthorized.is_unauthorized());
    assert!(!unauthorized.is_not_found());

    let forbidden = make_api_err(StatusCode::FORBIDDEN);
    assert!(forbidden.is_forbidden());

    let conflict = make_api_err(StatusCode::CONFLICT);
    assert!(conflict.is_conflict());

    let rate_limited = make_api_err(StatusCode::TOO_MANY_REQUESTS);
    assert!(rate_limited.is_rate_limited());
    assert!(rate_limited.is_retryable());

    let server_error = make_api_err(StatusCode::INTERNAL_SERVER_ERROR);
    assert!(server_error.is_server_error());

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
    use immich_rs::ImmichErrorCode;

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
