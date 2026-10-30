//! Advisory version compatibility checking between `immich-rs` and the remote Immich server.

use immich_models::TARGET_IMMICH_VERSION;

use crate::client::ImmichClient;
use crate::error::Result;

/// The assessed compatibility status between the remote server version and the SDK.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Compatibility {
    /// The remote server version exactly matches the SDK's target version.
    ExactMatch {
        /// The server version string.
        server: String,
    },
    /// The major and minor versions match; only the patch or pre-release version differs.
    Compatible {
        /// The server version string.
        server: String,
        /// The SDK target version.
        target: &'static str,
    },
    /// The remote server version is older than the SDK's target version.
    ServerOlder {
        /// The server version string.
        server: String,
        /// The SDK target version.
        target: &'static str,
    },
    /// The remote server version is newer than the SDK's target version.
    ServerNewer {
        /// The server version string.
        server: String,
        /// The SDK target version.
        target: &'static str,
    },
    /// Either the server version or SDK target version could not be parsed as `SemVer`.
    Unparseable {
        /// The unparseable server version string.
        server: String,
        /// The SDK target version.
        target: &'static str,
    },
}

impl Compatibility {
    /// Returns `true` if the server is considered compatible (exact match or same major/minor).
    #[must_use]
    pub const fn is_compatible(&self) -> bool {
        matches!(self, Self::ExactMatch { .. } | Self::Compatible { .. })
    }
}

fn parse_semver(s: &str) -> Option<(u64, u64, u64)> {
    let clean = s.trim().trim_start_matches(['v', 'V']);
    if clean.is_empty() {
        return None;
    }
    let core = clean.split(['-', '+']).next()?;
    let mut parts = core.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next()?.parse().ok()?;
    let patch = parts.next().unwrap_or("0").parse().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some((major, minor, patch))
}

/// Evaluates the compatibility between a server `SemVer` string and the target SDK version.
#[must_use]
pub fn evaluate_compatibility(server_version: &str, target_version: &'static str) -> Compatibility {
    let parsed_server = parse_semver(server_version);
    let parsed_target = parse_semver(target_version);

    let (Some((s_maj, s_min, s_patch)), Some((t_maj, t_min, t_patch))) = (parsed_server, parsed_target) else {
        return Compatibility::Unparseable {
            server: server_version.to_string(),
            target: target_version,
        };
    };

    let server_clean = server_version.trim().trim_start_matches(['v', 'V']);
    let target_clean = target_version.trim().trim_start_matches(['v', 'V']);

    if server_clean == target_clean {
        return Compatibility::ExactMatch {
            server: server_version.to_string(),
        };
    }

    if (s_maj, s_min) == (t_maj, t_min) {
        Compatibility::Compatible {
            server: server_version.to_string(),
            target: target_version,
        }
    } else if (s_maj, s_min, s_patch) < (t_maj, t_min, t_patch) {
        Compatibility::ServerOlder {
            server: server_version.to_string(),
            target: target_version,
        }
    } else {
        Compatibility::ServerNewer {
            server: server_version.to_string(),
            target: target_version,
        }
    }
}

impl ImmichClient {
    /// Checks the remote Immich server version and compares it with this SDK's target version.
    ///
    /// This is an advisory check. It never returns an error on version mismatch, but logs
    /// a [`tracing::warn!`] if the server is older, newer, or unparseable compared to this SDK's target `OpenAPI` version.
    ///
    /// # Errors
    ///
    /// Returns an error if the network request fails or if the server response cannot be parsed.
    pub async fn check_compatibility(&self) -> Result<Compatibility> {
        let version_dto = self.server().get_server_version().await?;
        let server_ver = format!("{}.{}.{}", version_dto.major, version_dto.minor, version_dto.patch);
        let compat = evaluate_compatibility(&server_ver, TARGET_IMMICH_VERSION);

        match &compat {
            Compatibility::ExactMatch { server } => {
                tracing::info!(
                    server = %server,
                    target = TARGET_IMMICH_VERSION,
                    "Immich server version matches SDK target"
                );
            }
            Compatibility::Compatible { server, target } => {
                tracing::debug!(
                    server = %server,
                    target = *target,
                    "Immich server version is compatible with SDK target"
                );
            }
            Compatibility::ServerOlder { server, target } => {
                tracing::warn!(
                    server = %server,
                    target = *target,
                    "Immich server is older than this SDK target version; some endpoints may fail"
                );
            }
            Compatibility::ServerNewer { server, target } => {
                tracing::warn!(
                    server = %server,
                    target = *target,
                    "Immich server is newer than this SDK target version; some new features may be absent"
                );
            }
            Compatibility::Unparseable { server, target } => {
                tracing::warn!(
                    server = %server,
                    target = *target,
                    "Could not parse Immich server or target SemVer version"
                );
            }
        }

        Ok(compat)
    }
}

#[cfg(test)]
#[path = "compatibility_tests.rs"]
mod tests;
