// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! API endpoints for `admin`.

use immich_models::database::admin::{DatabaseBackupDeleteDTO, DatabaseBackupListResponseDTO};

/// Fluent request builder for [`Admin::delete_database_backup`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteDatabaseBackupBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a DatabaseBackupDeleteDTO,
}

/// Type alias for [`DeleteDatabaseBackupBuilder`].
pub type DeleteDatabaseBackupArgs<'a> = DeleteDatabaseBackupBuilder<'a>;
impl DeleteDatabaseBackupBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header to this specific request.
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<(), crate::Error> {
        tracing::trace!(
            module = "Admin",
            method = "DELETE",
            path = "/admin/database-backups",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::DELETE, "/admin/database-backups")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);
        self.client.execute_unit("admin", request).await?;
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

/// Fluent request builder for [`Admin::download_database_backup`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DownloadDatabaseBackupBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub filename: String,
}

/// Type alias for [`DownloadDatabaseBackupBuilder`].
pub type DownloadDatabaseBackupArgs<'a> = DownloadDatabaseBackupBuilder<'a>;
impl DownloadDatabaseBackupBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header to this specific request.
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<reqwest::Response, crate::Error> {
        tracing::trace!(
            module = "Admin",
            method = "GET",
            path = "/admin/database-backups/{filename}",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(
            reqwest::Method::GET,
            &format!("/admin/database-backups/{}", ::urlencoding::encode(&self.filename)),
        )?;
        request = self.request_config.apply(request);
        let raw_response = self.client.execute_stream("admin", request).await?;
        Ok(raw_response)
    }
}

impl<'a> std::future::IntoFuture for DownloadDatabaseBackupBuilder<'a> {
    type Output = Result<reqwest::Response, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Admin::list_database_backups`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct ListDatabaseBackupsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`ListDatabaseBackupsBuilder`].
pub type ListDatabaseBackupsArgs<'a> = ListDatabaseBackupsBuilder<'a>;
impl ListDatabaseBackupsBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header to this specific request.
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<DatabaseBackupListResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Admin",
            method = "GET",
            path = "/admin/database-backups",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/admin/database-backups")?;
        request = self.request_config.apply(request);

        let response = self
            .client
            .execute::<DatabaseBackupListResponseDTO>("admin", request)
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

/// Fluent request builder for [`Admin::start_database_restore_flow`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct StartDatabaseRestoreFlowBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`StartDatabaseRestoreFlowBuilder`].
pub type StartDatabaseRestoreFlowArgs<'a> = StartDatabaseRestoreFlowBuilder<'a>;
impl StartDatabaseRestoreFlowBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header to this specific request.
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<(), crate::Error> {
        tracing::trace!(
            module = "Admin",
            method = "POST",
            path = "/admin/database-backups/start-restore",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/admin/database-backups/start-restore")?;
        request = self.request_config.apply(request);
        self.client.execute_unit("admin", request).await?;
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

/// Fluent request builder for [`Admin::upload_database_backup`].
#[derive(Debug)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UploadDatabaseBackupBuilder<'a> {
    client: &'a crate::ImmichClient,
    pub request_config: crate::core::RequestConfig,
    pub form: reqwest::multipart::Form,
}

/// Type alias for [`UploadDatabaseBackupBuilder`].
pub type UploadDatabaseBackupArgs<'a> = UploadDatabaseBackupBuilder<'a>;
impl UploadDatabaseBackupBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header to this specific request.
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<(), crate::Error> {
        tracing::trace!(
            module = "Admin",
            method = "POST",
            path = "/admin/database-backups/upload",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/admin/database-backups/upload")?;
        request = self.request_config.apply(request);
        request = request.multipart(self.form);
        self.client.execute_unit("admin", request).await?;
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

///Client handle for `Admin` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Admin<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Admin<'a> {
    pub(crate) fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }

    /// Delete database backup
    /// Delete a backup by its filename
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `DatabaseBackupDeleteDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    pub fn delete_database_backup(&self, body: &'a DatabaseBackupDeleteDTO) -> DeleteDatabaseBackupBuilder<'a> {
        DeleteDatabaseBackupBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Download database backup
    /// Downloads the database backup file
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `filename` | `String` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `reqwest::Response`
    pub fn download_database_backup(&self, filename: impl Into<String>) -> DownloadDatabaseBackupBuilder<'a> {
        DownloadDatabaseBackupBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            filename: filename.into(),
        }
    }

    /// List database backups
    /// Get the list of the successful and failed backups
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `DatabaseBackupListResponseDTO`
    pub fn list_database_backups(&self) -> ListDatabaseBackupsBuilder<'a> {
        ListDatabaseBackupsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Start database backup restore flow
    /// Put Immich into maintenance mode to restore a backup (Immich must not be configured)
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `201` |  |
    pub fn start_database_restore_flow(&self) -> StartDatabaseRestoreFlowBuilder<'a> {
        StartDatabaseRestoreFlowBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Upload database backup
    /// Uploads .sql/.sql.gz file to restore backup from
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `DatabaseBackupUploadDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `201` |  |
    pub fn upload_database_backup(&self, form: reqwest::multipart::Form) -> UploadDatabaseBackupBuilder<'a> {
        UploadDatabaseBackupBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            form,
        }
    }
}
