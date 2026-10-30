use immich_rs::ImmichClient;
use immich_rs::error::Result;
use serde::{Deserialize, Serialize};
use wiremock::matchers::{body_json, header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct TestPayload {
    message: String,
    count: u32,
}

#[tokio::test]
async fn escape_hatch_get_and_send_json() -> Result<()> {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/custom/status"))
        .and(header("x-api-key", "test-key"))
        .and(query_param("filter", "active"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "message": "all systems operational",
            "count": 42
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = ImmichClient::new(server.uri(), "test-key")?;

    let result: TestPayload = client
        .get("/custom/status")
        .query_param("filter", "active")
        .send_json()
        .await?;

    assert_eq!(
        result,
        TestPayload {
            message: "all systems operational".into(),
            count: 42,
        }
    );

    Ok(())
}

#[tokio::test]
async fn escape_hatch_post_with_json_and_headers() -> Result<()> {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/custom/submit"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-trace-source", "integration-test"))
        .and(body_json(serde_json::json!({
            "message": "hello server",
            "count": 1
        })))
        .respond_with(ResponseTemplate::new(201).set_body_bytes(b"received"))
        .expect(1)
        .mount(&server)
        .await;

    let client = ImmichClient::new(server.uri(), "test-key")?;

    let payload = TestPayload {
        message: "hello server".into(),
        count: 1,
    };

    let stream = client
        .post("/custom/submit")
        .header("x-trace-source", "integration-test")
        .json(&payload)?
        .send()
        .await?;

    assert_eq!(stream.status(), reqwest::StatusCode::CREATED);
    let bytes = stream.bytes().await?;
    assert_eq!(&bytes[..], b"received");

    Ok(())
}

#[tokio::test]
async fn escape_hatch_error_status_maps_to_api_error() -> Result<()> {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/custom/missing"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "message": "resource not found",
            "statusCode": 404
        })))
        .mount(&server)
        .await;

    let client = ImmichClient::new(server.uri(), "test-key")?;

    let err = client
        .get("/custom/missing")
        .send_json::<TestPayload>()
        .await
        .unwrap_err();

    assert_eq!(err.status(), Some(reqwest::StatusCode::NOT_FOUND));

    Ok(())
}

#[tokio::test]
async fn escape_hatch_supports_put_and_delete_methods() -> Result<()> {
    let server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/custom/item"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(b"updated"))
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/custom/item"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    let client = ImmichClient::new(server.uri(), "test-key")?;

    let put_stream = client.put("/custom/item").send().await?;
    assert_eq!(put_stream.status(), reqwest::StatusCode::OK);

    let del_stream = client.delete("/custom/item").send().await?;
    assert_eq!(del_stream.status(), reqwest::StatusCode::NO_CONTENT);

    Ok(())
}
