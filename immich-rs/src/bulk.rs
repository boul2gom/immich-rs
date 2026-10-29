//! Helper for bulk uploading assets with concurrency control and upstream deduplication.

use std::path::PathBuf;

use futures_util::stream::{self, StreamExt};
use immich_models::assets::{
    AssetBulkUploadCheckDTO, AssetBulkUploadCheckItem, AssetMediaResponseDTO, AssetRejectReason, AssetUploadAction,
};
use immich_models::id::AssetId;

use crate::client::ImmichClient;
use crate::error::Result;
use crate::upload::UploadOptions;

/// Item to be uploaded in bulk (type alias for [`UploadOptions`]).
pub type BulkUploadItem<'a> = UploadOptions<'a>;

/// Result of an individual asset processing step within a bulk upload operation.
#[derive(Debug, Clone)]
pub enum BulkUploadResult {
    /// Asset was uploaded successfully to Immich.
    Uploaded(AssetMediaResponseDTO),
    /// Asset was skipped because it already exists on Immich according to the server's bulk check.
    Duplicate {
        /// Local path of the asset that was skipped.
        path: PathBuf,
        /// Existing asset ID on the Immich server, if returned by the check.
        existing_asset_id: Option<AssetId>,
    },
}

impl BulkUploadResult {
    /// Returns `true` if the asset was newly uploaded.
    #[must_use]
    pub const fn is_uploaded(&self) -> bool {
        matches!(self, Self::Uploaded(_))
    }

    /// Returns `true` if the asset was skipped as a duplicate.
    #[must_use]
    pub const fn is_duplicate(&self) -> bool {
        matches!(self, Self::Duplicate { .. })
    }

    /// Returns the uploaded asset DTO if successful, or `None` if skipped as a duplicate.
    #[must_use]
    pub fn uploaded(self) -> Option<AssetMediaResponseDTO> {
        match self {
            Self::Uploaded(dto) => Some(dto),
            Self::Duplicate { .. } => None,
        }
    }
}

/// Fluent builder for orchestrating bulk asset uploads with concurrency control and upstream deduplication.
#[derive(Debug)]
#[must_use = "bulk upload operations are lazy and do nothing unless .into_stream() is called"]
pub struct BulkUploadBuilder<'a, I> {
    client: ImmichClient,
    assets: I,
    concurrency: usize,
    check_duplicates: bool,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a, I> BulkUploadBuilder<'a, I>
where
    I: IntoIterator<Item = UploadOptions<'a>> + Send + 'a,
    I::IntoIter: Send,
{
    pub(crate) fn new(client: ImmichClient, assets: I) -> Self {
        Self {
            client,
            assets,
            concurrency: 4,
            check_duplicates: true,
            _marker: std::marker::PhantomData,
        }
    }

    /// Sets the maximum number of simultaneous upload requests (defaults to `4`).
    ///
    /// Clamped to at least `1`.
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency.max(1);
        self
    }

    /// Enables or disables pre-upload duplicate checks via Immich's `checkBulkUpload` API (defaults to `true`).
    pub fn check_duplicates(mut self, check: bool) -> Self {
        self.check_duplicates = check;
        self
    }

    /// Converts this builder into an asynchronous [`Stream`](futures_core::Stream) yielding [`BulkUploadResult`] items.
    pub fn into_stream(self) -> impl futures_core::Stream<Item = Result<BulkUploadResult>> + 'a {
        let client = self.client;
        let concurrency = self.concurrency;
        let check_duplicates = self.check_duplicates;
        let items: Vec<UploadOptions<'a>> = self.assets.into_iter().collect();

        async_stream::stream! {
            if check_duplicates && !items.is_empty() {
                // Compute checksums for all items that do not already have one
                let mut check_items = Vec::new();
                let mut valid_indices = std::collections::HashSet::new();
                let mut computed_checksums = std::collections::HashMap::new();

                for (idx, item) in items.iter().enumerate() {
                    let checksum = match &item.checksum {
                        Some(cs) => Ok(cs.to_string()),
                        None => crate::utils::checksum::calculate_file_checksum(item.path).await,
                    };
                    match checksum {
                        Ok(cs) => {
                            check_items.push(AssetBulkUploadCheckItem::new(cs.clone(), idx.to_string()));
                            computed_checksums.insert(idx, cs);
                            valid_indices.insert(idx);
                        }
                        Err(e) => {
                            yield Err(e);
                        }
                    }
                }

                if check_items.is_empty() {
                    return;
                }

                let check_dto = AssetBulkUploadCheckDTO::new(check_items);
                let check_resp = match client.assets().check_bulk_upload(&check_dto).await {
                    Ok(resp) => resp,
                    Err(e) => {
                        yield Err(e);
                        return;
                    }
                };

                let mut to_upload = Vec::new();
                for (idx, mut item) in items.into_iter().enumerate() {
                    if !valid_indices.contains(&idx) {
                        continue;
                    }
                    if let Some(cs) = computed_checksums.get(&idx) {
                        item.checksum = Some(std::borrow::Cow::Owned(cs.clone()));
                    }

                    let match_res = check_resp.results.iter().find(|r| r.id == idx.to_string());
                    if let Some(r) = match_res
                        && r.action == AssetUploadAction::Reject
                        && r.reason == Some(AssetRejectReason::Duplicate)
                    {
                        yield Ok(BulkUploadResult::Duplicate {
                            path: item.path.to_path_buf(),
                            existing_asset_id: r.asset_id.map(Into::into),
                        });
                        continue;
                    }
                    to_upload.push(item);
                }

                let upload_stream = stream::iter(to_upload.into_iter().map(|item| {
                    let client = client.clone();
                    async move { client.upload_asset(item).await }
                }))
                .buffer_unordered(concurrency);

                for await res in upload_stream {
                    yield res.map(BulkUploadResult::Uploaded);
                }
            } else {
                let upload_stream = stream::iter(items.into_iter().map(|item| {
                    let client = client.clone();
                    async move { client.upload_asset(item).await }
                }))
                .buffer_unordered(concurrency);

                for await res in upload_stream {
                    yield res.map(BulkUploadResult::Uploaded);
                }
            }
        }
    }
}

impl ImmichClient {
    /// Initiates a bulk upload operation with configurable concurrency and upstream deduplication checks.
    ///
    /// By default:
    /// - Upstream duplicate checking is **enabled** via Immich's `POST /api/assets/bulk-upload-check` API.
    /// - Concurrency is set to `4` simultaneous uploads.
    ///
    /// Returns a [`BulkUploadBuilder`] that can be customized with:
    /// - [`.concurrency(usize)`](BulkUploadBuilder::concurrency) to change the number of concurrent uploads.
    /// - [`.check_duplicates(bool)`](BulkUploadBuilder::check_duplicates) to disable or enable the pre-upload check.
    ///
    /// Calling [`.into_stream()`](BulkUploadBuilder::into_stream) executes the upload lazily as an asynchronous stream.
    #[must_use = "bulk upload operations are lazy and do nothing unless .into_stream() is called"]
    pub fn upload_bulk<'a, I>(&self, assets: I) -> BulkUploadBuilder<'a, I>
    where
        I: IntoIterator<Item = UploadOptions<'a>> + Send + 'a,
        I::IntoIter: Send,
    {
        BulkUploadBuilder::new(self.clone(), assets)
    }
}
