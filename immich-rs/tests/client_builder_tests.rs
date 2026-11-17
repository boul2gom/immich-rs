#![cfg(not(target_arch = "wasm32"))]

use std::time::Duration;

use immich_rs::ImmichClient;
use pretty_assertions::assert_eq;
use reqwest::header::{HeaderMap, HeaderValue};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[test]
fn builder_normalizes_base_url_trailing_slash() {
    let client = ImmichClient::builder()
        .base_url("http://example.com/api")
        .api_key("test-key")
        .build()
        .expect("build client");

    assert_eq!(client.base_url().as_str(), "http://example.com/api/");
}

#[test]
fn builder_configures_timeouts_and_headers_without_error() {
    let mut headers = HeaderMap::new();
    headers.insert("cf-access-client-id", HeaderValue::from_static("test-id"));

    let client = ImmichClient::builder()
        .base_url("http://example.com/api/")
        .api_key("test-key")
        .connect_timeout(Duration::from_secs(5))
        .global_timeout(Duration::from_secs(30))
        .user_agent("custom-agent/1.0")
        .default_headers(headers)
        .max_retries(5)
        .build()
        .expect("build client");

    assert_eq!(client.base_url().as_str(), "http://example.com/api/");
}

#[test]
fn builder_rejects_empty_credentials_cleanly() {
    let empty_key = ImmichClient::builder()
        .base_url("http://example.com/api/")
        .api_key("   ")
        .build();
    assert!(empty_key.is_err());

    let empty_bearer = ImmichClient::builder()
        .base_url("http://example.com/api/")
        .bearer_token("   ")
        .build();
    assert!(empty_bearer.is_err());
}

#[test]
fn builder_rejects_malformed_urls_with_validation_error() {
    let invalid = ImmichClient::builder()
        .base_url("not a valid url")
        .api_key("test-key")
        .build();
    assert!(invalid.is_err());

    let cannot_be_base = ImmichClient::builder()
        .base_url("data:text/plain,hello")
        .api_key("test-key")
        .build();
    assert!(cannot_be_base.is_err());
}

#[tokio::test]
async fn builder_sends_custom_user_agent_and_default_headers_to_server() -> anyhow::Result<()> {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/server/ping"))
        .and(header("user-agent", "ImmichCustomApp/3.0.0"))
        .and(header("cf-access-client-id", "test-cf-client-id"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"res": "pong"})))
        .expect(1)
        .mount(&server)
        .await;

    let mut headers = HeaderMap::new();
    headers.insert("cf-access-client-id", HeaderValue::from_static("test-cf-client-id"));

    let client = ImmichClient::builder()
        .base_url(server.uri())
        .api_key("test-key")
        .user_agent("ImmichCustomApp/3.0.0")
        .default_headers(headers)
        .build()?;

    let ping = client.server().ping_server().await?;
    assert_eq!(ping.res, "pong");
    Ok(())
}

#[tokio::test]
async fn check_compatibility_and_build_checked_advisory_flow() -> anyhow::Result<()> {
    use immich_rs::Compatibility;
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/server/version"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "featureLevel": 0,
            "major": 1,
            "minor": 100,
            "patch": 0
        })))
        .mount(&server)
        .await;

    let client = ImmichClient::builder()
        .base_url(server.uri())
        .api_key("test-key")
        .build_checked()
        .await?;

    let compat = client.check_compatibility().await?;
    assert!(matches!(compat, Compatibility::ServerOlder { .. }));
    assert!(!compat.is_compatible());

    Ok(())
}

#[tokio::test]
async fn builder_custom_tower_layer_intercepts_requests() -> anyhow::Result<()> {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::task::{Context, Poll};

    use tower_layer::Layer;
    use tower_service::Service;

    #[derive(Clone)]
    struct CountingLayer {
        counter: Arc<AtomicUsize>,
    }

    impl<S> Layer<S> for CountingLayer {
        type Service = CountingService<S>;
        fn layer(&self, inner: S) -> Self::Service {
            CountingService {
                inner,
                counter: Arc::clone(&self.counter),
            }
        }
    }

    #[derive(Clone)]
    struct CountingService<S> {
        inner: S,
        counter: Arc<AtomicUsize>,
    }

    impl<S> Service<reqwest::Request> for CountingService<S>
    where
        S: Service<reqwest::Request, Response = reqwest::Response, Error = immich_rs::Error> + Clone + Send + 'static,
        S::Future: Send + 'static,
    {
        type Response = reqwest::Response;
        type Error = immich_rs::Error;
        type Future = immich_rs::BoxFuture<'static, Result<Self::Response, Self::Error>>;

        fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            self.inner.poll_ready(cx)
        }

        fn call(&mut self, mut req: reqwest::Request) -> Self::Future {
            self.counter.fetch_add(1, Ordering::Relaxed);
            req.headers_mut().insert(
                reqwest::header::HeaderName::from_static("x-custom-layer"),
                reqwest::header::HeaderValue::from_static("injected"),
            );
            let mut inner = self.inner.clone();
            Box::pin(async move { inner.call(req).await })
        }
    }

    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/server/ping"))
        .and(header("x-custom-layer", "injected"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"res": "pong"})))
        .expect(1)
        .mount(&server)
        .await;

    let counter = Arc::new(AtomicUsize::new(0));
    let client = ImmichClient::builder()
        .base_url(server.uri())
        .api_key("test-key")
        .layer(CountingLayer {
            counter: Arc::clone(&counter),
        })
        .build()?;

    let res = client.server().ping_server().await?;
    assert_eq!(res.res, "pong");
    assert_eq!(counter.load(Ordering::Relaxed), 1);

    Ok(())
}

#[tokio::test(start_paused = true)]
async fn builder_rate_limiting_smoothly_throttles_requests() -> anyhow::Result<()> {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/server/ping"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"res": "pong"})))
        .expect(3)
        .mount(&server)
        .await;

    // 2 rps = burst capacity of 2 tokens, 500ms refill per token
    let client = ImmichClient::builder()
        .base_url(server.uri())
        .api_key("test-key")
        .rate_limit_per_second(2)
        .build()?;

    for _ in 0..3 {
        let ping = client.server().ping_server().await?;
        assert_eq!(ping.res, "pong");
    }

    Ok(())
}

const TEST_CA_PEM: &[u8] = include_bytes!("fixtures/tls/ca.pem");
const TEST_SERVER_CERT_PEM: &[u8] = include_bytes!("fixtures/tls/server.pem");
const TEST_SERVER_KEY_PEM: &[u8] = include_bytes!("fixtures/tls/server.key");
const TEST_CLIENT_CERT_PEM: &[u8] = include_bytes!("fixtures/tls/client.pem");
const TEST_CLIENT_KEY_PEM: &[u8] = include_bytes!("fixtures/tls/client.key");

#[test]
fn builder_client_identity_pem_and_custom_ca_build_successfully() {
    let client = ImmichClient::builder()
        .base_url("https://immich.example.com/api")
        .api_key("test-key")
        .client_identity_pem(TEST_CLIENT_CERT_PEM, TEST_CLIENT_KEY_PEM)
        .add_root_certificate_pem(TEST_CA_PEM)
        .build()
        .expect("build client with mTLS identity and custom CA");

    assert_eq!(client.base_url().as_str(), "https://immich.example.com/api/");
}

#[test]
fn builder_client_identity_pem_bundle_builds_successfully() {
    let mut bundle = TEST_CLIENT_CERT_PEM.to_vec();
    bundle.push(b'\n');
    bundle.extend_from_slice(TEST_CLIENT_KEY_PEM);

    let client = ImmichClient::builder()
        .base_url("https://immich.example.com/api")
        .api_key("test-key")
        .client_identity_pem_bundle(&bundle)
        .build()
        .expect("build client with combined mTLS PEM bundle");

    assert_eq!(client.base_url().as_str(), "https://immich.example.com/api/");
}

#[test]
fn builder_client_identity_and_ca_reject_empty_or_invalid_inputs() {
    let empty_cert = ImmichClient::builder()
        .base_url("https://immich.example.com/api")
        .api_key("test-key")
        .client_identity_pem(b"  ", TEST_CLIENT_KEY_PEM)
        .build();
    assert!(matches!(
        empty_cert,
        Err(immich_rs::Error::Validation(ref msg)) if msg.contains("client certificate PEM cannot be empty")
    ));

    let invalid_pem_cert = ImmichClient::builder()
        .base_url("https://immich.example.com/api")
        .api_key("test-key")
        .client_identity_pem(b"not a valid cert", TEST_CLIENT_KEY_PEM)
        .build();
    assert!(matches!(invalid_pem_cert, Err(immich_rs::Error::Validation(_))));

    let invalid_ca = ImmichClient::builder()
        .base_url("https://immich.example.com/api")
        .api_key("test-key")
        .add_root_certificate_pem(b"not a valid ca cert")
        .build();
    assert!(matches!(invalid_ca, Err(immich_rs::Error::Validation(_))));

    let invalid_der = ImmichClient::builder()
        .base_url("https://immich.example.com/api")
        .api_key("test-key")
        .add_root_certificate_der(b"invalid der bytes")
        .build();
    assert!(matches!(
        invalid_der,
        Err(immich_rs::Error::Validation(_) | immich_rs::Error::Transport(_))
    ));

    // Error preservation: first error on add_root_certificate_* must stick even if followed by valid input
    let sticky_err = ImmichClient::builder()
        .base_url("https://immich.example.com/api")
        .api_key("test-key")
        .add_root_certificate_pem(b"   ")
        .add_root_certificate_pem(TEST_CA_PEM)
        .build();
    assert!(matches!(
        sticky_err,
        Err(immich_rs::Error::Validation(ref msg)) if msg.contains("CA certificate PEM cannot be empty")
    ));
}

#[test]
fn builder_raw_reqwest_identity_and_certificate_work_across_features() {
    #[cfg(feature = "rustls")]
    let identity = {
        let mut bundle = TEST_CLIENT_CERT_PEM.to_vec();
        bundle.push(b'\n');
        bundle.extend_from_slice(TEST_CLIENT_KEY_PEM);
        reqwest::Identity::from_pem(&bundle).expect("valid rustls identity")
    };
    #[cfg(all(feature = "native-tls", not(feature = "rustls")))]
    let identity = reqwest::Identity::from_pkcs8_pem(TEST_CLIENT_CERT_PEM, TEST_CLIENT_KEY_PEM)
        .expect("valid native-tls pkcs8 identity");

    let ca_cert = reqwest::Certificate::from_pem(TEST_CA_PEM).expect("valid reqwest certificate");

    let client = ImmichClient::builder()
        .base_url("https://immich.example.com/api")
        .api_key("test-key")
        .client_identity(identity)
        .add_root_certificate(ca_cert)
        .build()
        .expect("build client with raw reqwest Identity and Certificate");

    assert_eq!(client.base_url().as_str(), "https://immich.example.com/api/");
}

#[test]
fn builder_debug_output_redacts_client_identity_private_key() {
    let builder = ImmichClient::builder()
        .base_url("https://immich.example.com/api")
        .api_key("secret-api-key")
        .client_identity_pem(TEST_CLIENT_CERT_PEM, TEST_CLIENT_KEY_PEM);

    let debug_str = format!("{builder:?}");
    assert!(debug_str.contains("Pem(<redacted>)"));
    assert!(!debug_str.contains("BEGIN PRIVATE KEY"));
    assert!(!debug_str.contains("secret-api-key"));
}

#[cfg(feature = "fs")]
#[test]
fn builder_client_identity_and_ca_from_files_load_from_tempdir_and_reject_missing_files() {
    let dir = tempfile::tempdir().expect("create tempdir");
    let cert_path = dir.path().join("client.crt");
    let key_path = dir.path().join("client.key");
    let ca_path = dir.path().join("ca.pem");

    std::fs::write(&cert_path, TEST_CLIENT_CERT_PEM).expect("write client cert");
    std::fs::write(&key_path, TEST_CLIENT_KEY_PEM).expect("write client key");
    std::fs::write(&ca_path, TEST_CA_PEM).expect("write ca cert");

    let client = ImmichClient::builder()
        .base_url("https://immich.example.com/api")
        .api_key("test-key")
        .client_identity_from_files(&cert_path, &key_path)
        .add_root_certificate_from_file(&ca_path)
        .build()
        .expect("build client from certificate files");

    assert_eq!(client.base_url().as_str(), "https://immich.example.com/api/");

    let missing = ImmichClient::builder()
        .base_url("https://immich.example.com/api")
        .api_key("test-key")
        .client_identity_from_files(dir.path().join("nonexistent.crt"), &key_path)
        .build();
    assert!(missing.is_err());
    assert!(
        missing
            .unwrap_err()
            .to_string()
            .contains("failed to read client certificate")
    );
}

#[cfg(feature = "fs")]
#[test]
fn builder_from_env_map_loads_mtls_and_ca_paths_and_rejects_partial_mtls_vars() {
    let dir = tempfile::tempdir().expect("create tempdir");
    let cert_path = dir.path().join("client.crt");
    let key_path = dir.path().join("client.key");
    let ca_path = dir.path().join("ca.pem");

    std::fs::write(&cert_path, TEST_CLIENT_CERT_PEM).expect("write client cert");
    std::fs::write(&key_path, TEST_CLIENT_KEY_PEM).expect("write client key");
    std::fs::write(&ca_path, TEST_CA_PEM).expect("write ca cert");

    let cert_str = cert_path.to_string_lossy().into_owned();
    let key_str = key_path.to_string_lossy().into_owned();
    let ca_str = ca_path.to_string_lossy().into_owned();

    let client = ImmichClient::builder()
        .from_env_map(|k| match k {
            "IMMICH_INSTANCE_URL" => Some("https://immich.example.com/api".to_string()),
            "IMMICH_API_KEY" => Some("env-key".to_string()),
            "IMMICH_CLIENT_CERT_PATH" => Some(cert_str.clone()),
            "IMMICH_CLIENT_KEY_PATH" => Some(key_str.clone()),
            "IMMICH_CA_CERT_PATH" => Some(ca_str.clone()),
            _ => None,
        })
        .expect("from_env_map succeeds")
        .build()
        .expect("build succeeds");

    assert_eq!(client.base_url().as_str(), "https://immich.example.com/api/");
    assert_eq!(client.api_key().as_deref(), Some("env-key"));

    // Only IMMICH_CLIENT_CERT_PATH without IMMICH_CLIENT_KEY_PATH must fail validation
    let partial = ImmichClient::builder().from_env_map(|k| match k {
        "IMMICH_INSTANCE_URL" => Some("https://immich.example.com/api".to_string()),
        "IMMICH_CLIENT_CERT_PATH" => Some(cert_str.clone()),
        _ => None,
    });
    assert!(matches!(
        partial,
        Err(immich_rs::Error::Validation(ref msg))
            if msg.contains("both IMMICH_CLIENT_CERT_PATH and IMMICH_CLIENT_KEY_PATH must be set")
    ));

    // Whitespace-only IMMICH_CLIENT_CERT_PATH with valid IMMICH_CLIENT_KEY_PATH must also fail
    let whitespace_cert = ImmichClient::builder().from_env_map(|k| match k {
        "IMMICH_SERVER_URL" => Some("  https://immich.example.com/api  ".to_string()),
        "IMMICH_CLIENT_CERT_PATH" => Some("   ".to_string()),
        "IMMICH_CLIENT_KEY_PATH" => Some(key_str.clone()),
        _ => None,
    });
    assert!(matches!(
        whitespace_cert,
        Err(immich_rs::Error::Validation(ref msg))
            if msg.contains("both IMMICH_CLIENT_CERT_PATH and IMMICH_CLIENT_KEY_PATH must be set")
    ));

    // Explicit builder TLS settings take precedence over env vars (even invalid env paths)
    let explicit_wins = ImmichClient::builder()
        .client_identity_pem(TEST_CLIENT_CERT_PEM, TEST_CLIENT_KEY_PEM)
        .add_root_certificate_pem(TEST_CA_PEM)
        .from_env_map(|k| match k {
            "IMMICH_INSTANCE_URL" => Some("https://immich.example.com/api".to_string()),
            "IMMICH_CLIENT_CERT_PATH" => Some("/nonexistent/cert.pem".to_string()),
            "IMMICH_CLIENT_KEY_PATH" => Some("/nonexistent/key.pem".to_string()),
            "IMMICH_CA_CERT_PATH" => Some("/nonexistent/ca.pem".to_string()),
            _ => None,
        })
        .expect("explicit TLS settings should bypass env paths")
        .build()
        .expect("build succeeds with explicit TLS settings");
    assert_eq!(explicit_wins.base_url().as_str(), "https://immich.example.com/api/");
}

#[cfg(feature = "fs")]
#[test]
fn builder_add_root_certificate_from_file_supports_binary_der_cer_and_crt_and_rejects_missing() {
    use rustls_pki_types::pem::PemObject;

    let der_bytes = rustls_pki_types::CertificateDer::from_pem_slice(TEST_CA_PEM)
        .expect("parse CA PEM to DER")
        .as_ref()
        .to_vec();

    let dir = tempfile::tempdir().expect("create tempdir");
    let der_path = dir.path().join("ca.der");
    let cer_path = dir.path().join("ca.cer");
    let crt_der_path = dir.path().join("ca.crt");
    std::fs::write(&der_path, &der_bytes).expect("write ca.der");
    std::fs::write(&cer_path, &der_bytes).expect("write ca.cer");
    std::fs::write(&crt_der_path, &der_bytes).expect("write binary ca.crt");

    let client = ImmichClient::builder()
        .base_url("https://immich.example.com/api")
        .api_key("test-key")
        .add_root_certificate_from_file(&der_path)
        .add_root_certificate_from_file(&cer_path)
        .add_root_certificate_from_file(&crt_der_path)
        .build()
        .expect("build client from binary DER/CER/CRT CA files");

    assert_eq!(client.base_url().as_str(), "https://immich.example.com/api/");

    let missing_ca = ImmichClient::builder()
        .base_url("https://immich.example.com/api")
        .api_key("test-key")
        .add_root_certificate_from_file(dir.path().join("missing-ca.pem"))
        .build();
    assert!(matches!(
        missing_ca,
        Err(immich_rs::Error::Validation(ref msg)) if msg.contains("failed to read CA certificate")
    ));
}

#[tokio::test]
async fn mtls_handshake_requires_client_cert_and_custom_ca_and_succeeds_with_both() -> anyhow::Result<()> {
    use std::sync::Arc;

    use rustls_pki_types::pem::PemObject;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use tokio_rustls::TlsAcceptor;

    let ca_der = rustls_pki_types::CertificateDer::from_pem_slice(TEST_CA_PEM)?;
    let server_cert_der = rustls_pki_types::CertificateDer::from_pem_slice(TEST_SERVER_CERT_PEM)?;
    let server_key_der = rustls_pki_types::PrivateKeyDer::from_pem_slice(TEST_SERVER_KEY_PEM)?;

    let mut client_roots = rustls::RootCertStore::empty();
    client_roots.add(ca_der)?;

    let provider = Arc::new(rustls::crypto::ring::default_provider());
    let client_verifier =
        rustls::server::WebPkiClientVerifier::builder_with_provider(Arc::new(client_roots), Arc::clone(&provider))
            .build()?;

    let server_config = rustls::ServerConfig::builder_with_provider(provider)
        .with_safe_default_protocol_versions()?
        .with_client_cert_verifier(client_verifier)
        .with_single_cert(vec![server_cert_der], server_key_der)?;

    let acceptor = TlsAcceptor::from(Arc::new(server_config));
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;

    let server_task = tokio::spawn(async move {
        loop {
            let Ok((stream, _)) = listener.accept().await else {
                break;
            };
            let acceptor = acceptor.clone();
            tokio::spawn(async move {
                let Ok(mut tls_stream) = acceptor.accept(stream).await else {
                    return;
                };
                let mut buf = [0_u8; 2048];
                let _ = tls_stream.read(&mut buf).await;
                let body = r#"{"res":"pong"}"#;
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                let _ = tls_stream.write_all(response.as_bytes()).await;
                let _ = tls_stream.shutdown().await;
            });
        }
    });

    let base_url = format!("https://127.0.0.1:{}/api", addr.port());

    // 1. Without custom CA: TLS handshake must fail
    let no_ca_client = ImmichClient::builder()
        .base_url(&base_url)
        .api_key("test-key")
        .max_retries(0)
        .build()?;
    let no_ca_res = no_ca_client.server().ping_server().await;
    assert!(
        matches!(no_ca_res, Err(immich_rs::Error::Transport(_))),
        "expected Transport error without custom CA, got {no_ca_res:?}"
    );

    // 2. With custom CA, without client certificate: mTLS handshake must fail
    let ca_only_client = ImmichClient::builder()
        .base_url(&base_url)
        .api_key("test-key")
        .add_root_certificate_pem(TEST_CA_PEM)
        .max_retries(0)
        .build()?;
    let ca_only_res = ca_only_client.server().ping_server().await;
    assert!(
        matches!(ca_only_res, Err(immich_rs::Error::Transport(_))),
        "expected Transport error without client mTLS cert, got {ca_only_res:?}"
    );

    // 3. With custom CA + client_identity_pem: full mTLS request succeeds
    let mtls_client = ImmichClient::builder()
        .base_url(&base_url)
        .api_key("test-key")
        .add_root_certificate_pem(TEST_CA_PEM)
        .client_identity_pem(TEST_CLIENT_CERT_PEM, TEST_CLIENT_KEY_PEM)
        .max_retries(0)
        .build()?;
    let ping = mtls_client.server().ping_server().await?;
    assert_eq!(ping.res, "pong");

    // 4. With custom CA + client_identity_pem_bundle: full mTLS request succeeds
    let mut bundle = TEST_CLIENT_CERT_PEM.to_vec();
    bundle.push(b'\n');
    bundle.extend_from_slice(TEST_CLIENT_KEY_PEM);
    let mtls_bundle_client = ImmichClient::builder()
        .base_url(&base_url)
        .api_key("test-key")
        .add_root_certificate_pem(TEST_CA_PEM)
        .client_identity_pem_bundle(&bundle)
        .max_retries(0)
        .build()?;
    let ping_bundle = mtls_bundle_client.server().ping_server().await?;
    assert_eq!(ping_bundle.res, "pong");

    server_task.abort();
    Ok(())
}
