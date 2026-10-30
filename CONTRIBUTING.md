# Contributing to `immich-rs`

Thank you for your interest in contributing to `immich-rs`!

## Prerequisites

- **Rust stable** (latest, Rust 2024 edition)
- **Rust nightly** `rustfmt` (`rustup toolchain install nightly --component rustfmt`)
- **`wasm32-unknown-unknown` target** (`rustup target add wasm32-unknown-unknown`)
- **Docker** *(optional, only required to run the full end-to-end container suite)*

## Workspace Layout

- **`immich-models/`**: Generated `OpenAPI` DTOs, forward-compatible enums, and strongly typed domain IDs (`id.rs`).
- **`immich-rs/`**: Async HTTP SDK, Tower middleware (`RetryLayer`, `RateLimitLayer`), streaming uploads, atomic downloads, Socket.IO events, and runnable `examples/`.
- **`xtask/`**: `OpenAPI` AST code generator (`typify` + `syn` + `prettyplease`) keeping models and client endpoints synchronized with upstream Immich releases.

## Code Generation (`cargo xtask`)

Never edit `@generated` files (`immich-models/src/*` except `id.rs`, and `immich-rs/src/api/*`) by hand. Instead, update `xtask/` and regenerate:

```sh
# Regenerate models and client from the local or latest upstream Immich OpenAPI spec
cargo xtask generate

# Regenerate offline from open-api/immich-openapi-specs.json without network calls
cargo xtask generate --offline
```

## Quality Gates Before Opening a PR

```sh
# 1. Format with nightly rustfmt
cargo +nightly fmt --all -- --check

# 2. Lint all targets, examples, and features
cargo clippy --workspace --all-targets --all-features -- -D warnings

# 3. Run unit, integration, xtask snapshot, and README doctests
cargo test --workspace --all-features
cargo test --doc --workspace --all-features

# 4. Verify WebAssembly compilation
cargo check -p immich-models -p immich-rs --target wasm32-unknown-unknown --all-features

# 5. Check documentation links
RUSTDOCFLAGS="-D warnings --cfg docsrs" cargo +nightly doc --workspace --all-features --no-deps
```

### Running Docker E2E Tests (Optional)

```sh
cargo test -p immich-rs --test e2e_workflow_tests -- --ignored --nocapture
```

## License

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in `immich-rs` by you shall be dual-licensed under `MIT OR Apache-2.0`, without any additional terms or conditions.
