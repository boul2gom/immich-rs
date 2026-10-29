//! Real-time event streaming via Socket.IO.
//!
//! Provides real-time notifications for upload completions, asset deletions,
//! library scans, and background job updates.
//!
//! Activated with the `socketio` cargo feature.

use std::sync::Arc;

use futures_util::FutureExt;
use rust_socketio::Payload;
use rust_socketio::asynchronous::{Client as SocketIoClient, ClientBuilder};
use tokio::sync::mpsc;

use crate::client::ImmichClient;
use crate::error::{Error, Result};

/// Strongly typed Immich real-time event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerEvent {
    /// An asset was successfully uploaded and registered.
    UploadSuccess(serde_json::Value),
    /// An asset was deleted.
    AssetDelete(serde_json::Value),
    /// An asset was moved to trash.
    AssetTrash(serde_json::Value),
    /// A person's facial thumbnail or metadata was updated.
    PersonThumbnail(serde_json::Value),
    /// Server version update notification.
    ServerVersion(serde_json::Value),
    /// Server configuration change notification.
    ConfigUpdate(serde_json::Value),
    /// Any other event emitted by Immich.
    Other {
        /// Name of the Socket.IO event.
        event: String,
        /// Raw payload received with the event.
        payload: serde_json::Value,
    },
}

/// Active subscription to real-time Immich events.
pub struct EventSubscription {
    client: SocketIoClient,
    receiver: mpsc::UnboundedReceiver<ServerEvent>,
}

impl std::fmt::Debug for EventSubscription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventSubscription").finish_non_exhaustive()
    }
}

impl EventSubscription {
    /// Waits for and receives the next real-time event from the server.
    ///
    /// Returns `None` if the connection has been closed.
    pub async fn next_event(&mut self) -> Option<ServerEvent> {
        self.receiver.recv().await
    }

    /// Explicitly disconnects the event stream.
    ///
    /// # Errors
    /// Returns an error if the disconnection handshake fails.
    pub async fn disconnect(self) -> Result<()> {
        self.client
            .disconnect()
            .await
            .map_err(|e| Error::Validation(format!("Failed to disconnect Socket.IO client: {e}")))
    }
}

impl ImmichClient {
    /// Subscribes to Immich real-time events over Socket.IO.
    ///
    /// # Errors
    /// Returns an error if connecting to the Socket.IO endpoint fails.
    pub async fn subscribe_events(&self) -> Result<EventSubscription> {
        let (tx, rx) = mpsc::unbounded_channel();
        let tx = Arc::new(tx);

        let base = self.base_url();
        let mut socket_url = base.clone();
        // Socket.IO server connects to the root origin
        socket_url.set_path("");

        let mut builder = ClientBuilder::new(socket_url.as_str());

        if let Some(key) = self.api_key() {
            builder = builder.opening_header("x-api-key", key);
        }

        let known_events = [
            "on_upload_success",
            "on_asset_delete",
            "on_asset_trash",
            "on_person_thumbnail",
            "on_server_version",
            "on_config",
        ];

        for &event_name in &known_events {
            let tx_outer = Arc::clone(&tx);
            let name_outer = event_name.to_string();
            builder = builder.on(event_name, move |payload: Payload, _| {
                let tx_inner = Arc::clone(&tx_outer);
                let name_inner = name_outer.clone();
                async move {
                    let val = match payload {
                        Payload::Text(mut vals) => {
                            if vals.len() == 1 {
                                vals.remove(0)
                            } else {
                                serde_json::Value::Array(vals)
                            }
                        }
                        Payload::Binary(b) => serde_json::json!({ "binary_len": b.len() }),
                        #[allow(deprecated)]
                        Payload::String(s) => serde_json::from_str(&s).unwrap_or(serde_json::Value::String(s)),
                    };

                    let event = match name_inner.as_str() {
                        "on_upload_success" => ServerEvent::UploadSuccess(val),
                        "on_asset_delete" => ServerEvent::AssetDelete(val),
                        "on_asset_trash" => ServerEvent::AssetTrash(val),
                        "on_person_thumbnail" => ServerEvent::PersonThumbnail(val),
                        "on_server_version" => ServerEvent::ServerVersion(val),
                        "on_config" => ServerEvent::ConfigUpdate(val),
                        _ => ServerEvent::Other {
                            event: name_inner,
                            payload: val,
                        },
                    };
                    let _ = tx_inner.send(event);
                }
                .boxed()
            });
        }

        let client = builder
            .connect()
            .await
            .map_err(|e| Error::Validation(format!("Failed to connect to Socket.IO: {e}")))?;

        Ok(EventSubscription { client, receiver: rx })
    }
}
