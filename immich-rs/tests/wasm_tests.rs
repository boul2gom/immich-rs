#![cfg(target_arch = "wasm32")]

use std::time::Duration;

use immich_rs::ImmichClient;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn wasm_client_builder_constructs_valid_client() {
    let client = ImmichClient::builder()
        .base_url("https://immich.example.com/api")
        .api_key("wasm-test-key")
        .max_retries(2)
        .build()
        .expect("client should build on wasm32");

    assert_eq!(client.base_url().as_str(), "https://immich.example.com/api/");
}

#[wasm_bindgen_test]
async fn wasm_timer_and_instant_operate_smoothly() {
    let start = immich_rs::core::time::Instant::now();
    immich_rs::core::time::sleep(Duration::from_millis(10)).await;
    assert!(start.elapsed() >= Duration::from_millis(5));
}

#[wasm_bindgen_test]
fn wasm_dto_roundtrip_serialization() {
    use immich_models::albums::CreateAlbumDTO;

    let dto = CreateAlbumDTO::new("WASM Album").description("Created in browser");
    let json = serde_json::to_string(&dto).expect("serialize DTO");
    let back: CreateAlbumDTO = serde_json::from_str(&json).expect("deserialize DTO");

    assert_eq!(back.album_name, "WASM Album");
    assert_eq!(back.description.as_deref(), Some("Created in browser"));
}
