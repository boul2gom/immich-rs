#![cfg(not(target_arch = "wasm32"))]

mod common;

use anyhow::Result;
use pretty_assertions::assert_eq;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn retries_on_429_with_retry_after_and_then_succeeds() -> Result<()> {
    let (client, server) = common::mock_client(3).await;

    // Fail twice with 429 + Retry-After: 0, succeed on the third attempt
    Mock::given(method("GET"))
        .and(path("/server/ping"))
        .respond_with(ResponseTemplate::new(429).insert_header("Retry-After", "0"))
        .up_to_n_times(2)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/server/ping"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"res": "pong"})))
        .mount(&server)
        .await;

    let result = client.server().ping_server().await?;
    assert_eq!(result.res, "pong");
    Ok(())
}

#[tokio::test]
async fn retries_on_503() -> Result<()> {
    let (client, server) = common::mock_client(2).await;

    Mock::given(method("GET"))
        .and(path("/server/ping"))
        .respond_with(ResponseTemplate::new(503))
        .up_to_n_times(1)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/server/ping"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"res": "pong"})))
        .mount(&server)
        .await;

    let result = client.server().ping_server().await?;
    assert_eq!(result.res, "pong");
    Ok(())
}

#[tokio::test]
async fn does_not_retry_on_404() -> Result<()> {
    let (client, server) = common::mock_client(3).await;

    Mock::given(method("GET"))
        .and(path("/server/ping"))
        .respond_with(ResponseTemplate::new(404))
        .expect(1)
        .mount(&server)
        .await;

    let result = client.server().ping_server().await;
    assert!(result.is_err(), "expected error on 404");
    Ok(())
}

#[tokio::test]
async fn does_not_retry_on_401() -> Result<()> {
    let (client, server) = common::mock_client(3).await;

    Mock::given(method("GET"))
        .and(path("/server/ping"))
        .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
            "message": "Unauthorized",
            "statusCode": 401
        })))
        .expect(1)
        .mount(&server)
        .await;

    let result = client.server().ping_server().await;
    assert!(result.is_err(), "expected error on 401, got: {result:?}");
    Ok(())
}

#[tokio::test]
async fn internal_server_error_500_retried_on_idempotent_get_but_not_post() -> Result<()> {
    let (client, server) = common::mock_client(3).await;

    // 1. GET is idempotent: retries on 500
    Mock::given(method("GET"))
        .and(path("/server/ping"))
        .respond_with(ResponseTemplate::new(500))
        .up_to_n_times(1)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/server/ping"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"res": "pong"})))
        .mount(&server)
        .await;

    let result = client.server().ping_server().await?;
    assert_eq!(result.res, "pong");

    // 2. POST is non-idempotent: must NOT retry on 500
    Mock::given(method("POST"))
        .and(path("/albums"))
        .respond_with(ResponseTemplate::new(500).set_body_json(serde_json::json!({
            "message": "Database write failed",
            "statusCode": 500
        })))
        .expect(1)
        .mount(&server)
        .await;

    let create_payload = immich_models::albums::CreateAlbumDTO::new("Vacation");
    let post_result = client.albums().create_album(create_payload).await;
    assert!(post_result.is_err(), "POST should not be retried on 500");

    Ok(())
}

#[tokio::test]
async fn returns_error_when_all_retries_exhausted() -> Result<()> {
    let (client, server) = common::mock_client(2).await;

    Mock::given(method("GET"))
        .and(path("/server/ping"))
        .respond_with(ResponseTemplate::new(429).insert_header("Retry-After", "0"))
        .expect(3)
        .mount(&server)
        .await;

    let result = client.server().ping_server().await;
    assert!(result.is_err(), "expected error after exhausting retries");
    Ok(())
}
