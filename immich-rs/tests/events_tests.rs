#![cfg(all(feature = "socketio", not(target_arch = "wasm32")))]

use std::net::TcpListener;

use immich_rs::{Error, ImmichClient, ServerEvent};
use pretty_assertions::assert_eq;
use serde_json::json;

#[test]
fn server_event_variants_format_and_contain_expected_data() {
    let upload_evt = ServerEvent::UploadSuccess(json!({ "id": "test-asset-123" }));
    let delete_evt = ServerEvent::AssetDelete(json!({ "id": "deleted-456" }));
    let trash_evt = ServerEvent::AssetTrash(json!({ "id": "trash-789" }));
    let person_evt = ServerEvent::PersonThumbnail(json!({ "id": "person-1" }));
    let version_evt = ServerEvent::ServerVersion(json!({ "version": "v1.132.0" }));
    let config_evt = ServerEvent::ConfigUpdate(json!({ "key": "ffmpeg" }));
    let other_evt = ServerEvent::Other {
        event: "on_custom_plugin_event".to_string(),
        payload: json!({ "status": "ok" }),
    };

    assert!(matches!(upload_evt, ServerEvent::UploadSuccess(_)));
    assert!(matches!(delete_evt, ServerEvent::AssetDelete(_)));
    assert!(matches!(trash_evt, ServerEvent::AssetTrash(_)));
    assert!(matches!(person_evt, ServerEvent::PersonThumbnail(_)));
    assert!(matches!(version_evt, ServerEvent::ServerVersion(_)));
    assert!(matches!(config_evt, ServerEvent::ConfigUpdate(_)));

    if let ServerEvent::Other { event, payload } = other_evt {
        assert_eq!(event, "on_custom_plugin_event");
        assert_eq!(payload["status"], "ok");
    } else {
        panic!("expected ServerEvent::Other");
    }
}

#[tokio::test]
async fn subscribe_events_fails_with_socket_io_error_on_closed_port() {
    // Find an unused port and immediately close the listener
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind ephemeral port");
    let port = listener.local_addr().expect("local addr").port();
    drop(listener);

    let client = ImmichClient::builder()
        .base_url(format!("http://127.0.0.1:{port}"))
        .api_key("test-api-key")
        .build()
        .expect("valid client");

    let err = client.subscribe_events().await.unwrap_err();
    match err {
        Error::SocketIo(msg) => {
            assert!(msg.contains("failed to connect to Socket.IO"), "unexpected msg: {msg}");
        }
        other => panic!("expected Error::SocketIo, got {other:?}"),
    }
}
