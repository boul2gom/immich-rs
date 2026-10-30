//! Real-time event streaming via Socket.IO.
//!
//! Provides real-time notifications for upload completions, asset deletions,
//! facial recognition updates, and server configuration changes.
//!
//! Activated with the `socketio` Cargo feature on native targets.

use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use futures_core::Stream;
use futures_util::FutureExt;
use rust_socketio::asynchronous::{Client as SocketIoClient, ClientBuilder};
use rust_socketio::{Event, Payload};
use tokio::sync::mpsc;

use crate::client::ImmichClient;
use crate::error::{Error, Result};

/// Strongly typed Immich real-time event.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
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
    /// Any other custom or future event emitted by Immich.
    Other {
        /// Name of the Socket.IO event.
        event: String,
        /// Raw JSON payload received with the event.
        payload: serde_json::Value,
    },
}

/// Active subscription to real-time Immich events.
///
/// Implements [`futures_core::Stream`] so it can be consumed with `StreamExt::next`
/// or directly via [`Self::next_event`].
pub struct EventSubscription {
    client: Option<SocketIoClient>,
    receiver: mpsc::UnboundedReceiver<ServerEvent>,
}

impl std::fmt::Debug for EventSubscription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventSubscription").finish_non_exhaustive()
    }
}

impl EventSubscription {
    /// Creates an in-memory `EventSubscription` backed by an `mpsc` receiver (used internally and in tests).
    #[cfg(test)]
    pub(crate) const fn from_receiver(receiver: mpsc::UnboundedReceiver<ServerEvent>) -> Self {
        Self { client: None, receiver }
    }

    /// Waits for and receives the next real-time event from the server.
    ///
    /// Returns `None` if the connection has been closed.
    pub async fn next_event(&mut self) -> Option<ServerEvent> {
        self.receiver.recv().await
    }

    /// Explicitly disconnects the Socket.IO event stream.
    ///
    /// # Errors
    ///
    /// Returns [`Error::SocketIo`] if the disconnection handshake fails.
    pub async fn disconnect(self) -> Result<()> {
        if let Some(client) = self.client {
            client
                .disconnect()
                .await
                .map_err(|e| Error::SocketIo(format!("failed to disconnect Socket.IO client: {e}")))?;
        }
        Ok(())
    }
}

impl Stream for EventSubscription {
    type Item = ServerEvent;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.receiver.poll_recv(cx)
    }
}

pub(crate) fn payload_to_json(payload: Payload) -> serde_json::Value {
    match payload {
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
    }
}

/// Maps a raw `rust_socketio` [`Event`] and [`Payload`] into a [`ServerEvent`],
/// ignoring internal Socket.IO lifecycle events (`Connect`, `Close`, `Error`).
#[must_use]
pub(crate) fn map_socket_event(event: Event, payload: Payload) -> Option<ServerEvent> {
    let event_name = match event {
        Event::Connect | Event::Close | Event::Error => return None,
        Event::Message => "message".to_string(),
        Event::Custom(name) => name,
    };

    let val = payload_to_json(payload);

    let mapped = match event_name.as_str() {
        "on_upload_success" => ServerEvent::UploadSuccess(val),
        "on_asset_delete" => ServerEvent::AssetDelete(val),
        "on_asset_trash" => ServerEvent::AssetTrash(val),
        "on_person_thumbnail" => ServerEvent::PersonThumbnail(val),
        "on_server_version" => ServerEvent::ServerVersion(val),
        "on_config" => ServerEvent::ConfigUpdate(val),
        _ => ServerEvent::Other {
            event: event_name,
            payload: val,
        },
    };
    Some(mapped)
}

/// Builds the list of opening HTTP authentication headers (`x-api-key` or `Authorization: Bearer ...`)
/// forwarded during the Socket.IO handshake.
#[must_use]
pub(crate) fn build_opening_headers(client: &ImmichClient) -> Vec<(&'static str, String)> {
    let mut headers = Vec::new();
    if let Some(key) = client.api_key() {
        headers.push(("x-api-key", key));
    } else if let Some(token) = client.bearer_token() {
        headers.push(("Authorization", format!("Bearer {token}")));
    }
    headers
}

impl ImmichClient {
    /// Subscribes to Immich real-time events over Socket.IO.
    ///
    /// Forwards either `x-api-key` or `Authorization: Bearer <token>` depending on how the client was authenticated,
    /// and delivers both known events and unknown server events (`ServerEvent::Other`) via `on_any`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::SocketIo`] if connecting to the Socket.IO endpoint fails.
    pub async fn subscribe_events(&self) -> Result<EventSubscription> {
        let (tx, rx) = mpsc::unbounded_channel();
        let tx = Arc::new(tx);

        let mut socket_url = self.base_url().clone();
        socket_url.set_path("");

        let mut builder = ClientBuilder::new(socket_url.as_str());

        for (header_name, header_val) in build_opening_headers(self) {
            builder = builder.opening_header(header_name, header_val);
        }

        let tx_any = Arc::clone(&tx);
        builder = builder.on_any(move |event: Event, payload: Payload, _| {
            let tx_inner = Arc::clone(&tx_any);
            async move {
                if let Some(mapped) = map_socket_event(event, payload) {
                    let _ = tx_inner.send(mapped);
                }
            }
            .boxed()
        });

        let client = builder
            .connect()
            .await
            .map_err(|e| Error::SocketIo(format!("failed to connect to Socket.IO: {e}")))?;

        Ok(EventSubscription {
            client: Some(client),
            receiver: rx,
        })
    }
}

#[cfg(test)]
#[path = "events_tests.rs"]
mod tests;
