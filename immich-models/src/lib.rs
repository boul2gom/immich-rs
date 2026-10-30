// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]
#![doc = include_str!("../README.md")]

pub mod activities;
pub mod admin;
pub mod albums;
pub mod api_keys;
pub mod assets;
pub mod authentication;
pub mod clusters;
pub mod common;
pub mod duplicates;
pub mod id;
pub mod jobs;
pub mod libraries;
pub mod map;
pub mod memories;
pub mod notifications;
pub mod partners;
pub mod people;
pub mod plugins;
pub mod queues;
pub mod search;
pub mod server;
pub mod sessions;
pub mod shared_links;
pub mod sync;
pub mod system;
pub mod tags;
pub mod timeline;
pub mod trash;
pub mod users;
pub mod workflows;

pub use id::*;

/// Target Immich `OpenAPI` version from which these models were generated.
pub const TARGET_IMMICH_VERSION: &str = "v3.3.5";
