#![cfg(not(target_arch = "wasm32"))]

mod common;

use anyhow::Result;
use pretty_assertions::assert_eq;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

fn sample_asset_json(id: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "deviceAssetId": "device-asset-1",
        "deviceId": "test-device",
        "type": "IMAGE",
        "originalPath": "/upload/test.jpg",
        "originalFileName": "test.jpg",
        "originalMimeType": "image/jpeg",
        "thumbhash": "",
        "fileCreatedAt": "2024-01-01T00:00:00.000Z",
        "fileModifiedAt": "2024-01-01T00:00:00.000Z",
        "localDateTime": "2024-01-01T00:00:00.000Z",
        "createdAt": "2024-01-01T00:00:00.000Z",
        "updatedAt": "2024-01-01T00:00:00.000Z",
        "isFavorite": false,
        "isArchived": false,
        "isTrashed": false,
        "isOffline": false,
        "isEdited": false,
        "visibility": "timeline",
        "checksum": "abc123",
        "duration": null,
        "width": 0,
        "height": 0,
        "hasMetadata": true,
        "people": [],
        "tags": [],
        "unassignedFaces": [],
        "ownerId": "00000000-0000-0000-0000-000000000001",
        "owner": {
            "id": "00000000-0000-0000-0000-000000000001",
            "name": "Test User",
            "email": "test@test.com",
            "avatarColor": "primary",
            "profileImagePath": "",
            "profileChangedAt": "2024-01-01T00:00:00.000Z"
        }
    })
}

#[tokio::test]
async fn get_asset_info_returns_asset() -> Result<()> {
    let (client, server) = common::mock_client(0).await;
    let asset_id = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();

    Mock::given(method("GET"))
        .and(path(format!("/assets/{asset_id}")))
        .and(query_param("slug", "public-share"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(sample_asset_json("00000000-0000-0000-0000-000000000001")),
        )
        .mount(&server)
        .await;

    let asset = client.assets().get_asset_info(asset_id).slug("public-share").await?;
    assert_eq!(asset.id, immich_models::id::AssetId::from(asset_id));
    Ok(())
}

#[tokio::test]
async fn get_asset_info_404_maps_to_asset_error() -> Result<()> {
    let (client, server) = common::mock_client(0).await;
    let asset_id = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000099").unwrap();

    Mock::given(method("GET"))
        .and(path(format!("/assets/{asset_id}")))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "message": "Asset not found",
            "statusCode": 404
        })))
        .mount(&server)
        .await;

    let result = client.assets().get_asset_info(asset_id).await;
    assert!(
        matches!(result, Err(immich_rs::Error::Asset(_))),
        "expected Asset error on 404, got: {result:?}"
    );
    Ok(())
}
