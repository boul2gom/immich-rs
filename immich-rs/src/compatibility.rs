//! Advisory version compatibility checking between `immich-rs` and the remote Immich server.

use immich_models::TARGET_IMMICH_VERSION;

use crate::client::ImmichClient;
use crate::error::Result;

/// The assessed compatibility status between the remote server version and the SDK.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Compatibility {
    /// The remote server version exactly matches the SDK's target version.
    ExactMatch {
        /// The server version string.
        server: String,
    },
    /// The major and minor versions match; only the patch version differs.
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
}

impl Compatibility {
    /// Returns true if the server is considered compatible (exact match or same major/minor).
    #[must_use]
    pub fn is_compatible(&self) -> bool {
        matches!(self, Self::ExactMatch { .. } | Self::Compatible { .. })
    }
}

fn parse_semver(s: &str) -> Option<(u64, u64, u64)> {
    let clean = s.trim().trim_start_matches('v');
    let mut parts = clean.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next()?.parse().ok()?;
    let patch = parts.next().unwrap_or("0").parse().ok()?;
    Some((major, minor, patch))
}

/// Evaluates the compatibility between a server semver string and the target SDK version.
#[must_use]
pub fn evaluate_compatibility(server_version: &str, target_version: &'static str) -> Compatibility {
    let server_clean = server_version.trim().trim_start_matches('v');
    let target_clean = target_version.trim().trim_start_matches('v');

    if server_clean == target_clean {
        return Compatibility::ExactMatch {
            server: server_version.to_string(),
        };
    }

    let parsed_server = parse_semver(server_version);
    let parsed_target = parse_semver(target_version);

    match (parsed_server, parsed_target) {
        (Some((s_maj, s_min, s_patch)), Some((t_maj, t_min, t_patch))) => {
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
        _ => Compatibility::Compatible {
            server: server_version.to_string(),
            target: target_version,
        },
    }
}

impl ImmichClient {
    /// Checks the remote Immich server version and compares it with this SDK's target version.
    ///
    /// This is an advisory check. It never returns an error on version mismatch, but logs
    /// a [`tracing::warn!`] if the server is older or newer than this SDK's target `OpenAPI` version.
    ///
    /// # Errors
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
        }

        Ok(compat)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_compatibility() {
        assert_eq!(
            evaluate_compatibility("v3.3.5", "v3.3.5"),
            Compatibility::ExactMatch {
                server: "v3.3.5".into()
            }
        );

        assert_eq!(
            evaluate_compatibility("v3.3.4", "v3.3.5"),
            Compatibility::Compatible {
                server: "v3.3.4".into(),
                target: "v3.3.5"
            }
        );

        assert_eq!(
            evaluate_compatibility("v3.2.0", "v3.3.5"),
            Compatibility::ServerOlder {
                server: "v3.2.0".into(),
                target: "v3.3.5"
            }
        );

        assert_eq!(
            evaluate_compatibility("v3.4.0", "v3.3.5"),
            Compatibility::ServerNewer {
                server: "v3.4.0".into(),
                target: "v3.3.5"
            }
        );
    }
}
