use pretty_assertions::assert_eq;

use super::*;

const TEST_CA_PEM: &[u8] = include_bytes!("../../tests/fixtures/tls/ca.pem");
const TEST_CLIENT_CERT_PEM: &[u8] = include_bytes!("../../tests/fixtures/tls/client.pem");
const TEST_CLIENT_KEY_PEM: &[u8] = include_bytes!("../../tests/fixtures/tls/client.key");

#[test]
fn client_identity_from_pem_pair_rejects_empty_inputs() {
    let empty_cert = ClientIdentity::from_pem_pair(b"   \n ", TEST_CLIENT_KEY_PEM);
    assert_eq!(
        empty_cert.unwrap_err(),
        "client certificate PEM cannot be empty".to_string()
    );

    let empty_key = ClientIdentity::from_pem_pair(TEST_CLIENT_CERT_PEM, b"  \t ");
    assert_eq!(
        empty_key.unwrap_err(),
        "client private key PEM cannot be empty".to_string()
    );
}

#[test]
fn client_identity_from_pem_bundle_rejects_empty_input() {
    let empty = ClientIdentity::from_pem_bundle(b"   ");
    assert_eq!(
        empty.unwrap_err(),
        "client identity PEM bundle cannot be empty".to_string()
    );
}

#[test]
fn client_identity_debug_always_redacts_private_key() {
    let pair = ClientIdentity::from_pem_pair(TEST_CLIENT_CERT_PEM, TEST_CLIENT_KEY_PEM).expect("valid pair");
    let debug_pair = format!("{pair:?}");
    assert_eq!(debug_pair, "Pem(<redacted>)");
    assert!(!debug_pair.contains("PRIVATE KEY"));

    let mut bundle_bytes = TEST_CLIENT_CERT_PEM.to_vec();
    bundle_bytes.push(b'\n');
    bundle_bytes.extend_from_slice(TEST_CLIENT_KEY_PEM);
    let bundle = ClientIdentity::from_pem_bundle(&bundle_bytes).expect("valid bundle");
    let debug_bundle = format!("{bundle:?}");
    assert_eq!(debug_bundle, "CombinedPem(<redacted>)");
    assert!(!debug_bundle.contains("PRIVATE KEY"));
}

#[test]
fn root_cert_input_rejects_empty_pem_and_der() {
    assert_eq!(
        RootCertInput::from_pem(b"  \n ").unwrap_err(),
        "CA certificate PEM cannot be empty".to_string()
    );
    assert_eq!(
        RootCertInput::from_der(b"").unwrap_err(),
        "CA certificate DER cannot be empty".to_string()
    );
}

#[cfg(all(not(target_arch = "wasm32"), any(feature = "rustls", feature = "native-tls")))]
#[test]
fn root_cert_input_resolve_all_rejects_pem_without_certificate_block() {
    let bad = RootCertInput::from_pem(b"not a valid PEM block").expect("non-empty");
    let err = RootCertInput::resolve_all(vec![bad]).unwrap_err();
    assert!(
        err.to_string().contains("no valid X.509 certificate") || err.to_string().contains("invalid certificate"),
        "unexpected error: {err}"
    );
}

#[cfg(all(not(target_arch = "wasm32"), any(feature = "rustls", feature = "native-tls")))]
#[test]
fn root_cert_input_resolve_all_parses_bundled_ca_certificates() {
    let mut bundle = TEST_CA_PEM.to_vec();
    bundle.push(b'\n');
    bundle.extend_from_slice(TEST_CLIENT_CERT_PEM);

    let input = RootCertInput::from_pem(&bundle).expect("non-empty bundle");
    let resolved = RootCertInput::resolve_all(vec![input]).expect("resolve bundle");
    assert_eq!(resolved.len(), 2);
}

#[test]
fn split_pem_bundle_for_pkcs8_extracts_cert_chain_and_key() {
    let mut bundle = TEST_CLIENT_CERT_PEM.to_vec();
    bundle.push(b'\n');
    bundle.extend_from_slice(TEST_CA_PEM);
    bundle.push(b'\n');
    bundle.extend_from_slice(TEST_CLIENT_KEY_PEM);

    let parts = split_pem_bundle_for_pkcs8(&bundle).expect("valid pkcs8 bundle");
    let expected_chain = format!(
        "{}\n{}",
        std::str::from_utf8(TEST_CLIENT_CERT_PEM.trim_ascii()).expect("utf8"),
        std::str::from_utf8(TEST_CA_PEM.trim_ascii()).expect("utf8")
    );
    assert_eq!(
        parts,
        Pkcs8PemParts {
            cert_chain_pem: expected_chain.into_bytes(),
            private_key_pem: TEST_CLIENT_KEY_PEM.trim_ascii().to_vec(),
        }
    );
}

#[test]
fn split_pem_bundle_for_pkcs8_rejects_rsa_ec_and_encrypted_keys_with_clear_error() {
    for key_header in [
        "-----BEGIN RSA PRIVATE KEY-----\nAAAA\n-----END RSA PRIVATE KEY-----",
        "-----BEGIN EC PRIVATE KEY-----\nAAAA\n-----END EC PRIVATE KEY-----",
        "-----BEGIN ENCRYPTED PRIVATE KEY-----\nAAAA\n-----END ENCRYPTED PRIVATE KEY-----",
    ] {
        let mut bundle = TEST_CLIENT_CERT_PEM.to_vec();
        bundle.push(b'\n');
        bundle.extend_from_slice(key_header.as_bytes());
        let err = split_pem_bundle_for_pkcs8(&bundle).unwrap_err();
        assert!(
            err.contains("native-tls requires an unencrypted PKCS#8 private key"),
            "unexpected error: {err}"
        );
    }
}

#[test]
fn split_pem_bundle_for_pkcs8_rejects_missing_cert_or_unterminated_blocks() {
    let no_cert = split_pem_bundle_for_pkcs8(TEST_CLIENT_KEY_PEM).unwrap_err();
    assert!(no_cert.contains("no valid X.509 certificate"), "unexpected: {no_cert}");

    let unterminated_cert = split_pem_bundle_for_pkcs8(b"-----BEGIN CERTIFICATE-----\nAAAA").unwrap_err();
    assert!(
        unterminated_cert.contains("unterminated X.509 certificate"),
        "unexpected: {unterminated_cert}"
    );

    let mut unterminated_key = TEST_CLIENT_CERT_PEM.to_vec();
    unterminated_key.extend_from_slice(b"\n-----BEGIN PRIVATE KEY-----\nAAAA");
    let err = split_pem_bundle_for_pkcs8(&unterminated_key).unwrap_err();
    assert!(err.contains("unterminated PKCS#8 private key"), "unexpected: {err}");
}
