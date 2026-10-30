#![cfg(all(feature = "fs", not(target_arch = "wasm32")))]

mod common;

use std::pin::pin;

use anyhow::Result;
use futures_util::StreamExt;
use immich_models::admin::maintenance::IntegrityReport;
use immich_models::albums::CreateAlbumDTO;
use immich_rs::{UploadOptions, calculate_file_checksum};
use pretty_assertions::assert_eq;
use tempfile::tempdir;
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn builder_validation_rejects_missing_url_and_invalid_headers_without_panic() {
    let missing_url = immich_rs::ImmichClient::builder().build();
    match missing_url {
        Err(immich_rs::Error::Validation(msg)) => assert_eq!(msg, "missing base_url"),
        other => panic!("expected Validation(missing base_url), got: {other:?}"),
    }

    let invalid_api_key = immich_rs::ImmichClient::builder()
        .base_url("http://localhost:2283/api/")
        .api_key("invalid\nheader\nvalue")
        .build();
    assert!(
        matches!(invalid_api_key, Err(immich_rs::Error::Validation(_))),
        "expected Validation error on invalid API key, got: {invalid_api_key:?}"
    );

    let invalid_bearer = immich_rs::ImmichClient::builder()
        .base_url("http://localhost:2283/api/")
        .bearer_token("invalid\nheader\nvalue")
        .build();
    assert!(
        matches!(invalid_bearer, Err(immich_rs::Error::Validation(_))),
        "expected Validation error on invalid bearer token, got: {invalid_bearer:?}"
    );
}

#[tokio::test]
async fn calculate_file_checksum_computes_exact_sha1_and_propagates_io_error() -> Result<()> {
    let dir = tempdir()?;
    let file_path = dir.path().join("sample.bin");
    tokio::fs::write(&file_path, b"hello immich-rs").await?;

    let sha1 = calculate_file_checksum(&file_path).await?;
    // SHA-1("hello immich-rs") = 8a0bd6a3ceb8d56bcb393803ad2a346f6072c191
    assert_eq!(sha1, "8a0bd6a3ceb8d56bcb393803ad2a346f6072c191");

    let missing_path = dir.path().join("does_not_exist.bin");
    let err = calculate_file_checksum(&missing_path).await;
    assert!(
        matches!(err, Err(immich_rs::Error::Io(_))),
        "expected Error::Io for missing file, got: {err:?}"
    );
    Ok(())
}

#[tokio::test]
async fn download_streams_to_disk_and_leaves_no_temporary_file() -> Result<()> {
    let (client, server) = common::mock_client(0).await;
    let asset_id = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000010").unwrap();
    let payload = b"binary-jpeg-content-1234567890";

    Mock::given(method("GET"))
        .and(path(format!("/assets/{asset_id}/original")))
        .respond_with(ResponseTemplate::new(200).set_body_raw(payload.to_vec(), "application/octet-stream"))
        .mount(&server)
        .await;

    let dir = tempdir()?;
    let dest = dir.path().join("downloaded.jpg");
    client.download(asset_id, &dest).await?;

    let written = tokio::fs::read(&dest).await?;
    assert_eq!(written, payload);

    let leftovers: Vec<_> = std::fs::read_dir(dir.path())?
        .map(|e| e.map(|e| e.file_name()))
        .collect::<std::io::Result<_>>()?;
    assert_eq!(leftovers, vec![std::ffi::OsString::from("downloaded.jpg")]);
    Ok(())
}

#[tokio::test]
async fn download_failure_propagates_error_and_creates_no_files() -> Result<()> {
    let (client, server) = common::mock_client(0).await;
    let asset_id = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000011").unwrap();

    Mock::given(method("GET"))
        .and(path(format!("/assets/{asset_id}/original")))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "message": "Asset not found"
        })))
        .mount(&server)
        .await;

    let dir = tempdir()?;
    let dest = dir.path().join("missing.jpg");
    let err = client.download(asset_id, &dest).await.unwrap_err();
    assert!(err.is_not_found(), "got {err:?}");
    assert_eq!(std::fs::read_dir(dir.path())?.count(), 0);
    Ok(())
}

#[tokio::test]
async fn download_to_unwritable_destination_returns_io_error_without_leftovers() -> Result<()> {
    let (client, server) = common::mock_client(0).await;
    let asset_id = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000012").unwrap();

    Mock::given(method("GET"))
        .and(path(format!("/assets/{asset_id}/original")))
        .respond_with(ResponseTemplate::new(200).set_body_raw(b"data".to_vec(), "image/jpeg"))
        .mount(&server)
        .await;

    let dir = tempdir()?;
    let dest = dir.path().join("no-such-dir").join("file.jpg");
    let err = client.download(asset_id, &dest).await.unwrap_err();
    assert!(matches!(err, immich_rs::Error::Io(_)), "got {err:?}");
    assert_eq!(std::fs::read_dir(dir.path())?.count(), 0);
    Ok(())
}

#[tokio::test]
async fn upload_asset_auto_resolves_metadata_and_retries_on_503() -> Result<()> {
    let (client, server) = common::mock_client(2).await;
    let dir = tempdir()?;
    let file_path = dir.path().join("photo.jpg");
    tokio::fs::write(&file_path, b"fake-jpeg-bytes").await?;

    let checksum = calculate_file_checksum(&file_path).await?;
    let expected_id = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000077").unwrap();

    // Fail once with 503 to verify multipart stream re-opening on retry
    Mock::given(method("POST"))
        .and(path("/assets"))
        .and(header("x-immich-checksum", checksum.as_str()))
        .respond_with(ResponseTemplate::new(503))
        .up_to_n_times(1)
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/assets"))
        .and(header("x-immich-checksum", checksum.as_str()))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": expected_id,
            "status": "created"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let options = UploadOptions::new(&file_path).checksum(&checksum).is_favorite(true);
    let res = client.upload_asset(options).await?;
    assert_eq!(res.id, immich_models::id::AssetId::from(expected_id));

    // The successful attempt must carry the auto-resolved metadata and the file content.
    let requests = server.received_requests().await.expect("recording enabled");
    let last = requests.last().expect("at least one request");
    let body = String::from_utf8_lossy(&last.body);
    for needle in [
        "name=\"deviceAssetId\"",
        "photo.jpg-15",
        "name=\"deviceId\"",
        "immich-rs",
        "name=\"fileCreatedAt\"",
        "name=\"fileModifiedAt\"",
        "name=\"isFavorite\"",
        "true",
        "filename=\"photo.jpg\"",
        "image/jpeg",
        "fake-jpeg-bytes",
    ] {
        assert!(body.contains(needle), "multipart body misses {needle:?}");
    }
    assert!(
        last.headers.get("content-length").is_some(),
        "exact content-length expected instead of chunked encoding"
    );
    Ok(())
}

#[tokio::test]
async fn upload_asset_explicit_metadata_overrides_auto_resolution() -> Result<()> {
    let (client, server) = common::mock_client(0).await;
    let dir = tempdir()?;
    let file_path = dir.path().join("clip.mov");
    tokio::fs::write(&file_path, b"video").await?;
    let id = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000078").unwrap();

    Mock::given(method("POST"))
        .and(path("/assets"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": id,
            "status": "duplicate"
        })))
        .mount(&server)
        .await;

    let options = UploadOptions::new(&file_path)
        .device_asset_id("custom-asset-id")
        .device_id("custom-device")
        .file_created_at(
            chrono::DateTime::parse_from_rfc3339("2020-01-02T03:04:05.000Z")
                .unwrap()
                .with_timezone(&chrono::Utc),
        )
        .file_modified_at(
            chrono::DateTime::parse_from_rfc3339("2021-02-03T04:05:06.000Z")
                .unwrap()
                .with_timezone(&chrono::Utc),
        );
    client.upload_asset(options).await?;

    let requests = server.received_requests().await.expect("recording enabled");
    let body = String::from_utf8_lossy(&requests[0].body);
    for needle in [
        "custom-asset-id",
        "custom-device",
        "2020-01-02T03:04:05.000Z",
        "2021-02-03T04:05:06.000Z",
        "video/quicktime",
    ] {
        assert!(body.contains(needle), "multipart body misses {needle:?}");
    }
    Ok(())
}

#[tokio::test]
async fn upload_asset_missing_file_returns_io_error_without_request() -> Result<()> {
    let (client, server) = common::mock_client(0).await;
    let dir = tempdir()?;
    let missing = dir.path().join("ghost.jpg");

    let err = client.upload_asset(UploadOptions::new(&missing)).await.unwrap_err();
    assert!(matches!(err, immich_rs::Error::Io(_)), "got {err:?}");
    assert!(server.received_requests().await.expect("recording enabled").is_empty());
    Ok(())
}

#[tokio::test]
async fn upload_profile_image_streams_file_and_retries_on_503() -> Result<()> {
    let (client, server) = common::mock_client(2).await;
    let dir = tempdir()?;
    let file_path = dir.path().join("avatar.png");
    tokio::fs::write(&file_path, b"png-bytes").await?;
    let user_id = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000090").unwrap();

    Mock::given(method("POST"))
        .and(path("/users/profile-image"))
        .respond_with(ResponseTemplate::new(503))
        .up_to_n_times(1)
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/users/profile-image"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "userId": user_id,
            "profileImagePath": "/profile/avatar.png",
            "profileChangedAt": "2024-01-01T00:00:00.000Z"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let res = client.upload_profile_image(&file_path).await?;
    assert_eq!(res.user_id, immich_models::id::UserId::from(user_id));

    let requests = server.received_requests().await.expect("recording enabled");
    let body = String::from_utf8_lossy(&requests.last().unwrap().body);
    assert!(body.contains("name=\"file\""));
    assert!(body.contains("image/png"));
    assert!(body.contains("png-bytes"));

    let missing = client
        .upload_profile_image(dir.path().join("none.png"))
        .await
        .unwrap_err();
    assert!(matches!(missing, immich_rs::Error::Io(_)));
    Ok(())
}

#[tokio::test]
async fn upload_bulk_returns_distinct_results_and_isolates_failures() -> Result<()> {
    let (client, server) = common::mock_client(0).await;
    let dir = tempdir()?;
    let file1 = dir.path().join("one.jpg");
    let file2 = dir.path().join("two.jpg");
    let missing = dir.path().join("three.jpg");
    tokio::fs::write(&file1, b"img-1").await?;
    tokio::fs::write(&file2, b"img-2").await?;

    let id1 = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
    let id2 = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap();
    for (name, id) in [("img-1", id1), ("img-2", id2)] {
        Mock::given(method("POST"))
            .and(path("/assets"))
            .and(wiremock::matchers::body_string_contains(name))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "id": id,
                "status": "created"
            })))
            .expect(1)
            .mount(&server)
            .await;
    }

    let items = vec![
        UploadOptions::new(&file1).device_id("bulk-test"),
        UploadOptions::new(&missing).device_id("bulk-test"),
        UploadOptions::new(&file2).device_id("bulk-test"),
    ];
    let mut stream = pin!(
        client
            .upload_bulk(items)
            .concurrency(2)
            .check_duplicates(false)
            .into_stream()
    );
    let mut ok_ids = std::collections::BTreeSet::new();
    let mut failures = 0;
    while let Some(res) = stream.next().await {
        match res {
            Ok(result) => {
                if let Some(dto) = result.uploaded() {
                    ok_ids.insert(dto.id);
                }
            }
            Err(immich_rs::Error::Io(_)) => failures += 1,
            Err(other) => panic!("unexpected error: {other:?}"),
        }
    }
    assert_eq!(
        ok_ids,
        std::collections::BTreeSet::from([
            immich_models::id::AssetId::from(id1),
            immich_models::id::AssetId::from(id2)
        ])
    );
    assert_eq!(failures, 1);
    Ok(())
}

#[tokio::test]
async fn upload_bulk_with_zero_concurrency_is_clamped_and_completes() -> Result<()> {
    let (client, server) = common::mock_client(0).await;
    let dir = tempdir()?;
    let file = dir.path().join("only.jpg");
    tokio::fs::write(&file, b"img").await?;
    let id = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000003").unwrap();

    Mock::given(method("POST"))
        .and(path("/assets"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": id,
            "status": "created"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let items = vec![UploadOptions::new(&file)];
    let mut stream = pin!(
        client
            .upload_bulk(items)
            .concurrency(0)
            .check_duplicates(false)
            .into_stream()
    );
    let first = tokio::time::timeout(std::time::Duration::from_secs(10), stream.next())
        .await
        .expect("stream must not hang")
        .expect("one item")?;
    assert_eq!(
        first.uploaded().expect("uploaded dto").id,
        immich_models::id::AssetId::from(id)
    );
    assert!(stream.next().await.is_none());
    Ok(())
}

#[tokio::test]
async fn page_based_paginator_streams_until_has_next_page_is_false() -> Result<()> {
    let (client, server) = common::mock_client(0).await;

    let person1 = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000101").unwrap();
    let person2 = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000102").unwrap();

    Mock::given(method("GET"))
        .and(path("/people"))
        .and(query_param("page", "1"))
        .and(query_param("withHidden", "true"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "hasNextPage": true,
            "hidden": 0,
            "total": 2,
            "people": [{
                "id": person1,
                "name": "Alice",
                "birthDate": null,
                "isHidden": false,
                "isFavorite": true,
                "otherPeople": [],
                "sharedBy": [],
                "sharedWith": [],
                "thumbnailPath": "/thumb/1.jpg",
                "updatedAt": "2024-01-01T00:00:00.000Z"
            }]
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/people"))
        .and(query_param("page", "2"))
        .and(query_param("withHidden", "true"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "hasNextPage": false,
            "hidden": 0,
            "total": 2,
            "people": [{
                "id": person2,
                "name": "Bob",
                "birthDate": null,
                "isHidden": false,
                "isFavorite": false,
                "otherPeople": [],
                "sharedBy": [],
                "sharedWith": [],
                "thumbnailPath": "/thumb/2.jpg",
                "updatedAt": "2024-01-01T00:00:00.000Z"
            }]
        })))
        .mount(&server)
        .await;

    let mut pages = pin!(
        client
            .people()
            .get_all_people()
            .with_hidden(true)
            .into_paginator()
            .pages()
    );
    let mut collected = Vec::new();
    while let Some(page) = pages.next().await {
        collected.extend(page?);
    }

    assert_eq!(collected.len(), 2);
    assert_eq!(collected[0].name, "Alice");
    assert_eq!(collected[1].name, "Bob");
    Ok(())
}

#[tokio::test]
async fn page_based_paginator_items_stream_flattens_items_directly() -> Result<()> {
    let (client, server) = common::mock_client(0).await;

    let person1 = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000101").unwrap();
    let person2 = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000102").unwrap();

    Mock::given(method("GET"))
        .and(path("/people"))
        .and(query_param("page", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "hasNextPage": true,
            "hidden": 0,
            "total": 2,
            "people": [{
                "id": person1,
                "name": "Alice",
                "birthDate": null,
                "isHidden": false,
                "isFavorite": true,
                "otherPeople": [],
                "sharedBy": [],
                "sharedWith": [],
                "thumbnailPath": "/thumb/1.jpg",
                "updatedAt": "2024-01-01T00:00:00.000Z"
            }]
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/people"))
        .and(query_param("page", "2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "hasNextPage": false,
            "hidden": 0,
            "total": 2,
            "people": [{
                "id": person2,
                "name": "Bob",
                "birthDate": null,
                "isHidden": false,
                "isFavorite": false,
                "otherPeople": [],
                "sharedBy": [],
                "sharedWith": [],
                "thumbnailPath": "/thumb/2.jpg",
                "updatedAt": "2024-01-01T00:00:00.000Z"
            }]
        })))
        .mount(&server)
        .await;

    let mut items = pin!(client.people().get_all_people().into_paginator().items());
    let mut names = Vec::new();
    while let Some(person) = items.next().await {
        names.push(person?.name);
    }

    assert_eq!(names, vec!["Alice", "Bob"]);
    Ok(())
}

#[tokio::test]
async fn cursor_based_paginator_streams_until_next_cursor_is_none() -> Result<()> {
    let (client, server) = common::mock_client(0).await;

    let item1 = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000201").unwrap();
    let item2 = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000202").unwrap();

    Mock::given(method("GET"))
        .and(path("/admin/integrity/report"))
        .and(query_param("type", "missing_file"))
        .and(query_param("cursor", "cursor-2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "nextCursor": null,
            "items": [{
                "id": item2,
                "path": "/library/second.jpg",
                "type": "missing_file"
            }]
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/admin/integrity/report"))
        .and(query_param("type", "missing_file"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "nextCursor": "cursor-2",
            "items": [{
                "id": item1,
                "path": "/library/first.jpg",
                "type": "missing_file"
            }]
        })))
        .mount(&server)
        .await;

    let mut pages = pin!(
        client
            .admin()
            .maintenance()
            .get_integrity_report(IntegrityReport::MissingFile)
            .into_paginator()
            .pages()
    );
    let mut collected = Vec::new();
    while let Some(page) = pages.next().await {
        collected.extend(page?);
    }

    assert_eq!(collected.len(), 2);
    assert_eq!(collected[0].path, "/library/first.jpg");
    assert_eq!(collected[1].path, "/library/second.jpg");
    Ok(())
}

#[tokio::test]
#[allow(deprecated)]
async fn query_array_parameters_serialize_cleanly_without_urlencoded_error() -> Result<()> {
    let (client, server) = common::mock_client(0).await;
    let album1 = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000301").unwrap();
    let album2 = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000302").unwrap();

    Mock::given(method("POST"))
        .and(path("/search/large-assets"))
        .and(query_param("albumIds", album1.to_string()))
        .and(query_param("albumIds", album2.to_string()))
        .and(query_param("minFileSize", "1024"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
        .mount(&server)
        .await;

    let assets = client
        .search()
        .search_large_assets()
        .album_ids(vec![album1, album2])
        .min_file_size(1024)
        .await?;
    assert_eq!(assets.len(), 0);
    Ok(())
}

#[test]
fn forward_compatible_enums_and_dto_constructors_work() {
    // 1. Unknown enum values deserialize into Unknown(String) instead of failing
    let report: IntegrityReport = serde_json::from_str("\"future-check-v99\"").unwrap();
    assert_eq!(report, IntegrityReport::Unknown("future-check-v99".to_string()));
    assert_eq!(report.to_string(), "future-check-v99");

    // 2. DTO constructor + fluent setter
    let dto = CreateAlbumDTO::new("Summer Trip").description("Photos from vacation");
    assert_eq!(dto.album_name, "Summer Trip");
    assert_eq!(dto.description.as_deref(), Some("Photos from vacation"));
    assert_eq!(dto.asset_ids.len(), 0);
}

#[test]
fn enums_parse_known_values_and_reject_unknown_ones() {
    use immich_models::albums::{AlbumUserRole, AssetOrder};

    assert_eq!("editor".parse::<AlbumUserRole>().unwrap(), AlbumUserRole::Editor);
    assert_eq!(AlbumUserRole::from("viewer"), AlbumUserRole::Viewer);
    assert_eq!(AlbumUserRole::from(String::from("owner")), AlbumUserRole::Owner);
    assert_eq!("asc".parse::<AssetOrder>().unwrap(), AssetOrder::Asc);

    let parsed = "nonsense".parse::<AlbumUserRole>().unwrap();
    assert_eq!(parsed, AlbumUserRole::Unknown("nonsense".to_string()));
    assert_eq!(parsed.to_string(), "nonsense");

    // Display round-trips through FromStr.
    assert_eq!(
        AlbumUserRole::Editor.to_string().parse::<AlbumUserRole>().unwrap(),
        AlbumUserRole::Editor
    );
}

#[test]
fn builders_expose_set_option_variants_and_accept_string_slices() {
    let dto = CreateAlbumDTO::new("Holidays").description("Summer");
    assert_eq!(dto.description.as_deref(), Some("Summer"));
    let cleared = dto.set_description(None);
    assert_eq!(cleared.description, None);
}

#[tokio::test]
#[allow(deprecated)]
async fn optional_query_array_is_repeated_per_value_in_builder_requests() -> Result<()> {
    let (client, server) = common::mock_client(0).await;
    let tag1 = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000401").unwrap();
    let tag2 = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000402").unwrap();

    Mock::given(method("POST"))
        .and(path("/search/large-assets"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
        .expect(1)
        .mount(&server)
        .await;

    client
        .search()
        .search_large_assets()
        .tag_ids(vec![tag1, tag2])
        .send()
        .await?;

    let requests = server.received_requests().await.expect("recording enabled");
    let query = requests[0].url.query().unwrap_or_default().to_owned();
    assert!(
        query.contains(&format!("tagIds={tag1}")) && query.contains(&format!("tagIds={tag2}")),
        "array parameters must be serialized as repeated keys: {query}"
    );
    Ok(())
}

#[tokio::test]
async fn search_assets_stream_follows_next_page_tokens() -> Result<()> {
    use immich_models::search::MetadataSearchDTO;

    let (client, server) = common::mock_client(0).await;
    let asset = |id: &str| {
        serde_json::json!({
            "id": id,
            "checksum": "c",
            "createdAt": "2024-01-01T00:00:00.000Z",
            "deviceAssetId": "d",
            "deviceId": "dev",
            "duration": 0,
            "fileCreatedAt": "2024-01-01T00:00:00.000Z",
            "fileModifiedAt": "2024-01-01T00:00:00.000Z",
            "hasMetadata": true,
            "height": 100,
            "width": 100,
            "isArchived": false,
            "isEdited": false,
            "isFavorite": false,
            "isOffline": false,
            "isTrashed": false,
            "localDateTime": "2024-01-01T00:00:00.000Z",
            "originalFileName": "a.jpg",
            "originalPath": "/a.jpg",
            "ownerId": "00000000-0000-0000-0000-000000000001",
            "thumbhash": null,
            "type": "IMAGE",
            "updatedAt": "2024-01-01T00:00:00.000Z",
            "visibility": "timeline"
        })
    };
    let page = |items: Vec<serde_json::Value>, next: Option<&str>| {
        serde_json::json!({
            "albums": {"count": 0, "facets": [], "items": [], "total": 0},
            "assets": {
                "count": items.len(), "facets": [], "items": items,
                "nextCursor": null,
                "nextPage": next, "total": 2
            }
        })
    };

    // 1. More specific mock for page 2 must be mounted first
    Mock::given(method("POST"))
        .and(path("/search/metadata"))
        .and(wiremock::matchers::body_partial_json(serde_json::json!({"page": 2})))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(page(vec![asset("00000000-0000-0000-0000-0000000000b2")], None)),
        )
        .expect(1)
        .mount(&server)
        .await;

    // 2. Initial request (page 1) fallback
    Mock::given(method("POST"))
        .and(path("/search/metadata"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(page(vec![asset("00000000-0000-0000-0000-0000000000b1")], Some("2"))),
        )
        .expect(1)
        .mount(&server)
        .await;

    let body = MetadataSearchDTO::new();
    let mut pages = pin!(client.search().search_assets(body).into_paginator().pages());
    let mut ids = Vec::new();
    while let Some(items) = pages.next().await {
        ids.extend(items?.into_iter().map(|a| a.id.to_string()));
    }
    assert_eq!(
        ids,
        vec![
            "00000000-0000-0000-0000-0000000000b1".to_owned(),
            "00000000-0000-0000-0000-0000000000b2".to_owned()
        ]
    );
    Ok(())
}
