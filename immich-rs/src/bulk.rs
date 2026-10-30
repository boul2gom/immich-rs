//! Helper for bulk uploading assets with bounded concurrency and upstream SHA-1 deduplication.

use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
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
    /// Asset was rejected by Immich's bulk check for a non-duplicate reason (e.g. unsupported format).
    Rejected {
        /// Local path of the asset that was rejected.
        path: PathBuf,
        /// Server-reported rejection reason, if any.
        reason: Option<AssetRejectReason>,
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

    /// Returns `true` if the asset was rejected for a non-duplicate reason.
    #[must_use]
    pub const fn is_rejected(&self) -> bool {
        matches!(self, Self::Rejected { .. })
    }

    /// Returns the uploaded asset DTO if newly uploaded, or `None` if skipped or rejected.
    #[must_use]
    pub fn uploaded(self) -> Option<AssetMediaResponseDTO> {
        match self {
            Self::Uploaded(dto) => Some(dto),
            Self::Duplicate { .. } | Self::Rejected { .. } => None,
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
    pub(crate) const fn new(client: ImmichClient, assets: I) -> Self {
        Self {
            client,
            assets,
            concurrency: 4,
            check_duplicates: true,
            _marker: std::marker::PhantomData,
        }
    }

    /// Sets the maximum number of simultaneous checksum and upload workers (defaults to `4`).
    ///
    /// Clamped to at least `1`.
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency.max(1);
        self
    }

    /// Enables or disables pre-upload duplicate and format checks via Immich's `checkBulkUpload` API (defaults to `true`).
    pub const fn check_duplicates(mut self, check: bool) -> Self {
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
                let mut valid_indices = BTreeSet::new();
                let mut computed_checksums = BTreeMap::new();

                // Compute SHA-1 checksums concurrently across `concurrency` blocking worker tasks
                {
                    let checksum_tasks = items.iter().enumerate().map(|(idx, item)| {
                        let existing = item.checksum.as_ref().map(ToString::to_string);
                        let path = item.path.to_path_buf();
                        async move {
                            let cs = match existing {
                                Some(cs) => Ok(cs),
                                None => crate::utils::checksum::calculate_file_checksum(&path).await,
                            };
                            (idx, cs)
                        }
                    });

                    let mut checksum_stream = stream::iter(checksum_tasks).buffer_unordered(concurrency);
                    while let Some((idx, res)) = checksum_stream.next().await {
                        match res {
                            Ok(cs) => {
                                computed_checksums.insert(idx, cs);
                                valid_indices.insert(idx);
                            }
                            Err(e) => {
                                yield Err(e);
                            }
                        }
                    }
                }

                if computed_checksums.is_empty() {
                    return;
                }

                let check_items: Vec<AssetBulkUploadCheckItem> = computed_checksums
                    .iter()
                    .map(|(idx, cs)| AssetBulkUploadCheckItem::new(cs.clone(), idx.to_string()))
                    .collect();

                let check_dto = AssetBulkUploadCheckDTO::new(check_items);
                let check_resp = match client.assets().check_bulk_upload(check_dto).await {
                    Ok(resp) => resp,
                    Err(e) => {
                        yield Err(e);
                        return;
                    }
                };

                let results_by_id: BTreeMap<&str, _> = check_resp
                    .results
                    .iter()
                    .map(|r| (r.id.as_str(), r))
                    .collect();

                let mut to_upload = Vec::new();
                for (idx, mut item) in items.into_iter().enumerate() {
                    if !valid_indices.contains(&idx) {
                        continue;
                    }
                    if let Some(cs) = computed_checksums.remove(&idx) {
                        item.checksum = Some(Cow::Owned(cs));
                    }

                    let idx_key = idx.to_string();
                    if let Some(r) = results_by_id.get(idx_key.as_str())
                        && r.action == AssetUploadAction::Reject
                    {
                        if r.reason == Some(AssetRejectReason::Duplicate) {
                            yield Ok(BulkUploadResult::Duplicate {
                                path: item.path.to_path_buf(),
                                existing_asset_id: r.asset_id,
                            });
                        } else {
                            yield Ok(BulkUploadResult::Rejected {
                                path: item.path.to_path_buf(),
                                reason: r.reason.clone(),
                            });
                        }
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
    /// - Upstream duplicate and rejection checking is **enabled** via Immich's `POST /assets/bulk-upload-check` API.
    /// - Concurrency is set to `4` simultaneous workers for both SHA-1 hashing and HTTP uploads.
    #[must_use = "bulk upload operations are lazy and do nothing unless .into_stream() is called"]
    pub fn upload_bulk<'a, I>(&self, assets: I) -> BulkUploadBuilder<'a, I>
    where
        I: IntoIterator<Item = UploadOptions<'a>> + Send + 'a,
        I::IntoIter: Send,
    {
        BulkUploadBuilder::new(self.clone(), assets)
    }
}
