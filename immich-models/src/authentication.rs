// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! `OpenAPI` data models for `authentication`.

pub use crate::activities::UserAvatarColor;
pub use crate::server::LicenseResponseDTO;
pub use crate::sessions::SessionUnlockPinCode;
pub use crate::users::UserLicenseLicenseKey;
pub use crate::users::admin::UserAdminResponseEmail;

/// `AuthStatusResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `expires_at` | [`String`] |  | Session expiration date |
/// | `is_elevated` | `bool` | ✓ | Is elevated session |
/// | `password` | `bool` | ✓ | Has password set |
/// | `pin_code` | `bool` | ✓ | Has PIN code set |
/// | `pin_expires_at` | [`String`] |  | PIN expiration date |
///
/// See: <https://api.immich.app/models/AuthStatusResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AuthStatusResponseDTO {
    /// Session expiration date
    #[serde(
        rename = "expiresAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub expires_at: Option<String>,
    /// Is elevated session (required)
    #[serde(rename = "isElevated")]
    pub is_elevated: bool,
    pub password: bool,
    /// Has PIN code set (required)
    #[serde(rename = "pinCode")]
    pub pin_code: bool,
    /// PIN expiration date
    #[serde(
        rename = "pinExpiresAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub pin_expires_at: Option<String>,
}

impl AuthStatusResponseDTO {
    ///Creates a new [`AuthStatusResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(is_elevated: bool, password: bool, pin_code: bool) -> Self {
        Self {
            expires_at: None,
            is_elevated,
            password,
            pin_code,
            pin_expires_at: None,
        }
    }

    ///Sets `expires_at`.
    #[must_use]
    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    ///Sets or clears `expires_at`.
    #[must_use]
    pub fn set_expires_at(mut self, value: Option<String>) -> Self {
        self.expires_at = value;
        self
    }

    ///Sets `is_elevated`.
    #[must_use]
    pub fn is_elevated(mut self, value: bool) -> Self {
        self.is_elevated = value;
        self
    }

    ///Sets `password`.
    #[must_use]
    pub fn password(mut self, value: bool) -> Self {
        self.password = value;
        self
    }

    ///Sets `pin_code`.
    #[must_use]
    pub fn pin_code(mut self, value: bool) -> Self {
        self.pin_code = value;
        self
    }

    ///Sets `pin_expires_at`.
    #[must_use]
    pub fn pin_expires_at(mut self, value: impl Into<String>) -> Self {
        self.pin_expires_at = Some(value.into());
        self
    }

    ///Sets or clears `pin_expires_at`.
    #[must_use]
    pub fn set_pin_expires_at(mut self, value: Option<String>) -> Self {
        self.pin_expires_at = value;
        self
    }
}

/// `ChangePasswordDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `invalidate_sessions` | `bool` |  | Invalidate all other sessions |
/// | `new_password` | [`String`] | ✓ | New password (min 8 characters) |
/// | `password` | [`String`] | ✓ | Current password |
///
/// See: <https://api.immich.app/models/ChangePasswordDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct ChangePasswordDTO {
    /// Invalidate all other sessions
    #[serde(rename = "invalidateSessions", default)]
    pub invalidate_sessions: bool,
    /// New password (min 8 characters) (required)
    #[serde(rename = "newPassword")]
    pub new_password: ChangePasswordNewPassword,
    pub password: String,
}

impl ChangePasswordDTO {
    ///Creates a new [`ChangePasswordDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(new_password: ChangePasswordNewPassword, password: impl Into<String>) -> Self {
        Self {
            invalidate_sessions: <bool>::default(),
            new_password,
            password: password.into(),
        }
    }

    ///Sets `invalidate_sessions`.
    #[must_use]
    pub fn invalidate_sessions(mut self, value: bool) -> Self {
        self.invalidate_sessions = value;
        self
    }

    ///Sets `new_password`.
    #[must_use]
    pub fn new_password(mut self, value: ChangePasswordNewPassword) -> Self {
        self.new_password = value;
        self
    }

    ///Sets `password`.
    #[must_use]
    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.password = value.into();
        self
    }
}

#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ChangePasswordNewPassword(pub String);

impl ::std::ops::Deref for ChangePasswordNewPassword {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::str::FromStr for ChangePasswordNewPassword {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        if value.chars().count() < 8usize {
            return Err("shorter than 8 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for ChangePasswordNewPassword {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for ChangePasswordNewPassword {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for ChangePasswordNewPassword {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl<'de> ::serde::Deserialize<'de> for ChangePasswordNewPassword {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: crate::common::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}

impl From<LicenseResponseDTO> for UserLicense {
    fn from(value: LicenseResponseDTO) -> Self {
        value.0
    }
}

/// `LoginCredentialDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `email` | [`String`] | ✓ | User email |
/// | `password` | [`String`] | ✓ | User password |
///
/// See: <https://api.immich.app/models/LoginCredentialDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct LoginCredentialDTO {
    pub email: LoginCredentialEmail,
    pub password: String,
}

impl LoginCredentialDTO {
    ///Creates a new [`LoginCredentialDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(email: LoginCredentialEmail, password: impl Into<String>) -> Self {
        Self {
            email,
            password: password.into(),
        }
    }

    ///Sets `email`.
    #[must_use]
    pub fn email(mut self, value: LoginCredentialEmail) -> Self {
        self.email = value;
        self
    }

    ///Sets `password`.
    #[must_use]
    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.password = value.into();
        self
    }
}

#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LoginCredentialEmail(pub String);

impl ::std::ops::Deref for LoginCredentialEmail {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::str::FromStr for LoginCredentialEmail {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regex::Regex> = ::std::sync::LazyLock::new(|| {
            ::regex::Regex::new(
                    "^[\\p{L}\\p{M}\\p{N}.!#$%&'*+/=?^_`{|}~-]+@[\\p{L}\\p{N}](?:[\\p{L}\\p{M}\\p{N}-]{0,61}[\\p{L}\\p{M}\\p{N}])?(?:\\.[\\p{L}\\p{N}](?:[\\p{L}\\p{M}\\p{N}-]{0,61}[\\p{L}\\p{M}\\p{N}])?)*$",
                )
                .unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^[\\p{L}\\p{M}\\p{N}.!#$%&'*+/=?^_`{|}~-]+@[\\p{L}\\p{N}](?:[\\p{L}\\p{M}\\p{N}-]{0,61}[\\p{L}\\p{M}\\p{N}])?(?:\\.[\\p{L}\\p{N}](?:[\\p{L}\\p{M}\\p{N}-]{0,61}[\\p{L}\\p{M}\\p{N}])?)*$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for LoginCredentialEmail {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for LoginCredentialEmail {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for LoginCredentialEmail {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl<'de> ::serde::Deserialize<'de> for LoginCredentialEmail {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: crate::common::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}

/// `LoginResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `access_token` | [`String`] | ✓ | Access token |
/// | `is_admin` | `bool` | ✓ | Is admin user |
/// | `is_onboarded` | `bool` | ✓ | Is onboarded |
/// | `name` | [`String`] | ✓ | User name |
/// | `profile_image_path` | [`String`] | ✓ | Profile image path |
/// | `should_change_password` | `bool` | ✓ | Should change password |
/// | `user_email` | [`String`] | ✓ | User email |
/// | `user_id` | [`Uuid`](::uuid::Uuid) | ✓ | User ID |
///
/// See: <https://api.immich.app/models/LoginResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct LoginResponseDTO {
    /// Access token (required)
    #[serde(rename = "accessToken")]
    pub access_token: String,
    /// Is admin user (required)
    #[serde(rename = "isAdmin")]
    pub is_admin: bool,
    /// Is onboarded (required)
    #[serde(rename = "isOnboarded")]
    pub is_onboarded: bool,
    pub name: String,
    /// Profile image path (required)
    #[serde(rename = "profileImagePath")]
    pub profile_image_path: String,
    /// Should change password (required)
    #[serde(rename = "shouldChangePassword")]
    pub should_change_password: bool,
    /// User email (required)
    #[serde(rename = "userEmail")]
    pub user_email: LoginResponseUserEmail,
    /// User ID (required)
    #[serde(rename = "userId")]
    pub user_id: ::uuid::Uuid,
}

impl LoginResponseDTO {
    ///Creates a new [`LoginResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        access_token: impl Into<String>,
        is_admin: bool,
        is_onboarded: bool,
        name: impl Into<String>,
        profile_image_path: impl Into<String>,
        should_change_password: bool,
        user_email: LoginResponseUserEmail,
        user_id: ::uuid::Uuid,
    ) -> Self {
        Self {
            access_token: access_token.into(),
            is_admin,
            is_onboarded,
            name: name.into(),
            profile_image_path: profile_image_path.into(),
            should_change_password,
            user_email,
            user_id,
        }
    }

    ///Sets `access_token`.
    #[must_use]
    pub fn access_token(mut self, value: impl Into<String>) -> Self {
        self.access_token = value.into();
        self
    }

    ///Sets `is_admin`.
    #[must_use]
    pub fn is_admin(mut self, value: bool) -> Self {
        self.is_admin = value;
        self
    }

    ///Sets `is_onboarded`.
    #[must_use]
    pub fn is_onboarded(mut self, value: bool) -> Self {
        self.is_onboarded = value;
        self
    }

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }

    ///Sets `profile_image_path`.
    #[must_use]
    pub fn profile_image_path(mut self, value: impl Into<String>) -> Self {
        self.profile_image_path = value.into();
        self
    }

    ///Sets `should_change_password`.
    #[must_use]
    pub fn should_change_password(mut self, value: bool) -> Self {
        self.should_change_password = value;
        self
    }

    ///Sets `user_email`.
    #[must_use]
    pub fn user_email(mut self, value: LoginResponseUserEmail) -> Self {
        self.user_email = value;
        self
    }

    ///Sets `user_id`.
    #[must_use]
    pub fn user_id(mut self, value: ::uuid::Uuid) -> Self {
        self.user_id = value;
        self
    }
}

#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LoginResponseUserEmail(pub String);

impl ::std::ops::Deref for LoginResponseUserEmail {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::str::FromStr for LoginResponseUserEmail {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regex::Regex> = ::std::sync::LazyLock::new(|| {
            ::regex::Regex::new(
                    "^[\\p{L}\\p{M}\\p{N}.!#$%&'*+/=?^_`{|}~-]+@[\\p{L}\\p{N}](?:[\\p{L}\\p{M}\\p{N}-]{0,61}[\\p{L}\\p{M}\\p{N}])?(?:\\.[\\p{L}\\p{N}](?:[\\p{L}\\p{M}\\p{N}-]{0,61}[\\p{L}\\p{M}\\p{N}])?)*$",
                )
                .unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^[\\p{L}\\p{M}\\p{N}.!#$%&'*+/=?^_`{|}~-]+@[\\p{L}\\p{N}](?:[\\p{L}\\p{M}\\p{N}-]{0,61}[\\p{L}\\p{M}\\p{N}])?(?:\\.[\\p{L}\\p{N}](?:[\\p{L}\\p{M}\\p{N}-]{0,61}[\\p{L}\\p{M}\\p{N}])?)*$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for LoginResponseUserEmail {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for LoginResponseUserEmail {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for LoginResponseUserEmail {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl<'de> ::serde::Deserialize<'de> for LoginResponseUserEmail {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: crate::common::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}

/// `LogoutResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `redirect_uri` | [`String`] | ✓ | Redirect URI |
/// | `successful` | `bool` | ✓ | Logout successful |
///
/// See: <https://api.immich.app/models/LogoutResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct LogoutResponseDTO {
    /// Redirect URI (required)
    #[serde(rename = "redirectUri")]
    pub redirect_uri: String,
    pub successful: bool,
}

impl LogoutResponseDTO {
    ///Creates a new [`LogoutResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(redirect_uri: impl Into<String>, successful: bool) -> Self {
        Self {
            redirect_uri: redirect_uri.into(),
            successful,
        }
    }

    ///Sets `redirect_uri`.
    #[must_use]
    pub fn redirect_uri(mut self, value: impl Into<String>) -> Self {
        self.redirect_uri = value.into();
        self
    }

    ///Sets `successful`.
    #[must_use]
    pub fn successful(mut self, value: bool) -> Self {
        self.successful = value;
        self
    }
}

/// `OAuthAuthorizeResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `url` | [`String`] | ✓ | OAuth authorization URL |
///
/// See: <https://api.immich.app/models/OAuthAuthorizeResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct OAuthAuthorizeResponseDTO {
    pub url: String,
}

impl OAuthAuthorizeResponseDTO {
    ///Creates a new [`OAuthAuthorizeResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }

    ///Sets `url`.
    #[must_use]
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = value.into();
        self
    }
}

/// `OAuthBackchannelLogoutDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `logout_token` | [`String`] | ✓ | OAuth logout token |
///
/// See: <https://api.immich.app/models/OAuthBackchannelLogoutDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct OAuthBackchannelLogoutDTO {
    pub logout_token: String,
}

impl OAuthBackchannelLogoutDTO {
    ///Creates a new [`OAuthBackchannelLogoutDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(logout_token: impl Into<String>) -> Self {
        Self {
            logout_token: logout_token.into(),
        }
    }

    ///Sets `logout_token`.
    #[must_use]
    pub fn logout_token(mut self, value: impl Into<String>) -> Self {
        self.logout_token = value.into();
        self
    }
}

/// `OAuthCallbackDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `code_verifier` | [`String`] |  | OAuth code verifier (PKCE) |
/// | `state` | [`String`] |  | OAuth state parameter |
/// | `url` | [`String`] | ✓ | OAuth callback URL |
///
/// See: <https://api.immich.app/models/OAuthCallbackDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct OAuthCallbackDTO {
    /// OAuth code verifier (PKCE)
    #[serde(
        rename = "codeVerifier",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub code_verifier: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub state: Option<String>,
    pub url: OAuthCallbackUrl,
}

impl OAuthCallbackDTO {
    ///Creates a new [`OAuthCallbackDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(url: OAuthCallbackUrl) -> Self {
        Self {
            code_verifier: None,
            state: None,
            url,
        }
    }

    ///Sets `code_verifier`.
    #[must_use]
    pub fn code_verifier(mut self, value: impl Into<String>) -> Self {
        self.code_verifier = Some(value.into());
        self
    }

    ///Sets or clears `code_verifier`.
    #[must_use]
    pub fn set_code_verifier(mut self, value: Option<String>) -> Self {
        self.code_verifier = value;
        self
    }

    ///Sets `state`.
    #[must_use]
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    ///Sets or clears `state`.
    #[must_use]
    pub fn set_state(mut self, value: Option<String>) -> Self {
        self.state = value;
        self
    }

    ///Sets `url`.
    #[must_use]
    pub fn url(mut self, value: OAuthCallbackUrl) -> Self {
        self.url = value;
        self
    }
}

#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct OAuthCallbackUrl(pub String);

impl ::std::ops::Deref for OAuthCallbackUrl {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::str::FromStr for OAuthCallbackUrl {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        if value.is_empty() {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for OAuthCallbackUrl {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for OAuthCallbackUrl {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for OAuthCallbackUrl {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl<'de> ::serde::Deserialize<'de> for OAuthCallbackUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: crate::common::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}

/// `OAuthConfigDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `code_challenge` | [`String`] |  | OAuth code challenge (PKCE) |
/// | `redirect_uri` | [`String`] | ✓ | OAuth redirect URI |
/// | `state` | [`String`] |  | OAuth state parameter |
///
/// See: <https://api.immich.app/models/OAuthConfigDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct OAuthConfigDTO {
    /// OAuth code challenge (PKCE)
    #[serde(
        rename = "codeChallenge",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub code_challenge: Option<String>,
    /// OAuth redirect URI (required)
    #[serde(rename = "redirectUri")]
    pub redirect_uri: String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub state: Option<String>,
}

impl OAuthConfigDTO {
    ///Creates a new [`OAuthConfigDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(redirect_uri: impl Into<String>) -> Self {
        Self {
            code_challenge: None,
            redirect_uri: redirect_uri.into(),
            state: None,
        }
    }

    ///Sets `code_challenge`.
    #[must_use]
    pub fn code_challenge(mut self, value: impl Into<String>) -> Self {
        self.code_challenge = Some(value.into());
        self
    }

    ///Sets or clears `code_challenge`.
    #[must_use]
    pub fn set_code_challenge(mut self, value: Option<String>) -> Self {
        self.code_challenge = value;
        self
    }

    ///Sets `redirect_uri`.
    #[must_use]
    pub fn redirect_uri(mut self, value: impl Into<String>) -> Self {
        self.redirect_uri = value.into();
        self
    }

    ///Sets `state`.
    #[must_use]
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    ///Sets or clears `state`.
    #[must_use]
    pub fn set_state(mut self, value: Option<String>) -> Self {
        self.state = value;
        self
    }
}

/// `PinCodeChangeDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `new_pin_code` | [`String`] | ✓ | New PIN code (4-6 digits) |
/// | `password` | [`String`] |  | User password (required if PIN code is not provided) |
/// | `pin_code` | [`String`] |  | New PIN code (4-6 digits) |
///
/// See: <https://api.immich.app/models/PinCodeChangeDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PinCodeChangeDTO {
    /// New PIN code (4-6 digits) (required)
    #[serde(rename = "newPinCode")]
    pub new_pin_code: PinCodeChangeNewPinCode,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub password: Option<String>,
    /// New PIN code (4-6 digits)
    #[serde(rename = "pinCode", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pin_code: Option<PinCodeChangePinCode>,
}

impl PinCodeChangeDTO {
    ///Creates a new [`PinCodeChangeDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(new_pin_code: PinCodeChangeNewPinCode) -> Self {
        Self {
            new_pin_code,
            password: None,
            pin_code: None,
        }
    }

    ///Sets `new_pin_code`.
    #[must_use]
    pub fn new_pin_code(mut self, value: PinCodeChangeNewPinCode) -> Self {
        self.new_pin_code = value;
        self
    }

    ///Sets `password`.
    #[must_use]
    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.password = Some(value.into());
        self
    }

    ///Sets or clears `password`.
    #[must_use]
    pub fn set_password(mut self, value: Option<String>) -> Self {
        self.password = value;
        self
    }

    ///Sets `pin_code`.
    #[must_use]
    pub fn pin_code(mut self, value: PinCodeChangePinCode) -> Self {
        self.pin_code = Some(value);
        self
    }

    ///Sets or clears `pin_code`.
    #[must_use]
    pub fn set_pin_code(mut self, value: Option<PinCodeChangePinCode>) -> Self {
        self.pin_code = value;
        self
    }
}

#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct PinCodeChangeNewPinCode(pub String);

impl ::std::ops::Deref for PinCodeChangeNewPinCode {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::str::FromStr for PinCodeChangeNewPinCode {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regex::Regex> =
            ::std::sync::LazyLock::new(|| ::regex::Regex::new("^\\d{6}$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^\\d{6}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for PinCodeChangeNewPinCode {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for PinCodeChangeNewPinCode {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for PinCodeChangeNewPinCode {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl<'de> ::serde::Deserialize<'de> for PinCodeChangeNewPinCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: crate::common::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}

#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct PinCodeChangePinCode(pub String);

impl ::std::ops::Deref for PinCodeChangePinCode {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::str::FromStr for PinCodeChangePinCode {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regex::Regex> =
            ::std::sync::LazyLock::new(|| ::regex::Regex::new("^\\d{6}$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^\\d{6}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for PinCodeChangePinCode {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for PinCodeChangePinCode {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for PinCodeChangePinCode {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl<'de> ::serde::Deserialize<'de> for PinCodeChangePinCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: crate::common::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}

/// `PinCodeResetDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `password` | [`String`] |  | User password (required if PIN code is not provided) |
/// | `pin_code` | [`String`] |  | New PIN code (4-6 digits) |
///
/// See: <https://api.immich.app/models/PinCodeResetDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct PinCodeResetDTO {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub password: Option<String>,
    /// New PIN code (4-6 digits)
    #[serde(rename = "pinCode", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pin_code: Option<PinCodeResetPinCode>,
}

impl PinCodeResetDTO {
    ///Creates a new [`PinCodeResetDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            password: None,
            pin_code: None,
        }
    }

    ///Sets `password`.
    #[must_use]
    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.password = Some(value.into());
        self
    }

    ///Sets or clears `password`.
    #[must_use]
    pub fn set_password(mut self, value: Option<String>) -> Self {
        self.password = value;
        self
    }

    ///Sets `pin_code`.
    #[must_use]
    pub fn pin_code(mut self, value: PinCodeResetPinCode) -> Self {
        self.pin_code = Some(value);
        self
    }

    ///Sets or clears `pin_code`.
    #[must_use]
    pub fn set_pin_code(mut self, value: Option<PinCodeResetPinCode>) -> Self {
        self.pin_code = value;
        self
    }
}

#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct PinCodeResetPinCode(pub String);

impl ::std::ops::Deref for PinCodeResetPinCode {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::str::FromStr for PinCodeResetPinCode {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regex::Regex> =
            ::std::sync::LazyLock::new(|| ::regex::Regex::new("^\\d{6}$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^\\d{6}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for PinCodeResetPinCode {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for PinCodeResetPinCode {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for PinCodeResetPinCode {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl<'de> ::serde::Deserialize<'de> for PinCodeResetPinCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: crate::common::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}

/// `PinCodeSetupDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `pin_code` | [`String`] | ✓ | PIN code (4-6 digits) |
///
/// See: <https://api.immich.app/models/PinCodeSetupDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PinCodeSetupDTO {
    /// PIN code (4-6 digits) (required)
    #[serde(rename = "pinCode")]
    pub pin_code: PinCodeSetupPinCode,
}

impl PinCodeSetupDTO {
    ///Creates a new [`PinCodeSetupDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(pin_code: PinCodeSetupPinCode) -> Self {
        Self { pin_code }
    }

    ///Sets `pin_code`.
    #[must_use]
    pub fn pin_code(mut self, value: PinCodeSetupPinCode) -> Self {
        self.pin_code = value;
        self
    }
}

#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct PinCodeSetupPinCode(pub String);

impl ::std::ops::Deref for PinCodeSetupPinCode {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::str::FromStr for PinCodeSetupPinCode {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regex::Regex> =
            ::std::sync::LazyLock::new(|| ::regex::Regex::new("^\\d{6}$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^\\d{6}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for PinCodeSetupPinCode {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for PinCodeSetupPinCode {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for PinCodeSetupPinCode {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl<'de> ::serde::Deserialize<'de> for PinCodeSetupPinCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: crate::common::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}

/// `SessionUnlockDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `password` | [`String`] |  | User password (required if PIN code is not provided) |
/// | `pin_code` | [`String`] |  | New PIN code (4-6 digits) |
///
/// See: <https://api.immich.app/models/SessionUnlockDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct SessionUnlockDTO {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub password: Option<String>,
    /// New PIN code (4-6 digits)
    #[serde(rename = "pinCode", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pin_code: Option<SessionUnlockPinCode>,
}

impl SessionUnlockDTO {
    ///Creates a new [`SessionUnlockDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            password: None,
            pin_code: None,
        }
    }

    ///Sets `password`.
    #[must_use]
    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.password = Some(value.into());
        self
    }

    ///Sets or clears `password`.
    #[must_use]
    pub fn set_password(mut self, value: Option<String>) -> Self {
        self.password = value;
        self
    }

    ///Sets `pin_code`.
    #[must_use]
    pub fn pin_code(mut self, value: SessionUnlockPinCode) -> Self {
        self.pin_code = Some(value);
        self
    }

    ///Sets or clears `pin_code`.
    #[must_use]
    pub fn set_pin_code(mut self, value: Option<SessionUnlockPinCode>) -> Self {
        self.pin_code = value;
        self
    }
}

/// `SignUpDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `email` | [`String`] | ✓ | User email |
/// | `name` | [`String`] | ✓ | User name |
/// | `password` | [`String`] | ✓ | User password |
///
/// See: <https://api.immich.app/models/SignUpDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SignUpDTO {
    pub email: SignUpEmail,
    pub name: String,
    pub password: String,
}

impl SignUpDTO {
    ///Creates a new [`SignUpDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(email: SignUpEmail, name: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            email,
            name: name.into(),
            password: password.into(),
        }
    }

    ///Sets `email`.
    #[must_use]
    pub fn email(mut self, value: SignUpEmail) -> Self {
        self.email = value;
        self
    }

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }

    ///Sets `password`.
    #[must_use]
    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.password = value.into();
        self
    }
}

#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SignUpEmail(pub String);

impl ::std::ops::Deref for SignUpEmail {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::str::FromStr for SignUpEmail {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regex::Regex> = ::std::sync::LazyLock::new(|| {
            ::regex::Regex::new(
                    "^[\\p{L}\\p{M}\\p{N}.!#$%&'*+/=?^_`{|}~-]+@[\\p{L}\\p{N}](?:[\\p{L}\\p{M}\\p{N}-]{0,61}[\\p{L}\\p{M}\\p{N}])?(?:\\.[\\p{L}\\p{N}](?:[\\p{L}\\p{M}\\p{N}-]{0,61}[\\p{L}\\p{M}\\p{N}])?)*$",
                )
                .unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^[\\p{L}\\p{M}\\p{N}.!#$%&'*+/=?^_`{|}~-]+@[\\p{L}\\p{N}](?:[\\p{L}\\p{M}\\p{N}-]{0,61}[\\p{L}\\p{M}\\p{N}])?(?:\\.[\\p{L}\\p{N}](?:[\\p{L}\\p{M}\\p{N}-]{0,61}[\\p{L}\\p{M}\\p{N}])?)*$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for SignUpEmail {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for SignUpEmail {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for SignUpEmail {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl<'de> ::serde::Deserialize<'de> for SignUpEmail {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: crate::common::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}

/// `UserAdminResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `avatar_color` | [`UserAvatarColor`] | ✓ |  |
/// | `cluster_group_id` | [`Uuid`](::uuid::Uuid) | ✓ | Cluster group the user is a member of |
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Creation date |
/// | `deleted_at` | [`DateTime`](::chrono::DateTime) | ✓ | Deletion date |
/// | `email` | [`String`] | ✓ | User email |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | User ID |
/// | `is_admin` | `bool` | ✓ | Is admin user |
/// | `license` | [`UserLicense`] | ✓ |  |
/// | `name` | [`String`] | ✓ | User name |
/// | `oauth_id` | [`String`] | ✓ | OAuth ID |
/// | `profile_changed_at` | [`DateTime`](::chrono::DateTime) | ✓ | Profile change date |
/// | `profile_image_path` | [`String`] | ✓ | Profile image path |
/// | `quota_size_in_bytes` | `i64` | ✓ | Storage quota in bytes |
/// | `quota_usage_in_bytes` | `i64` | ✓ | Storage usage in bytes |
/// | `should_change_password` | `bool` | ✓ | Require password change on next login |
/// | `status` | [`UserStatus`] | ✓ |  |
/// | `storage_label` | [`String`] | ✓ | Storage label |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Last update date |
///
/// See: <https://api.immich.app/models/UserAdminResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct UserAdminResponseDTO {
    /// (required)
    #[serde(rename = "avatarColor")]
    pub avatar_color: UserAvatarColor,
    /// Cluster group the user is a member of (required)
    #[serde(rename = "clusterGroupId")]
    pub cluster_group_id: ::uuid::Uuid,
    /// Creation date (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Deletion date (required)
    #[serde(rename = "deletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub email: UserAdminResponseEmail,
    pub id: ::uuid::Uuid,
    /// Is admin user (required)
    #[serde(rename = "isAdmin")]
    pub is_admin: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<UserLicense>,
    pub name: String,
    /// OAuth ID (required)
    #[serde(rename = "oauthId")]
    pub oauth_id: String,
    /// Profile change date (required)
    #[serde(rename = "profileChangedAt")]
    pub profile_changed_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Profile image path (required)
    #[serde(rename = "profileImagePath")]
    pub profile_image_path: String,
    /// Storage quota in bytes (required)
    #[serde(rename = "quotaSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_size_in_bytes: Option<i64>,
    /// Storage usage in bytes (required)
    #[serde(rename = "quotaUsageInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_usage_in_bytes: Option<i64>,
    /// Require password change on next login (required)
    #[serde(rename = "shouldChangePassword")]
    pub should_change_password: bool,
    pub status: UserStatus,
    /// Storage label (required)
    #[serde(rename = "storageLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_label: Option<String>,
    /// Last update date (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl UserAdminResponseDTO {
    ///Creates a new [`UserAdminResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        avatar_color: UserAvatarColor,
        cluster_group_id: ::uuid::Uuid,
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        email: UserAdminResponseEmail,
        id: ::uuid::Uuid,
        is_admin: bool,
        name: impl Into<String>,
        oauth_id: impl Into<String>,
        profile_changed_at: ::chrono::DateTime<::chrono::offset::Utc>,
        profile_image_path: impl Into<String>,
        should_change_password: bool,
        status: UserStatus,
        updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ) -> Self {
        Self {
            avatar_color,
            cluster_group_id,
            created_at,
            deleted_at: None,
            email,
            id,
            is_admin,
            license: None,
            name: name.into(),
            oauth_id: oauth_id.into(),
            profile_changed_at,
            profile_image_path: profile_image_path.into(),
            quota_size_in_bytes: None,
            quota_usage_in_bytes: None,
            should_change_password,
            status,
            storage_label: None,
            updated_at,
        }
    }

    ///Sets `avatar_color`.
    #[must_use]
    pub fn avatar_color(mut self, value: UserAvatarColor) -> Self {
        self.avatar_color = value;
        self
    }

    ///Sets `cluster_group_id`.
    #[must_use]
    pub fn cluster_group_id(mut self, value: ::uuid::Uuid) -> Self {
        self.cluster_group_id = value;
        self
    }

    ///Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_at = value;
        self
    }

    ///Sets `deleted_at`.
    #[must_use]
    pub fn deleted_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    ///Sets or clears `deleted_at`.
    #[must_use]
    pub fn set_deleted_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.deleted_at = value;
        self
    }

    ///Sets `email`.
    #[must_use]
    pub fn email(mut self, value: UserAdminResponseEmail) -> Self {
        self.email = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `is_admin`.
    #[must_use]
    pub fn is_admin(mut self, value: bool) -> Self {
        self.is_admin = value;
        self
    }

    ///Sets `license`.
    #[must_use]
    pub fn license(mut self, value: UserLicense) -> Self {
        self.license = Some(value);
        self
    }

    ///Sets or clears `license`.
    #[must_use]
    pub fn set_license(mut self, value: Option<UserLicense>) -> Self {
        self.license = value;
        self
    }

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }

    ///Sets `oauth_id`.
    #[must_use]
    pub fn oauth_id(mut self, value: impl Into<String>) -> Self {
        self.oauth_id = value.into();
        self
    }

    ///Sets `profile_changed_at`.
    #[must_use]
    pub fn profile_changed_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.profile_changed_at = value;
        self
    }

    ///Sets `profile_image_path`.
    #[must_use]
    pub fn profile_image_path(mut self, value: impl Into<String>) -> Self {
        self.profile_image_path = value.into();
        self
    }

    ///Sets `quota_size_in_bytes`.
    #[must_use]
    pub fn quota_size_in_bytes(mut self, value: i64) -> Self {
        self.quota_size_in_bytes = Some(value);
        self
    }

    ///Sets or clears `quota_size_in_bytes`.
    #[must_use]
    pub fn set_quota_size_in_bytes(mut self, value: Option<i64>) -> Self {
        self.quota_size_in_bytes = value;
        self
    }

    ///Sets `quota_usage_in_bytes`.
    #[must_use]
    pub fn quota_usage_in_bytes(mut self, value: i64) -> Self {
        self.quota_usage_in_bytes = Some(value);
        self
    }

    ///Sets or clears `quota_usage_in_bytes`.
    #[must_use]
    pub fn set_quota_usage_in_bytes(mut self, value: Option<i64>) -> Self {
        self.quota_usage_in_bytes = value;
        self
    }

    ///Sets `should_change_password`.
    #[must_use]
    pub fn should_change_password(mut self, value: bool) -> Self {
        self.should_change_password = value;
        self
    }

    ///Sets `status`.
    #[must_use]
    pub fn status(mut self, value: UserStatus) -> Self {
        self.status = value;
        self
    }

    ///Sets `storage_label`.
    #[must_use]
    pub fn storage_label(mut self, value: impl Into<String>) -> Self {
        self.storage_label = Some(value.into());
        self
    }

    ///Sets or clears `storage_label`.
    #[must_use]
    pub fn set_storage_label(mut self, value: Option<String>) -> Self {
        self.storage_label = value;
        self
    }

    ///Sets `updated_at`.
    #[must_use]
    pub fn updated_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.updated_at = value;
        self
    }
}

/// `UserLicense`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `activated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Activation date |
/// | `activation_key` | [`String`] | ✓ | Activation key |
/// | `license_key` | [`String`] | ✓ | License key (format: /^IM(SV|CL)(-[\dA-Za-z]{4}){8}$/) |
///
/// See: <https://api.immich.app/models/UserLicense>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct UserLicense {
    /// Activation date (required)
    #[serde(rename = "activatedAt")]
    pub activated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Activation key (required)
    #[serde(rename = "activationKey")]
    pub activation_key: String,
    /// License key (format: /^IM(SV|CL)(-[\dA-Za-z]{4}){8}$/) (required)
    #[serde(rename = "licenseKey")]
    pub license_key: UserLicenseLicenseKey,
}

impl UserLicense {
    ///Creates a new [`UserLicense`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        activated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        activation_key: impl Into<String>,
        license_key: UserLicenseLicenseKey,
    ) -> Self {
        Self {
            activated_at,
            activation_key: activation_key.into(),
            license_key,
        }
    }

    ///Sets `activated_at`.
    #[must_use]
    pub fn activated_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.activated_at = value;
        self
    }

    ///Sets `activation_key`.
    #[must_use]
    pub fn activation_key(mut self, value: impl Into<String>) -> Self {
        self.activation_key = value.into();
        self
    }

    ///Sets `license_key`.
    #[must_use]
    pub fn license_key(mut self, value: UserLicenseLicenseKey) -> Self {
        self.license_key = value;
        self
    }
}

/// User status
///
/// See: <https://api.immich.app/models/UserStatus>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum UserStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "removing")]
    Removing,
    #[serde(rename = "deleted")]
    Deleted,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for UserStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Active => f.write_str("active"),
            Self::Removing => f.write_str("removing"),
            Self::Deleted => f.write_str("deleted"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for UserStatus {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "active" => Ok(Self::Active),
            "removing" => Ok(Self::Removing),
            "deleted" => Ok(Self::Deleted),
            _ => Err(format!("invalid value for {}: {}", "UserStatus", value).into()),
        }
    }
}

impl TryFrom<&str> for UserStatus {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for UserStatus {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for UserStatus {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// `ValidateAccessTokenResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `auth_status` | `bool` | ✓ | Authentication status |
///
/// See: <https://api.immich.app/models/ValidateAccessTokenResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct ValidateAccessTokenResponseDTO {
    /// Authentication status (required)
    #[serde(rename = "authStatus")]
    pub auth_status: bool,
}

impl ValidateAccessTokenResponseDTO {
    ///Creates a new [`ValidateAccessTokenResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(auth_status: bool) -> Self {
        Self { auth_status }
    }

    ///Sets `auth_status`.
    #[must_use]
    pub fn auth_status(mut self, value: bool) -> Self {
        self.auth_status = value;
        self
    }
}
