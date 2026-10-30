//! `cargo-xtask` — `OpenAPI` AST code generation tasks for `immich-rs` and `immich-models`.
//!
//! # Usage
//!
//! ```bash
//! cargo xtask generate [--offline] [VERSION]         # Fetch spec + generate models & client
//! cargo xtask generate-models [--offline] [VERSION]  # Generate immich-models
//! cargo xtask generate-client [--offline] [VERSION]  # Generate immich-rs HTTP client
//! cargo xtask fetch-spec [VERSION]                   # Fetch the `OpenAPI` spec only
//! ```

#![allow(clippy::print_stdout, clippy::print_stderr)]

pub mod client;
pub mod common;
pub mod models;

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

const GITHUB_REPO: &str = "immich-app/immich";
const SPEC_PATH: &str = "open-api/immich-openapi-specs.json";
const USER_AGENT: &str = concat!("immich-rs-xtask/", env!("CARGO_PKG_VERSION"));

fn parse_cli_opts(args: impl Iterator<Item = String>) -> (bool, Option<String>) {
    let mut offline = false;
    let mut version = None;
    for arg in args {
        if arg == "--offline" || arg == "--local" {
            offline = true;
        } else if version.is_none() {
            version = Some(arg);
        }
    }
    (offline, version)
}

/// Runs the selected `xtask` command.
///
/// # Errors
/// Returns an error if the command execution fails.
pub fn run() -> Result<()> {
    let mut args = std::env::args().skip(1);

    match args.next().as_deref() {
        Some("generate") => {
            let (offline, version) = parse_cli_opts(args);
            let root = workspace_root()?;
            let (spec, resolved_version) = common::spec::resolve_spec(&root, version.as_deref(), offline)?;
            let model_index = models::run_with_spec(&root, &spec, &resolved_version)?;
            client::run_with_spec(&root, &spec, &model_index)
        }
        Some("generate-models") => {
            let (offline, version) = parse_cli_opts(args);
            let root = workspace_root()?;
            let (spec, resolved_version) = common::spec::resolve_spec(&root, version.as_deref(), offline)?;
            models::run_with_spec(&root, &spec, &resolved_version).map(|_| ())
        }
        Some("generate-client") => {
            let (offline, version) = parse_cli_opts(args);
            let root = workspace_root()?;
            let (spec, resolved_version) = common::spec::resolve_spec(&root, version.as_deref(), offline)?;
            let model_index = models::build_model_index(&spec, &resolved_version)?;
            client::run_with_spec(&root, &spec, &model_index)
        }
        Some("fetch-spec") => {
            let (_, version) = parse_cli_opts(args);
            let (changed, _) = fetch_spec(version.as_deref())?;
            if !changed {
                println!("[xtask] Nothing to do — spec is already current.");
            }
            Ok(())
        }
        _ => {
            eprintln!(
                "immich-rs xtask\n\
                \n\
                USAGE:\n\
                    cargo xtask generate [--offline] [VERSION]         Fetch spec + generate models & client\n\
                    cargo xtask generate-models [--offline] [VERSION]  Fetch spec + generate immich-models\n\
                    cargo xtask generate-client [--offline] [VERSION]  Fetch spec + generate immich-rs HTTP client\n\
                    cargo xtask fetch-spec [VERSION]                   Fetch the OpenAPI spec only\n\
                \n\
                VERSION is optional (e.g. v3.3.5). Without VERSION (and without --offline),\n\
                the latest Immich GitHub release is resolved automatically."
            );
            Ok(())
        }
    }
}

/// Ensures the local `OpenAPI` spec is in sync with the given Immich release.
///
/// # Errors
/// Returns an error if the network request or disk write fails.
pub fn fetch_spec(version: Option<&str>) -> Result<(bool, String)> {
    let tag = if let Some(v) = version {
        println!("[xtask] Target version: {v}");
        v.to_owned()
    } else {
        println!("[xtask] No version specified — resolving latest release...");
        latest_immich_release().context("failed to resolve the latest Immich version")?
    };

    let remote_sha = github_blob_sha(&tag).with_context(|| format!("fetching blob SHA for {tag}"))?;
    println!("[xtask] Remote blob SHA: {remote_sha}");

    let workspace_root = workspace_root()?;
    let out_path = workspace_root.join(SPEC_PATH);

    if let Ok(existing) = std::fs::read(&out_path) {
        let local_sha = git_blob_sha(&existing);
        if local_sha == remote_sha {
            println!("[xtask] ✓ Spec {tag} already up to date — skipping download.");
            return Ok((false, tag));
        }
        println!("[xtask] Spec changed ({local_sha:.12}… → {remote_sha:.12}…) — downloading.");
    } else {
        println!("[xtask] No local spec found — downloading.");
    }

    let raw_url = format!("https://raw.githubusercontent.com/{GITHUB_REPO}/{tag}/{SPEC_PATH}");
    println!("[xtask] Fetching: {raw_url}");

    let spec_bytes = fetch_url(&raw_url).with_context(|| format!("downloading Immich spec {tag}"))?;
    let _: serde_json::Value = serde_json::from_slice(&spec_bytes).context("downloaded spec is not valid JSON")?;

    let downloaded_sha = git_blob_sha(&spec_bytes);
    if downloaded_sha != remote_sha {
        bail!("blob SHA mismatch after download (expected {remote_sha}, got {downloaded_sha})");
    }

    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent).with_context(|| format!("creating directory {}", parent.display()))?;
    }
    std::fs::write(&out_path, &spec_bytes).with_context(|| format!("writing {}", out_path.display()))?;

    println!(
        "[xtask] ✓ Immich spec {tag} saved → {SPEC_PATH} ({} KB)",
        spec_bytes.len() / 1024
    );
    Ok((true, tag))
}

fn latest_immich_release() -> Result<String> {
    #[derive(serde::Deserialize)]
    struct GitHubRelease {
        tag_name: String,
    }

    let url = format!("https://api.github.com/repos/{GITHUB_REPO}/releases/latest");
    let body = fetch_url(&url).context("calling the GitHub API")?;
    let release: GitHubRelease = serde_json::from_slice(&body).context("parsing GitHub releases/latest response")?;
    println!("[xtask] Latest Immich release detected: {}", release.tag_name);
    Ok(release.tag_name)
}

/// Fetches a URL and returns the raw bytes.
///
/// # Errors
/// Returns an error if the HTTP request fails.
pub fn fetch_url(url: &str) -> Result<Vec<u8>> {
    let agent = ureq::Agent::new_with_defaults();

    let mut req = agent
        .get(url)
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", USER_AGENT);

    if url.starts_with("https://api.github.com/")
        && let Ok(token) = std::env::var("GITHUB_TOKEN").or_else(|_| std::env::var("GH_TOKEN"))
        && !token.trim().is_empty()
    {
        req = req.header("Authorization", &format!("Bearer {}", token.trim()));
    }

    let response = req.call().with_context(|| format!("GET request to {url}"))?;

    let status = response.status();
    if status != 200 {
        bail!("HTTP {status} for {url}");
    }

    response.into_body().read_to_vec().context("reading response body")
}

/// Finds the root of the Cargo workspace.
///
/// # Errors
/// Returns an error if the workspace root cannot be found.
pub fn workspace_root() -> Result<PathBuf> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .context("CARGO_MANIFEST_DIR is not set — this binary must be run via `cargo xtask`")?;

    let xtask_dir = Path::new(&manifest_dir);
    let root = xtask_dir
        .parent()
        .with_context(|| format!("cannot go up from {}", xtask_dir.display()))?
        .to_path_buf();

    let workspace_toml = root.join("Cargo.toml");
    if !workspace_toml.exists() {
        bail!(
            "workspace Cargo.toml not found in {} — unexpected layout",
            root.display()
        );
    }

    Ok(root)
}

fn github_blob_sha(tag: &str) -> Result<String> {
    #[derive(serde::Deserialize)]
    struct ContentsResponse {
        sha: String,
    }

    let url = format!("https://api.github.com/repos/{GITHUB_REPO}/contents/{SPEC_PATH}?ref={tag}");
    let body = fetch_url(&url).context("querying GitHub Contents API")?;
    let resp: ContentsResponse = serde_json::from_slice(&body).context("parsing GitHub Contents API response")?;
    Ok(resp.sha)
}

fn git_blob_sha(content: &[u8]) -> String {
    use sha1::{Digest, Sha1};

    let header = format!("blob {}\0", content.len());
    let mut hasher = Sha1::new();
    hasher.update(header.as_bytes());
    hasher.update(content);
    hex::encode(hasher.finalize())
}
