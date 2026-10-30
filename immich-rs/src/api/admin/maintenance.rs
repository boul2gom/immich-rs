// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! API endpoints for `maintenance`.

use immich_models::admin::maintenance::{
    IntegrityReport, IntegrityReportResponseDTO, IntegrityReportResponseItemsItem, IntegrityReportSummaryResponseDTO,
    MaintenanceAuthDTO, MaintenanceDetectInstallResponseDTO, MaintenanceLoginDTO, MaintenanceStatusResponseDTO,
    SetMaintenanceModeDTO,
};

/// Fluent request builder for [`Maintenance::delete_integrity_report`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteIntegrityReportBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: uuid::Uuid,
}

impl DeleteIntegrityReportBuilder<'_> {
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
            module = "admin/maintenance",
            method = "DELETE",
            path = "/admin/integrity/report/{id}",
            "HTTP request"
        );
        let __seg_3 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::DELETE,
            &[
                crate::core::PathSegment::Static("admin"),
                crate::core::PathSegment::Static("integrity"),
                crate::core::PathSegment::Static("report"),
                crate::core::PathSegment::Param(__seg_3.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        self.client.execute_unit("admin/maintenance", request).await?;
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

/// Fluent request builder for [`Maintenance::detect_prior_install`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DetectPriorInstallBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
}

impl DetectPriorInstallBuilder<'_> {
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
    pub async fn send(self) -> Result<MaintenanceDetectInstallResponseDTO, crate::Error> {
        tracing::trace!(
            module = "admin/maintenance",
            method = "GET",
            path = "/admin/maintenance/detect-install",
            "HTTP request"
        );
        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/admin/maintenance/detect-install")?;
        request = self.request_config.apply(request)?;
        let response = self
            .client
            .execute::<MaintenanceDetectInstallResponseDTO>("admin/maintenance", request)
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

/// Fluent request builder for [`Maintenance::get_integrity_report`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetIntegrityReportBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    cursor: Option<String>,
    limit: Option<i64>,
    type_: IntegrityReport,
}

impl<'a> GetIntegrityReportBuilder<'a> {
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
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<IntegrityReportResponseDTO, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
            cursor: Option<&'b String>,
            #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
            limit: Option<&'b i64>,
            #[serde(rename = "type")]
            type_: &'b IntegrityReport,
        }
        tracing::trace!(
            module = "admin/maintenance",
            method = "GET",
            path = "/admin/integrity/report",
            "HTTP request"
        );
        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/admin/integrity/report")?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            cursor: self.cursor.as_ref(),
            limit: self.limit.as_ref(),
            type_: &self.type_,
        };
        request = request.query(&query);
        let response = self
            .client
            .execute::<IntegrityReportResponseDTO>("admin/maintenance", request)
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
    /// Returns an asynchronous paginated stream yielding pages of items.
    #[must_use = "streams do nothing unless polled or awaited"]
    pub fn pages(self) -> crate::client::PaginationStream<'a, Vec<IntegrityReportResponseItemsItem>> {
        let stream = {
            let mut builder = self.builder;
            async_stream::try_stream! {
                loop { let res = builder.clone().send(). await ?; let items = res.items;
                let is_empty = items.is_empty(); let next = res.next_cursor; yield items;
                if is_empty { break; } match next { Some(c) if ! c.is_empty() => builder
                = builder.cursor(c), _ => break, } }
            }
        };
        crate::client::PaginationStream::new(stream)
    }
    /// Returns an asynchronous paginated stream yielding individual items, flattened across pages.
    #[must_use = "streams do nothing unless polled or awaited"]
    pub fn items(self) -> crate::client::PaginationStream<'a, IntegrityReportResponseItemsItem> {
        use futures_util::StreamExt;
        let stream = self.pages().flat_map(|page_res| match page_res {
            Ok(items) => futures_util::stream::iter(items.into_iter().map(Ok)).left_stream(),
            Err(err) => futures_util::stream::once(async move { Err(err) }).right_stream(),
        });
        crate::client::PaginationStream::new(stream)
    }
}

/// Fluent request builder for [`Maintenance::get_integrity_report_csv`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetIntegrityReportCsvBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    type_: IntegrityReport,
}

impl GetIntegrityReportCsvBuilder<'_> {
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
            module = "admin/maintenance",
            method = "GET",
            path = "/admin/integrity/report/{type}/csv",
            "HTTP request"
        );
        let __seg_3 = self.type_.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("admin"),
                crate::core::PathSegment::Static("integrity"),
                crate::core::PathSegment::Static("report"),
                crate::core::PathSegment::Param(__seg_3.as_str()),
                crate::core::PathSegment::Static("csv"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        let raw_response = self.client.execute_stream("admin/maintenance", request).await?;
        Ok(crate::ByteStream::new(raw_response))
    }
}

impl<'a> std::future::IntoFuture for GetIntegrityReportCsvBuilder<'a> {
    type Output = Result<crate::ByteStream, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Maintenance::get_integrity_report_file`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetIntegrityReportFileBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: uuid::Uuid,
}

impl GetIntegrityReportFileBuilder<'_> {
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
            module = "admin/maintenance",
            method = "GET",
            path = "/admin/integrity/report/{id}/file",
            "HTTP request"
        );
        let __seg_3 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("admin"),
                crate::core::PathSegment::Static("integrity"),
                crate::core::PathSegment::Static("report"),
                crate::core::PathSegment::Param(__seg_3.as_str()),
                crate::core::PathSegment::Static("file"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        let raw_response = self.client.execute_stream("admin/maintenance", request).await?;
        Ok(crate::ByteStream::new(raw_response))
    }
}

impl<'a> std::future::IntoFuture for GetIntegrityReportFileBuilder<'a> {
    type Output = Result<crate::ByteStream, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Maintenance::get_integrity_report_summary`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetIntegrityReportSummaryBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
}

impl GetIntegrityReportSummaryBuilder<'_> {
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
    pub async fn send(self) -> Result<IntegrityReportSummaryResponseDTO, crate::Error> {
        tracing::trace!(
            module = "admin/maintenance",
            method = "GET",
            path = "/admin/integrity/summary",
            "HTTP request"
        );
        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/admin/integrity/summary")?;
        request = self.request_config.apply(request)?;
        let response = self
            .client
            .execute::<IntegrityReportSummaryResponseDTO>("admin/maintenance", request)
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

/// Fluent request builder for [`Maintenance::get_maintenance_status`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetMaintenanceStatusBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
}

impl GetMaintenanceStatusBuilder<'_> {
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
    pub async fn send(self) -> Result<MaintenanceStatusResponseDTO, crate::Error> {
        tracing::trace!(
            module = "admin/maintenance",
            method = "GET",
            path = "/admin/maintenance/status",
            "HTTP request"
        );
        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/admin/maintenance/status")?;
        request = self.request_config.apply(request)?;
        let response = self
            .client
            .execute::<MaintenanceStatusResponseDTO>("admin/maintenance", request)
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

/// Fluent request builder for [`Maintenance::maintenance_login`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct MaintenanceLoginBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    body: MaintenanceLoginDTO,
}

impl MaintenanceLoginBuilder<'_> {
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
    pub async fn send(self) -> Result<MaintenanceAuthDTO, crate::Error> {
        tracing::trace!(
            module = "admin/maintenance",
            method = "POST",
            path = "/admin/maintenance/login",
            "HTTP request"
        );
        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/admin/maintenance/login")?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self
            .client
            .execute::<MaintenanceAuthDTO>("admin/maintenance", request)
            .await?;
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

/// Fluent request builder for [`Maintenance::set_maintenance_mode`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SetMaintenanceModeBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    body: SetMaintenanceModeDTO,
}

impl SetMaintenanceModeBuilder<'_> {
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
            module = "admin/maintenance",
            method = "POST",
            path = "/admin/maintenance",
            "HTTP request"
        );
        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/admin/maintenance")?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        self.client.execute_unit("admin/maintenance", request).await?;
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

/// Client handle for `Maintenance` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Maintenance<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Maintenance<'a> {
    #[inline]
    pub(crate) const fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }
    /// Delete integrity report item
    ///
    /// Delete a given report item and perform corresponding deletion (e.g. trash asset, delete file)
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn delete_integrity_report(&self, id: uuid::Uuid) -> DeleteIntegrityReportBuilder<'a> {
        DeleteIntegrityReportBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
        }
    }
    /// Detect existing install
    ///
    /// Collect integrity checks and other heuristics about local data.
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `MaintenanceDetectInstallResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn detect_prior_install(&self) -> DetectPriorInstallBuilder<'a> {
        DetectPriorInstallBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }
    /// Get integrity report by type
    ///
    /// Get all flagged items by integrity report type
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `cursor` | `String` |  | Cursor for pagination |
    /// | `limit` | `i64` |  | Number of items per page |
    /// | `type_` | `IntegrityReport` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `IntegrityReportResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
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
    ///
    /// Get all integrity report entries for a given type as a CSV
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `type_` | `IntegrityReport` | ✓ |  |
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
    pub fn get_integrity_report_csv(&self, type_: IntegrityReport) -> GetIntegrityReportCsvBuilder<'a> {
        GetIntegrityReportCsvBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            type_,
        }
    }
    /// Download flagged file
    ///
    /// Download the untracked/broken file if one exists
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
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
    pub fn get_integrity_report_file(&self, id: uuid::Uuid) -> GetIntegrityReportFileBuilder<'a> {
        GetIntegrityReportFileBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
        }
    }
    /// Get integrity report summary
    ///
    /// Get a count of the items flagged in each integrity report
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `IntegrityReportSummaryResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_integrity_report_summary(&self) -> GetIntegrityReportSummaryBuilder<'a> {
        GetIntegrityReportSummaryBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }
    /// Get maintenance mode status
    ///
    /// Fetch information about the currently running maintenance action.
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `MaintenanceStatusResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_maintenance_status(&self) -> GetMaintenanceStatusBuilder<'a> {
        GetMaintenanceStatusBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }
    /// Log into maintenance mode
    ///
    /// Login with maintenance token or cookie to receive current information and perform further actions.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `body` | `MaintenanceLoginDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `201` |  |
    ///
    /// **Returns:** `MaintenanceAuthDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn maintenance_login(&self, body: impl Into<MaintenanceLoginDTO>) -> MaintenanceLoginBuilder<'a> {
        MaintenanceLoginBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body: body.into(),
        }
    }
    /// Set maintenance mode
    ///
    /// Put Immich into or take it out of maintenance mode
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `body` | `SetMaintenanceModeDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `201` |  |
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn set_maintenance_mode(&self, body: impl Into<SetMaintenanceModeDTO>) -> SetMaintenanceModeBuilder<'a> {
        SetMaintenanceModeBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body: body.into(),
        }
    }
}
