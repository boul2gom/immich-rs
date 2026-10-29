// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! API endpoints for `admin`.

use immich_models::maintenance::admin::{
    IntegrityReport, IntegrityReportResponseDTO, IntegrityReportResponseItemsItem, IntegrityReportSummaryResponseDTO,
    MaintenanceAuthDTO, MaintenanceDetectInstallResponseDTO, MaintenanceLoginDTO, MaintenanceStatusResponseDTO,
    SetMaintenanceModeDTO,
};

/// Fluent request builder for [`Admin::delete_integrity_report`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteIntegrityReportBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
}

/// Type alias for [`DeleteIntegrityReportBuilder`].
pub type DeleteIntegrityReportArgs<'a> = DeleteIntegrityReportBuilder<'a>;
impl DeleteIntegrityReportBuilder<'_> {
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
            path = "/admin/integrity/report/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::DELETE, &format!("/admin/integrity/report/{}", self.id))?;
        request = self.request_config.apply(request);
        self.client.execute_unit("admin", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for DeleteIntegrityReportBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Admin::detect_prior_install`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DetectPriorInstallBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`DetectPriorInstallBuilder`].
pub type DetectPriorInstallArgs<'a> = DetectPriorInstallBuilder<'a>;
impl DetectPriorInstallBuilder<'_> {
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
    pub async fn send(self) -> Result<MaintenanceDetectInstallResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Admin",
            method = "GET",
            path = "/admin/maintenance/detect-install",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/admin/maintenance/detect-install")?;
        request = self.request_config.apply(request);

        let response = self
            .client
            .execute::<MaintenanceDetectInstallResponseDTO>("admin", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for DetectPriorInstallBuilder<'a> {
    type Output = Result<MaintenanceDetectInstallResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Admin::get_integrity_report`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetIntegrityReportBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "type")]
    pub type_: IntegrityReport,
}

/// Type alias for [`GetIntegrityReportBuilder`].
pub type GetIntegrityReportArgs<'a> = GetIntegrityReportBuilder<'a>;
impl<'a> GetIntegrityReportBuilder<'a> {
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

    /// Cursor for pagination
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Sets or clears `cursor`.
    pub fn set_cursor(mut self, value: Option<String>) -> Self {
        self.cursor = value;
        self
    }

    /// Number of items per page
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Sets or clears `limit`.
    pub fn set_limit(mut self, value: Option<i64>) -> Self {
        self.limit = value;
        self
    }

    /// Creates a paginator for this request, providing streams over pages or individual items.
    #[must_use = "paginators are lazy and do nothing unless .pages() or .items() is called"]
    pub fn into_paginator(self) -> GetIntegrityReportPaginator<'a> {
        GetIntegrityReportPaginator { builder: self }
    }

    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<IntegrityReportResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Admin",
            method = "GET",
            path = "/admin/integrity/report",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/admin/integrity/report")?;
        request = self.request_config.apply(request);
        request = request.query(&self);

        let response = self
            .client
            .execute::<IntegrityReportResponseDTO>("admin", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetIntegrityReportBuilder<'a> {
    type Output = Result<IntegrityReportResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Paginator for [`GetIntegrityReportBuilder`].
#[derive(Debug, Clone)]
#[must_use = "paginators are lazy and do nothing unless .pages() or .items() is called"]
pub struct GetIntegrityReportPaginator<'a> {
    builder: GetIntegrityReportBuilder<'a>,
}

impl<'a> GetIntegrityReportPaginator<'a> {
    /// Returns an asynchronous [`Stream`](futures_core::Stream) yielding pages of items.
    #[must_use = "streams do nothing unless polled"]
    pub fn pages(
        self,
    ) -> impl futures_core::Stream<Item = Result<Vec<IntegrityReportResponseItemsItem>, crate::Error>> + 'a {
        let mut builder = self.builder;
        async_stream::try_stream! {
            loop { let res = builder.clone().send(). await ?; let items = res.items; let
            is_empty = items.is_empty(); let next = res.next_cursor; yield items; if
            is_empty { break; } match next { Some(c) if ! c.is_empty() => builder =
            builder.cursor(c), _ => break, } }
        }
    }

    /// Returns an asynchronous [`Stream`](futures_core::Stream) yielding individual items, flattened across pages.
    #[must_use = "streams do nothing unless polled"]
    pub fn items(
        self,
    ) -> impl futures_core::Stream<Item = Result<IntegrityReportResponseItemsItem, crate::Error>> + 'a {
        use futures_util::StreamExt;
        self.pages().flat_map(|page_res| match page_res {
            Ok(items) => futures_util::stream::iter(items.into_iter().map(Ok)).left_stream(),
            Err(err) => futures_util::stream::once(async move { Err(err) }).right_stream(),
        })
    }
}

/// Fluent request builder for [`Admin::get_integrity_report_csv`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetIntegrityReportCsvBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub type_: IntegrityReport,
}

/// Type alias for [`GetIntegrityReportCsvBuilder`].
pub type GetIntegrityReportCsvArgs<'a> = GetIntegrityReportCsvBuilder<'a>;
impl GetIntegrityReportCsvBuilder<'_> {
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
            path = "/admin/integrity/report/{type}/csv",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(
            reqwest::Method::GET,
            &format!("/admin/integrity/report/{}/csv", self.type_),
        )?;
        request = self.request_config.apply(request);
        let raw_response = self.client.execute_stream("admin", request).await?;
        Ok(raw_response)
    }
}

impl<'a> std::future::IntoFuture for GetIntegrityReportCsvBuilder<'a> {
    type Output = Result<reqwest::Response, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Admin::get_integrity_report_file`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetIntegrityReportFileBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
}

/// Type alias for [`GetIntegrityReportFileBuilder`].
pub type GetIntegrityReportFileArgs<'a> = GetIntegrityReportFileBuilder<'a>;
impl GetIntegrityReportFileBuilder<'_> {
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
            path = "/admin/integrity/report/{id}/file",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(
            reqwest::Method::GET,
            &format!("/admin/integrity/report/{}/file", self.id),
        )?;
        request = self.request_config.apply(request);
        let raw_response = self.client.execute_stream("admin", request).await?;
        Ok(raw_response)
    }
}

impl<'a> std::future::IntoFuture for GetIntegrityReportFileBuilder<'a> {
    type Output = Result<reqwest::Response, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Admin::get_integrity_report_summary`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetIntegrityReportSummaryBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`GetIntegrityReportSummaryBuilder`].
pub type GetIntegrityReportSummaryArgs<'a> = GetIntegrityReportSummaryBuilder<'a>;
impl GetIntegrityReportSummaryBuilder<'_> {
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
    pub async fn send(self) -> Result<IntegrityReportSummaryResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Admin",
            method = "GET",
            path = "/admin/integrity/summary",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/admin/integrity/summary")?;
        request = self.request_config.apply(request);

        let response = self
            .client
            .execute::<IntegrityReportSummaryResponseDTO>("admin", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetIntegrityReportSummaryBuilder<'a> {
    type Output = Result<IntegrityReportSummaryResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Admin::get_maintenance_status`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetMaintenanceStatusBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`GetMaintenanceStatusBuilder`].
pub type GetMaintenanceStatusArgs<'a> = GetMaintenanceStatusBuilder<'a>;
impl GetMaintenanceStatusBuilder<'_> {
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
    pub async fn send(self) -> Result<MaintenanceStatusResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Admin",
            method = "GET",
            path = "/admin/maintenance/status",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/admin/maintenance/status")?;
        request = self.request_config.apply(request);

        let response = self
            .client
            .execute::<MaintenanceStatusResponseDTO>("admin", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetMaintenanceStatusBuilder<'a> {
    type Output = Result<MaintenanceStatusResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Admin::maintenance_login`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct MaintenanceLoginBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a MaintenanceLoginDTO,
}

/// Type alias for [`MaintenanceLoginBuilder`].
pub type MaintenanceLoginArgs<'a> = MaintenanceLoginBuilder<'a>;
impl MaintenanceLoginBuilder<'_> {
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
    pub async fn send(self) -> Result<MaintenanceAuthDTO, crate::Error> {
        tracing::trace!(
            module = "Admin",
            method = "POST",
            path = "/admin/maintenance/login",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/admin/maintenance/login")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<MaintenanceAuthDTO>("admin", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for MaintenanceLoginBuilder<'a> {
    type Output = Result<MaintenanceAuthDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Admin::set_maintenance_mode`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SetMaintenanceModeBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a SetMaintenanceModeDTO,
}

/// Type alias for [`SetMaintenanceModeBuilder`].
pub type SetMaintenanceModeArgs<'a> = SetMaintenanceModeBuilder<'a>;
impl SetMaintenanceModeBuilder<'_> {
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
            path = "/admin/maintenance",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/admin/maintenance")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);
        self.client.execute_unit("admin", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for SetMaintenanceModeBuilder<'a> {
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

    /// Delete integrity report item
    /// Delete a given report item and perform corresponding deletion (e.g. trash asset, delete file)
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    pub fn delete_integrity_report(&self, id: uuid::Uuid) -> DeleteIntegrityReportBuilder<'a> {
        DeleteIntegrityReportBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
        }
    }

    /// Detect existing install
    /// Collect integrity checks and other heuristics about local data.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `MaintenanceDetectInstallResponseDTO`
    pub fn detect_prior_install(&self) -> DetectPriorInstallBuilder<'a> {
        DetectPriorInstallBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Get integrity report by type
    /// Get all flagged items by integrity report type
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `cursor` | `String` |  | Cursor for pagination |
    /// | `limit` | `i64` |  | Number of items per page |
    /// | `type` | `IntegrityReport` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `IntegrityReportResponseDTO`
    pub fn get_integrity_report(&self, type_: IntegrityReport) -> GetIntegrityReportBuilder<'a> {
        GetIntegrityReportBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            cursor: None,
            limit: None,
            type_,
        }
    }

    /// Export integrity report by type as CSV
    /// Get all integrity report entries for a given type as a CSV
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `type` | `IntegrityReport` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `reqwest::Response`
    pub fn get_integrity_report_csv(&self, type_: IntegrityReport) -> GetIntegrityReportCsvBuilder<'a> {
        GetIntegrityReportCsvBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            type_,
        }
    }

    /// Download flagged file
    /// Download the untracked/broken file if one exists
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `reqwest::Response`
    pub fn get_integrity_report_file(&self, id: uuid::Uuid) -> GetIntegrityReportFileBuilder<'a> {
        GetIntegrityReportFileBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
        }
    }

    /// Get integrity report summary
    /// Get a count of the items flagged in each integrity report
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `IntegrityReportSummaryResponseDTO`
    pub fn get_integrity_report_summary(&self) -> GetIntegrityReportSummaryBuilder<'a> {
        GetIntegrityReportSummaryBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Get maintenance mode status
    /// Fetch information about the currently running maintenance action.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `MaintenanceStatusResponseDTO`
    pub fn get_maintenance_status(&self) -> GetMaintenanceStatusBuilder<'a> {
        GetMaintenanceStatusBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Log into maintenance mode
    /// Login with maintenance token or cookie to receive current information and perform further actions.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `MaintenanceLoginDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `201` |  |
    ///
    /// **Returns:** `MaintenanceAuthDTO`
    pub fn maintenance_login(&self, body: &'a MaintenanceLoginDTO) -> MaintenanceLoginBuilder<'a> {
        MaintenanceLoginBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Set maintenance mode
    /// Put Immich into or take it out of maintenance mode
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `SetMaintenanceModeDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `201` |  |
    pub fn set_maintenance_mode(&self, body: &'a SetMaintenanceModeDTO) -> SetMaintenanceModeBuilder<'a> {
        SetMaintenanceModeBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }
}
