// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! API endpoints for `common`.

use immich_models::activities::UserResponseDTO;
use immich_models::assets::{AssetFileResponseDTO, AssetFileType};
use immich_models::common::{ClusterGroupRequestCreateDTO, ClusterGroupRequestResponseDTO, PublicConfigDTO};
use immich_models::users::UserConfigDTO;

/// Fluent request builder for [`Common::accept_cluster_group_request`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct AcceptClusterGroupRequestBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
}

/// Type alias for [`AcceptClusterGroupRequestBuilder`].
pub type AcceptClusterGroupRequestArgs<'a> = AcceptClusterGroupRequestBuilder<'a>;
impl AcceptClusterGroupRequestBuilder<'_> {
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
            module = "Common",
            method = "POST",
            path = "/cluster-groups/requests/{id}/accept",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(
            reqwest::Method::POST,
            &format!("/cluster-groups/requests/{}/accept", self.id),
        )?;
        request = self.request_config.apply(request);
        self.client.execute_unit("common", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for AcceptClusterGroupRequestBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Common::cluster_group_regenerate_people`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct ClusterGroupRegeneratePeopleBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
}

/// Type alias for [`ClusterGroupRegeneratePeopleBuilder`].
pub type ClusterGroupRegeneratePeopleArgs<'a> = ClusterGroupRegeneratePeopleBuilder<'a>;
impl ClusterGroupRegeneratePeopleBuilder<'_> {
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
            module = "Common",
            method = "POST",
            path = "/cluster-groups/{id}/regenerate-people",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(
            reqwest::Method::POST,
            &format!("/cluster-groups/{}/regenerate-people", self.id),
        )?;
        request = self.request_config.apply(request);
        self.client.execute_unit("common", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for ClusterGroupRegeneratePeopleBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Common::create_cluster_group_request`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct CreateClusterGroupRequestBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
    #[serde(skip)]
    pub body: &'a ClusterGroupRequestCreateDTO,
}

/// Type alias for [`CreateClusterGroupRequestBuilder`].
pub type CreateClusterGroupRequestArgs<'a> = CreateClusterGroupRequestBuilder<'a>;
impl CreateClusterGroupRequestBuilder<'_> {
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
    pub async fn send(self) -> Result<ClusterGroupRequestResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Common",
            method = "PUT",
            path = "/cluster-groups/{id}/requests",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::PUT, &format!("/cluster-groups/{}/requests", self.id))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self
            .client
            .execute::<ClusterGroupRequestResponseDTO>("common", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for CreateClusterGroupRequestBuilder<'a> {
    type Output = Result<ClusterGroupRequestResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Common::delete_asset_file`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteAssetFileBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
}

/// Type alias for [`DeleteAssetFileBuilder`].
pub type DeleteAssetFileArgs<'a> = DeleteAssetFileBuilder<'a>;
impl DeleteAssetFileBuilder<'_> {
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
            module = "Common",
            method = "DELETE",
            path = "/asset-files/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::DELETE, &format!("/asset-files/{}", self.id))?;
        request = self.request_config.apply(request);
        self.client.execute_unit("common", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for DeleteAssetFileBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Common::delete_cluster_group_request`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteClusterGroupRequestBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
}

/// Type alias for [`DeleteClusterGroupRequestBuilder`].
pub type DeleteClusterGroupRequestArgs<'a> = DeleteClusterGroupRequestBuilder<'a>;
impl DeleteClusterGroupRequestBuilder<'_> {
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
            module = "Common",
            method = "DELETE",
            path = "/cluster-groups/requests/{id}",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(
            reqwest::Method::DELETE,
            &format!("/cluster-groups/requests/{}", self.id),
        )?;
        request = self.request_config.apply(request);
        self.client.execute_unit("common", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for DeleteClusterGroupRequestBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Common::download_asset_file`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DownloadAssetFileBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
}

/// Type alias for [`DownloadAssetFileBuilder`].
pub type DownloadAssetFileArgs<'a> = DownloadAssetFileBuilder<'a>;
impl DownloadAssetFileBuilder<'_> {
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
            module = "Common",
            method = "GET",
            path = "/asset-files/{id}/download",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, &format!("/asset-files/{}/download", self.id))?;
        request = self.request_config.apply(request);
        let raw_response = self.client.execute_stream("common", request).await?;
        Ok(raw_response)
    }
}

impl<'a> std::future::IntoFuture for DownloadAssetFileBuilder<'a> {
    type Output = Result<reqwest::Response, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Common::get_asset_file`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAssetFileBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
}

/// Type alias for [`GetAssetFileBuilder`].
pub type GetAssetFileArgs<'a> = GetAssetFileBuilder<'a>;
impl GetAssetFileBuilder<'_> {
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
    pub async fn send(self) -> Result<AssetFileResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Common",
            method = "GET",
            path = "/asset-files/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, &format!("/asset-files/{}", self.id))?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<AssetFileResponseDTO>("common", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetAssetFileBuilder<'a> {
    type Output = Result<AssetFileResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Common::get_cluster_group_requests`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetClusterGroupRequestsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`GetClusterGroupRequestsBuilder`].
pub type GetClusterGroupRequestsArgs<'a> = GetClusterGroupRequestsBuilder<'a>;
impl GetClusterGroupRequestsBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<ClusterGroupRequestResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "Common",
            method = "GET",
            path = "/cluster-groups/requests",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/cluster-groups/requests")?;
        request = self.request_config.apply(request);

        let response = self
            .client
            .execute::<Vec<ClusterGroupRequestResponseDTO>>("common", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetClusterGroupRequestsBuilder<'a> {
    type Output = Result<Vec<ClusterGroupRequestResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Common::get_cluster_group_requests_for_group`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetClusterGroupRequestsForGroupBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
}

/// Type alias for [`GetClusterGroupRequestsForGroupBuilder`].
pub type GetClusterGroupRequestsForGroupArgs<'a> = GetClusterGroupRequestsForGroupBuilder<'a>;
impl GetClusterGroupRequestsForGroupBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<ClusterGroupRequestResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "Common",
            method = "GET",
            path = "/cluster-groups/{id}/requests",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, &format!("/cluster-groups/{}/requests", self.id))?;
        request = self.request_config.apply(request);

        let response = self
            .client
            .execute::<Vec<ClusterGroupRequestResponseDTO>>("common", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetClusterGroupRequestsForGroupBuilder<'a> {
    type Output = Result<Vec<ClusterGroupRequestResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Common::get_cluster_group_users`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetClusterGroupUsersBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
}

/// Type alias for [`GetClusterGroupUsersBuilder`].
pub type GetClusterGroupUsersArgs<'a> = GetClusterGroupUsersBuilder<'a>;
impl GetClusterGroupUsersBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<UserResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "Common",
            method = "GET",
            path = "/cluster-groups/{id}/users",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, &format!("/cluster-groups/{}/users", self.id))?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<Vec<UserResponseDTO>>("common", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetClusterGroupUsersBuilder<'a> {
    type Output = Result<Vec<UserResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Common::get_public_config`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetPublicConfigBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`GetPublicConfigBuilder`].
pub type GetPublicConfigArgs<'a> = GetPublicConfigBuilder<'a>;
impl GetPublicConfigBuilder<'_> {
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
    pub async fn send(self) -> Result<PublicConfigDTO, crate::Error> {
        tracing::trace!(
            module = "Common",
            method = "GET",
            path = "/public/config",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/public/config")?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<PublicConfigDTO>("common", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetPublicConfigBuilder<'a> {
    type Output = Result<PublicConfigDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Common::get_public_config_defaults`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetPublicConfigDefaultsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`GetPublicConfigDefaultsBuilder`].
pub type GetPublicConfigDefaultsArgs<'a> = GetPublicConfigDefaultsBuilder<'a>;
impl GetPublicConfigDefaultsBuilder<'_> {
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
    pub async fn send(self) -> Result<PublicConfigDTO, crate::Error> {
        tracing::trace!(
            module = "Common",
            method = "GET",
            path = "/public/config/defaults",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/public/config/defaults")?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<PublicConfigDTO>("common", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetPublicConfigDefaultsBuilder<'a> {
    type Output = Result<PublicConfigDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Common::get_user_config`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetUserConfigBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`GetUserConfigBuilder`].
pub type GetUserConfigArgs<'a> = GetUserConfigBuilder<'a>;
impl GetUserConfigBuilder<'_> {
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
    pub async fn send(self) -> Result<UserConfigDTO, crate::Error> {
        tracing::trace!(module = "Common", method = "GET", path = "/config", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/config")?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<UserConfigDTO>("common", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetUserConfigBuilder<'a> {
    type Output = Result<UserConfigDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Common::get_user_config_defaults`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetUserConfigDefaultsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`GetUserConfigDefaultsBuilder`].
pub type GetUserConfigDefaultsArgs<'a> = GetUserConfigDefaultsBuilder<'a>;
impl GetUserConfigDefaultsBuilder<'_> {
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
    pub async fn send(self) -> Result<UserConfigDTO, crate::Error> {
        tracing::trace!(
            module = "Common",
            method = "GET",
            path = "/config/defaults",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/config/defaults")?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<UserConfigDTO>("common", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetUserConfigDefaultsBuilder<'a> {
    type Output = Result<UserConfigDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Common::leave_cluster_group`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct LeaveClusterGroupBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
}

/// Type alias for [`LeaveClusterGroupBuilder`].
pub type LeaveClusterGroupArgs<'a> = LeaveClusterGroupBuilder<'a>;
impl LeaveClusterGroupBuilder<'_> {
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
            module = "Common",
            method = "POST",
            path = "/cluster-groups/{id}/leave",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, &format!("/cluster-groups/{}/leave", self.id))?;
        request = self.request_config.apply(request);
        self.client.execute_unit("common", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for LeaveClusterGroupBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Common::search_asset_files`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SearchAssetFilesBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "assetId")]
    pub asset_id: immich_models::id::AssetId,
    #[serde(rename = "isEdited", skip_serializing_if = "Option::is_none")]
    pub is_edited: Option<bool>,
    #[serde(rename = "isProgressive", skip_serializing_if = "Option::is_none")]
    pub is_progressive: Option<bool>,
    #[serde(rename = "isTransparent", skip_serializing_if = "Option::is_none")]
    pub is_transparent: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<AssetFileType>,
}

/// Type alias for [`SearchAssetFilesBuilder`].
pub type SearchAssetFilesArgs<'a> = SearchAssetFilesBuilder<'a>;
impl SearchAssetFilesBuilder<'_> {
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

    /// The file was generated from an edit
    pub fn is_edited(mut self, value: bool) -> Self {
        self.is_edited = Some(value);
        self
    }

    /// Sets or clears `isEdited`.
    pub fn set_is_edited(mut self, value: Option<bool>) -> Self {
        self.is_edited = value;
        self
    }

    /// The file is a progressively encoded JPEG
    pub fn is_progressive(mut self, value: bool) -> Self {
        self.is_progressive = Some(value);
        self
    }

    /// Sets or clears `isProgressive`.
    pub fn set_is_progressive(mut self, value: Option<bool>) -> Self {
        self.is_progressive = value;
        self
    }

    /// The file is transparent
    pub fn is_transparent(mut self, value: bool) -> Self {
        self.is_transparent = Some(value);
        self
    }

    /// Sets or clears `isTransparent`.
    pub fn set_is_transparent(mut self, value: Option<bool>) -> Self {
        self.is_transparent = value;
        self
    }

    /// Filter by type of file
    pub fn type_(mut self, value: AssetFileType) -> Self {
        self.type_ = Some(value);
        self
    }

    /// Sets or clears `type`.
    pub fn set_type_(mut self, value: Option<AssetFileType>) -> Self {
        self.type_ = value;
        self
    }

    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<Vec<AssetFileResponseDTO>, crate::Error> {
        tracing::trace!(module = "Common", method = "GET", path = "/asset-files", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/asset-files")?;
        request = self.request_config.apply(request);
        request = request.query(&self);

        let response = self
            .client
            .execute::<Vec<AssetFileResponseDTO>>("common", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for SearchAssetFilesBuilder<'a> {
    type Output = Result<Vec<AssetFileResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

///Client handle for `Common` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Common<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Common<'a> {
    pub(crate) fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }

    /// Accept a cluster group request
    /// Join the cluster group the request was created for.
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
    /// | `204` |  |
    pub fn accept_cluster_group_request(&self, id: uuid::Uuid) -> AcceptClusterGroupRequestBuilder<'a> {
        AcceptClusterGroupRequestBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
        }
    }

    /// Regenerate people of users in cluster group
    /// Forcefully re-run facial recognition for all faces of users in this group.
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
    /// | `204` |  |
    pub fn cluster_group_regenerate_people(&self, id: uuid::Uuid) -> ClusterGroupRegeneratePeopleBuilder<'a> {
        ClusterGroupRegeneratePeopleBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
        }
    }

    /// Create a cluster group request
    /// Ask another user to join the cluster group of the current user.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `body` | `ClusterGroupRequestCreateDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `ClusterGroupRequestResponseDTO`
    pub fn create_cluster_group_request(
        &self,
        id: uuid::Uuid,
        body: &'a ClusterGroupRequestCreateDTO,
    ) -> CreateClusterGroupRequestBuilder<'a> {
        CreateClusterGroupRequestBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
            body,
        }
    }

    /// Delete an asset file
    /// Delete a file and remove it from the database.
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
    /// | `204` |  |
    pub fn delete_asset_file(&self, id: uuid::Uuid) -> DeleteAssetFileBuilder<'a> {
        DeleteAssetFileBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
        }
    }

    /// Decline a cluster group request
    /// Delete a pending request to join a cluster group.
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
    /// | `204` |  |
    pub fn delete_cluster_group_request(&self, id: uuid::Uuid) -> DeleteClusterGroupRequestBuilder<'a> {
        DeleteClusterGroupRequestBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
        }
    }

    /// Download an asset file
    /// Serve the contents of a specific asset file.
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
    pub fn download_asset_file(&self, id: uuid::Uuid) -> DownloadAssetFileBuilder<'a> {
        DownloadAssetFileBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
        }
    }

    /// Retrieve an asset file
    /// Returns metadata about a specific asset file.
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
    /// **Returns:** `AssetFileResponseDTO`
    pub fn get_asset_file(&self, id: uuid::Uuid) -> GetAssetFileBuilder<'a> {
        GetAssetFileBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
        }
    }

    /// Retrieve cluster group requests
    /// Retrieve the pending requests for the current user to join a cluster group.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<ClusterGroupRequestResponseDTO>`
    pub fn get_cluster_group_requests(&self) -> GetClusterGroupRequestsBuilder<'a> {
        GetClusterGroupRequestsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Retrieve the requests sent by a cluster group
    /// Retrieve the pending requests for other users to join the cluster group.
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
    /// **Returns:** `Vec<ClusterGroupRequestResponseDTO>`
    pub fn get_cluster_group_requests_for_group(&self, id: uuid::Uuid) -> GetClusterGroupRequestsForGroupBuilder<'a> {
        GetClusterGroupRequestsForGroupBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
        }
    }

    /// Retrieve the users of a cluster group
    /// Retrieve the users that are a member of the cluster group.
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
    /// **Returns:** `Vec<UserResponseDTO>`
    pub fn get_cluster_group_users(&self, id: uuid::Uuid) -> GetClusterGroupUsersBuilder<'a> {
        GetClusterGroupUsersBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
        }
    }

    /// Get the public configuration
    /// Retrieve the system configuration properties that are visible to everyone.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `PublicConfigDTO`
    pub fn get_public_config(&self) -> GetPublicConfigBuilder<'a> {
        GetPublicConfigBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Get the public configuration defaults
    /// Retrieve the default value of the configuration properties that are visible to everyone.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `PublicConfigDTO`
    pub fn get_public_config_defaults(&self) -> GetPublicConfigDefaultsBuilder<'a> {
        GetPublicConfigDefaultsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Get the configuration with user visibility
    /// Retrieve the system configuration properties that are visible to logged in users.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `UserConfigDTO`
    pub fn get_user_config(&self) -> GetUserConfigBuilder<'a> {
        GetUserConfigBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Get the default configuration with user visibility
    /// Retrieve the default value of the configuration properties that are visible to logged in users.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `UserConfigDTO`
    pub fn get_user_config_defaults(&self) -> GetUserConfigDefaultsBuilder<'a> {
        GetUserConfigDefaultsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Leave a cluster group
    /// Move the current user into a new cluster group of their own.
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
    /// | `204` |  |
    pub fn leave_cluster_group(&self, id: uuid::Uuid) -> LeaveClusterGroupBuilder<'a> {
        LeaveClusterGroupBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
        }
    }

    /// Search asset files
    /// Returns all matching asset files.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `assetId` | `uuid::Uuid` | ✓ | Asset ID to filter files by |
    /// | `isEdited` | `bool` |  | The file was generated from an edit |
    /// | `isProgressive` | `bool` |  | The file is a progressively encoded JPEG |
    /// | `isTransparent` | `bool` |  | The file is transparent |
    /// | `type` | `AssetFileType` |  | Filter by type of file |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<AssetFileResponseDTO>`
    pub fn search_asset_files(&self, asset_id: impl Into<immich_models::id::AssetId>) -> SearchAssetFilesBuilder<'a> {
        SearchAssetFilesBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            asset_id: asset_id.into(),
            is_edited: None,
            is_progressive: None,
            is_transparent: None,
            type_: None,
        }
    }
}
