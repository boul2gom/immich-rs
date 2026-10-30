//! Configuration loaders supporting environment variables, `@immich/cli` YAML, and `immich-go` TOML/YAML/JSON.

#[cfg(all(feature = "cli-config", not(target_arch = "wasm32")))]
use crate::error::{Error, Result};

/// Attempts to resolve the base URL and API key using a custom environment variable lookup function.
#[must_use]
pub fn resolve_from_env_with(lookup: impl Fn(&str) -> Option<String>) -> Option<(String, String)> {
    let non_empty = |v: String| (!v.trim().is_empty()).then_some(v);

    let url = lookup("IMMICH_INSTANCE_URL")
        .and_then(non_empty)
        .or_else(|| lookup("IMMICH_BASE_URL").and_then(non_empty))
        .or_else(|| lookup("IMMICH_URL").and_then(non_empty))
        .or_else(|| lookup("IMMICH_SERVER_URL").and_then(non_empty))?;

    let key = lookup("IMMICH_API_KEY").and_then(non_empty)?;

    Some((url.trim().to_string(), key.trim().to_string()))
}

/// Attempts to resolve the base URL and API key from standard environment variables.
///
/// Inspects the following URL variables in order:
/// 1. `IMMICH_INSTANCE_URL`
/// 2. `IMMICH_BASE_URL`
/// 3. `IMMICH_URL`
/// 4. `IMMICH_SERVER_URL`
///
/// And inspects `IMMICH_API_KEY` for authentication.
#[must_use]
pub fn resolve_from_env() -> Option<(String, String)> {
    resolve_from_env_with(|key| std::env::var(key).ok())
}

#[cfg(all(feature = "cli-config", not(target_arch = "wasm32")))]
mod cli_impl {
    use std::path::{Path, PathBuf};

    use serde::Deserialize;

    use super::{Error, Result, resolve_from_env_with};

    #[derive(Deserialize, Debug)]
    struct ImmichCliConfig {
        #[serde(alias = "instanceUrl", alias = "instance_url", alias = "url")]
        instance_url: String,
        #[serde(alias = "apiKey", alias = "api_key")]
        api_key: String,
    }

    #[derive(Deserialize, Debug)]
    struct ImmichGoConfig {
        #[serde(alias = "server", alias = "url", alias = "instance_url", alias = "instanceUrl")]
        server: String,
        #[serde(alias = "api-key", alias = "apiKey", alias = "api_key", alias = "key")]
        api_key: String,
    }

    fn user_home() -> Option<PathBuf> {
        std::env::var_os("HOME")
            .or_else(|| std::env::var_os("USERPROFILE"))
            .map(PathBuf::from)
    }

    /// Resolves configuration from the official `@immich/cli` auth file.
    ///
    /// Checks `$IMMICH_CONFIG_DIR/auth.yml` if set, otherwise `~/.config/immich/auth.yml`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Validation`] or [`Error::Io`] if the file cannot be found or read.
    pub fn resolve_from_immich_cli() -> Result<(String, String)> {
        let custom_dir = std::env::var_os("IMMICH_CONFIG_DIR").map(PathBuf::from);
        resolve_from_immich_cli_in(custom_dir.as_deref(), user_home().as_deref())
    }

    /// Resolves `@immich/cli` configuration from explicit directories.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Validation`] or [`Error::Io`] if the file cannot be found or read.
    pub fn resolve_from_immich_cli_in(custom_dir: Option<&Path>, home_dir: Option<&Path>) -> Result<(String, String)> {
        let auth_path = if let Some(dir) = custom_dir {
            dir.join("auth.yml")
        } else if let Some(home) = home_dir {
            home.join(".config").join("immich").join("auth.yml")
        } else {
            return Err(Error::Validation("could not determine user home directory".into()));
        };

        if !auth_path.exists() {
            return Err(Error::Validation(format!(
                "@immich/cli configuration file not found at {}",
                auth_path.display()
            )));
        }

        let content = std::fs::read_to_string(&auth_path)
            .map_err(|e| Error::Validation(format!("failed to read {}: {e}", auth_path.display())))?;

        let parsed: ImmichCliConfig = serde_saphyr::from_str(&content)
            .map_err(|e| Error::Validation(format!("failed to parse {}: {e}", auth_path.display())))?;

        if parsed.instance_url.trim().is_empty() || parsed.api_key.trim().is_empty() {
            return Err(Error::Validation(format!(
                "empty instanceUrl or apiKey in {}",
                auth_path.display()
            )));
        }

        Ok((
            parsed.instance_url.trim().to_string(),
            parsed.api_key.trim().to_string(),
        ))
    }

    fn parse_immich_go_file(path: &Path) -> Result<(String, String)> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| Error::Validation(format!("failed to read {}: {e}", path.display())))?;

        let config: ImmichGoConfig = match path.extension().and_then(|ext| ext.to_str()) {
            Some("toml") => toml::from_str(&content)
                .map_err(|e| Error::Validation(format!("failed to parse {}: {e}", path.display())))?,
            Some("yaml" | "yml") => serde_saphyr::from_str(&content)
                .map_err(|e| Error::Validation(format!("failed to parse {}: {e}", path.display())))?,
            Some("json") => serde_json::from_str(&content)
                .map_err(|e| Error::Validation(format!("failed to parse {}: {e}", path.display())))?,
            _ => {
                // Fallback attempt: try TOML first, then YAML, then JSON
                toml::from_str(&content)
                    .or_else(|_| serde_saphyr::from_str(&content))
                    .or_else(|_| serde_json::from_str(&content))
                    .map_err(|_| {
                        Error::Validation(format!("unrecognized configuration format in {}", path.display()))
                    })?
            }
        };

        if config.server.trim().is_empty() || config.api_key.trim().is_empty() {
            return Err(Error::Validation(format!(
                "empty server or api-key in {}",
                path.display()
            )));
        }

        Ok((config.server.trim().to_string(), config.api_key.trim().to_string()))
    }

    /// Resolves configuration from `immich-go` files in the current directory or user config directory.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Validation`] or [`Error::Io`] if no configuration file is found or parsing fails.
    pub fn resolve_from_immich_go() -> Result<(String, String)> {
        resolve_from_immich_go_in(None, user_home().as_deref())
    }

    /// Resolves `immich-go` configuration from explicit local and user home directories.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Validation`] or [`Error::Io`] if no configuration file is found or parsing fails.
    pub fn resolve_from_immich_go_in(local_dir: Option<&Path>, home_dir: Option<&Path>) -> Result<(String, String)> {
        let candidate_names = ["immich-go.toml", "immich-go.yaml", "immich-go.yml", "immich-go.json"];

        // 1. Local or custom working directory
        let base_local = local_dir.unwrap_or_else(|| Path::new("."));
        for name in candidate_names {
            let p = base_local.join(name);
            if p.exists() {
                return parse_immich_go_file(&p);
            }
        }

        // 2. User configuration directory ~/.config/immich-go/
        if let Some(home) = home_dir {
            let global_dir = home.join(".config").join("immich-go");
            for name in candidate_names {
                let p = global_dir.join(name);
                if p.exists() {
                    return parse_immich_go_file(&p);
                }
            }
        }

        Err(Error::Validation(
            "no immich-go configuration file found in current directory or ~/.config/immich-go/".into(),
        ))
    }

    /// Cascades through environment variables, local `immich-go` configuration, global `@immich/cli` auth,
    /// and global `immich-go` configuration.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Validation`] if neither environment variables nor configuration files provide valid credentials.
    pub fn resolve_env_or_config() -> Result<(String, String)> {
        let custom_cli_dir = std::env::var_os("IMMICH_CONFIG_DIR").map(PathBuf::from);
        resolve_env_or_config_with(
            |key| std::env::var(key).ok(),
            custom_cli_dir.as_deref(),
            None,
            user_home().as_deref(),
        )
    }

    /// Cascades through custom environment lookup and directory paths.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Validation`] if neither environment variables nor configuration files provide valid credentials.
    pub fn resolve_env_or_config_with(
        env_lookup: impl Fn(&str) -> Option<String>,
        cli_dir: Option<&Path>,
        local_go_dir: Option<&Path>,
        home_dir: Option<&Path>,
    ) -> Result<(String, String)> {
        // 1. Environment variables
        if let Some(creds) = resolve_from_env_with(env_lookup) {
            return Ok(creds);
        }

        // 2. Local immich-go file in current directory
        let candidate_names = ["immich-go.toml", "immich-go.yaml", "immich-go.yml", "immich-go.json"];
        let base_local = local_go_dir.unwrap_or_else(|| Path::new("."));
        for name in candidate_names {
            let p = base_local.join(name);
            if let Ok(creds) = parse_immich_go_file(&p) {
                return Ok(creds);
            }
        }

        // 3. Global @immich/cli config (~/.config/immich/auth.yml)
        if let Ok(creds) = resolve_from_immich_cli_in(cli_dir, home_dir) {
            return Ok(creds);
        }

        // 4. Global immich-go config (~/.config/immich-go/)
        if let Ok(creds) = resolve_from_immich_go_in(None, home_dir) {
            return Ok(creds);
        }

        Err(Error::Validation(
            "no Immich configuration found: set IMMICH_INSTANCE_URL and IMMICH_API_KEY, or run `@immich/cli login`, or configure `immich-go.toml`".into(),
        ))
    }
}

#[cfg(all(feature = "cli-config", not(target_arch = "wasm32")))]
pub use cli_impl::*;
