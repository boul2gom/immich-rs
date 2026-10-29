//! Middleware layers and interceptors for `immich-rs`.
//!
//! Provides extensible request/response processing powered by Tower,
//! including client-side rate limiting and retries.

pub mod rate_limit;

pub use rate_limit::{RateLimitLayer, RateLimitService, RateLimiter};
