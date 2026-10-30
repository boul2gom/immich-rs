// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! API endpoints for `database`.

use immich_models::admin::database::{DatabaseBackupDeleteDTO, DatabaseBackupListResponseDTO};

/// Fluent request builder for [`Database::delete_database_backup`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteDatabaseBackupBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    body: DatabaseBackupDeleteDTO,
}

impl DeleteDatabaseBackupBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }
    /// Adds a custom HTTP header to this specific request.
    ///
    /// Invalid header names or values are recorded and surfaced as [`Error::Validation`](crate::Error::Validation)
    /// when the request is sent.
    pub fn header(
        mut self,
        key: impl TryInto<reqwest::header::HeaderName>,
        value: impl TryInto<reqwest::header::HeaderValue>,
    ) -> Self {
        self.request_config.insert_header(key, value);
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<(), crate::Error> {
        tracing::trace!(
            module = "admin/database",
            method = "DELETE",
            path = "/admin/database-backups",
            "HTTP request"
        );
        let mut request = self
            .client
            .prepare_request(reqwest::Method::DELETE, "/admin/database-backups")?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        self.client.execute_unit("admin/database", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for DeleteDatabaseBackupBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Database::download_database_backup`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DownloadDatabaseBackupBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    filename: String,
}

impl DownloadDatabaseBackupBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }
    /// Adds a custom HTTP header to this specific request.
    ///
    /// Invalid header names or values are recorded and surfaced as [`Error::Validation`](crate::Error::Validation)
    /// when the request is sent.
    pub fn header(
        mut self,
        key: impl TryInto<reqwest::header::HeaderName>,
        value: impl TryInto<reqwest::header::HeaderValue>,
    ) -> Self {
        self.request_config.insert_header(key, value);
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<crate::ByteStream, crate::Error> {
        tracing::trace!(
            module = "admin/database",
            method = "GET",
            path = "/admin/database-backups/{filename}",
            "HTTP request"
        );
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("admin"),
                crate::core::PathSegment::Static("database-backups"),
                crate::core::PathSegment::Param(self.filename.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        let raw_response = self.client.execute_stream("admin/database", request).await?;
        Ok(crate::ByteStream::new(raw_response))
    }
}

impl<'a> std::future::IntoFuture for DownloadDatabaseBackupBuilder<'a> {
    type Output = Result<crate::ByteStream, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Database::list_database_backups`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct ListDatabaseBackupsBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
}

impl ListDatabaseBackupsBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }
    /// Adds a custom HTTP header to this specific request.
    ///
    /// Invalid header names or values are recorded and surfaced as [`Error::Validation`](crate::Error::Validation)
    /// when the request is sent.
    pub fn header(
        mut self,
        key: impl TryInto<reqwest::header::HeaderName>,
        value: impl TryInto<reqwest::header::HeaderValue>,
    ) -> Self {
        self.request_config.insert_header(key, value);
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<DatabaseBackupListResponseDTO, crate::Error> {
        tracing::trace!(
            module = "admin/database",
            method = "GET",
            path = "/admin/database-backups",
            "HTTP request"
        );
        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/admin/database-backups")?;
        request = self.request_config.apply(request)?;
        let response = self
            .client
            .execute::<DatabaseBackupListResponseDTO>("admin/database", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for ListDatabaseBackupsBuilder<'a> {
    type Output = Result<DatabaseBackupListResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Database::start_database_restore_flow`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct StartDatabaseRestoreFlowBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
}

impl StartDatabaseRestoreFlowBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }
    /// Adds a custom HTTP header to this specific request.
    ///
    /// Invalid header names or values are recorded and surfaced as [`Error::Validation`](crate::Error::Validation)
    /// when the request is sent.
    pub fn header(
        mut self,
        key: impl TryInto<reqwest::header::HeaderName>,
        value: impl TryInto<reqwest::header::HeaderValue>,
    ) -> Self {
        self.request_config.insert_header(key, value);
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<(), crate::Error> {
        tracing::trace!(
            module = "admin/database",
            method = "POST",
            path = "/admin/database-backups/start-restore",
            "HTTP request"
        );
        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/admin/database-backups/start-restore")?;
        request = self.request_config.apply(request)?;
        self.client.execute_unit("admin/database", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for StartDatabaseRestoreFlowBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Database::upload_database_backup`].
#[derive(Debug)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UploadDatabaseBackupBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    form: reqwest::multipart::Form,
}

impl UploadDatabaseBackupBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }
    /// Adds a custom HTTP header to this specific request.
    ///
    /// Invalid header names or values are recorded and surfaced as [`Error::Validation`](crate::Error::Validation)
    /// when the request is sent.
    pub fn header(
        mut self,
        key: impl TryInto<reqwest::header::HeaderName>,
        value: impl TryInto<reqwest::header::HeaderValue>,
    ) -> Self {
        self.request_config.insert_header(key, value);
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<(), crate::Error> {
        tracing::trace!(
            module = "admin/database",
            method = "POST",
            path = "/admin/database-backups/upload",
            "HTTP request"
        );
        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/admin/database-backups/upload")?;
        request = self.request_config.apply(request)?;
        request = request.multipart(self.form);
        self.client.execute_unit("admin/database", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for UploadDatabaseBackupBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Client handle for `Database` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Database<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Database<'a> {
    #[inline]
    pub(crate) const fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }
    /// Delete database backup
    ///
    /// Delete a backup by its filename
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `body` | `DatabaseBackupDeleteDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn delete_database_backup(&self, body: impl Into<DatabaseBackupDeleteDTO>) -> DeleteDatabaseBackupBuilder<'a> {
        DeleteDatabaseBackupBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body: body.into(),
        }
    }
    /// Download database backup
    ///
    /// Downloads the database backup file
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `filename` | `String` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `crate::ByteStream`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn download_database_backup(&self, filename: impl Into<String>) -> DownloadDatabaseBackupBuilder<'a> {
        DownloadDatabaseBackupBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            filename: filename.into(),
        }
    }
    /// List database backups
    ///
    /// Get the list of the successful and failed backups
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `DatabaseBackupListResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn list_database_backups(&self) -> ListDatabaseBackupsBuilder<'a> {
        ListDatabaseBackupsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }
    /// Start database backup restore flow
    ///
    /// Put Immich into maintenance mode to restore a backup (Immich must not be configured)
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `201` |  |
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn start_database_restore_flow(&self) -> StartDatabaseRestoreFlowBuilder<'a> {
        StartDatabaseRestoreFlowBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }
    /// Upload database backup
    ///
    /// Uploads .sql/.sql.gz file to restore backup from
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `form` | `reqwest::multipart::Form` | ✓ | Multipart form payload |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `201` |  |
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn upload_database_backup(&self, form: reqwest::multipart::Form) -> UploadDatabaseBackupBuilder<'a> {
        UploadDatabaseBackupBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            form,
        }
    }
}
