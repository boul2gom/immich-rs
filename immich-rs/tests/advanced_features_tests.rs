mod common;

use std::pin::pin;
use std::time::Duration;

use anyhow::Result;
use futures_util::StreamExt;
use immich_rs::prelude::*;
use tempfile::tempdir;
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn request_config_applies_custom_headers_and_custom_timeout() -> Result<()> {
    let (client, server) = common::mock_client(0).await;

    Mock::given(method("GET"))
        .and(path("/albums"))
        .and(header("x-custom-tenant", "tenant-alpha-42"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
        .expect(1)
        .mount(&server)
        .await;

    let res = client
        .albums()
        .get_all_albums()
        .header("x-custom-tenant", "tenant-alpha-42")
        .timeout(Duration::from_secs(5))
        .await?;

    assert_eq!(res.len(), 0);
    Ok(())
}

#[tokio::test]
async fn request_config_timeout_triggers_timeout_error_when_server_is_slow() -> Result<()> {
    let (client, server) = common::mock_client(0).await;

    Mock::given(method("GET"))
        .and(path("/albums"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_delay(Duration::from_millis(400))
                .set_body_json(serde_json::json!([])),
        )
        .expect(1)
        .mount(&server)
        .await;

    let err = client
        .albums()
        .get_all_albums()
        .timeout(Duration::from_millis(50))
        .await
        .unwrap_err();

    assert!(
        err.is_timeout(),
        "Expected timeout error when request exceeds timeout, got: {err:?}"
    );
    Ok(())
}

#[tokio::test]
async fn upload_bulk_skips_upstream_duplicates_via_bulk_upload_check() -> Result<()> {
    let (client, server) = common::mock_client(0).await;

    let dir = tempdir()?;
    let file1 = dir.path().join("duplicate.jpg");
    let file2 = dir.path().join("fresh.jpg");
    tokio::fs::write(&file1, b"duplicate-content").await?;
    tokio::fs::write(&file2, b"fresh-content").await?;

    let existing_id = uuid::Uuid::parse_str("00000000-0000-0000-0000-0000000000aa").unwrap();
    let fresh_id = uuid::Uuid::parse_str("00000000-0000-0000-0000-0000000000bb").unwrap();

    // 1. Mock bulk-upload-check
    Mock::given(method("POST"))
        .and(path("/assets/bulk-upload-check"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "results": [
                {
                    "id": "0",
                    "action": "reject",
                    "reason": "duplicate",
                    "assetId": existing_id
                },
                {
                    "id": "1",
                    "action": "accept"
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    // 2. Mock upload for only fresh.jpg
    Mock::given(method("POST"))
        .and(path("/assets"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": fresh_id,
            "status": "created"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let items = vec![UploadOptions::new(&file1), UploadOptions::new(&file2)];
    let mut stream = pin!(client.upload_bulk(items).check_duplicates(true).into_stream());

    let mut results = Vec::new();
    while let Some(res) = stream.next().await {
        results.push(res?);
    }

    assert_eq!(results.len(), 2);

    let dup = results
        .iter()
        .find(|r| r.is_duplicate())
        .expect("expected duplicate result");
    match dup {
        BulkUploadResult::Duplicate {
            path,
            existing_asset_id,
        } => {
            assert_eq!(path, &file1);
            assert_eq!(existing_asset_id, &Some(AssetId(existing_id)));
        }
        BulkUploadResult::Uploaded(_) => unreachable!(),
    }

    let fresh = results
        .iter()
        .find(|r| r.is_uploaded())
        .expect("expected uploaded result");
    assert_eq!(fresh.clone().uploaded().unwrap().id, fresh_id);

    Ok(())
}

#[tokio::test]
async fn upload_live_photo_orchestrates_video_upload_and_photo_linking() -> Result<()> {
    let (client, server) = common::mock_client(0).await;

    let dir = tempdir()?;
    let photo_path = dir.path().join("live.heic");
    let video_path = dir.path().join("live.mov");
    tokio::fs::write(&photo_path, b"heic-image-content").await?;
    tokio::fs::write(&video_path, b"mov-video-content").await?;

    let video_id = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
    let photo_id = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap();

    // 1. Mock first upload (video)
    Mock::given(method("POST"))
        .and(path("/assets"))
        .and(wiremock::matchers::body_string_contains("live.mov"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": video_id,
            "status": "created"
        })))
        .expect(1)
        .mount(&server)
        .await;

    // 2. Mock second upload (photo linked with livePhotoVideoId)
    Mock::given(method("POST"))
        .and(path("/assets"))
        .and(wiremock::matchers::body_string_contains("livePhotoVideoId"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": photo_id,
            "status": "created"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let result = client.upload_live_photo(&photo_path, &video_path).await?;
    assert_eq!(result.id, photo_id);

    Ok(())
}

#[tokio::test]
async fn upload_with_sidecar_attaches_sidecar_multipart_part() -> Result<()> {
    let (client, server) = common::mock_client(0).await;

    let dir = tempdir()?;
    let photo_path = dir.path().join("photo.jpg");
    let sidecar_path = dir.path().join("photo.xmp");
    tokio::fs::write(&photo_path, b"photo-bytes").await?;
    tokio::fs::write(&sidecar_path, b"<xmp>metadata</xmp>").await?;

    let photo_id = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000099").unwrap();

    Mock::given(method("POST"))
        .and(path("/assets"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": photo_id,
            "status": "created"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let result = client.upload_with_sidecar(&photo_path, &sidecar_path).await?;
    assert_eq!(result.id, photo_id);

    Ok(())
}

#[tokio::test]
async fn direct_page_paginator_paginates_raw_json_array() -> Result<()> {
    let (client, server) = common::mock_client(0).await;

    let mem1_id = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000101").unwrap();
    let mem2_id = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000102").unwrap();
    let mem3_id = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000103").unwrap();

    let memory_json = |id: uuid::Uuid| {
        serde_json::json!({
            "id": id,
            "ownerId": "00000000-0000-0000-0000-000000000001",
            "type": "on_this_day",
            "data": { "year": 2023 },
            "isSaved": false,
            "isTrashed": false,
            "assets": [],
            "createdAt": "2024-01-01T00:00:00.000Z",
            "updatedAt": "2024-01-01T00:00:00.000Z",
            "deletedAt": null,
            "memoryAt": "2023-01-01T00:00:00.000Z",
            "showAt": null,
            "seenAt": null
        })
    };

    // Page 1: 2 items (full size, triggers page 2 request)
    Mock::given(method("GET"))
        .and(path("/memories"))
        .and(query_param("page", "1"))
        .and(query_param("size", "2"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!([memory_json(mem1_id), memory_json(mem2_id)])),
        )
        .expect(1)
        .mount(&server)
        .await;

    // Page 2: 1 item (less than size -> terminal page)
    Mock::given(method("GET"))
        .and(path("/memories"))
        .and(query_param("page", "2"))
        .and(query_param("size", "2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([memory_json(mem3_id)])))
        .expect(1)
        .mount(&server)
        .await;

    let mut paginator = pin!(client.memories().search_memories().size(2).into_paginator().pages());

    let mut total_pages = 0;
    let mut total_items = 0;
    while let Some(page) = paginator.next().await {
        let memories = page?;
        total_pages += 1;
        total_items += memories.len();
    }

    assert_eq!(total_pages, 2);
    assert_eq!(total_items, 3);

    Ok(())
}
