mod common;

use anyhow::Result;
use pretty_assertions::assert_eq;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, ResponseTemplate};

/// A successful ping verifies the server can be reached without any credentials.
#[tokio::test]
async fn ping_server_without_credentials_sends_no_auth_headers() -> Result<()> {
    let (client, server) = common::mock_unauthenticated_client(0).await;

    Mock::given(method("GET"))
        .and(path("/server/ping"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"res": "pong"})))
        .mount(&server)
        .await;

    let ping = client.server().ping_server().await?;
    assert_eq!(ping.res, "pong");

    let requests = server.received_requests().await.expect("recording enabled");
    assert_eq!(requests.len(), 1);
    assert!(requests[0].headers.get("x-api-key").is_none());
    assert!(requests[0].headers.get("authorization").is_none());
    Ok(())
}

/// Credentials must never leak through `Debug` of the client or its builder.
#[test]
fn debug_output_redacts_credentials() -> Result<()> {
    let builder = immich_rs::ImmichClient::builder()
        .base_url("http://localhost:2283/api/")
        .api_key("super-secret-api-key");
    let builder_debug = format!("{builder:?}");
    assert!(!builder_debug.contains("super-secret-api-key"));
    assert!(builder_debug.contains("<redacted>"));

    let client = builder.build()?;
    let client_debug = format!("{client:?}");
    assert!(!client_debug.contains("super-secret-api-key"));

    let bearer = immich_rs::ImmichClient::builder()
        .base_url("http://localhost:2283/api/")
        .bearer_token("super-secret-jwt")
        .build()?;
    assert!(!format!("{bearer:?}").contains("super-secret-jwt"));
    Ok(())
}

/// Empty credentials are rejected at build time instead of sending blank headers.
#[test]
fn empty_credentials_are_rejected() {
    let empty_key = immich_rs::ImmichClient::builder()
        .base_url("http://localhost:2283/api/")
        .api_key("  ")
        .build();
    assert!(matches!(empty_key, Err(immich_rs::Error::Validation(_))));

    let empty_token = immich_rs::ImmichClient::builder()
        .base_url("http://localhost:2283/api/")
        .bearer_token("")
        .build();
    assert!(matches!(empty_token, Err(immich_rs::Error::Validation(_))));
}

/// The default `User-Agent` identifies the SDK and the API key header is sent.
#[tokio::test]
async fn requests_carry_default_user_agent_and_api_key() -> Result<()> {
    let (client, server) = common::mock_client(0).await;

    Mock::given(method("GET"))
        .and(path("/server/ping"))
        .and(header("x-api-key", "test-api-key-00000000-0000-0000-0000-000000000000"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"res": "pong"})))
        .expect(1)
        .mount(&server)
        .await;

    client.server().ping_server().await?;

    let requests = server.received_requests().await.expect("recording enabled");
    let user_agent = requests[0]
        .headers
        .get("user-agent")
        .expect("user-agent header")
        .to_str()?
        .to_owned();
    assert!(user_agent.starts_with("immich-rs/"), "got {user_agent}");
    Ok(())
}

/// Verifies that bearer token authentication sends the `Authorization: Bearer <token>` header.
#[tokio::test]
async fn bearer_token_auth_sends_authorization_header() -> Result<()> {
    let server = wiremock::MockServer::start().await;
    let client = immich_rs::ImmichClient::builder()
        .base_url(server.uri())
        .bearer_token("secret-jwt-token")
        .max_retries(0)
        .build()?;

    Mock::given(method("GET"))
        .and(path("/server/ping"))
        .and(header("Authorization", "Bearer secret-jwt-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"res": "pong"})))
        .mount(&server)
        .await;

    let ping = client.server().ping_server().await?;
    assert_eq!(ping.res, "pong");
    Ok(())
}

/// A 401 from the users endpoint maps to `Error::User` (module-tagged errors).
#[tokio::test]
async fn unauthorized_response_maps_to_user_error() -> Result<()> {
    let (client, server) = common::mock_client(0).await;

    Mock::given(method("GET"))
        .and(path("/users/me"))
        .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
            "message": "Unauthorized",
            "error": "Unauthorized",
            "statusCode": 401
        })))
        .mount(&server)
        .await;

    let result = client.users().get_my_user().await;
    match result {
        Err(ref err @ immich_rs::Error::User(ref details)) => {
            assert_eq!(details.status, reqwest::StatusCode::UNAUTHORIZED);
            assert_eq!(details.kind, immich_rs::ErrorKind::Unauthorized);
            assert_eq!(details.message, "Unauthorized");
            assert!(err.is_unauthorized());
        }
        other => panic!("expected User error on 401 from users endpoint, got: {other:?}"),
    }
    Ok(())
}

/// A 403 Forbidden from the authentication endpoint maps to `Error::Auth` with `ErrorKind::Forbidden`.
#[tokio::test]
async fn forbidden_response_maps_to_auth_error() -> Result<()> {
    let (client, server) = common::mock_client(0).await;

    Mock::given(method("POST"))
        .and(path("/auth/validateToken"))
        .respond_with(ResponseTemplate::new(403).set_body_json(serde_json::json!({
            "message": ["Forbidden", "Token expired"],
            "correlationId": "req-123",
            "statusCode": 403
        })))
        .mount(&server)
        .await;

    let result = client.authentication().validate_access_token().await;
    match result {
        Err(immich_rs::Error::Auth(details)) => {
            assert_eq!(details.kind, immich_rs::error::ErrorKind::Forbidden);
            assert_eq!(details.message, "Forbidden, Token expired");
            assert_eq!(details.correlation_id.as_deref(), Some("req-123"));
        }
        other => panic!("expected Error::Auth(Forbidden), got: {other:?}"),
    }
    Ok(())
}

/// `ImmichClient::new` sets base URL and API key directly.
#[test]
fn immich_client_new_constructor_builds_client_properly() -> Result<()> {
    let client = immich_rs::ImmichClient::new("http://localhost:2283/api", "my-key")?;
    assert_eq!(client.base_url().as_str(), "http://localhost:2283/api/");
    Ok(())
}

/// `ImmichClientBuilder::from_env_map` populates client from key-value lookup without mutating process environment.
#[test]
fn immich_client_from_env_map_reads_variables_and_builds() -> Result<()> {
    let mut vars = std::collections::HashMap::new();
    vars.insert("IMMICH_INSTANCE_URL", "http://env-host:2283/api".to_string());
    vars.insert("IMMICH_API_KEY", "env-key".to_string());

    let client = immich_rs::ImmichClient::builder()
        .from_env_map(|key| vars.get(key).cloned())?
        .build()?;
    assert_eq!(client.base_url().as_str(), "http://env-host:2283/api/");
    Ok(())
}

/// `ImmichClientBuilder::from_env_map` fails when required URL variable is missing.
#[test]
fn immich_client_from_env_map_fails_when_url_missing() {
    let vars = std::collections::HashMap::<String, String>::new();
    let res = immich_rs::ImmichClient::builder().from_env_map(|key| vars.get(key).cloned());
    assert!(res.is_err());
}
