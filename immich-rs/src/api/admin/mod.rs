// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! API endpoints for `admin`.

pub mod database;
pub mod maintenance;
pub mod notifications;

/// Client handle for `Admin` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Admin<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Admin<'a> {
    #[inline]
    pub(crate) const fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }
    /// Returns a sub-client handle for [`database::Database`] endpoints.
    #[must_use]
    pub const fn database(&self) -> database::Database<'a> {
        database::Database::new(self.client)
    }
    /// Returns a sub-client handle for [`maintenance::Maintenance`] endpoints.
    #[must_use]
    pub const fn maintenance(&self) -> maintenance::Maintenance<'a> {
        maintenance::Maintenance::new(self.client)
    }
    /// Returns a sub-client handle for [`notifications::Notifications`] endpoints.
    #[must_use]
    pub const fn notifications(&self) -> notifications::Notifications<'a> {
        notifications::Notifications::new(self.client)
    }
}
