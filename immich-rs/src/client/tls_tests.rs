use pretty_assertions::assert_eq;

use super::*;

const TEST_CA_PEM: &[u8] = b"-----BEGIN CERTIFICATE-----
MIIDCDCCAfCgAwIBAgIUX7Gg3LhSlU/4dMHUdx5WOLMlqP8wDQYJKoZIhvcNAQEL
BQAwGzEZMBcGA1UEAwwQSW1taWNoVGVzdFJvb3RDQTAgFw0yMDAxMDEwMDAwMDBa
GA8yMTIwMDEwMTAwMDAwMFowGzEZMBcGA1UEAwwQSW1taWNoVGVzdFJvb3RDQTCC
ASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBAMaUWd8lx0E0GLw+Ktm4mhxx
EqbM5qggAVPRvVUZK8dSTCjUk6CE/36lWu/5Rr8EnfbhTUHfSmaRPuFYxWKTTM6f
L8Br3mvp1zbRVZZTvZKksGXPHjxuCOuFoODj+H+rJ/1ToWpF6clkyAV+1dCiqPOU
xpz14uhfURaYRN99TZQ5/R57iuz4CXWNVPuSdMeCv9eRwidIoNLkfEb8qYqKJok8
DBmRraZAPcb9C+8A1Jjk4eha52uBTUWsHvyxFnf9GT9wmMRDRKuYmDajNGPSolE1
KW9vOdKFrrBB6vORJVMQSz4ej7PLWFVS5Zf118aBxv/Zs/xTAcQoHK9lhp9p5VcC
AwEAAaNCMEAwDwYDVR0TAQH/BAUwAwEB/zAOBgNVHQ8BAf8EBAMCAQYwHQYDVR0O
BBYEFMcW0bHOG/lDUlXx1TBuIeskjxATMA0GCSqGSIb3DQEBCwUAA4IBAQA2P50H
B+gz6Ke/flxKvjpY5t+azFu5jrLUAKe0TsFTh6GiwHbW0wvvGWUEMXnlts3i1E8E
Nit1vp0boWjakvbONKsgPipBiRfEPpJZkepsJUl4wN+1MD9GRMlk78VZaNx2qCy5
kXyuhmUWOmY84RBugap+m87Fd6lRqUMsy0toHntE2Cgirdz49KCH6JB+Or9B8oMr
dHO9Vx2QQdnfXL+m8TA+pmHF0TqKfynLCJ4FnCBnpQMJM0aatim8HlFSHHXH+JYw
Si6ebQS7iNAlHiHJcwgXZA4b//rWfaLNMQosAet6c4TkFOVwvil4fHHmYO2/8BVS
NH4bsLFi0WKzMv0u
-----END CERTIFICATE-----";

const TEST_CLIENT_CERT_PEM: &[u8] = b"-----BEGIN CERTIFICATE-----
MIIC9zCCAd+gAwIBAgIUe/z4xnPlXHaOoOThiJQsVARfT7UwDQYJKoZIhvcNAQEL
BQAwGzEZMBcGA1UEAwwQSW1taWNoVGVzdFJvb3RDQTAgFw0yMDAxMDEwMDAwMDBa
GA8yMTIwMDEwMTAwMDAwMFowHTEbMBkGA1UEAwwSaW1taWNoLXRlc3QtY2xpZW50
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAo6lMHu9pW7UR0nLN1Xfk
O6Lse3ORHehS4JcfzrupmGVtmiqvHfTzwdRRfcLM9SjSoDsQ2k47DhoBHDdcme6w
pPThqrZHIMZXRvQKxuWRnYU0o338onKDZyifl58urGDOfZwFhX2xQhUhCodtXfXG
KOFgGm4BRdTLc8pzFb2rLLDxUU7fmBhW1B7teaCXSqgyPcYz4Xe+nwfRBmZTMLq2
uSzd5ni8KvZAmRfnUF+i3vekqrudWJENheBeqyW/QU7VBInQKJx0ktc1Vn93ZBZt
fon8MSy7l1zM0P0Pr6ebZbgTalXIrdKnJ2LKhqNi9t8Zp2v7msIeaLc7C59Iaagf
OQIDAQABoy8wLTAJBgNVHRMEAjAAMAsGA1UdDwQEAwIFoDATBgNVHSUEDDAKBggr
BgEFBQcDAjANBgkqhkiG9w0BAQsFAAOCAQEAg8LHvL6nsn+0QwDoJ7zbxEtMPENF
SCwBbjJgDN9G34wClNGfAuogbvvt9SQcHE5bS2dqSLYpHJ9dHD4sWfrnuHu1Q0tJ
UojDTj9Uy1lmHe78zYdkehCaH8sD1XEXcIL4zHOIaL3nHj8QMidOWbMBngSOWRO7
kHD5l1ZCyODRfaqSQbgujHU4igapBgFChjq20dWKGqpHcze7ttsje8m+W2qjV+24
Uz72OmJB9pZ8y/QbUqD+MCaSs091cTGj3sT5g0koyjB3awDFgC3wXXXn47n1mQAa
nqqxIhtCYdXcclMzlXjQEhr1fxS75HJH96ECiP+94HY895/mRUayHdtHPQ==
-----END CERTIFICATE-----";

const TEST_CLIENT_KEY_PEM: &[u8] = b"-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCjqUwe72lbtRHS
cs3Vd+Q7oux7c5Ed6FLglx/Ou6mYZW2aKq8d9PPB1FF9wsz1KNKgOxDaTjsOGgEc
N1yZ7rCk9OGqtkcgxldG9ArG5ZGdhTSjffyicoNnKJ+Xny6sYM59nAWFfbFCFSEK
h21d9cYo4WAabgFF1MtzynMVvasssPFRTt+YGFbUHu15oJdKqDI9xjPhd76fB9EG
ZlMwura5LN3meLwq9kCZF+dQX6Le96Squ51YkQ2F4F6rJb9BTtUEidAonHSS1zVW
f3dkFm1+ifwxLLuXXMzQ/Q+vp5tluBNqVcit0qcnYsqGo2L23xmna/uawh5otzsL
n0hpqB85AgMBAAECggEASHy8lVsOvCqnuSFWH31ZnjxcJ5WZKbjztDvbP7KIvFH4
SHctVsRejnQen+f+5XruRz/CCB7sMyO7aCbxIdDBdl5r8TOIr3CJyYhfmZamdzE/
jkZH9h9oKfGpNfuwLXM3IhVPsL9qGk3Hc6VmAF7SJNsluK8Oa0KD1kZN0MxQG5uR
UqStS+MV5x52LnF9/aunPqCQTCrsPFvL2pRur4o2TFURdtxf31h7YucJ+Gve/sAP
3JhHve1T0xCUPpcSGI6nCwReOqx756PhrR85HjbkMFJG0yyKvOaetOJAz2QEBuWh
jNvHfmMoaF2P4+85zqLKSBMNkwz1FLT6qDjbLZX6LQKBgQDfbGJPInQ/SMcyKsLl
6WoOg9R1D2n9F9OKKu9wBUfjX1zpwBo+JtvLxDj6wYlRPlps6r0dmtUDiqWPrNz6
29CZg8uzzkpO+H8NnUDoXfT00oDgCaRg91an+LjT7UJw4iHJAODVFpmpLAqHbO8L
hl8eS4XtmNKqTmqBD7aHlmJKiwKBgQC7hjQ45VOAm5Nx7EJqRtBvpBv24z2h/xOt
slrSYLxxPULB3woX2tfa1UFOKG2RRXcjnk2JBrASwxK9SpPv/nDf5iKmQtHdLVa5
QIjTFEblsfwMFOBdAuCRammqYkh9l+IrK12Tu0C6gAqWbihjoQlYKFrzJtJ2jHUJ
YhF8g+xpywKBgGiAy6L0HKeVJQcEFVFw0hij9PYkFZE6Wcngt0oQTkW9zznOjvfP
OTOgPn6bk90aazVSvs5gI46ahUIkut/jfTiw+tDJhtxVQyhdC4UknXT+GSWxnN92
EWRT5C/MZC1kuCTKcl95SLpmTS5294DOgyMqdCRkPPVH5G0lqlzVLHhtAoGAA3Qi
TBX1Jr42xfda9cgeGKBGH5Lp1Kz0+NL7FzrO3asKKrXn4q6WISKiVEUsV2uXGSHW
PbrQzVeoZbIU7WubOpRrePJsGQ23tiu5GWIbOCnCAZr00uG1rxftoES1HxMICFGB
JLJjK8wl5cVbmxIhMO11V1sUrzhBA4450jg/x2cCgYEAn3KRA1rbxbjmz6QYCrSf
nzdrfC4FIVw5uj+Ggq6CjevVoAQ6y2TUDh57MYnUYJT4Y4h96+lfWl/D8MUJFsRH
5ebDHHntU+fTPY+eAHmrjHihfbNlTsABlDwyySjBZOR71HthbUR9YeFDvU8r573Y
UJZJ0l7O+Opr9anid8BMDjg=
-----END PRIVATE KEY-----";

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
        std::str::from_utf8(TEST_CLIENT_CERT_PEM).expect("utf8"),
        std::str::from_utf8(TEST_CA_PEM).expect("utf8")
    );
    assert_eq!(
        parts,
        Pkcs8PemParts {
            cert_chain_pem: expected_chain.into_bytes(),
            private_key_pem: TEST_CLIENT_KEY_PEM.to_vec(),
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
