#![cfg(feature = "socketio")]

use immich_rs::ServerEvent;
use serde_json::json;

#[test]
fn server_event_variants_format_and_contain_expected_data() {
    let upload_evt = ServerEvent::UploadSuccess(json!({ "id": "test-asset-123" }));
    let delete_evt = ServerEvent::AssetDelete(json!({ "id": "deleted-456" }));
    let other_evt = ServerEvent::Other {
        event: "on_custom_plugin_event".to_string(),
        payload: json!({ "status": "ok" }),
    };

    assert!(matches!(upload_evt, ServerEvent::UploadSuccess(_)));
    assert!(matches!(delete_evt, ServerEvent::AssetDelete(_)));
    if let ServerEvent::Other { event, payload } = other_evt {
        assert_eq!(event, "on_custom_plugin_event");
        assert_eq!(payload["status"], "ok");
    } else {
        panic!("expected ServerEvent::Other");
    }
}
