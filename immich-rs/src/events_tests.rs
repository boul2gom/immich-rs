use futures_util::StreamExt;
use pretty_assertions::assert_eq;
use serde_json::json;

use super::*;

#[test]
fn map_socket_event_routes_all_known_events_and_custom_other_events() {
    let cases = [
        (
            Event::Custom("on_upload_success".into()),
            Payload::Text(vec![json!({"id": "asset-1"})]),
            Some(ServerEvent::UploadSuccess(json!({"id": "asset-1"}))),
        ),
        (
            Event::Custom("on_asset_delete".into()),
            Payload::Text(vec![json!("asset-2")]),
            Some(ServerEvent::AssetDelete(json!("asset-2"))),
        ),
        (
            Event::Custom("on_asset_trash".into()),
            Payload::Text(vec![json!(["a", "b"])]),
            Some(ServerEvent::AssetTrash(json!(["a", "b"]))),
        ),
        (
            Event::Custom("on_person_thumbnail".into()),
            Payload::Text(vec![json!("person-1")]),
            Some(ServerEvent::PersonThumbnail(json!("person-1"))),
        ),
        (
            Event::Custom("on_server_version".into()),
            Payload::Text(vec![json!({"major": 3, "minor": 3, "patch": 5})]),
            Some(ServerEvent::ServerVersion(json!({"major": 3, "minor": 3, "patch": 5}))),
        ),
        (
            Event::Custom("on_config".into()),
            Payload::Text(vec![json!({"oauth": true})]),
            Some(ServerEvent::ConfigUpdate(json!({"oauth": true}))),
        ),
        (
            Event::Custom("on_new_future_event".into()),
            Payload::Text(vec![json!(1), json!(2)]),
            Some(ServerEvent::Other {
                event: "on_new_future_event".into(),
                payload: json!([1, 2]),
            }),
        ),
        (
            Event::Message,
            Payload::Binary(b"hello".as_slice().into()),
            Some(ServerEvent::Other {
                event: "message".into(),
                payload: json!({"binary_len": 5}),
            }),
        ),
    ];

    for (ev, payload, expected) in cases {
        assert_eq!(map_socket_event(ev, payload), expected);
    }
}

#[test]
fn map_socket_event_ignores_internal_lifecycle_events() {
    for lifecycle in [Event::Connect, Event::Close, Event::Error] {
        assert_eq!(map_socket_event(lifecycle, Payload::Text(vec![json!("ignored")])), None);
    }
}

#[test]
fn build_opening_headers_supports_both_api_key_and_bearer_auth() {
    let api_key_client = ImmichClient::new("http://localhost:2283/api", "secret-key").unwrap();
    assert_eq!(
        build_opening_headers(&api_key_client),
        vec![("x-api-key", "secret-key".to_string())]
    );

    let bearer_client = ImmichClient::builder()
        .base_url("http://localhost:2283/api")
        .bearer_token("jwt-token-xyz")
        .build()
        .unwrap();
    assert_eq!(
        build_opening_headers(&bearer_client),
        vec![("Authorization", "Bearer jwt-token-xyz".to_string())]
    );

    let anon_client = ImmichClient::builder()
        .base_url("http://localhost:2283/api")
        .build()
        .unwrap();
    assert_eq!(build_opening_headers(&anon_client), Vec::new());
}

#[tokio::test]
async fn event_subscription_implements_futures_stream() {
    let (tx, rx) = mpsc::unbounded_channel();
    tx.send(ServerEvent::AssetDelete(json!("id-1"))).unwrap();
    tx.send(ServerEvent::Other {
        event: "on_job".into(),
        payload: json!({"active": 2}),
    })
    .unwrap();
    drop(tx);

    let sub = EventSubscription::from_receiver(rx);
    let collected: Vec<ServerEvent> = sub.collect().await;
    assert_eq!(
        collected,
        vec![
            ServerEvent::AssetDelete(json!("id-1")),
            ServerEvent::Other {
                event: "on_job".into(),
                payload: json!({"active": 2}),
            },
        ]
    );
}
