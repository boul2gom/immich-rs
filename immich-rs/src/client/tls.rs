#[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
use std::path::Path;

#[cfg(all(not(target_arch = "wasm32"), any(feature = "rustls", feature = "native-tls")))]
use crate::error::{Error, Result};

/// Internal representation of a client mTLS certificate and private key identity.
///
/// The [`Debug`](std::fmt::Debug) implementation always redacts private key material.
pub(crate) enum ClientIdentity {
    /// Separate PEM-encoded client certificate(s) and private key.
    Pem { cert_pem: Vec<u8>, key_pem: Vec<u8> },
    /// Single PEM bundle containing both the client certificate chain and private key.
    CombinedPem(Vec<u8>),
    /// Pre-constructed [`reqwest::Identity`] supplied directly by the caller.
    #[cfg(all(not(target_arch = "wasm32"), any(feature = "rustls", feature = "native-tls")))]
    Raw(reqwest::Identity),
}

impl std::fmt::Debug for ClientIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pem { .. } => f.write_str("Pem(<redacted>)"),
            Self::CombinedPem(_) => f.write_str("CombinedPem(<redacted>)"),
            #[cfg(all(not(target_arch = "wasm32"), any(feature = "rustls", feature = "native-tls")))]
            Self::Raw(_) => f.write_str("Raw(<redacted>)"),
        }
    }
}

impl ClientIdentity {
    pub(crate) fn from_pem_pair(cert_pem: &[u8], key_pem: &[u8]) -> std::result::Result<Self, String> {
        let cert_trimmed = cert_pem.trim_ascii();
        if cert_trimmed.is_empty() {
            return Err("client certificate PEM cannot be empty".into());
        }
        let key_trimmed = key_pem.trim_ascii();
        if key_trimmed.is_empty() {
            return Err("client private key PEM cannot be empty".into());
        }
        Ok(Self::Pem {
            cert_pem: cert_trimmed.to_vec(),
            key_pem: key_trimmed.to_vec(),
        })
    }

    pub(crate) fn from_pem_bundle(pem_bundle: &[u8]) -> std::result::Result<Self, String> {
        let trimmed = pem_bundle.trim_ascii();
        if trimmed.is_empty() {
            return Err("client identity PEM bundle cannot be empty".into());
        }
        Ok(Self::CombinedPem(trimmed.to_vec()))
    }

    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    pub(crate) fn from_files(cert_path: &Path, key_path: &Path) -> std::result::Result<Self, String> {
        let cert_pem = std::fs::read(cert_path)
            .map_err(|e| format!("failed to read client certificate {}: {e}", cert_path.display()))?;
        let key_pem = std::fs::read(key_path)
            .map_err(|e| format!("failed to read client private key {}: {e}", key_path.display()))?;
        Self::from_pem_pair(&cert_pem, &key_pem)
    }

    #[cfg(all(not(target_arch = "wasm32"), any(feature = "rustls", feature = "native-tls")))]
    pub(crate) fn apply_to_builder(self, cb: reqwest::ClientBuilder) -> Result<reqwest::ClientBuilder> {
        match self {
            Self::Pem { cert_pem, key_pem } => {
                #[cfg(feature = "rustls")]
                {
                    let mut combined = Vec::with_capacity(cert_pem.len() + 1 + key_pem.len());
                    combined.extend_from_slice(&cert_pem);
                    combined.push(b'\n');
                    combined.extend_from_slice(&key_pem);
                    let identity = reqwest::Identity::from_pem(&combined)
                        .map_err(|e| Error::Validation(format!("failed to parse client TLS identity PEM: {e}")))?;
                    Ok(cb.identity(identity))
                }
                #[cfg(all(feature = "native-tls", not(feature = "rustls")))]
                {
                    validate_pkcs8_key_pem(&key_pem).map_err(Error::Validation)?;
                    let identity = reqwest::Identity::from_pkcs8_pem(&cert_pem, &key_pem)
                        .map_err(|e| Error::Validation(format!("failed to parse client PKCS#8 identity PEM: {e}")))?;
                    Ok(cb.identity(identity))
                }
            }
            Self::CombinedPem(bundle) => {
                #[cfg(feature = "rustls")]
                {
                    let identity = reqwest::Identity::from_pem(&bundle).map_err(|e| {
                        Error::Validation(format!("failed to parse client TLS identity PEM bundle: {e}"))
                    })?;
                    Ok(cb.identity(identity))
                }
                #[cfg(all(feature = "native-tls", not(feature = "rustls")))]
                {
                    let parts = split_pem_bundle_for_pkcs8(&bundle).map_err(Error::Validation)?;
                    let identity = reqwest::Identity::from_pkcs8_pem(&parts.cert_chain_pem, &parts.private_key_pem)
                        .map_err(|e| {
                            Error::Validation(format!("failed to parse client PKCS#8 identity PEM bundle: {e}"))
                        })?;
                    Ok(cb.identity(identity))
                }
            }
            Self::Raw(identity) => Ok(cb.identity(identity)),
        }
    }
}

#[cfg(any(
    test,
    all(not(target_arch = "wasm32"), feature = "native-tls", not(feature = "rustls"))
))]
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Pkcs8PemParts {
    pub(crate) cert_chain_pem: Vec<u8>,
    pub(crate) private_key_pem: Vec<u8>,
}

#[cfg(any(
    test,
    all(not(target_arch = "wasm32"), feature = "native-tls", not(feature = "rustls"))
))]
fn contains_bytes(haystack: &[u8], needle: &[u8]) -> bool {
    haystack.windows(needle.len()).any(|w| w == needle)
}

#[cfg(any(
    test,
    all(not(target_arch = "wasm32"), feature = "native-tls", not(feature = "rustls"))
))]
pub(crate) fn validate_pkcs8_key_pem(key_pem: &[u8]) -> std::result::Result<(), String> {
    const PKCS8_BEGIN: &[u8] = b"-----BEGIN PRIVATE KEY-----";
    const PKCS8_END: &[u8] = b"-----END PRIVATE KEY-----";
    const RSA_BEGIN: &[u8] = b"-----BEGIN RSA PRIVATE KEY-----";
    const EC_BEGIN: &[u8] = b"-----BEGIN EC PRIVATE KEY-----";
    const ENCRYPTED_BEGIN: &[u8] = b"-----BEGIN ENCRYPTED PRIVATE KEY-----";

    if let Some(start) = key_pem.windows(PKCS8_BEGIN.len()).position(|w| w == PKCS8_BEGIN) {
        let after_start = &key_pem[start..];
        if contains_bytes(after_start, PKCS8_END) {
            return Ok(());
        }
        return Err("unterminated PKCS#8 private key PEM block".into());
    }

    if contains_bytes(key_pem, RSA_BEGIN)
        || contains_bytes(key_pem, EC_BEGIN)
        || contains_bytes(key_pem, ENCRYPTED_BEGIN)
    {
        return Err(
            "native-tls requires an unencrypted PKCS#8 private key (-----BEGIN PRIVATE KEY-----); convert the key to PKCS#8 or enable the `rustls` feature"
                .into(),
        );
    }

    Err("no PKCS#8 private key (-----BEGIN PRIVATE KEY-----) found in client identity PEM".into())
}

#[cfg(any(
    test,
    all(not(target_arch = "wasm32"), feature = "native-tls", not(feature = "rustls"))
))]
pub(crate) fn split_pem_bundle_for_pkcs8(bundle: &[u8]) -> std::result::Result<Pkcs8PemParts, String> {
    const CERT_BEGIN: &[u8] = b"-----BEGIN CERTIFICATE-----";
    const CERT_END: &[u8] = b"-----END CERTIFICATE-----";
    const KEY_BEGIN: &[u8] = b"-----BEGIN PRIVATE KEY-----";
    const KEY_END: &[u8] = b"-----END PRIVATE KEY-----";

    let mut cert_chain_pem = Vec::new();
    let mut search = bundle;
    while let Some(start) = search.windows(CERT_BEGIN.len()).position(|w| w == CERT_BEGIN) {
        let after_start = &search[start..];
        let Some(end_rel) = after_start.windows(CERT_END.len()).position(|w| w == CERT_END) else {
            return Err("unterminated X.509 certificate PEM block in client identity bundle".into());
        };
        let block_end = end_rel + CERT_END.len();
        if !cert_chain_pem.is_empty() {
            cert_chain_pem.push(b'\n');
        }
        cert_chain_pem.extend_from_slice(&after_start[..block_end]);
        search = &after_start[block_end..];
    }

    if cert_chain_pem.is_empty() {
        return Err("no valid X.509 certificate found in client identity PEM bundle".into());
    }

    validate_pkcs8_key_pem(bundle)?;

    let start = bundle
        .windows(KEY_BEGIN.len())
        .position(|w| w == KEY_BEGIN)
        .ok_or_else(|| {
            "no PKCS#8 private key (-----BEGIN PRIVATE KEY-----) found in client identity PEM".to_string()
        })?;
    let after_start = &bundle[start..];
    let end_rel = after_start
        .windows(KEY_END.len())
        .position(|w| w == KEY_END)
        .ok_or_else(|| "unterminated PKCS#8 private key PEM block".to_string())?;
    let private_key_pem = after_start[..end_rel + KEY_END.len()].to_vec();

    Ok(Pkcs8PemParts {
        cert_chain_pem,
        private_key_pem,
    })
}

/// Internal representation of a custom root CA certificate input.
pub(crate) enum RootCertInput {
    /// PEM-encoded CA certificate or bundle of multiple CA certificates.
    Pem(Vec<u8>),
    /// DER-encoded binary CA certificate.
    Der(Vec<u8>),
    /// Pre-constructed [`reqwest::Certificate`].
    #[cfg(all(not(target_arch = "wasm32"), any(feature = "rustls", feature = "native-tls")))]
    Raw(reqwest::Certificate),
}

impl std::fmt::Debug for RootCertInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pem(b) => write!(f, "Pem({} bytes)", b.len()),
            Self::Der(b) => write!(f, "Der({} bytes)", b.len()),
            #[cfg(all(not(target_arch = "wasm32"), any(feature = "rustls", feature = "native-tls")))]
            Self::Raw(_) => f.write_str("Raw(Certificate)"),
        }
    }
}

impl RootCertInput {
    pub(crate) fn from_pem(ca_pem: &[u8]) -> std::result::Result<Self, String> {
        let trimmed = ca_pem.trim_ascii();
        if trimmed.is_empty() {
            return Err("CA certificate PEM cannot be empty".into());
        }
        Ok(Self::Pem(trimmed.to_vec()))
    }

    pub(crate) fn from_der(ca_der: &[u8]) -> std::result::Result<Self, String> {
        if ca_der.is_empty() {
            return Err("CA certificate DER cannot be empty".into());
        }
        Ok(Self::Der(ca_der.to_vec()))
    }

    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    pub(crate) fn from_file(ca_path: &Path) -> std::result::Result<Self, String> {
        let bytes =
            std::fs::read(ca_path).map_err(|e| format!("failed to read CA certificate {}: {e}", ca_path.display()))?;
        let has_pem_header = bytes.windows(11).any(|w| w == b"-----BEGIN ");
        let is_der_ext = ca_path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("der") || ext.eq_ignore_ascii_case("cer"));
        let is_asn1_sequence = bytes.first() == Some(&0x30);

        if !has_pem_header && (is_der_ext || is_asn1_sequence) {
            Self::from_der(&bytes)
        } else {
            Self::from_pem(&bytes)
        }
    }

    #[cfg(all(not(target_arch = "wasm32"), any(feature = "rustls", feature = "native-tls")))]
    pub(crate) fn resolve_all(inputs: Vec<Self>) -> Result<Vec<reqwest::Certificate>> {
        let mut resolved = Vec::with_capacity(inputs.len());
        for input in inputs {
            match input {
                Self::Pem(pem) => {
                    let certs = reqwest::Certificate::from_pem_bundle(&pem)
                        .map_err(|e| Error::Validation(format!("failed to parse CA certificate PEM: {e}")))?;
                    if certs.is_empty() {
                        return Err(Error::Validation(
                            "no valid X.509 certificate found in CA certificate PEM".into(),
                        ));
                    }
                    resolved.extend(certs);
                }
                Self::Der(der) => {
                    let cert = reqwest::Certificate::from_der(&der)
                        .map_err(|e| Error::Validation(format!("failed to parse CA certificate DER: {e}")))?;
                    resolved.push(cert);
                }
                Self::Raw(cert) => resolved.push(cert),
            }
        }
        Ok(resolved)
    }
}

#[cfg(test)]
#[path = "tls_tests.rs"]
mod tests;
