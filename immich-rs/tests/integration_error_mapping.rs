#![cfg(not(target_arch = "wasm32"))]

mod common;

use std::time::Duration;

use anyhow::Result;
use immich_rs::{Error, ErrorKind};
use pretty_assertions::assert_eq;
use reqwest::StatusCode;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

async fn mount_get(server: &wiremock::MockServer, route: &str, response: ResponseTemplate) {
    Mock::given(method("GET"))
        .and(path(route))
        .respond_with(response)
        .mount(server)
        .await;
}

#[tokio::test]
async fn tags_404_maps_to_generic_api_error_with_module_name() -> Result<()> {
    let (client, server) = common::mock_client(0).await;
    mount_get(
        &server,
        "/tags",
        ResponseTemplate::new(404).set_body_json(serde_json::json!({"message": "Not here"})),
    )
    .await;

    let err = client.tags().get_all_tags().await.unwrap_err();
    match &err {
        Error::Api { module, details } => {
            assert_eq!(*module, "tags");
            assert_eq!(details.status, StatusCode::NOT_FOUND);
            assert_eq!(details.kind, ErrorKind::NotFound);
            assert_eq!(details.message, "Not here");
        }
        other => panic!("expected Error::Api, got {other:?}"),
    }
    assert!(err.is_not_found());
    assert_eq!(err.status(), Some(StatusCode::NOT_FOUND));
    assert_eq!(err.kind(), Some(ErrorKind::NotFound));
    Ok(())
}

#[tokio::test]
async fn message_array_is_joined_and_correlation_id_is_exposed() -> Result<()> {
    let (client, server) = common::mock_client(0).await;
    mount_get(
        &server,
        "/tags",
        ResponseTemplate::new(400).set_body_json(serde_json::json!({
            "message": ["name must be a string", "color is invalid"],
            "correlationId": "abc-123"
        })),
    )
    .await;

    let err = client.tags().get_all_tags().await.unwrap_err();
    let details = err.api_details().expect("API error details");
    assert_eq!(details.kind, ErrorKind::BadRequest);
    assert_eq!(details.message, "name must be a string, color is invalid");
    assert_eq!(err.correlation_id(), Some("abc-123"));
    assert!(err.to_string().contains("[correlation_id: abc-123]"));
    Ok(())
}

#[tokio::test]
async fn non_json_proxy_error_keeps_raw_body_text() -> Result<()> {
    let (client, server) = common::mock_client(0).await;
    mount_get(
        &server,
        "/tags",
        ResponseTemplate::new(502).set_body_string("<html>Bad Gateway</html>"),
    )
    .await;

    let err = client.tags().get_all_tags().await.unwrap_err();
    let details = err.api_details().expect("API error details");
    assert_eq!(details.status, StatusCode::BAD_GATEWAY);
    assert_eq!(details.kind, ErrorKind::InternalServerError);
    assert_eq!(details.message, "<html>Bad Gateway</html>");
    assert!(err.is_server_error());
    assert!(err.is_retryable());
    Ok(())
}

#[tokio::test]
async fn empty_error_body_falls_back_to_unknown_error() -> Result<()> {
    let (client, server) = common::mock_client(0).await;
    mount_get(&server, "/tags", ResponseTemplate::new(418)).await;

    let err = client.tags().get_all_tags().await.unwrap_err();
    let details = err.api_details().expect("API error details");
    assert_eq!(details.kind, ErrorKind::Unknown);
    assert_eq!(details.message, "unknown error");
    Ok(())
}

#[tokio::test]
async fn retry_after_header_is_parsed_and_capped_at_sixty_seconds() -> Result<()> {
    let (client, server) = common::mock_client(0).await;
    mount_get(
        &server,
        "/tags",
        ResponseTemplate::new(429).insert_header("retry-after", "86400"),
    )
    .await;

    let err = client.tags().get_all_tags().await.unwrap_err();
    assert!(err.is_rate_limited());
    assert_eq!(err.retry_after(), Some(Duration::from_secs(60)));
    Ok(())
}

#[tokio::test]
async fn config_and_user_modules_map_to_dedicated_variants() -> Result<()> {
    let (client, server) = common::mock_client(0).await;
    mount_get(&server, "/users/me", ResponseTemplate::new(403)).await;
    mount_get(&server, "/system-config", ResponseTemplate::new(403)).await;

    let user_err = client.users().get_my_user().await.unwrap_err();
    assert!(matches!(user_err, Error::User(_)));
    assert!(user_err.is_forbidden());

    #[allow(deprecated)]
    let config_err = client.system().config().get_config().await.unwrap_err();
    assert!(matches!(config_err, Error::Config(_)), "got {config_err:?}");
    Ok(())
}

#[tokio::test]
async fn malformed_success_body_maps_to_serialization_error() -> Result<()> {
    let (client, server) = common::mock_client(0).await;
    mount_get(
        &server,
        "/server/ping",
        ResponseTemplate::new(200).set_body_string("not json"),
    )
    .await;

    let err = client.server().ping_server().await.unwrap_err();
    assert!(matches!(err, Error::Serialization(_)), "got {err:?}");
    assert_eq!(err.status(), None);
    assert!(!err.is_not_found());
    Ok(())
}

#[test]
fn error_kind_classification_covers_gateway_statuses() {
    for (status, kind) in [
        (400, ErrorKind::BadRequest),
        (401, ErrorKind::Unauthorized),
        (403, ErrorKind::Forbidden),
        (404, ErrorKind::NotFound),
        (409, ErrorKind::Conflict),
        (429, ErrorKind::RateLimited),
        (500, ErrorKind::InternalServerError),
        (502, ErrorKind::InternalServerError),
        (503, ErrorKind::InternalServerError),
        (504, ErrorKind::InternalServerError),
        (418, ErrorKind::Unknown),
    ] {
        let code = StatusCode::from_u16(status).unwrap();
        assert_eq!(ErrorKind::from(code), kind, "status {status}");
    }
}

#[test]
fn conversion_error_converts_into_validation_error() {
    let conversion = immich_models::common::error::ConversionError::from("bad conversion");
    let err: Error = conversion.into();
    assert!(matches!(err, Error::Validation(msg) if msg.contains("bad conversion")));
}
