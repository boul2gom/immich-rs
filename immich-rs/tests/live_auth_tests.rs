use std::sync::Arc;

use immich_rs::ImmichClient;
use immich_rs::error::Result;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn live_auth_dynamic_token_rotation() -> Result<()> {
    let server = MockServer::start().await;

    // First request expects initial API key
    Mock::given(method("GET"))
        .and(path("/server/version"))
        .and(header("x-api-key", "initial-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "major": 3, "minor": 3, "patch": 5
        })))
        .expect(1)
        .mount(&server)
        .await;

    // Second request expects rotated Bearer token
    Mock::given(method("GET"))
        .and(path("/server/version"))
        .and(header("authorization", "Bearer new-secret-jwt"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "major": 3, "minor": 3, "patch": 5
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = ImmichClient::new(server.uri(), "initial-key")?;
    assert_eq!(client.api_key().as_deref(), Some("initial-key"));
    assert_eq!(client.bearer_token(), None);

    let ver1 = client.server().get_server_version().await?;
    assert_eq!(ver1.major, 3);

    // Rotate token dynamically
    client.set_bearer_token("new-secret-jwt")?;
    assert_eq!(client.api_key(), None);
    assert_eq!(client.bearer_token().as_deref(), Some("new-secret-jwt"));

    let ver2 = client.server().get_server_version().await?;
    assert_eq!(ver2.major, 3);

    Ok(())
}

#[tokio::test]
async fn live_auth_concurrent_requests_during_rotation() -> Result<()> {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/server/version"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "major": 3, "minor": 3, "patch": 5
        })))
        .mount(&server)
        .await;

    let client = Arc::new(ImmichClient::new(server.uri(), "base-key")?);

    let mut handles = Vec::new();

    // Spawn 20 concurrent readers
    for i in 0..20 {
        let c = Arc::clone(&client);
        handles.push(tokio::spawn(async move {
            if i == 10 {
                c.set_api_key(format!("rotated-key-{i}")).unwrap();
            }
            c.server().get_server_version().await.unwrap();
        }));
    }

    for h in handles {
        h.await.unwrap();
    }

    Ok(())
}

#[tokio::test]
async fn client_login_flow() -> Result<()> {
    let server = MockServer::start().await;

    // Login mock
    Mock::given(method("POST"))
        .and(path("/auth/login"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "accessToken": "returned-jwt-access-token",
            "isAdmin": false,
            "isOnboarded": true,
            "name": "Jane Doe",
            "profileImagePath": "",
            "shouldChangePassword": false,
            "userEmail": "jane@example.com",
            "userId": "11111111-1111-4111-8111-111111111111"
        })))
        .expect(1)
        .mount(&server)
        .await;

    // Authenticated request mock using the token received from login
    Mock::given(method("GET"))
        .and(path("/server/version"))
        .and(header("authorization", "Bearer returned-jwt-access-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "major": 3, "minor": 3, "patch": 5
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = ImmichClient::builder().base_url(server.uri()).build()?;

    assert_eq!(client.bearer_token(), None);

    let login_res = client.login("jane@example.com", "mypassword").await?;
    assert_eq!(login_res.access_token, "returned-jwt-access-token");
    assert_eq!(client.bearer_token().as_deref(), Some("returned-jwt-access-token"));

    // Server check should now carry the Bearer token
    let ver = client.server().get_server_version().await?;
    assert_eq!(ver.major, 3);

    Ok(())
}

#[tokio::test]
async fn client_login_to_creates_authenticated_client() -> Result<()> {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/auth/login"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "accessToken": "login-to-token",
            "isAdmin": true,
            "isOnboarded": true,
            "name": "Super Admin",
            "profileImagePath": "",
            "shouldChangePassword": false,
            "userEmail": "admin@example.com",
            "userId": "22222222-2222-4222-8222-222222222222"
        })))
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/server/version"))
        .and(header("authorization", "Bearer login-to-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "major": 3, "minor": 3, "patch": 5
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = ImmichClient::login_to(server.uri(), "admin@example.com", "adminpass").await?;
    assert_eq!(client.bearer_token().as_deref(), Some("login-to-token"));

    let ver = client.server().get_server_version().await?;
    assert_eq!(ver.major, 3);

    Ok(())
}

#[test]
fn live_auth_set_bearer_token_rejects_invalid_ascii() -> Result<()> {
    let client = ImmichClient::new("https://example.com/api", "good-api-key")?;

    // Header values cannot contain newline or non-visible ASCII characters
    let err = client.set_bearer_token("invalid\ntoken").unwrap_err();
    assert!(err.to_string().contains("invalid Bearer token format"));

    // Original auth must remain untouched
    assert_eq!(client.api_key().as_deref(), Some("good-api-key"));
    assert_eq!(client.bearer_token(), None);

    Ok(())
}

#[test]
fn live_auth_set_api_key_rejects_invalid_ascii() -> Result<()> {
    let client = ImmichClient::new("https://example.com/api", "good-api-key")?;

    let err = client.set_api_key("bad\r\nkey").unwrap_err();
    assert!(err.to_string().contains("invalid API key format"));

    assert_eq!(client.api_key().as_deref(), Some("good-api-key"));

    Ok(())
}
