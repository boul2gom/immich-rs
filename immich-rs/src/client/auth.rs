use reqwest::header::HeaderValue;

#[derive(Clone, Default)]
pub(crate) enum Auth {
    ApiKey(HeaderValue),
    Bearer(HeaderValue),
    #[default]
    None,
}

impl Auth {
    pub(crate) fn from_api_key(key: &str) -> std::result::Result<Self, String> {
        let trimmed = key.trim();
        if trimmed.is_empty() {
            return Err("API key cannot be empty".into());
        }
        let mut val = HeaderValue::try_from(trimmed).map_err(|e| format!("invalid API key format: {e}"))?;
        val.set_sensitive(true);
        Ok(Self::ApiKey(val))
    }

    pub(crate) fn from_bearer_token(token: &str) -> std::result::Result<Self, String> {
        let trimmed = token.trim();
        if trimmed.is_empty() {
            return Err("Bearer token cannot be empty".into());
        }
        let raw_token = trimmed.strip_prefix("Bearer ").unwrap_or(trimmed);
        if raw_token.trim().is_empty() {
            return Err("Bearer token cannot be empty".into());
        }
        let mut val = HeaderValue::try_from(format!("Bearer {raw_token}"))
            .map_err(|e| format!("invalid Bearer token format: {e}"))?;
        val.set_sensitive(true);
        Ok(Self::Bearer(val))
    }
}

impl std::fmt::Debug for Auth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ApiKey(_) => f.write_str("ApiKey(<redacted>)"),
            Self::Bearer(_) => f.write_str("Bearer(<redacted>)"),
            Self::None => f.write_str("None"),
        }
    }
}
