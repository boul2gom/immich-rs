// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! Strongly typed entity identifiers for Immich.
//!
//! Provides zero-cost `#[repr(transparent)]` wrappers around [`uuid::Uuid`] for domain entities
//! such as [`AssetId`], [`AlbumId`], [`UserId`], [`TagId`], and others.
//!
//! Each identifier is `#[serde(transparent)]` and implements [`std::ops::Deref<Target = uuid::Uuid>`]
//! and [`std::borrow::Borrow<uuid::Uuid>`], allowing direct access to UUID methods while preventing
//! accidental ID mixing at compile time.

use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

macro_rules! define_id {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub Uuid);

        impl $name {
            /// Creates a new strongly typed identifier from a [`Uuid`].
            #[inline]
            #[must_use]
            pub const fn new(uuid: Uuid) -> Self {
                Self(uuid)
            }

            /// Unwraps and returns the inner [`Uuid`].
            #[inline]
            #[must_use]
            pub const fn into_inner(self) -> Uuid {
                self.0
            }

            /// Returns a reference to the inner [`Uuid`].
            #[inline]
            #[must_use]
            pub const fn as_uuid(&self) -> &Uuid {
                &self.0
            }

            /// Parses an identifier from a string slice representing a UUID.
            ///
            /// # Errors
            /// Returns an error if the string is not a valid UUID.
            #[inline]
            pub fn parse_str(s: &str) -> std::result::Result<Self, uuid::Error> {
                Uuid::parse_str(s).map(Self)
            }
        }

        impl Deref for $name {
            type Target = Uuid;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl AsRef<Uuid> for $name {
            #[inline]
            fn as_ref(&self) -> &Uuid {
                &self.0
            }
        }

        impl Borrow<Uuid> for $name {
            #[inline]
            fn borrow(&self) -> &Uuid {
                &self.0
            }
        }

        impl From<Uuid> for $name {
            #[inline]
            fn from(uuid: Uuid) -> Self {
                Self(uuid)
            }
        }

        impl From<&Uuid> for $name {
            #[inline]
            fn from(uuid: &Uuid) -> Self {
                Self(*uuid)
            }
        }

        impl From<&$name> for $name {
            #[inline]
            fn from(id: &$name) -> Self {
                *id
            }
        }

        impl From<$name> for Uuid {
            #[inline]
            fn from(id: $name) -> Self {
                id.0
            }
        }

        impl From<&$name> for Uuid {
            #[inline]
            fn from(id: &$name) -> Self {
                id.0
            }
        }

        impl From<$name> for String {
            #[inline]
            fn from(id: $name) -> Self {
                id.0.to_string()
            }
        }

        impl TryFrom<&str> for $name {
            type Error = uuid::Error;

            #[inline]
            fn try_from(s: &str) -> std::result::Result<Self, Self::Error> {
                Self::parse_str(s)
            }
        }

        impl TryFrom<String> for $name {
            type Error = uuid::Error;

            #[inline]
            fn try_from(s: String) -> std::result::Result<Self, Self::Error> {
                Self::parse_str(&s)
            }
        }

        impl fmt::Display for $name {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(&self.0, f)
            }
        }

        impl FromStr for $name {
            type Err = uuid::Error;

            #[inline]
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::parse_str(s)
            }
        }
    };
}

define_id!(ActivityId, "A strongly typed identifier for an Immich activity.");
define_id!(AlbumId, "A strongly typed identifier for an Immich album.");
define_id!(ApiKeyId, "A strongly typed identifier for an Immich API key.");
define_id!(
    AssetFileId,
    "A strongly typed identifier for an Immich auxiliary asset file."
);
define_id!(AssetId, "A strongly typed identifier for an Immich media asset.");
define_id!(
    ClusterGroupId,
    "A strongly typed identifier for an Immich facial-recognition cluster group."
);
define_id!(
    DuplicateId,
    "A strongly typed identifier for an Immich duplicate asset group."
);
define_id!(
    FaceId,
    "A strongly typed identifier for a detected face bounding box on an asset."
);
define_id!(LibraryId, "A strongly typed identifier for an Immich library.");
define_id!(MemoryId, "A strongly typed identifier for an Immich memory.");
define_id!(
    NotificationId,
    "A strongly typed identifier for an Immich user notification."
);
define_id!(
    PartnerId,
    "A strongly typed identifier for an Immich partner sharing relationship."
);
define_id!(PersonId, "A strongly typed identifier for an Immich person.");
define_id!(PluginId, "A strongly typed identifier for an Immich plugin.");
define_id!(
    SessionId,
    "A strongly typed identifier for an Immich authentication or streaming session."
);
define_id!(SharedLinkId, "A strongly typed identifier for an Immich shared link.");
define_id!(StackId, "A strongly typed identifier for an Immich asset stack.");
define_id!(TagId, "A strongly typed identifier for an Immich tag.");
define_id!(UserId, "A strongly typed identifier for an Immich user.");
define_id!(
    WorkflowId,
    "A strongly typed identifier for an Immich automation workflow."
);
