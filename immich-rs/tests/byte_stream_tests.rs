use bytes::Bytes;
use immich_rs::ByteStream;
use immich_rs::error::Result;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn byte_stream_metadata_and_consumption() -> Result<()> {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/media/download"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_bytes(b"hello binary world")
                .insert_header("Content-Type", "image/jpeg")
                .insert_header("Content-Disposition", r#"attachment; filename="vacation_2026.jpg""#),
        )
        .mount(&server)
        .await;

    let resp = reqwest::get(format!("{}/media/download", server.uri())).await.unwrap();
    let stream = ByteStream::new(resp);

    assert_eq!(stream.status(), reqwest::StatusCode::OK);
    assert_eq!(stream.content_type(), Some("image/jpeg"));
    assert_eq!(stream.content_length(), Some(18));
    assert_eq!(stream.filename().as_deref(), Some("vacation_2026.jpg"));

    let data = stream.bytes().await?;
    assert_eq!(&data[..], b"hello binary world");

    Ok(())
}

#[tokio::test]
async fn byte_stream_filename_rfc5987_percent_decoded() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/media/utf8"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(b"data").insert_header(
            "Content-Disposition",
            "attachment; filename*=UTF-8''my%20summer%20photo.png",
        ))
        .mount(&server)
        .await;

    let resp = reqwest::get(format!("{}/media/utf8", server.uri())).await.unwrap();
    let stream = ByteStream::new(resp);

    assert_eq!(stream.filename().as_deref(), Some("my summer photo.png"));
}

#[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
#[tokio::test]
async fn byte_stream_save_to_path_streams_directly_to_disk() -> Result<()> {
    let server = MockServer::start().await;
    let payload = vec![0x42u8; 128 * 1024];

    Mock::given(method("GET"))
        .and(path("/download/large"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(payload.clone()))
        .mount(&server)
        .await;

    let resp = reqwest::get(format!("{}/download/large", server.uri())).await.unwrap();
    let mut stream = ByteStream::new(resp);

    let temp_dir = tempfile::tempdir().unwrap();
    let dest_file = temp_dir.path().join("nested").join("test_file.bin");

    let written = stream.save_to_path(&dest_file).await?;
    assert_eq!(written, 128 * 1024);

    let read_back = tokio::fs::read(&dest_file).await?;
    assert_eq!(read_back, payload);

    Ok(())
}

#[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
#[tokio::test]
async fn byte_stream_into_async_read_reads_chunks() -> Result<()> {
    use tokio::io::AsyncReadExt;

    let server = MockServer::start().await;
    let payload = b"streaming async read chunk";

    Mock::given(method("GET"))
        .and(path("/download/read"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(payload.to_vec()))
        .mount(&server)
        .await;

    let resp = reqwest::get(format!("{}/download/read", server.uri())).await.unwrap();
    let stream = ByteStream::new(resp);

    let mut async_reader = stream.into_async_read();
    let mut buf = Vec::new();
    async_reader.read_to_end(&mut buf).await?;

    assert_eq!(&buf[..], payload);
    Ok(())
}

#[tokio::test]
async fn byte_stream_into_inner_returns_reqwest_response_before_poll() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/inner"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(b"inner payload"))
        .mount(&server)
        .await;

    let resp = reqwest::get(format!("{}/inner", server.uri())).await.unwrap();
    let stream = ByteStream::new(resp);

    let inner = stream.into_inner();
    assert!(inner.is_some());
    let bytes: Bytes = inner.unwrap().bytes().await.unwrap();
    assert_eq!(&bytes[..], b"inner payload");
}

#[tokio::test]
async fn byte_stream_into_inner_returns_none_after_consumption() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/consumed"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(b"consumed payload"))
        .mount(&server)
        .await;

    let resp = reqwest::get(format!("{}/consumed", server.uri())).await.unwrap();
    let mut stream = ByteStream::new(resp);

    // Consume first chunk
    let first_chunk = stream.next().await;
    assert!(first_chunk.is_some());

    // Once streaming begins, into_inner() must safely return None
    assert!(stream.into_inner().is_none());
}

#[tokio::test]
async fn byte_stream_filename_falls_back_to_ascii_when_rfc5987_is_absent() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/ascii-file"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_bytes(b"data")
                .insert_header("Content-Disposition", r#"inline; filename="photo_backup.png""#),
        )
        .mount(&server)
        .await;

    let resp = reqwest::get(format!("{}/ascii-file", server.uri())).await.unwrap();
    let stream = ByteStream::new(resp);

    assert_eq!(stream.filename().as_deref(), Some("photo_backup.png"));
}

#[tokio::test]
async fn byte_stream_filename_returns_none_for_missing_or_malformed_header() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/malformed"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_bytes(b"data")
                .insert_header("Content-Disposition", "inline; invalid-token"),
        )
        .mount(&server)
        .await;

    let resp = reqwest::get(format!("{}/malformed", server.uri())).await.unwrap();
    let stream = ByteStream::new(resp);

    assert_eq!(stream.filename(), None);
}
