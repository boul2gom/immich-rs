mod common;

use anyhow::Result;
use pretty_assertions::assert_eq;
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

fn sample_album_json(id: &str, name: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "albumName": name,
        "description": "",
        "albumThumbnailAssetId": "00000000-0000-0000-0000-000000000000",
        "createdAt": "2024-01-01T00:00:00.000Z",
        "updatedAt": "2024-01-01T00:00:00.000Z",
        "ownerId": "00000000-0000-0000-0000-000000000001",
        "owner": {
            "id": "00000000-0000-0000-0000-000000000001",
            "name": "Test User",
            "email": "test@test.com",
            "avatarColor": "primary",
            "profileImagePath": "",
            "profileChangedAt": "2024-01-01T00:00:00.000Z"
        },
        "albumUsers": [],
        "shared": false,
        "hasSharedLink": false,
        "assets": [],
        "assetCount": 0,
        "lastModifiedAssetTimestamp": null,
        "startDate": null,
        "endDate": null,
        "isActivityEnabled": true,
        "order": "desc"
    })
}

#[tokio::test]
async fn get_all_albums_returns_empty_list() -> Result<()> {
    let (client, server) = common::mock_client(0).await;

    Mock::given(method("GET"))
        .and(path("/albums"))
        .and(header("x-api-key", "test-api-key-00000000-0000-0000-0000-000000000000"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
        .mount(&server)
        .await;

    let albums = client.albums().get_all_albums().await?;
    assert!(albums.is_empty(), "expected empty album list");
    Ok(())
}

#[tokio::test]
async fn get_all_albums_with_shared_filter() -> Result<()> {
    let (client, server) = common::mock_client(0).await;

    Mock::given(method("GET"))
        .and(path("/albums"))
        .and(query_param("isShared", "true"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!([sample_album_json(
                "00000000-0000-0000-0000-000000000042",
                "Shared Album"
            )])),
        )
        .mount(&server)
        .await;

    let albums = client.albums().get_all_albums().is_shared(true).await?;
    assert_eq!(albums.len(), 1);
    assert_eq!(albums[0].album_name, "Shared Album");
    Ok(())
}

#[tokio::test]
async fn get_album_info_returns_album() -> Result<()> {
    let (client, server) = common::mock_client(0).await;
    let album_id = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000099").unwrap();

    Mock::given(method("GET"))
        .and(path(format!("/albums/{album_id}")))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(sample_album_json("00000000-0000-0000-0000-000000000099", "My Album")),
        )
        .mount(&server)
        .await;

    let album = client.albums().get_album_info(album_id).await?;
    assert_eq!(album.album_name, "My Album");
    Ok(())
}

#[tokio::test]
async fn get_album_info_404_maps_to_album_error() -> Result<()> {
    let (client, server) = common::mock_client(0).await;
    let album_id = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000099").unwrap();

    Mock::given(method("GET"))
        .and(path(format!("/albums/{album_id}")))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "message": "Album not found",
            "statusCode": 404
        })))
        .mount(&server)
        .await;

    let result = client.albums().get_album_info(album_id).await;
    assert!(
        matches!(result, Err(immich_rs::Error::Album(_))),
        "expected Album error on 404, got: {result:?}"
    );
    Ok(())
}
