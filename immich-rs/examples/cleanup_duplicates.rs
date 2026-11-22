//! Smart duplicate resolver and metadata merger for Immich.
//!
//! Scores duplicate groups using resolution, camera EXIF completeness, native file format
//! (`.HEIC`/RAW vs re-exported `.JPG`), capture date authenticity, and iCloud Shared Album
//! downscale detection (`2048px` long edge). Automatically pairs burst originals with their
//! respective downscaled copies, merges all albums, tags, favorites, shared links, stacks,
//! sidecars, and missing EXIF fields onto each kept original, and trashes redundant copies.
//!
//! # Usage
//! ```sh
//! export IMMICH_INSTANCE_URL="https://photos.example.com/api"
//! export IMMICH_API_KEY="your-api-key"
//!
//! # 1. Dry-run audit (default — read-only report)
//! cargo run -p immich-rs --example cleanup_duplicates
//!
//! # 2. Interactive review in the terminal
//! cargo run -p immich-rs --example cleanup_duplicates -- --mode interactive
//!
//! # 3. Interactive review only for groups with album/metadata merges or Immich disagreements
//! cargo run -p immich-rs --example cleanup_duplicates -- --mode interactive --filter special
//!
//! # 4. Automatic resolution of all groups
//! cargo run -p immich-rs --example cleanup_duplicates -- --mode auto
//! ```

#![allow(clippy::print_stdout)]

use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use clap::{Parser, ValueEnum};
use console::style;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use futures_util::stream::{self, StreamExt};
use immich_rs::ImmichClient;
use immich_rs::models::albums::BulkIdsDTO;
use immich_rs::models::assets::{AssetBulkUpdateDTO, AssetCopyDTO, AssetResponseDTO};
use immich_rs::models::duplicates::{DuplicateResolveDTO, DuplicateResolveGroupDTO, DuplicateResponseDTO};
use immich_rs::models::id::{AlbumId, AssetId, DuplicateId, TagId};

const ICLOUD_SHARED_ALBUM_LONG_EDGE: i64 = 2048;
const SECONDS_PER_HALF_DAY: i64 = 12 * 3600;
const SIGNIFICANT_DATE_DIFF_SECS: i64 = 60;
const MAX_BURST_SPAN_SECS: i64 = 30;

/// Smart duplicate resolver and maximum-metadata merger for Immich.
#[derive(Debug, Parser)]
#[command(
    name = "cleanup_duplicates",
    about = "Smart duplicate resolver and metadata merger for Immich"
)]
struct CliArgs {
    /// Execution mode (`dry-run` is read-only; `interactive` prompts per group; `auto` resolves all).
    #[arg(short, long, value_enum, default_value_t = ExecutionMode::DryRun)]
    mode: ExecutionMode,

    /// Filter which duplicate groups to display or process.
    #[arg(short, long, value_enum, default_value_t = GroupFilter::All)]
    filter: GroupFilter,

    /// Maximum concurrent requests when fetching album memberships.
    #[arg(short, long, default_value_t = 8)]
    concurrency: usize,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, ValueEnum)]
enum ExecutionMode {
    #[default]
    DryRun,
    Interactive,
    Auto,
}

impl fmt::Display for ExecutionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DryRun => f.write_str("dry-run"),
            Self::Interactive => f.write_str("interactive"),
            Self::Auto => f.write_str("auto"),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, ValueEnum)]
enum GroupFilter {
    #[default]
    All,
    Special,
    Disagreements,
    Merges,
}

impl fmt::Display for GroupFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::All => f.write_str("all"),
            Self::Special => f.write_str("special"),
            Self::Disagreements => f.write_str("disagreements"),
            Self::Merges => f.write_str("merges"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FileFormatTier {
    Raw,
    Heic,
    Standard,
}

impl fmt::Display for FileFormatTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Raw => f.write_str("RAW"),
            Self::Heic => f.write_str("HEIC"),
            Self::Standard => f.write_str("STD"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ImmichVerdict {
    SuggestedKeep,
    SuggestedTrash,
}

impl fmt::Display for ImmichVerdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SuggestedKeep => f.write_str("Immich:Keep"),
            Self::SuggestedTrash => f.write_str("Immich:Trash"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VerdictAlignment {
    Aligned,
    DisagreesWithImmich,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FavoritePromotion {
    PromoteToFavorite,
    Unchanged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UserDecision {
    ApplyRecommended,
    SelectSingleWinner(usize),
    Skip,
    UnlinkGroup,
    ApplyAllRemaining,
    Quit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ImageDimensions {
    width: i64,
    height: i64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct GpsCoordinates {
    latitude: f64,
    longitude: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CapturedTimestamp {
    timestamp: DateTime<Utc>,
    time_zone: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
enum MetadataEnrichment {
    CaptureDate(CapturedTimestamp),
    Gps(GpsCoordinates),
    Description(String),
    Rating(i64),
}

impl fmt::Display for MetadataEnrichment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CaptureDate(dt) => {
                write!(
                    f,
                    "DateTimeOriginal -> {}",
                    dt.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
                )
            }
            Self::Gps(gps) => write!(f, "GPS -> ({:.5}, {:.5})", gps.latitude, gps.longitude),
            Self::Description(desc) => write!(f, "Description -> \"{desc}\""),
            Self::Rating(rating) => write!(f, "Rating -> {rating}/5"),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct ScoreBreakdown {
    resolution: i64,
    exif: i64,
    format: i64,
    date: i64,
    filename: i64,
    size: i64,
    icloud_penalty: i64,
}

impl ScoreBreakdown {
    const fn total(self) -> i64 {
        self.resolution + self.exif + self.format + self.date + self.filename + self.size + self.icloud_penalty
    }
}

#[derive(Debug, Clone)]
struct EvaluatedAsset {
    asset: AssetResponseDTO,
    albums: BTreeMap<AlbumId, String>,
    score: ScoreBreakdown,
    immich_verdict: ImmichVerdict,
    format_tier: FileFormatTier,
    exif_field_count: u8,
}

impl EvaluatedAsset {
    fn from_dto(
        asset: AssetResponseDTO,
        suggested_keep_ids: &[AssetId],
        album_map: &BTreeMap<AssetId, BTreeMap<AlbumId, String>>,
    ) -> Self {
        let immich_verdict = if suggested_keep_ids.contains(&asset.id) {
            ImmichVerdict::SuggestedKeep
        } else {
            ImmichVerdict::SuggestedTrash
        };
        let albums = album_map.get(&asset.id).cloned().unwrap_or_default();
        let lower_name = asset.original_file_name.to_ascii_lowercase();
        let lower_path = asset.original_path.to_ascii_lowercase();
        let mime = asset
            .original_mime_type
            .as_deref()
            .unwrap_or_default()
            .to_ascii_lowercase();
        let format_tier = if [".dng", ".cr2", ".cr3", ".nef", ".arw", ".raf", ".orf", ".rw2"]
            .iter()
            .any(|ext| lower_name.ends_with(ext) || lower_path.ends_with(ext))
            || mime.contains("dng")
            || mime.contains("raw")
        {
            FileFormatTier::Raw
        } else if [".heic", ".heif", ".avif"]
            .iter()
            .any(|ext| lower_name.ends_with(ext) || lower_path.ends_with(ext))
            || mime.contains("heic")
            || mime.contains("heif")
        {
            FileFormatTier::Heic
        } else {
            FileFormatTier::Standard
        };
        let exif_field_count = asset.exif_info.as_ref().map_or(0, |e| {
            u8::from(e.make.is_some())
                + u8::from(e.model.is_some())
                + u8::from(e.lens_model.is_some())
                + u8::from(e.f_number.is_some())
                + u8::from(e.iso.is_some())
                + u8::from(e.exposure_time.is_some())
                + u8::from(e.focal_length.is_some())
                + u8::from(e.latitude.is_some() && e.longitude.is_some())
        });
        Self {
            asset,
            albums,
            score: ScoreBreakdown::default(),
            immich_verdict,
            format_tier,
            exif_field_count,
        }
    }

    fn dimensions(&self) -> ImageDimensions {
        let width = self
            .asset
            .exif_info
            .as_ref()
            .and_then(|e| e.exif_image_width)
            .or(self.asset.width)
            .unwrap_or(0);
        let height = self
            .asset
            .exif_info
            .as_ref()
            .and_then(|e| e.exif_image_height)
            .or(self.asset.height)
            .unwrap_or(0);
        ImageDimensions { width, height }
    }

    fn pixels(&self) -> i64 {
        let dims = self.dimensions();
        dims.width.saturating_mul(dims.height)
    }

    fn long_edge(&self) -> i64 {
        let dims = self.dimensions();
        dims.width.max(dims.height)
    }

    fn file_size(&self) -> i64 {
        self.asset
            .exif_info
            .as_ref()
            .and_then(|e| e.file_size_in_byte)
            .unwrap_or(0)
    }

    fn capture_date(&self) -> DateTime<Utc> {
        self.asset
            .exif_info
            .as_ref()
            .and_then(|e| e.date_time_original)
            .unwrap_or(self.asset.file_created_at)
    }

    fn has_collision_or_social_suffix(&self) -> bool {
        let has_plus_suffix = self.asset.original_path.rsplit('/').next().is_some_and(|file| {
            file.rfind('+').is_some_and(|idx| {
                let suffix = file[idx + 1..].split('.').next().unwrap_or_default();
                !suffix.is_empty() && suffix.chars().all(|c| c.is_ascii_digit())
            })
        });
        let name = self.asset.original_file_name.as_str();
        has_plus_suffix
            || name.contains("(1)")
            || name.contains("_copy")
            || name.contains(" Copy")
            || name.contains("-WA")
    }

    fn camera_label(&self) -> Cow<'_, str> {
        let Some(exif) = &self.asset.exif_info else {
            return Cow::Borrowed("Unknown");
        };
        match (exif.make.as_deref(), exif.model.as_deref()) {
            (Some(make), Some(model)) => Cow::Owned(format!("{make} {model}")),
            (Some(make), None) => Cow::Borrowed(make),
            (None, Some(model)) => Cow::Borrowed(model),
            (None, None) => Cow::Borrowed("Unknown"),
        }
    }
}

/// A kept original asset and the redundant copies paired with it for metadata/album merging.
#[derive(Debug, Clone)]
struct WinnerCluster {
    winner: EvaluatedAsset,
    losers: Vec<EvaluatedAsset>,
    transferred_albums: BTreeMap<AlbumId, String>,
    transferred_tags: BTreeMap<TagId, String>,
    enrichments: Vec<MetadataEnrichment>,
    favorite_promotion: FavoritePromotion,
}

impl WinnerCluster {
    fn new(winner: EvaluatedAsset, losers: Vec<EvaluatedAsset>) -> Self {
        let mut transferred_albums = BTreeMap::new();
        for loser in &losers {
            for (album_id, album_name) in &loser.albums {
                if !winner.albums.contains_key(album_id) {
                    transferred_albums.insert(*album_id, album_name.clone());
                }
            }
        }

        let winner_tags: BTreeSet<TagId> = winner.asset.tags.iter().map(|t| t.id).collect();
        let mut transferred_tags = BTreeMap::new();
        for loser in &losers {
            for tag in &loser.asset.tags {
                if !winner_tags.contains(&tag.id) {
                    transferred_tags.insert(tag.id, tag.value.clone());
                }
            }
        }

        let favorite_promotion = if !winner.asset.is_favorite && losers.iter().any(|l| l.asset.is_favorite) {
            FavoritePromotion::PromoteToFavorite
        } else {
            FavoritePromotion::Unchanged
        };

        let mut enrichments = Vec::new();
        let winner_exif = winner.asset.exif_info.as_ref();
        let winner_date = winner.capture_date();

        if let Some((earliest_date, tz)) = losers
            .iter()
            .filter_map(|l| {
                l.asset
                    .exif_info
                    .as_ref()
                    .and_then(|e| e.date_time_original.map(|dt| (dt, e.time_zone.clone())))
            })
            .min_by_key(|(dt, _)| *dt)
            && (winner_date - earliest_date).num_seconds() > SIGNIFICANT_DATE_DIFF_SECS
        {
            enrichments.push(MetadataEnrichment::CaptureDate(CapturedTimestamp {
                timestamp: earliest_date,
                time_zone: tz,
            }));
        }

        let winner_has_gps = winner_exif.is_some_and(|e| e.latitude.is_some() && e.longitude.is_some());
        if !winner_has_gps
            && let Some(gps) = losers.iter().find_map(|l| {
                l.asset.exif_info.as_ref().and_then(|e| {
                    Some(GpsCoordinates {
                        latitude: e.latitude?,
                        longitude: e.longitude?,
                    })
                })
            })
        {
            enrichments.push(MetadataEnrichment::Gps(gps));
        }

        let winner_has_desc = winner_exif
            .and_then(|e| e.description.as_deref())
            .is_some_and(|d| !d.trim().is_empty());
        if !winner_has_desc
            && let Some(desc) = losers.iter().find_map(|l| {
                l.asset
                    .exif_info
                    .as_ref()
                    .and_then(|e| e.description.as_deref())
                    .map(str::trim)
                    .filter(|d| !d.is_empty())
                    .map(str::to_owned)
            })
        {
            enrichments.push(MetadataEnrichment::Description(desc));
        }

        let winner_rating = winner_exif
            .and_then(|e| e.rating)
            .map_or(0, |r| i64::try_from(r.get()).unwrap_or(0));
        if let Some(best_loser_rating) = losers
            .iter()
            .filter_map(|l| l.asset.exif_info.as_ref().and_then(|e| e.rating))
            .filter_map(|r| i64::try_from(r.get()).ok())
            .max()
            && best_loser_rating > winner_rating
        {
            enrichments.push(MetadataEnrichment::Rating(best_loser_rating));
        }

        Self {
            winner,
            losers,
            transferred_albums,
            transferred_tags,
            enrichments,
            favorite_promotion,
        }
    }

    fn has_merge_actions(&self) -> bool {
        !self.transferred_albums.is_empty()
            || !self.transferred_tags.is_empty()
            || !self.enrichments.is_empty()
            || matches!(self.favorite_promotion, FavoritePromotion::PromoteToFavorite)
    }
}

#[derive(Debug, Clone)]
struct EvaluatedGroup {
    duplicate_id: DuplicateId,
    candidates: Vec<EvaluatedAsset>,
    clusters: Vec<WinnerCluster>,
    alignment: VerdictAlignment,
    reclaimed_bytes: i64,
}

impl EvaluatedGroup {
    fn is_kept(&self, asset_id: AssetId) -> bool {
        self.clusters.iter().any(|c| c.winner.asset.id == asset_id)
    }

    fn has_merge_actions(&self) -> bool {
        self.clusters.iter().any(WinnerCluster::has_merge_actions)
    }

    fn has_album_merges(&self) -> bool {
        self.clusters.iter().any(|c| !c.transferred_albums.is_empty())
    }

    fn has_metadata_or_tag_merges(&self) -> bool {
        self.clusters
            .iter()
            .any(|c| !c.enrichments.is_empty() || !c.transferred_tags.is_empty())
    }

    fn matches_filter(&self, filter: GroupFilter) -> bool {
        match filter {
            GroupFilter::All => true,
            GroupFilter::Special => {
                self.has_merge_actions() || matches!(self.alignment, VerdictAlignment::DisagreesWithImmich)
            }
            GroupFilter::Disagreements => matches!(self.alignment, VerdictAlignment::DisagreesWithImmich),
            GroupFilter::Merges => self.has_merge_actions(),
        }
    }
}

#[derive(Debug, Default)]
struct ResolutionSummary {
    groups_resolved: usize,
    groups_unlinked: usize,
    groups_skipped: usize,
    assets_trashed: usize,
    albums_transferred: usize,
    metadata_fields_rescued: usize,
    immich_disagreements_saved: usize,
    bytes_reclaimed: i64,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args = CliArgs::parse();
    let client = ImmichClient::from_env().context("failed to initialize ImmichClient from environment")?;

    println!(
        "{} Fetching duplicate groups from {} (mode: {}, filter: {})...",
        style("🔍").bold(),
        style(client.base_url()).cyan(),
        style(args.mode).yellow().bold(),
        style(args.filter).magenta().bold()
    );

    let raw_groups = client
        .duplicates()
        .get_asset_duplicates()
        .await
        .context("failed to fetch duplicate groups")?;
    if raw_groups.is_empty() {
        println!("{} No duplicate assets detected on the server!", style("✨").green());
        return Ok(());
    }

    let unique_ids: BTreeSet<AssetId> = raw_groups.iter().flat_map(|g| g.assets.iter().map(|a| a.id)).collect();
    println!(
        "{} Fetching album memberships for {} assets...",
        style("📂").bold(),
        unique_ids.len()
    );

    let client_ref = &client;
    let pairs: Vec<(AssetId, BTreeMap<AlbumId, String>)> = stream::iter(unique_ids)
        .map(|asset_id| async move {
            let albums = client_ref
                .albums()
                .get_all_albums()
                .asset_id(asset_id)
                .await
                .context("failed to fetch albums for asset")?;
            Ok::<_, anyhow::Error>((asset_id, albums.into_iter().map(|a| (a.id, a.album_name)).collect()))
        })
        .buffer_unordered(args.concurrency.max(1))
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<Result<Vec<_>>>()
        .context("failed to resolve album memberships")?;
    let album_map: BTreeMap<AssetId, BTreeMap<AlbumId, String>> = pairs.into_iter().collect();

    let evaluated: Vec<EvaluatedGroup> = raw_groups
        .into_iter()
        .filter(|g| g.assets.len() >= 2)
        .map(|g| evaluate_duplicate_group(g, &album_map))
        .collect();

    let disagreements = evaluated
        .iter()
        .filter(|g| matches!(g.alignment, VerdictAlignment::DisagreesWithImmich))
        .count();
    let album_merges = evaluated.iter().filter(|g| g.has_album_merges()).count();
    let meta_merges = evaluated.iter().filter(|g| g.has_metadata_or_tag_merges()).count();
    let burst_groups = evaluated.iter().filter(|g| g.clusters.len() > 1).count();
    let total_bytes: i64 = evaluated.iter().map(|g| g.reclaimed_bytes).sum();

    println!(
        "\n{} Audit of {} duplicate groups: {} Immich disagreements, {} album merges, {} EXIF/tag merges, {} burst multi-keeps, {} reclaimable.\n",
        style("📊").bold(),
        style(evaluated.len()).cyan().bold(),
        style(disagreements).red().bold(),
        style(album_merges).yellow().bold(),
        style(meta_merges).magenta().bold(),
        style(burst_groups).blue().bold(),
        style(format_mib(total_bytes)).green().bold()
    );

    let filtered: Vec<EvaluatedGroup> = evaluated
        .into_iter()
        .filter(|g| g.matches_filter(args.filter))
        .collect();
    if filtered.is_empty() {
        println!(
            "{} No duplicate groups matched filter `{}`.",
            style("ℹ️").blue(),
            args.filter
        );
        return Ok(());
    }

    process_filtered_groups(&client, &filtered, args.mode)
        .await
        .context("failed to process duplicate groups")
}

async fn process_filtered_groups(
    client: &ImmichClient,
    filtered: &[EvaluatedGroup],
    mode: ExecutionMode,
) -> Result<()> {
    let mut summary = ResolutionSummary::default();
    let mut auto_remaining = matches!(mode, ExecutionMode::Auto);

    for (idx, group) in filtered.iter().enumerate() {
        print_group_card(idx, filtered.len(), group);
        if matches!(mode, ExecutionMode::DryRun) {
            continue;
        }
        let decision = if auto_remaining {
            UserDecision::ApplyRecommended
        } else {
            prompt_user_decision(group).context("failed to read user decision from terminal")?
        };
        match decision {
            UserDecision::ApplyRecommended => apply_group_resolution(client, group, &mut summary)
                .await
                .context("failed to resolve duplicate group")?,
            UserDecision::SelectSingleWinner(winner_idx) => {
                let mut reordered = group.candidates.clone();
                let chosen = reordered.remove(winner_idx);
                reordered.insert(0, chosen);
                let custom_group = build_single_winner_group_plan(group.duplicate_id, reordered);
                apply_group_resolution(client, &custom_group, &mut summary)
                    .await
                    .context("failed to resolve group with custom winner")?;
            }
            UserDecision::Skip => {
                summary.groups_skipped += 1;
                println!("   {} Skipped group.\n", style("⏭️").dim());
            }
            UserDecision::UnlinkGroup => {
                client
                    .duplicates()
                    .delete_duplicate(group.duplicate_id)
                    .await
                    .context("failed to unlink duplicate group")?;
                summary.groups_unlinked += 1;
                println!("   {} Unlinked group (all assets kept).\n", style("🔓").blue());
            }
            UserDecision::ApplyAllRemaining => {
                auto_remaining = true;
                apply_group_resolution(client, group, &mut summary)
                    .await
                    .context("failed to resolve duplicate group")?;
            }
            UserDecision::Quit => {
                println!("{} Stopped by user.", style("🛑").yellow());
                break;
            }
        }
    }

    if matches!(mode, ExecutionMode::DryRun) {
        println!(
            "{} Dry-run complete. Re-run with `--mode interactive` or `--mode auto` to apply.",
            style("💡").cyan()
        );
    } else {
        println!(
            "\n{} Summary: {} resolved, {} unlinked, {} skipped, {} trashed ({}), {} albums merged, {} metadata fields rescued, {} Immich mistakes prevented.\n",
            style("🎉").bold(),
            style(summary.groups_resolved).green().bold(),
            style(summary.groups_unlinked).blue().bold(),
            style(summary.groups_skipped).dim().bold(),
            style(summary.assets_trashed).yellow().bold(),
            style(format_mib(summary.bytes_reclaimed)).green().bold(),
            style(summary.albums_transferred).cyan().bold(),
            style(summary.metadata_fields_rescued).magenta().bold(),
            style(summary.immich_disagreements_saved).red().bold()
        );
    }
    Ok(())
}

fn evaluate_duplicate_group(
    group: DuplicateResponseDTO,
    album_map: &BTreeMap<AssetId, BTreeMap<AlbumId, String>>,
) -> EvaluatedGroup {
    let mut candidates: Vec<EvaluatedAsset> = group
        .assets
        .into_iter()
        .map(|asset| EvaluatedAsset::from_dto(asset, &group.suggested_keep_asset_ids, album_map))
        .collect();

    let max_pixels = candidates.iter().map(EvaluatedAsset::pixels).max().unwrap_or(0);
    let max_long_edge = candidates.iter().map(EvaluatedAsset::long_edge).max().unwrap_or(0);
    let max_size = candidates.iter().map(EvaluatedAsset::file_size).max().unwrap_or(0);
    let min_date = candidates
        .iter()
        .map(EvaluatedAsset::capture_date)
        .min()
        .unwrap_or_else(Utc::now);

    for candidate in &mut candidates {
        let resolution = if max_pixels > 0 {
            candidate.pixels().saturating_mul(1000) / max_pixels
        } else {
            0
        };
        let icloud_penalty = if candidate.long_edge() == ICLOUD_SHARED_ALBUM_LONG_EDGE
            && max_long_edge > ICLOUD_SHARED_ALBUM_LONG_EDGE
        {
            -150
        } else {
            0
        };
        let exif = candidate.asset.exif_info.as_ref().map_or(0, |e| {
            (if e.make.is_some() { 150 } else { 0 })
                + (if e.model.is_some() { 150 } else { 0 })
                + (if e.lens_model.is_some() { 50 } else { 0 })
                + (if e.f_number.is_some() { 50 } else { 0 })
                + (if e.iso.is_some() { 50 } else { 0 })
                + (if e.exposure_time.is_some() { 50 } else { 0 })
                + (if e.focal_length.is_some() { 50 } else { 0 })
                + (if e.latitude.is_some() && e.longitude.is_some() {
                    100
                } else {
                    0
                })
        });
        let format = match candidate.format_tier {
            FileFormatTier::Raw => 140,
            FileFormatTier::Heic => 90,
            FileFormatTier::Standard => 0,
        };
        let date = if (candidate.capture_date() - min_date).num_seconds().abs() <= SECONDS_PER_HALF_DAY {
            100
        } else {
            0
        };
        let filename = if candidate.has_collision_or_social_suffix() {
            -60
        } else {
            0
        };
        let size = if max_size > 0 {
            candidate.file_size().saturating_mul(50) / max_size
        } else {
            0
        };
        candidate.score = ScoreBreakdown {
            resolution,
            exif,
            format,
            date,
            filename,
            size,
            icloud_penalty,
        };
    }

    candidates.sort_by(|a, b| {
        b.score
            .total()
            .cmp(&a.score.total())
            .then_with(|| b.pixels().cmp(&a.pixels()))
            .then_with(|| b.exif_field_count.cmp(&a.exif_field_count))
            .then_with(|| a.capture_date().cmp(&b.capture_date()))
    });

    build_smart_group_plan(group.duplicate_id, candidates)
}

/// Partitions candidates into one or more `WinnerCluster`s (preserving distinct full-resolution burst
/// originals when the group also contains their lower-resolution duplicate copies).
fn build_smart_group_plan(duplicate_id: DuplicateId, candidates: Vec<EvaluatedAsset>) -> EvaluatedGroup {
    let primary = &candidates[0];
    let mut leader_indices = vec![0usize];

    if candidates.len() >= 3 {
        for (idx, candidate) in candidates.iter().enumerate().skip(1) {
            let same_top_resolution = candidate.pixels() > 0 && candidate.pixels() == primary.pixels();
            let same_format = candidate.format_tier == primary.format_tier;
            let has_camera_exif = candidate.exif_field_count >= 3;
            let clean_path = !candidate.has_collision_or_social_suffix();
            let within_burst_window =
                (candidate.capture_date() - primary.capture_date()).num_seconds().abs() <= MAX_BURST_SPAN_SECS;
            let distinct_shot = leader_indices.iter().all(|&leader_idx| {
                let leader = &candidates[leader_idx];
                !candidate
                    .asset
                    .original_file_name
                    .eq_ignore_ascii_case(&leader.asset.original_file_name)
                    && candidate.capture_date() != leader.capture_date()
            });

            if same_top_resolution
                && same_format
                && has_camera_exif
                && clean_path
                && within_burst_window
                && distinct_shot
            {
                leader_indices.push(idx);
            }
        }
    }

    if leader_indices.len() == candidates.len() {
        leader_indices.truncate(1);
    }

    let mut buckets: Vec<(EvaluatedAsset, Vec<EvaluatedAsset>)> = leader_indices
        .iter()
        .map(|&idx| (candidates[idx].clone(), Vec::new()))
        .collect();

    for (idx, candidate) in candidates.iter().enumerate() {
        if leader_indices.contains(&idx) {
            continue;
        }
        let target_bucket_idx = buckets
            .iter()
            .enumerate()
            .min_by_key(|(_, (leader, _))| {
                let name_mismatch = i64::from(
                    !candidate
                        .asset
                        .original_file_name
                        .eq_ignore_ascii_case(&leader.asset.original_file_name),
                );
                let time_diff = (candidate.capture_date() - leader.capture_date()).num_seconds().abs();
                (name_mismatch, time_diff)
            })
            .map_or(0, |(b_idx, _)| b_idx);
        buckets[target_bucket_idx].1.push(candidate.clone());
    }

    let clusters: Vec<WinnerCluster> = buckets
        .into_iter()
        .map(|(winner, losers)| WinnerCluster::new(winner, losers))
        .collect();

    let alignment = if clusters
        .iter()
        .all(|c| matches!(c.winner.immich_verdict, ImmichVerdict::SuggestedKeep))
    {
        VerdictAlignment::Aligned
    } else {
        VerdictAlignment::DisagreesWithImmich
    };

    let reclaimed_bytes = clusters
        .iter()
        .flat_map(|c| c.losers.iter())
        .map(EvaluatedAsset::file_size)
        .sum();

    EvaluatedGroup {
        duplicate_id,
        candidates,
        clusters,
        alignment,
        reclaimed_bytes,
    }
}

fn build_single_winner_group_plan(duplicate_id: DuplicateId, candidates: Vec<EvaluatedAsset>) -> EvaluatedGroup {
    let winner = candidates[0].clone();
    let losers = candidates[1..].to_vec();
    let alignment = match winner.immich_verdict {
        ImmichVerdict::SuggestedKeep => VerdictAlignment::Aligned,
        ImmichVerdict::SuggestedTrash => VerdictAlignment::DisagreesWithImmich,
    };
    let reclaimed_bytes = losers.iter().map(EvaluatedAsset::file_size).sum();
    let cluster = WinnerCluster::new(winner, losers);
    EvaluatedGroup {
        duplicate_id,
        candidates,
        clusters: vec![cluster],
        alignment,
        reclaimed_bytes,
    }
}

fn print_group_card(index: usize, total: usize, group: &EvaluatedGroup) {
    println!(
        "{} Group {}/{} (ID: {})",
        style("📦").bold(),
        style(index + 1).cyan().bold(),
        total,
        style(group.duplicate_id).dim()
    );
    if group.clusters.len() > 1 {
        println!(
            "   {}",
            style(format!(
                "📸 BURST DETECTED: Keeping {} distinct full-res originals and trashing only their degraded copies!",
                group.clusters.len()
            ))
            .on_blue()
            .white()
            .bold()
        );
    } else if matches!(group.alignment, VerdictAlignment::DisagreesWithImmich) {
        println!(
            "   {}",
            style("⚠️  DISAGREES WITH IMMICH: Our score rescues the authentic original that Immich marked for trash!")
                .on_red()
                .white()
                .bold()
        );
    }

    for (idx, candidate) in group.candidates.iter().enumerate() {
        let badge = if group.is_kept(candidate.asset.id) {
            style("🌟 KEEP").green().bold()
        } else {
            style("🗑️  TRASH").red().bold()
        };
        let dims = candidate.dimensions();
        let albums_str = if candidate.albums.is_empty() {
            Cow::Borrowed("0 albums")
        } else {
            let names: Vec<&str> = candidate.albums.values().map(String::as_str).collect();
            Cow::Owned(names.join(", "))
        };
        let fav_str = if candidate.asset.is_favorite { " ★" } else { "" };

        println!(
            "   #{idx} [{badge}] {} ({}) — Score: {} pts [{}]",
            style(&candidate.asset.original_file_name).bold(),
            candidate.format_tier,
            style(candidate.score.total()).cyan().bold(),
            style(candidate.immich_verdict).dim()
        );
        println!(
            "      Path: {} | Size: {} | Resolution: {}×{}{fav_str}",
            style(&candidate.asset.original_path).dim(),
            format_mib(candidate.file_size()),
            dims.width,
            dims.height
        );
        println!(
            "      Date: {} | Camera: {} (EXIF: {}/8) | Albums: {}",
            candidate.capture_date().format("%Y-%m-%d %H:%M:%S"),
            candidate.camera_label(),
            candidate.exif_field_count,
            style(albums_str).yellow()
        );
    }

    for cluster in &group.clusters {
        if cluster.has_merge_actions() {
            println!(
                "   {} Merge actions onto [{}] before trashing paired copy/copies:",
                style("🔀").magenta().bold(),
                style(&cluster.winner.asset.original_file_name).green().bold()
            );
            if !cluster.transferred_albums.is_empty() {
                let names: Vec<&str> = cluster.transferred_albums.values().map(String::as_str).collect();
                println!(
                    "      + Albums transferred : {}",
                    style(names.join(", ")).yellow().bold()
                );
            }
            if matches!(cluster.favorite_promotion, FavoritePromotion::PromoteToFavorite) {
                println!("      + Promote to Favorite ★");
            }
            if !cluster.transferred_tags.is_empty() {
                let tags: Vec<&str> = cluster.transferred_tags.values().map(String::as_str).collect();
                println!("      + Tags transferred   : {}", style(tags.join(", ")).cyan());
            }
            for enrichment in &cluster.enrichments {
                println!("      + Metadata rescued   : {}", style(enrichment).green());
            }
        }
    }
    println!();
}

fn prompt_user_decision(group: &EvaluatedGroup) -> Result<UserDecision> {
    let mut choices = Vec::with_capacity(group.candidates.len() + 4);
    let mut actions = Vec::with_capacity(group.candidates.len() + 4);

    let kept_names: Vec<&str> = group
        .clusters
        .iter()
        .map(|c| c.winner.asset.original_file_name.as_str())
        .collect();
    choices.push(format!(
        "✅ Keep [{}] + Merge metadata/albums + Trash copies (Recommended)",
        kept_names.join(", ")
    ));
    actions.push(UserDecision::ApplyRecommended);

    for (idx, alt) in group.candidates.iter().enumerate() {
        if group.clusters.len() == 1 && idx == 0 {
            continue;
        }
        choices.push(format!(
            "🔄 Keep ONLY #{idx} [{}] + Merge all onto #{idx} + Trash rest",
            alt.asset.original_file_name
        ));
        actions.push(UserDecision::SelectSingleWinner(idx));
    }

    choices.push("⏭️  Skip this group".to_owned());
    actions.push(UserDecision::Skip);
    choices.push("🔓 Unlink duplicate group (Keep all photos)".to_owned());
    actions.push(UserDecision::UnlinkGroup);
    choices.push("🚀 Apply recommended action to all remaining groups".to_owned());
    actions.push(UserDecision::ApplyAllRemaining);
    choices.push("🛑 Quit".to_owned());
    actions.push(UserDecision::Quit);

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select action for this duplicate group")
        .items(&choices)
        .default(0)
        .interact()
        .context("failed to display interactive terminal prompt")?;

    Ok(actions[selection])
}

async fn apply_group_resolution(
    client: &ImmichClient,
    group: &EvaluatedGroup,
    summary: &mut ResolutionSummary,
) -> Result<()> {
    let mut keep_ids = Vec::with_capacity(group.clusters.len());
    let mut trash_ids = Vec::new();

    for cluster in &group.clusters {
        let winner = &cluster.winner;
        keep_ids.push(winner.asset.id);

        for loser in &cluster.losers {
            trash_ids.push(loser.asset.id);
            client
                .assets()
                .copy_asset(AssetCopyDTO::new(loser.asset.id, winner.asset.id))
                .await
                .context("failed to copy asset associations from loser to winner")?;
        }

        if !cluster.enrichments.is_empty() || matches!(cluster.favorite_promotion, FavoritePromotion::PromoteToFavorite)
        {
            let mut update_dto = AssetBulkUpdateDTO::new([winner.asset.id]);
            if matches!(cluster.favorite_promotion, FavoritePromotion::PromoteToFavorite) {
                update_dto = update_dto.is_favorite(true);
            }
            for enrichment in &cluster.enrichments {
                match enrichment {
                    MetadataEnrichment::CaptureDate(dt) => {
                        update_dto = update_dto.date_time_original(dt.timestamp.to_rfc3339());
                        if let Some(tz) = &dt.time_zone {
                            update_dto = update_dto.time_zone(tz.clone());
                        }
                    }
                    MetadataEnrichment::Gps(gps) => {
                        update_dto = update_dto.latitude(gps.latitude).longitude(gps.longitude);
                    }
                    MetadataEnrichment::Description(desc) => {
                        update_dto = update_dto.description(desc.clone());
                    }
                    MetadataEnrichment::Rating(rating) => {
                        update_dto = update_dto.rating(*rating);
                    }
                }
            }
            client
                .assets()
                .update_assets(update_dto)
                .await
                .context("failed to update rescued metadata on winner asset")?;
        }

        for tag_id in cluster.transferred_tags.keys() {
            client
                .tags()
                .tag_assets(*tag_id, BulkIdsDTO::new([winner.asset.id]))
                .await
                .context("failed to assign rescued tag to winner asset")?;
        }

        summary.albums_transferred += cluster.transferred_albums.len();
        summary.metadata_fields_rescued += cluster.enrichments.len() + cluster.transferred_tags.len();
    }

    let trashed_count = trash_ids.len();
    let resolve_group = DuplicateResolveGroupDTO::new(group.duplicate_id, keep_ids, trash_ids);
    client
        .duplicates()
        .resolve_duplicates(DuplicateResolveDTO::new([resolve_group]))
        .await
        .context("failed to resolve duplicate group")?;

    summary.groups_resolved += 1;
    summary.assets_trashed += trashed_count;
    if matches!(group.alignment, VerdictAlignment::DisagreesWithImmich) {
        summary.immich_disagreements_saved += 1;
    }
    summary.bytes_reclaimed += group.reclaimed_bytes;

    let kept_names: Vec<&str> = group
        .clusters
        .iter()
        .map(|c| c.winner.asset.original_file_name.as_str())
        .collect();
    println!(
        "   {} Resolved: kept [{}], trashed {} copy/copies ({}).\n",
        style("✅").green(),
        style(kept_names.join(", ")).green().bold(),
        trashed_count,
        format_mib(group.reclaimed_bytes)
    );
    Ok(())
}

#[expect(
    clippy::cast_precision_loss,
    reason = "Formatting byte counts into human-readable MiB"
)]
fn format_mib(bytes: i64) -> String {
    let mib = bytes as f64 / (1024.0 * 1024.0);
    format!("{mib:.2} MiB")
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU64;

    use immich_rs::models::assets::{AssetTypeEnum, AssetVisibility, ExifResponseDTO};
    use immich_rs::models::id::UserId;
    use uuid::Uuid;

    use super::*;

    struct TestAssetSpec<'a> {
        id: Uuid,
        file_name: &'a str,
        path: &'a str,
        dims: ImageDimensions,
        size_bytes: i64,
        created_at: DateTime<Utc>,
        exif: ExifResponseDTO,
    }

    fn sample_asset(spec: TestAssetSpec<'_>) -> AssetResponseDTO {
        AssetResponseDTO::new(
            "sha1-checksum",
            spec.created_at,
            spec.created_at,
            spec.created_at,
            true,
            AssetId::from(spec.id),
            false,
            false,
            false,
            false,
            false,
            spec.created_at,
            spec.file_name,
            spec.path,
            UserId::from(Uuid::nil()),
            AssetTypeEnum::Image,
            spec.created_at,
            AssetVisibility::Timeline,
        )
        .width(spec.dims.width)
        .height(spec.dims.height)
        .exif_info(
            spec.exif
                .exif_image_width(spec.dims.width)
                .exif_image_height(spec.dims.height)
                .file_size_in_byte(spec.size_bytes),
        )
    }

    #[test]
    fn selects_full_res_original_and_transfers_icloud_shared_album() {
        let dt = DateTime::parse_from_rfc3339("2026-05-25T15:18:12Z").unwrap().to_utc();
        let orig_id = Uuid::new_v4();
        let icloud_id = Uuid::new_v4();
        let shared_album_id = AlbumId::from(Uuid::new_v4());

        let orig = sample_asset(TestAssetSpec {
            id: orig_id,
            file_name: "20260523_151812.jpg",
            path: "/data/library/user/2026/May/20260523_151812.jpg",
            dims: ImageDimensions {
                width: 5568,
                height: 4872,
            },
            size_bytes: 5_542_772,
            created_at: dt,
            exif: ExifResponseDTO::new().date_time_original(dt),
        });
        let icloud_copy = sample_asset(TestAssetSpec {
            id: icloud_id,
            file_name: "20260523_151812.jpg",
            path: "/data/library/user/2026/May/20260523_151812+1.jpg",
            dims: ImageDimensions {
                width: 2048,
                height: 1792,
            },
            size_bytes: 1_083_179,
            created_at: dt,
            exif: ExifResponseDTO::new().date_time_original(dt),
        });

        let dto = DuplicateResponseDTO::new(
            vec![orig, icloud_copy],
            DuplicateId::from(Uuid::new_v4()),
            vec![AssetId::from(orig_id)],
        );

        let mut album_map = BTreeMap::new();
        let mut icloud_albums = BTreeMap::new();
        icloud_albums.insert(shared_album_id, "Summer Trip 2026".to_owned());
        album_map.insert(AssetId::from(icloud_id), icloud_albums);

        let evaluated = evaluate_duplicate_group(dto, &album_map);

        assert_eq!(evaluated.clusters.len(), 1);
        assert_eq!(evaluated.clusters[0].winner.asset.id, AssetId::from(orig_id));
        assert_eq!(evaluated.candidates[1].score.icloud_penalty, -150);
        assert_eq!(
            evaluated.clusters[0]
                .transferred_albums
                .get(&shared_album_id)
                .map(String::as_str),
            Some("Summer Trip 2026")
        );
    }

    #[test]
    fn overrides_immich_mistake_when_stripped_jpeg_is_slightly_larger_than_original_heic() {
        let dt_2023 = DateTime::parse_from_rfc3339("2023-05-29T04:32:20Z").unwrap().to_utc();
        let dt_2024 = DateTime::parse_from_rfc3339("2024-05-11T21:13:25Z").unwrap().to_utc();
        let heic_id = Uuid::new_v4();
        let jpg_id = Uuid::new_v4();

        let heic_exif = ExifResponseDTO::new()
            .date_time_original(dt_2023)
            .make("Apple")
            .model("iPhone SE (2nd generation)")
            .lens_model("iPhone SE (2nd generation) back camera 3.99mm f/1.8")
            .f_number(1.8)
            .focal_length(3.99)
            .iso(25)
            .exposure_time("1/122");

        let heic_orig = sample_asset(TestAssetSpec {
            id: heic_id,
            file_name: "IMG_0947.HEIC",
            path: "/data/library/user/2023/May/20230529_063220.heic",
            dims: ImageDimensions {
                width: 4032,
                height: 3024,
            },
            size_bytes: 3_187_671,
            created_at: dt_2023,
            exif: heic_exif,
        });

        let jpg_stripped = sample_asset(TestAssetSpec {
            id: jpg_id,
            file_name: "IMG_1719.JPG",
            path: "/data/library/user/2024/May/20240511_231325+8.jpg",
            dims: ImageDimensions {
                width: 4032,
                height: 3024,
            },
            size_bytes: 3_207_593,
            created_at: dt_2024,
            exif: ExifResponseDTO::new().date_time_original(dt_2024),
        });

        let dto = DuplicateResponseDTO::new(
            vec![heic_orig, jpg_stripped],
            DuplicateId::from(Uuid::new_v4()),
            vec![AssetId::from(jpg_id)],
        );

        let evaluated = evaluate_duplicate_group(dto, &BTreeMap::new());

        assert_eq!(evaluated.clusters.len(), 1);
        assert_eq!(evaluated.clusters[0].winner.asset.id, AssetId::from(heic_id));
        assert_eq!(evaluated.alignment, VerdictAlignment::DisagreesWithImmich);
        assert!(evaluated.candidates[0].score.total() > evaluated.candidates[1].score.total() + 500);
    }

    #[test]
    fn rescues_missing_gps_date_description_rating_and_favorite_from_loser() {
        let dt_old = DateTime::parse_from_rfc3339("2023-07-14T10:00:00Z").unwrap().to_utc();
        let dt_new = DateTime::parse_from_rfc3339("2024-01-01T12:00:00Z").unwrap().to_utc();
        let winner_id = Uuid::new_v4();
        let loser_id = Uuid::new_v4();

        let high_res_winner = sample_asset(TestAssetSpec {
            id: winner_id,
            file_name: "IMG_0001.JPG",
            path: "/data/IMG_0001.JPG",
            dims: ImageDimensions {
                width: 5568,
                height: 4872,
            },
            size_bytes: 5_000_000,
            created_at: dt_new,
            exif: ExifResponseDTO::new().date_time_original(dt_new),
        });

        let mut metadata_rich_loser = sample_asset(TestAssetSpec {
            id: loser_id,
            file_name: "IMG_0001_small.JPG",
            path: "/data/IMG_0001+1.JPG",
            dims: ImageDimensions {
                width: 2048,
                height: 1792,
            },
            size_bytes: 1_000_000,
            created_at: dt_old,
            exif: ExifResponseDTO::new()
                .date_time_original(dt_old)
                .time_zone("UTC")
                .latitude(46.2044)
                .longitude(6.1432)
                .description("Mountain view")
                .rating(NonZeroU64::new(5).unwrap()),
        });
        metadata_rich_loser.is_favorite = true;

        let dto = DuplicateResponseDTO::new(
            vec![high_res_winner, metadata_rich_loser],
            DuplicateId::from(Uuid::new_v4()),
            vec![AssetId::from(winner_id)],
        );

        let evaluated = evaluate_duplicate_group(dto, &BTreeMap::new());

        assert_eq!(evaluated.clusters[0].winner.asset.id, AssetId::from(winner_id));
        assert_eq!(
            evaluated.clusters[0].favorite_promotion,
            FavoritePromotion::PromoteToFavorite
        );
        assert_eq!(evaluated.clusters[0].enrichments.len(), 4);
    }

    #[test]
    fn keeps_both_burst_originals_and_trashes_only_their_paired_copies() {
        let dt_a = DateTime::parse_from_rfc3339("2026-05-23T12:12:09Z").unwrap().to_utc();
        let dt_b = DateTime::parse_from_rfc3339("2026-05-23T12:12:11Z").unwrap().to_utc();
        let orig_a_id = Uuid::new_v4();
        let orig_b_id = Uuid::new_v4();
        let copy_a_id = Uuid::new_v4();
        let copy_b_id = Uuid::new_v4();
        let shared_album_id = AlbumId::from(Uuid::new_v4());

        let exif_base = ExifResponseDTO::new()
            .make("Apple")
            .model("iPhone 15")
            .lens_model("iPhone 15 back dual wide camera")
            .f_number(1.6)
            .iso(50)
            .exposure_time("1/1000")
            .focal_length(6.86);

        let orig_a = sample_asset(TestAssetSpec {
            id: orig_a_id,
            file_name: "IMG_6711.HEIC",
            path: "/data/library/user/2026/May/20260523_141209.heic",
            dims: ImageDimensions {
                width: 5712,
                height: 3213,
            },
            size_bytes: 1_970_000,
            created_at: dt_a,
            exif: exif_base.clone().date_time_original(dt_a),
        });
        let orig_b = sample_asset(TestAssetSpec {
            id: orig_b_id,
            file_name: "IMG_6712.HEIC",
            path: "/data/library/user/2026/May/20260523_141211.heic",
            dims: ImageDimensions {
                width: 5712,
                height: 3213,
            },
            size_bytes: 2_040_000,
            created_at: dt_b,
            exif: exif_base.clone().date_time_original(dt_b),
        });
        let copy_a = sample_asset(TestAssetSpec {
            id: copy_a_id,
            file_name: "IMG_6711.HEIC",
            path: "/data/library/user/2026/May/20260523_141209+1.heic",
            dims: ImageDimensions {
                width: 2730,
                height: 1536,
            },
            size_bytes: 910_000,
            created_at: dt_a,
            exif: exif_base.clone().date_time_original(dt_a),
        });
        let copy_b = sample_asset(TestAssetSpec {
            id: copy_b_id,
            file_name: "IMG_6712.HEIC",
            path: "/data/library/user/2026/May/20260523_141211+1.heic",
            dims: ImageDimensions {
                width: 2730,
                height: 1536,
            },
            size_bytes: 980_000,
            created_at: dt_b,
            exif: exif_base.date_time_original(dt_b),
        });

        let mut album_map = BTreeMap::new();
        let mut albums = BTreeMap::new();
        albums.insert(shared_album_id, "Summer Trip 2026".to_owned());
        album_map.insert(AssetId::from(copy_a_id), albums.clone());
        album_map.insert(AssetId::from(copy_b_id), albums);

        let dto = DuplicateResponseDTO::new(
            vec![orig_a, orig_b, copy_a, copy_b],
            DuplicateId::from(Uuid::new_v4()),
            vec![AssetId::from(orig_b_id)],
        );

        let evaluated = evaluate_duplicate_group(dto, &album_map);

        assert_eq!(evaluated.clusters.len(), 2);
        assert!(evaluated.is_kept(AssetId::from(orig_a_id)));
        assert!(evaluated.is_kept(AssetId::from(orig_b_id)));
        assert!(!evaluated.is_kept(AssetId::from(copy_a_id)));
        assert!(!evaluated.is_kept(AssetId::from(copy_b_id)));
        assert!(
            evaluated
                .clusters
                .iter()
                .all(|c| c.transferred_albums.contains_key(&shared_album_id))
        );
    }
}
