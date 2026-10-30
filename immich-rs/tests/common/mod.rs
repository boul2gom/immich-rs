#![allow(dead_code)]

use immich_rs::ImmichClient;
use wiremock::MockServer;

/// Creates a mock `WireMock` server and an authenticated `ImmichClient` pointing to it.
///
/// # Panics
///
/// Panics if the client builder fails with the provided mock configuration.
pub(crate) async fn mock_client(max_retries: u32) -> (ImmichClient, MockServer) {
    let server = MockServer::start().await;
    let client = ImmichClient::builder()
        .base_url(server.uri())
        .api_key("test-api-key-00000000-0000-0000-0000-000000000000")
        .max_retries(max_retries)
        .build()
        .expect("client should build from valid config");

    (client, server)
}

/// Creates a mock `WireMock` server and an `ImmichClient` configured without any credentials.
///
/// # Panics
///
/// Panics if the client builder fails with the provided mock configuration.
pub(crate) async fn mock_unauthenticated_client(max_retries: u32) -> (ImmichClient, MockServer) {
    let server = MockServer::start().await;
    let client = ImmichClient::builder()
        .base_url(server.uri())
        .max_retries(max_retries)
        .build()
        .expect("client should build from valid config");

    (client, server)
}
