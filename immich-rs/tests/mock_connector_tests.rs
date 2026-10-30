#![cfg(feature = "test-util")]

use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use immich_rs::error::Result;
use immich_rs::test_util::mock_client;

#[tokio::test]
async fn mock_connector_without_network() -> Result<()> {
    let call_count = Arc::new(AtomicU32::new(0));
    let cc = Arc::clone(&call_count);

    let client = mock_client(move |req| {
        let cc = Arc::clone(&cc);
        async move {
            cc.fetch_add(1, Ordering::SeqCst);
            assert_eq!(req.headers().get("x-api-key").unwrap(), "mock-api-key");
            http::Response::builder()
                .status(200)
                .body(reqwest::Body::from(r#"{"major":3,"minor":3,"patch":5}"#))
                .map(reqwest::Response::from)
                .unwrap()
        }
    });

    let ver = client.server().get_server_version().await?;
    assert_eq!(ver.major, 3);
    assert_eq!(ver.minor, 3);
    assert_eq!(ver.patch, 5);
    assert_eq!(call_count.load(Ordering::SeqCst), 1);

    Ok(())
}

#[tokio::test]
async fn mock_client_handles_error_responses() {
    let client = mock_client(|_req| async move {
        http::Response::builder()
            .status(404)
            .body(reqwest::Body::from(r#"{"message":"Asset not found"}"#))
            .map(reqwest::Response::from)
            .unwrap()
    });

    let err = client
        .assets()
        .get_asset_info(immich_rs::AssetId::new(uuid::Uuid::nil()))
        .await
        .unwrap_err();
    assert_eq!(err.status(), Some(reqwest::StatusCode::NOT_FOUND));
}

#[tokio::test]
async fn mock_connector_preserves_method_and_query_string() -> Result<()> {
    let client = mock_client(|req| async move {
        assert_eq!(req.method(), reqwest::Method::GET);
        assert_eq!(req.url().path(), "/api/people");
        assert_eq!(req.url().query(), Some("withHidden=true"));

        http::Response::builder()
            .status(200)
            .body(reqwest::Body::from(r#"{"people":[],"total":0,"hidden":0}"#))
            .map(reqwest::Response::from)
            .unwrap()
    });

    let people = client.people().get_all_people().with_hidden(true).await?;
    assert_eq!(people.total, 0);
    assert_eq!(people.hidden, 0);

    Ok(())
}
