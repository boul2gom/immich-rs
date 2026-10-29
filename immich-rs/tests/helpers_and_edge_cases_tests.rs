mod common;

use std::collections::HashMap;
use std::pin::pin;

use anyhow::Result;
use futures_util::StreamExt;
use immich_rs::calculate_file_checksum;
use pretty_assertions::assert_eq;
use sha1::{Digest, Sha1};
use tempfile::tempdir;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn calculate_file_checksum_handles_empty_file_and_multi_chunk_file() -> Result<()> {
    let dir = tempdir()?;

    // 1. Empty file (0 bytes) -> SHA-1 is well-known constant
    let empty_path = dir.path().join("empty.bin");
    tokio::fs::write(&empty_path, b"").await?;
    let empty_sha1 = calculate_file_checksum(&empty_path).await?;
    assert_eq!(empty_sha1, "da39a3ee5e6b4b0d3255bfef95601890afd80709");

    // 2. Large file (140 KB > 64 KB internal buffer)
    let large_path = dir.path().join("large.bin");
    let large_bytes = vec![0xABu8; 140 * 1024];
    tokio::fs::write(&large_path, &large_bytes).await?;

    let expected_hasher = Sha1::digest(&large_bytes);
    let expected_sha1 = hex::encode(expected_hasher);

    let calculated_sha1 = calculate_file_checksum(&large_path).await?;
    assert_eq!(calculated_sha1, expected_sha1);

    Ok(())
}

#[tokio::test]
async fn upload_bulk_handles_empty_collection_cleanly() -> Result<()> {
    let (client, _server) = common::mock_client(0).await;
    let mut stream = pin!(client.upload_bulk(Vec::new()).concurrency(4).into_stream());

    let first = stream.next().await;
    assert!(first.is_none(), "empty bulk upload must yield 0 items");
    Ok(())
}

#[test]
fn auth_env_resolution_respects_variable_precedence() -> Result<()> {
    let mut env = HashMap::new();
    env.insert("IMMICH_URL", "http://url.example.com/api");
    env.insert("IMMICH_BASE_URL", "http://base.example.com/api");
    env.insert("IMMICH_INSTANCE_URL", "http://instance.example.com/api");
    env.insert("IMMICH_API_KEY", "test-key");

    // 1. All three set: IMMICH_INSTANCE_URL wins
    let c1 = immich_rs::ImmichClient::builder()
        .from_env_map(|k| env.get(k).copied().map(String::from))?
        .build()?;
    assert_eq!(c1.base_url().as_str(), "http://instance.example.com/api/");

    // 2. Remove INSTANCE_URL: IMMICH_BASE_URL wins
    env.remove("IMMICH_INSTANCE_URL");
    let c2 = immich_rs::ImmichClient::builder()
        .from_env_map(|k| env.get(k).copied().map(String::from))?
        .build()?;
    assert_eq!(c2.base_url().as_str(), "http://base.example.com/api/");

    // 3. Remove BASE_URL: IMMICH_URL wins
    env.remove("IMMICH_BASE_URL");
    let c3 = immich_rs::ImmichClient::builder()
        .from_env_map(|k| env.get(k).copied().map(String::from))?
        .build()?;
    assert_eq!(c3.base_url().as_str(), "http://url.example.com/api/");

    Ok(())
}

#[tokio::test]
async fn download_cancellation_cleans_up_temporary_file() -> Result<()> {
    let (client, server) = common::mock_client(0).await;
    let asset_id = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000099").unwrap();

    // Set up mock returning infinite or delayed data
    Mock::given(method("GET"))
        .and(path(format!("/assets/{asset_id}/original")))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_raw(vec![42u8; 100_000], "application/octet-stream")
                .set_delay(std::time::Duration::from_millis(500)),
        )
        .mount(&server)
        .await;

    let dir = tempdir()?;
    let dest = dir.path().join("cancelled.jpg");

    // Run download with a very short timeout so it drops before completing
    let dl_future = client.download(asset_id, &dest);
    let _ = tokio::time::timeout(std::time::Duration::from_millis(20), dl_future).await;

    // Give asynchronous filesystem drop a brief moment
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    // Ensure neither the destination nor the temporary file remains on disk
    let remaining_files: Vec<_> = std::fs::read_dir(dir.path())?
        .filter_map(|e| e.ok().map(|e| e.file_name()))
        .collect();

    assert_eq!(
        remaining_files,
        Vec::<std::ffi::OsString>::new(),
        "temporary file must be removed when future is cancelled/dropped"
    );

    Ok(())
}

#[tokio::test]
async fn download_to_writer_streams_multi_chunk_payload_and_reports_accurate_size() -> Result<()> {
    let (client, server) = common::mock_client(0).await;
    let asset_id = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000098").unwrap();

    // 256 KiB payload spanning multiple 64 KiB chunks
    let payload = vec![0xABu8; 256 * 1024];

    Mock::given(method("GET"))
        .and(path(format!("/assets/{asset_id}/original")))
        .respond_with(ResponseTemplate::new(200).set_body_raw(payload.clone(), "application/octet-stream"))
        .mount(&server)
        .await;

    let mut output = Vec::new();
    let bytes_written = client.download_to_writer(asset_id, &mut output).await?;

    assert_eq!(bytes_written, 256 * 1024);
    assert_eq!(output.len(), 256 * 1024);
    assert_eq!(output, payload);

    Ok(())
}

#[tokio::test]
async fn download_atomic_multi_chunk_preserves_integrity_without_leftovers() -> Result<()> {
    let (client, server) = common::mock_client(0).await;
    let asset_id = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000097").unwrap();

    // 200 KiB payload across 3+ chunk boundaries
    let payload: Vec<u8> = (0u32..200 * 1024).map(|i| (i % 251) as u8).collect();

    Mock::given(method("GET"))
        .and(path(format!("/assets/{asset_id}/original")))
        .respond_with(ResponseTemplate::new(200).set_body_raw(payload.clone(), "application/octet-stream"))
        .mount(&server)
        .await;

    let dir = tempdir()?;
    let dest = dir.path().join("video.mp4");

    client.download(asset_id, &dest).await?;

    let downloaded_bytes = tokio::fs::read(&dest).await?;
    assert_eq!(downloaded_bytes.len(), 200 * 1024);
    assert_eq!(downloaded_bytes, payload);

    let entries: Vec<_> = std::fs::read_dir(dir.path())?
        .filter_map(|e| e.ok().map(|e| e.file_name()))
        .collect();
    assert_eq!(entries, vec![std::ffi::OsString::from("video.mp4")]);

    Ok(())
}

#[tokio::test]
async fn upload_asset_reports_progress_via_callback_and_watch_channel() -> Result<()> {
    let (client, server) = common::mock_client(0).await;
    let dir = tempdir()?;
    let file_path = dir.path().join("tracked_upload.jpg");
    // 128 KiB of data to ensure multiple chunks are transferred
    let dummy_data = vec![0xEEu8; 128 * 1024];
    tokio::fs::write(&file_path, &dummy_data).await?;

    let expected_id = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000088").unwrap();
    Mock::given(method("POST"))
        .and(path("/assets"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": expected_id,
            "status": "created"
        })))
        .mount(&server)
        .await;

    // Test 1: Via Callback
    let progress_reports = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let last_percentage = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));

    let pr_clone = progress_reports.clone();
    let lp_clone = last_percentage.clone();

    let options = immich_rs::UploadOptions::new(&file_path).on_progress(move |p| {
        pr_clone.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if let Some(pct) = p.percentage() {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            lp_clone.store(pct as u32, std::sync::atomic::Ordering::Relaxed);
        }
    });

    let res = client.upload_asset(options).await?;
    assert_eq!(res.id, expected_id);
    assert!(
        progress_reports.load(std::sync::atomic::Ordering::Relaxed) > 0,
        "callback must be called at least once"
    );
    assert_eq!(
        last_percentage.load(std::sync::atomic::Ordering::Relaxed),
        100,
        "final progress must reach 100%"
    );

    // Test 2: Via tokio watch channel
    let (tx, mut rx) = tokio::sync::watch::channel(immich_rs::TransferProgress::new(0, None));
    let options2 = immich_rs::UploadOptions::new(&file_path).progress_sender(tx);

    let res2 = client.upload_asset(options2).await?;
    assert_eq!(res2.id, expected_id);

    let final_progress = *rx.borrow_and_update();
    assert_eq!(final_progress.transferred, 128 * 1024);
    assert_eq!(final_progress.total, Some(128 * 1024));
    assert!(final_progress.is_complete());

    Ok(())
}

#[tokio::test]
async fn download_with_progress_and_download_to_writer_report_accurate_progress() -> Result<()> {
    let (client, server) = common::mock_client(0).await;
    let asset_id = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000089").unwrap();
    let payload = vec![0xCCu8; 128 * 1024];

    Mock::given(method("GET"))
        .and(path(format!("/assets/{asset_id}/original")))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_raw(payload.clone(), "application/octet-stream")
                .insert_header("content-length", (128 * 1024).to_string()),
        )
        .mount(&server)
        .await;

    // Test download_to_writer_with_progress via callback
    let total_reported = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let tr_clone = total_reported.clone();
    let progress_sender = immich_rs::ProgressSender::Callback(std::sync::Arc::new(move |p| {
        tr_clone.store(p.transferred, std::sync::atomic::Ordering::Relaxed);
    }));

    let mut dest_buf = Vec::new();
    let written = client
        .download_to_writer_with_progress(asset_id, &mut dest_buf, progress_sender)
        .await?;

    assert_eq!(written, 128 * 1024);
    assert_eq!(dest_buf, payload);
    assert_eq!(total_reported.load(std::sync::atomic::Ordering::Relaxed), 128 * 1024);

    // Test download_with_progress via watch channel
    let dir = tempdir()?;
    let dest_file = dir.path().join("tracked_dl.jpg");
    let (tx, rx) = tokio::sync::watch::channel(immich_rs::TransferProgress::new(0, None));

    client
        .download_with_progress(asset_id, &dest_file, immich_rs::ProgressSender::Watch(tx))
        .await?;

    let downloaded = tokio::fs::read(&dest_file).await?;
    assert_eq!(downloaded, payload);
    let final_p = *rx.borrow();
    assert_eq!(final_p.transferred, 128 * 1024);
    assert_eq!(final_p.total, Some(128 * 1024));
    assert!(final_p.is_complete());

    Ok(())
}

#[tokio::test]
async fn upload_asset_bytes_sends_in_memory_data_correctly() -> Result<()> {
    let (client, server) = common::mock_client(0).await;
    let fake_asset_id = uuid::Uuid::new_v4();

    Mock::given(method("POST"))
        .and(path("/assets"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": fake_asset_id.to_string(),
            "status": "created"
        })))
        .mount(&server)
        .await;

    let payload = b"Hello Immich from WASM / in-memory bytes!".to_vec();
    let checksum = immich_rs::calculate_bytes_checksum(&payload);
    let options = immich_rs::UploadBytesOptions::new("wasm_test.png")
        .checksum(&checksum)
        .is_favorite(true);

    let res = client.upload_asset_bytes(payload, options).await?;
    assert_eq!(res.id, fake_asset_id);

    Ok(())
}
