#![cfg(all(feature = "fs", not(target_arch = "wasm32")))]
#![allow(deprecated, clippy::print_stderr)]

use std::pin::pin;
use std::process::Command;

use anyhow::{Context, Result};
use futures_util::StreamExt;
use immich_models::activities::{ActivityCreateDTO, ReactionLevel, ReactionType};
use immich_models::albums::{AddUsersDTO, AlbumUserAddDTO, AlbumUserRole, BulkIdsDTO, CreateAlbumDTO};
use immich_models::api_keys::{ApiKeyCreateDTO, Permission};
use immich_models::assets::downloads::DownloadInfoDTO;
use immich_models::assets::stacks::StackCreateDTO;
use immich_models::assets::{AssetBulkDeleteDTO, AssetBulkUploadCheckDTO, AssetBulkUploadCheckItem, UpdateAssetDTO};
use immich_models::authentication::{LoginCredentialDTO, SignUpDTO};
use immich_models::memories::{MemoryCreateDTO, MemoryDataDTO, MemoryType};
use immich_models::partners::PartnerCreateDTO;
use immich_models::people::PersonCreateDTO;
use immich_models::search::{MetadataSearchDTO, RandomSearchDTO};
use immich_models::shared_links::{SharedLinkCreateDTO, SharedLinkType};
use immich_models::tags::TagCreateDTO;
use immich_models::users::admin::UserAdminCreateDTO;
use immich_rs::{UploadOptions, calculate_file_checksum};
use pretty_assertions::assert_eq;
use tempfile::tempdir;
use testcontainers::core::{IntoContainerPort, WaitFor};
use testcontainers::runners::AsyncRunner;
use testcontainers::{GenericImage, ImageExt};

// ─── Docker network ───────────────────────────────────────────────────────────

/// A unique Docker user-defined bridge network shared by all containers.
/// Docker DNS resolves container names within this network — no IP hacks needed.
struct TestNetwork {
    name: String,
}

impl TestNetwork {
    fn create() -> Result<Self> {
        let name = format!("immich-e2e-{}", &uuid::Uuid::new_v4().to_string()[..8]);
        let status = Command::new("docker")
            .args(["network", "create", "--driver", "bridge", &name])
            .status()
            .context("docker network create")?;
        anyhow::ensure!(status.success(), "docker network create failed");
        eprintln!("[e2e] Created Docker network: {name}");
        Ok(Self { name })
    }
}

impl Drop for TestNetwork {
    fn drop(&mut self) {
        match Command::new("docker").args(["network", "rm", &self.name]).output() {
            Ok(out) if out.status.success() => {
                eprintln!("[e2e] Removed Docker network: {}", self.name);
            }
            Ok(out) => eprintln!(
                "[e2e] WARNING: failed to remove Docker network {}: {}",
                self.name,
                String::from_utf8_lossy(&out.stderr).trim()
            ),
            Err(e) => eprintln!(
                "[e2e] WARNING: could not run docker to remove network {}: {e}",
                self.name
            ),
        }
    }
}

// ─── Container stack ──────────────────────────────────────────────────────────

/// Fields are dropped in declaration order: the server first, then its dependencies,
/// and finally the network once no container is attached to it anymore.
struct Containers {
    _immich: testcontainers::ContainerAsync<GenericImage>,
    _postgres: testcontainers::ContainerAsync<GenericImage>,
    _valkey: testcontainers::ContainerAsync<GenericImage>,
    _net: TestNetwork,
}

/// Spins up a fully isolated Immich stack on a shared Docker user-defined network.
async fn start_immich_stack() -> Result<(String, Containers)> {
    let net = TestNetwork::create()?;
    let network = net.name.clone();

    let valkey_name = format!("valkey-{}", &uuid::Uuid::new_v4().to_string()[..8]);
    eprintln!("[e2e] Starting Valkey as '{valkey_name}'…");
    let valkey = GenericImage::new("valkey/valkey", "8.0-alpine")
        .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
        .with_network(&network)
        .with_container_name(&valkey_name)
        .start()
        .await
        .context("start Valkey")?;
    eprintln!("[e2e] Valkey ready");

    let pg_name = format!("postgres-{}", &uuid::Uuid::new_v4().to_string()[..8]);
    eprintln!("[e2e] Starting Postgres as '{pg_name}'…");
    let postgres = GenericImage::new("ghcr.io/immich-app/postgres", "18-vectorchord1.1.1")
        .with_wait_for(WaitFor::message_on_stderr(
            "database system is ready to accept connections",
        ))
        .with_env_var("POSTGRES_PASSWORD", "immich")
        .with_env_var("POSTGRES_USER", "immich")
        .with_env_var("POSTGRES_DB", "immich")
        .with_network(&network)
        .with_container_name(&pg_name)
        .start()
        .await
        .context("start Postgres")?;
    eprintln!("[e2e] Postgres ready");

    let immich_version =
        std::env::var("IMMICH_E2E_VERSION").unwrap_or_else(|_| immich_rs::TARGET_IMMICH_VERSION.to_string());
    eprintln!("[e2e] Starting Immich server {immich_version}…");
    let immich = GenericImage::new("ghcr.io/immich-app/immich-server", &immich_version)
        .with_exposed_port(2283.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Immich Server is listening"))
        .with_network(&network)
        .with_env_var("DB_HOSTNAME", &pg_name)
        .with_env_var("DB_USERNAME", "immich")
        .with_env_var("DB_PASSWORD", "immich")
        .with_env_var("DB_DATABASE_NAME", "immich")
        .with_env_var("REDIS_HOSTNAME", &valkey_name)
        .start()
        .await
        .context("start Immich server")?;

    let host_port = immich.get_host_port_ipv4(2283).await.context("get Immich host port")?;

    let base_url = format!("http://localhost:{host_port}/api/");
    eprintln!("[e2e] Immich reachable at {base_url}");

    Ok((
        base_url,
        Containers {
            _net: net,
            _valkey: valkey,
            _postgres: postgres,
            _immich: immich,
        },
    ))
}

fn test_asset_path(filename: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("assets")
        .join(filename)
}

/// Complete E2E "Seeded Demo Instance" validation across all Immich API domains:
/// 1. Seed initial demo instance (Admin + Second User + Single & Bulk Uploads + SHA-1 check)
/// 2. Auth, Users, Sessions & API Keys
/// 3. Assets, Streaming Download & Archives
/// 4. Stacks
/// 5. Albums, Sharing & Activities
/// 6. Tags, Memories, Partners & Shared Links
/// 7. Search, Timeline & People (Paginated Streams)
/// 8. Server, System Config, Queues & Trash
#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires Docker (run with --ignored)"]
#[allow(clippy::too_many_lines)]
async fn full_user_workflow() -> Result<()> {
    if std::env::var("IMMICH_RUN_E2E").is_err() && std::env::var("CI_E2E").is_err() {
        eprintln!("[e2e] Skipped: Set IMMICH_RUN_E2E=1 or CI_E2E=1 to run containerized E2E tests");
        return Ok(());
    }

    let (base_url, _containers) = start_immich_stack().await?;

    let unauth = immich_rs::ImmichClient::builder()
        .base_url(&base_url)
        .max_retries(0)
        .global_timeout(std::time::Duration::from_secs(15))
        .build()
        .context("build unauthenticated client")?;

    // ── Wait for HTTP API ─────────────────────────────────────────────────────
    eprintln!("[e2e] Waiting for HTTP API…");
    let mut ping_ok = false;
    for i in 0..90 {
        match unauth.server().ping_server().await {
            Ok(p) if p.res == "pong" => {
                eprintln!("[e2e] Server ready after ~{}s", i * 2);
                ping_ok = true;
                break;
            }
            Ok(p) => eprintln!("[e2e] Unexpected ping: {:?}", p.res),
            Err(e) if i % 5 == 0 => eprintln!("[e2e] Ping [{i}/90]: {e}"),
            Err(_) => {}
        }
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }
    anyhow::ensure!(ping_ok, "Immich did not respond within 3 minutes");

    // ── 1 & 2. Seed Admin + Second User + Auth + Sessions + API Keys ──────────
    let signup = unauth
        .authentication()
        .sign_up_admin(SignUpDTO::new("admin@e2e.local", "E2E Admin", "e2e-password-123"))
        .await
        .context("admin signup")?;
    assert!(!signup.id.is_nil());

    let login = unauth
        .authentication()
        .login(LoginCredentialDTO::new("admin@e2e.local", "e2e-password-123"))
        .await
        .context("admin login")?;

    let bearer = immich_rs::ImmichClient::builder()
        .base_url(&base_url)
        .bearer_token(login.access_token)
        .build()
        .context("build bearer client")?;

    let token_valid = bearer
        .authentication()
        .validate_access_token()
        .await
        .context("validate_access_token")?;
    assert!(token_valid.auth_status);

    let auth_status = bearer
        .authentication()
        .get_auth_status()
        .await
        .context("get_auth_status")?;
    assert!(auth_status.password);

    let sessions = bearer.sessions().get_sessions().await.context("get_sessions")?;
    assert!(!sessions.is_empty(), "expected at least 1 active session");

    let api_key = bearer
        .api_keys()
        .create_api_key(ApiKeyCreateDTO::new(vec![Permission::All]).name("e2e-key"))
        .await
        .context("create API key")?;

    let temp_key = bearer
        .api_keys()
        .create_api_key(ApiKeyCreateDTO::new(vec![Permission::AssetRead]).name("temp-key"))
        .await
        .context("create temp API key")?;
    let all_keys = bearer.api_keys().get_api_keys().await.context("get_api_keys")?;
    assert!(all_keys.len() >= 2);
    bearer
        .api_keys()
        .delete_api_key(temp_key.api_key.id)
        .await
        .context("delete temp API key")?;

    let client = immich_rs::ImmichClient::builder()
        .base_url(&base_url)
        .api_key(api_key.secret)
        .build()
        .context("build API-key client")?;

    let me = client.users().get_my_user().await.context("get_my_user")?;
    assert!(me.is_admin);

    // Create second user (for sharing & partner tests)
    let second_user = client
        .users()
        .admin()
        .create_user_admin(UserAdminCreateDTO::new(
            "user@e2e.local",
            "Second User",
            "user-password-123",
        ))
        .await
        .context("create_user_admin")?;
    assert!(!second_user.id.is_nil());

    let admin_users = client
        .users()
        .admin()
        .search_users_admin()
        .await
        .context("search_users_admin")?;
    assert!(admin_users.len() >= 2);

    // Upload profile image
    let dummy_1 = test_asset_path("dummy_1.jpg");
    let profile_res = client
        .upload_profile_image(&dummy_1)
        .await
        .context("upload_profile_image")?;
    assert_eq!(profile_res.user_id, me.id);

    // ── 3. Seed Assets (Single + Bulk + SHA-1 check + Download) ───────────────
    let dummy_1_sha1 = calculate_file_checksum(&dummy_1).await?;
    let asset_1 = client
        .upload_asset(
            UploadOptions::new(&dummy_1)
                .device_id("immich-rs-e2e")
                .checksum(&dummy_1_sha1),
        )
        .await
        .context("upload_asset dummy_1")?;
    let asset_1_id = asset_1.id;

    let bulk_paths: Vec<std::path::PathBuf> = (2..=5).map(|i| test_asset_path(&format!("dummy_{i}.jpg"))).collect();
    let bulk_items: Vec<UploadOptions<'_>> = bulk_paths
        .iter()
        .map(|p| UploadOptions::new(p).device_id("immich-rs-e2e"))
        .collect();
    let mut bulk_stream = pin!(client.upload_bulk(bulk_items).concurrency(2).into_stream());
    let mut all_asset_ids = vec![asset_1_id];
    while let Some(res) = bulk_stream.next().await {
        let result = res.context("upload_bulk item")?;
        let uploaded = result.uploaded().expect("expected newly uploaded asset");
        all_asset_ids.push(uploaded.id);
    }
    assert_eq!(all_asset_ids.len(), 5);

    // Check bulk upload deduplication via SHA-1
    let bulk_check = client
        .assets()
        .check_bulk_upload(AssetBulkUploadCheckDTO::new(vec![AssetBulkUploadCheckItem::new(
            dummy_1_sha1,
            "dummy_1.jpg",
        )]))
        .await
        .context("check_bulk_upload")?;
    assert_eq!(bulk_check.results.len(), 1);

    // Update asset metadata & favorite
    let updated_asset = client
        .assets()
        .update_asset(
            asset_1_id,
            UpdateAssetDTO::new().is_favorite(true).description("Seeded demo photo"),
        )
        .await
        .context("update_asset")?;
    assert!(updated_asset.is_favorite);

    let asset_stats = client
        .assets()
        .get_asset_statistics()
        .is_favorite(true)
        .await
        .context("get_asset_statistics")?;
    assert!(asset_stats.images >= 1);

    // Download asset and verify binary integrity
    let tmp_dir = tempdir()?;
    let downloaded_path = tmp_dir.path().join("downloaded_dummy_1.jpg");
    client
        .download(asset_1_id, &downloaded_path)
        .await
        .context("download asset")?;
    let downloaded_sha1 = calculate_file_checksum(&downloaded_path).await?;
    assert_eq!(downloaded_sha1, calculate_file_checksum(&dummy_1).await?);

    let download_info = client
        .assets()
        .downloads()
        .get_download_info(DownloadInfoDTO::new().asset_ids(vec![asset_1_id]))
        .await
        .context("get_download_info")?;
    assert!(download_info.total_size > 0);

    // ── 4. Stacks ─────────────────────────────────────────────────────────────
    let stack = client
        .assets()
        .stacks()
        .create_stack(StackCreateDTO::new(vec![all_asset_ids[1], all_asset_ids[2]]))
        .await
        .context("create_stack")?;
    let stacks = client
        .assets()
        .stacks()
        .search_stacks()
        .await
        .context("search_stacks")?;
    assert!(!stacks.is_empty(), "expected created stack in search");
    let fetched_stack = client
        .assets()
        .stacks()
        .get_stack(stack.id)
        .await
        .context("get_stack")?;
    assert_eq!(fetched_stack.id, stack.id);
    client
        .assets()
        .stacks()
        .delete_stack(stack.id)
        .await
        .context("delete_stack")?;

    // ── 5. Albums, Sharing & Activities ───────────────────────────────────────
    let album_name = format!("e2e-demo-{}", uuid::Uuid::new_v4());
    let album = client
        .albums()
        .create_album(CreateAlbumDTO::new(&album_name).description("Seeded demo album"))
        .await
        .context("create_album")?;
    let album_id = album.id;

    client
        .albums()
        .add_assets_to_album(album_id, BulkIdsDTO::new(vec![asset_1_id]))
        .await
        .context("add_assets_to_album")?;

    // Share album with second_user
    client
        .albums()
        .add_users_to_album(
            album_id,
            AddUsersDTO::new(vec![AlbumUserAddDTO::new(second_user.id).role(AlbumUserRole::Editor)]),
        )
        .await
        .context("add_users_to_album")?;

    let shared_albums = client
        .albums()
        .get_all_albums()
        .is_shared(true)
        .await
        .context("get_all_albums shared")?;
    assert!(shared_albums.iter().any(|a| a.id == album_id));

    let album_stats = client
        .albums()
        .get_album_statistics()
        .await
        .context("get_album_statistics")?;
    assert!(album_stats.owned >= 1);

    let activity = client
        .activities()
        .create_activity(ActivityCreateDTO::new(album_id, ReactionType::Comment).comment("Great demo photo!"))
        .await
        .context("create_activity")?;
    let activities = client
        .activities()
        .get_activities(album_id)
        .level(ReactionLevel::Album)
        .await
        .context("get_activities")?;
    assert!(!activities.is_empty(), "expected album activity");
    client
        .activities()
        .delete_activity(activity.id)
        .await
        .context("delete_activity")?;

    // ── 6. Tags, Memories, Partners & Shared Links ────────────────────────────
    let tag = client
        .tags()
        .create_tag(TagCreateDTO::new("e2e-vacation"))
        .await
        .context("create_tag")?;
    client
        .tags()
        .tag_assets(tag.id, BulkIdsDTO::new(vec![asset_1_id]))
        .await
        .context("tag_assets")?;
    let all_tags = client.tags().get_all_tags().await.context("get_all_tags")?;
    assert!(all_tags.iter().any(|t| t.id == tag.id));

    let now = chrono::Utc::now();
    let memory = client
        .memories()
        .create_memory(
            MemoryCreateDTO::new(MemoryDataDTO::new(2024), now, MemoryType::OnThisDay).asset_ids(vec![asset_1_id]),
        )
        .await
        .context("create_memory")?;
    let memories = client.memories().search_memories().await.context("search_memories")?;
    assert!(memories.iter().any(|m| m.id == memory.id));
    client
        .memories()
        .delete_memory(memory.id)
        .await
        .context("delete_memory")?;

    let partner = client
        .partners()
        .create_partner(PartnerCreateDTO::new(second_user.id))
        .await
        .context("create_partner")?;
    assert_eq!(partner.id, second_user.id);
    client
        .partners()
        .remove_partner(second_user.id)
        .await
        .context("remove_partner")?;

    let shared_link = client
        .shared_links()
        .create_shared_link(
            SharedLinkCreateDTO::new(SharedLinkType::Album)
                .album_id(album_id)
                .description("Public demo share"),
        )
        .await
        .context("create_shared_link")?;
    let links = client
        .shared_links()
        .get_all_shared_links()
        .await
        .context("get_all_shared_links")?;
    assert!(links.iter().any(|l| l.id == shared_link.id));
    client
        .shared_links()
        .remove_shared_link(shared_link.id)
        .await
        .context("remove_shared_link")?;

    // ── 7. Search, Timeline & People (Paginated Stream) ───────────────────────
    let search_res = client
        .search()
        .search_assets(MetadataSearchDTO::new().is_favorite(true))
        .await
        .context("search_assets")?;
    assert!(!search_res.assets.items.is_empty(), "expected favorite asset in search");

    let album_assets = client
        .search()
        .search_assets(MetadataSearchDTO::new().album_ids(vec![album_id]))
        .await
        .context("search_assets in album")?;
    assert!(!album_assets.assets.items.is_empty(), "expected assets in album");

    let random_assets = client
        .search()
        .search_random(RandomSearchDTO::new().size(std::num::NonZeroU64::new(2).unwrap()))
        .await
        .context("search_random")?;
    assert!(!random_assets.is_empty(), "expected random assets");

    let _buckets = client.timeline().get_time_buckets().await.context("get_time_buckets")?;

    let created_person = client
        .people()
        .create_person(PersonCreateDTO::new().name("Demo Person"))
        .await
        .context("create_person")?;
    let person_info = client
        .people()
        .get_person(created_person.id)
        .await
        .context("get_person")?;
    assert_eq!(person_info.name, "Demo Person");

    let mut people_stream = pin!(
        client
            .people()
            .get_all_people()
            .with_hidden(true)
            .into_paginator()
            .pages()
    );
    while let Some(page) = people_stream.next().await {
        let _ = page.context("get_all_people page")?;
    }

    // ── 8. Server, System Config, Queues & Trash Cleanup ─────────────────────
    let about = client.server().get_about_info().await.context("get_about_info")?;
    assert!(!about.version.is_empty(), "expected non-empty version");
    let version = client
        .server()
        .get_server_version()
        .await
        .context("get_server_version")?;
    assert!(version.major >= 1);
    let _storage = client.server().get_storage().await.context("get_storage")?;
    let _server_stats = client
        .server()
        .get_server_statistics()
        .await
        .context("get_server_statistics")?;
    let _sys_config = client.system().config().get_config().await.context("get_config")?;
    let queues = client.queues().get_queues().await.context("get_queues")?;
    assert!(!queues.is_empty(), "expected background job queues");

    // Delete all seeded assets + empty trash + delete album
    client
        .assets()
        .delete_assets(AssetBulkDeleteDTO::new(all_asset_ids).force(true))
        .await
        .context("delete_assets")?;
    client.trash().empty_trash().await.context("empty_trash")?;

    client.albums().delete_album(album_id).await.context("delete_album")?;

    let album_gone = client.albums().get_album_info(album_id).await;
    assert!(
        matches!(album_gone, Err(immich_rs::Error::Album(_))),
        "deleted album should return Album error, got: {album_gone:?}"
    );

    eprintln!("[e2e] ✓ full_user_workflow (Seeded Demo Instance) passed");
    Ok(())
}
