// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! `OpenAPI` data models for `authentication`.

pub use crate::activities::UserAvatarColor;
pub use crate::id::{ClusterGroupId, UserId};
pub use crate::server::LicenseResponseDTO;

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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
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
    /// Has password set (required)
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
    /// Creates a new [`AuthStatusResponseDTO`].
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
    /// Sets `expires_at`.
    #[must_use]
    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }
    /// Sets or clears `expires_at`.
    #[must_use]
    pub fn set_expires_at(mut self, value: Option<String>) -> Self {
        self.expires_at = value;
        self
    }
    /// Sets `is_elevated`.
    #[must_use]
    pub fn is_elevated(mut self, value: bool) -> Self {
        self.is_elevated = value;
        self
    }
    /// Sets `password`.
    #[must_use]
    pub fn password(mut self, value: bool) -> Self {
        self.password = value;
        self
    }
    /// Sets `pin_code`.
    #[must_use]
    pub fn pin_code(mut self, value: bool) -> Self {
        self.pin_code = value;
        self
    }
    /// Sets `pin_expires_at`.
    #[must_use]
    pub fn pin_expires_at(mut self, value: impl Into<String>) -> Self {
        self.pin_expires_at = Some(value.into());
        self
    }
    /// Sets or clears `pin_expires_at`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct ChangePasswordDTO {
    /// Invalidate all other sessions
    #[serde(rename = "invalidateSessions", default)]
    pub invalidate_sessions: bool,
    /// New password (min 8 characters) (required)
    #[serde(rename = "newPassword")]
    pub new_password: String,
    /// Current password (required)
    pub password: String,
}

impl ChangePasswordDTO {
    /// Creates a new [`ChangePasswordDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(new_password: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            invalidate_sessions: <bool>::default(),
            new_password: new_password.into(),
            password: password.into(),
        }
    }
    /// Sets `invalidate_sessions`.
    #[must_use]
    pub fn invalidate_sessions(mut self, value: bool) -> Self {
        self.invalidate_sessions = value;
        self
    }
    /// Sets `new_password`.
    #[must_use]
    pub fn new_password(mut self, value: impl Into<String>) -> Self {
        self.new_password = value.into();
        self
    }
    /// Sets `password`.
    #[must_use]
    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.password = value.into();
        self
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
/// | `email` | [`String`] | ✓ | User email (pattern: `` ^[\p{L}\p{M}\p{N}.!#$%&'*+/=?^_`{\|}~-]+@[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?(?:\.[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?)*$ ``) |
/// | `password` | [`String`] | ✓ | User password |
///
/// See: <https://api.immich.app/models/LoginCredentialDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct LoginCredentialDTO {
    /// User email (pattern: `` ^[\p{L}\p{M}\p{N}.!#$%&'*+/=?^_`{|}~-]+@[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?(?:\.[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?)*$ ``) (required)
    pub email: String,
    /// User password (required)
    pub password: String,
}

impl LoginCredentialDTO {
    /// Creates a new [`LoginCredentialDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(email: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            email: email.into(),
            password: password.into(),
        }
    }
    /// Sets `email`.
    #[must_use]
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = value.into();
        self
    }
    /// Sets `password`.
    #[must_use]
    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.password = value.into();
        self
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
/// | `user_email` | [`String`] | ✓ | User email (pattern: `` ^[\p{L}\p{M}\p{N}.!#$%&'*+/=?^_`{\|}~-]+@[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?(?:\.[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?)*$ ``) |
/// | `user_id` | [`UserId`](crate::id::UserId) | ✓ | User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/LoginResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
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
    /// User name (required)
    pub name: String,
    /// Profile image path (required)
    #[serde(rename = "profileImagePath")]
    pub profile_image_path: String,
    /// Should change password (required)
    #[serde(rename = "shouldChangePassword")]
    pub should_change_password: bool,
    /// User email (pattern: `` ^[\p{L}\p{M}\p{N}.!#$%&'*+/=?^_`{|}~-]+@[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?(?:\.[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?)*$ ``) (required)
    #[serde(rename = "userEmail")]
    pub user_email: String,
    /// User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "userId")]
    pub user_id: UserId,
}

impl LoginResponseDTO {
    /// Creates a new [`LoginResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        access_token: impl Into<String>,
        is_admin: bool,
        is_onboarded: bool,
        name: impl Into<String>,
        profile_image_path: impl Into<String>,
        should_change_password: bool,
        user_email: impl Into<String>,
        user_id: impl Into<UserId>,
    ) -> Self {
        Self {
            access_token: access_token.into(),
            is_admin,
            is_onboarded,
            name: name.into(),
            profile_image_path: profile_image_path.into(),
            should_change_password,
            user_email: user_email.into(),
            user_id: user_id.into(),
        }
    }
    /// Sets `access_token`.
    #[must_use]
    pub fn access_token(mut self, value: impl Into<String>) -> Self {
        self.access_token = value.into();
        self
    }
    /// Sets `is_admin`.
    #[must_use]
    pub fn is_admin(mut self, value: bool) -> Self {
        self.is_admin = value;
        self
    }
    /// Sets `is_onboarded`.
    #[must_use]
    pub fn is_onboarded(mut self, value: bool) -> Self {
        self.is_onboarded = value;
        self
    }
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }
    /// Sets `profile_image_path`.
    #[must_use]
    pub fn profile_image_path(mut self, value: impl Into<String>) -> Self {
        self.profile_image_path = value.into();
        self
    }
    /// Sets `should_change_password`.
    #[must_use]
    pub fn should_change_password(mut self, value: bool) -> Self {
        self.should_change_password = value;
        self
    }
    /// Sets `user_email`.
    #[must_use]
    pub fn user_email(mut self, value: impl Into<String>) -> Self {
        self.user_email = value.into();
        self
    }
    /// Sets `user_id`.
    #[must_use]
    pub fn user_id(mut self, value: impl Into<UserId>) -> Self {
        self.user_id = value.into();
        self
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct LogoutResponseDTO {
    /// Redirect URI (required)
    #[serde(rename = "redirectUri")]
    pub redirect_uri: String,
    /// Logout successful (required)
    pub successful: bool,
}

impl LogoutResponseDTO {
    /// Creates a new [`LogoutResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(redirect_uri: impl Into<String>, successful: bool) -> Self {
        Self {
            redirect_uri: redirect_uri.into(),
            successful,
        }
    }
    /// Sets `redirect_uri`.
    #[must_use]
    pub fn redirect_uri(mut self, value: impl Into<String>) -> Self {
        self.redirect_uri = value.into();
        self
    }
    /// Sets `successful`.
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
/// | `url` | [`String`] | ✓ | `OAuth` authorization URL |
///
/// See: <https://api.immich.app/models/OAuthAuthorizeResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct OAuthAuthorizeResponseDTO {
    /// `OAuth` authorization URL (required)
    pub url: String,
}

impl OAuthAuthorizeResponseDTO {
    /// Creates a new [`OAuthAuthorizeResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }
    /// Sets `url`.
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
/// | `logout_token` | [`String`] | ✓ | `OAuth` logout token |
///
/// See: <https://api.immich.app/models/OAuthBackchannelLogoutDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct OAuthBackchannelLogoutDTO {
    /// `OAuth` logout token (required)
    pub logout_token: String,
}

impl OAuthBackchannelLogoutDTO {
    /// Creates a new [`OAuthBackchannelLogoutDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(logout_token: impl Into<String>) -> Self {
        Self {
            logout_token: logout_token.into(),
        }
    }
    /// Sets `logout_token`.
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
/// | `code_verifier` | [`String`] |  | `OAuth` code verifier (PKCE) |
/// | `state` | [`String`] |  | `OAuth` state parameter |
/// | `url` | [`String`] | ✓ | `OAuth` callback URL |
///
/// See: <https://api.immich.app/models/OAuthCallbackDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct OAuthCallbackDTO {
    /// `OAuth` code verifier (PKCE)
    #[serde(
        rename = "codeVerifier",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub code_verifier: Option<String>,
    /// `OAuth` state parameter
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub state: Option<String>,
    /// `OAuth` callback URL (required)
    pub url: String,
}

impl OAuthCallbackDTO {
    /// Creates a new [`OAuthCallbackDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            code_verifier: None,
            state: None,
            url: url.into(),
        }
    }
    /// Sets `code_verifier`.
    #[must_use]
    pub fn code_verifier(mut self, value: impl Into<String>) -> Self {
        self.code_verifier = Some(value.into());
        self
    }
    /// Sets or clears `code_verifier`.
    #[must_use]
    pub fn set_code_verifier(mut self, value: Option<String>) -> Self {
        self.code_verifier = value;
        self
    }
    /// Sets `state`.
    #[must_use]
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }
    /// Sets or clears `state`.
    #[must_use]
    pub fn set_state(mut self, value: Option<String>) -> Self {
        self.state = value;
        self
    }
    /// Sets `url`.
    #[must_use]
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = value.into();
        self
    }
}

/// `OAuthConfigDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `code_challenge` | [`String`] |  | `OAuth` code challenge (PKCE) |
/// | `redirect_uri` | [`String`] | ✓ | `OAuth` redirect URI |
/// | `state` | [`String`] |  | `OAuth` state parameter |
///
/// See: <https://api.immich.app/models/OAuthConfigDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct OAuthConfigDTO {
    /// `OAuth` code challenge (PKCE)
    #[serde(
        rename = "codeChallenge",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub code_challenge: Option<String>,
    /// `OAuth` redirect URI (required)
    #[serde(rename = "redirectUri")]
    pub redirect_uri: String,
    /// `OAuth` state parameter
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub state: Option<String>,
}

impl OAuthConfigDTO {
    /// Creates a new [`OAuthConfigDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(redirect_uri: impl Into<String>) -> Self {
        Self {
            code_challenge: None,
            redirect_uri: redirect_uri.into(),
            state: None,
        }
    }
    /// Sets `code_challenge`.
    #[must_use]
    pub fn code_challenge(mut self, value: impl Into<String>) -> Self {
        self.code_challenge = Some(value.into());
        self
    }
    /// Sets or clears `code_challenge`.
    #[must_use]
    pub fn set_code_challenge(mut self, value: Option<String>) -> Self {
        self.code_challenge = value;
        self
    }
    /// Sets `redirect_uri`.
    #[must_use]
    pub fn redirect_uri(mut self, value: impl Into<String>) -> Self {
        self.redirect_uri = value.into();
        self
    }
    /// Sets `state`.
    #[must_use]
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }
    /// Sets or clears `state`.
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
/// | `new_pin_code` | [`String`] | ✓ | New PIN code (4-6 digits) (pattern: `^\d{6}$`) |
/// | `password` | [`String`] |  | User password (required if PIN code is not provided) |
/// | `pin_code` | [`String`] |  | New PIN code (4-6 digits) (pattern: `^\d{6}$`) |
///
/// See: <https://api.immich.app/models/PinCodeChangeDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct PinCodeChangeDTO {
    /// New PIN code (4-6 digits) (pattern: `^\d{6}$`) (required)
    #[serde(rename = "newPinCode")]
    pub new_pin_code: String,
    /// User password (required if PIN code is not provided)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub password: Option<String>,
    /// New PIN code (4-6 digits) (pattern: `^\d{6}$`)
    #[serde(rename = "pinCode", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pin_code: Option<String>,
}

impl PinCodeChangeDTO {
    /// Creates a new [`PinCodeChangeDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(new_pin_code: impl Into<String>) -> Self {
        Self {
            new_pin_code: new_pin_code.into(),
            password: None,
            pin_code: None,
        }
    }
    /// Sets `new_pin_code`.
    #[must_use]
    pub fn new_pin_code(mut self, value: impl Into<String>) -> Self {
        self.new_pin_code = value.into();
        self
    }
    /// Sets `password`.
    #[must_use]
    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.password = Some(value.into());
        self
    }
    /// Sets or clears `password`.
    #[must_use]
    pub fn set_password(mut self, value: Option<String>) -> Self {
        self.password = value;
        self
    }
    /// Sets `pin_code`.
    #[must_use]
    pub fn pin_code(mut self, value: impl Into<String>) -> Self {
        self.pin_code = Some(value.into());
        self
    }
    /// Sets or clears `pin_code`.
    #[must_use]
    pub fn set_pin_code(mut self, value: Option<String>) -> Self {
        self.pin_code = value;
        self
    }
}

/// `PinCodeResetDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `password` | [`String`] |  | User password (required if PIN code is not provided) |
/// | `pin_code` | [`String`] |  | New PIN code (4-6 digits) (pattern: `^\d{6}$`) |
///
/// See: <https://api.immich.app/models/PinCodeResetDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct PinCodeResetDTO {
    /// User password (required if PIN code is not provided)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub password: Option<String>,
    /// New PIN code (4-6 digits) (pattern: `^\d{6}$`)
    #[serde(rename = "pinCode", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pin_code: Option<String>,
}

impl PinCodeResetDTO {
    /// Creates a new [`PinCodeResetDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            password: None,
            pin_code: None,
        }
    }
    /// Sets `password`.
    #[must_use]
    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.password = Some(value.into());
        self
    }
    /// Sets or clears `password`.
    #[must_use]
    pub fn set_password(mut self, value: Option<String>) -> Self {
        self.password = value;
        self
    }
    /// Sets `pin_code`.
    #[must_use]
    pub fn pin_code(mut self, value: impl Into<String>) -> Self {
        self.pin_code = Some(value.into());
        self
    }
    /// Sets or clears `pin_code`.
    #[must_use]
    pub fn set_pin_code(mut self, value: Option<String>) -> Self {
        self.pin_code = value;
        self
    }
}

/// `PinCodeSetupDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `pin_code` | [`String`] | ✓ | PIN code (4-6 digits) (pattern: `^\d{6}$`) |
///
/// See: <https://api.immich.app/models/PinCodeSetupDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct PinCodeSetupDTO {
    /// PIN code (4-6 digits) (pattern: `^\d{6}$`) (required)
    #[serde(rename = "pinCode")]
    pub pin_code: String,
}

impl PinCodeSetupDTO {
    /// Creates a new [`PinCodeSetupDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(pin_code: impl Into<String>) -> Self {
        Self {
            pin_code: pin_code.into(),
        }
    }
    /// Sets `pin_code`.
    #[must_use]
    pub fn pin_code(mut self, value: impl Into<String>) -> Self {
        self.pin_code = value.into();
        self
    }
}

/// `SessionUnlockDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `password` | [`String`] |  | User password (required if PIN code is not provided) |
/// | `pin_code` | [`String`] |  | New PIN code (4-6 digits) (pattern: `^\d{6}$`) |
///
/// See: <https://api.immich.app/models/SessionUnlockDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct SessionUnlockDTO {
    /// User password (required if PIN code is not provided)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub password: Option<String>,
    /// New PIN code (4-6 digits) (pattern: `^\d{6}$`)
    #[serde(rename = "pinCode", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pin_code: Option<String>,
}

impl SessionUnlockDTO {
    /// Creates a new [`SessionUnlockDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            password: None,
            pin_code: None,
        }
    }
    /// Sets `password`.
    #[must_use]
    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.password = Some(value.into());
        self
    }
    /// Sets or clears `password`.
    #[must_use]
    pub fn set_password(mut self, value: Option<String>) -> Self {
        self.password = value;
        self
    }
    /// Sets `pin_code`.
    #[must_use]
    pub fn pin_code(mut self, value: impl Into<String>) -> Self {
        self.pin_code = Some(value.into());
        self
    }
    /// Sets or clears `pin_code`.
    #[must_use]
    pub fn set_pin_code(mut self, value: Option<String>) -> Self {
        self.pin_code = value;
        self
    }
}

/// `SignUpDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `email` | [`String`] | ✓ | User email (pattern: `` ^[\p{L}\p{M}\p{N}.!#$%&'*+/=?^_`{\|}~-]+@[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?(?:\.[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?)*$ ``) |
/// | `name` | [`String`] | ✓ | User name |
/// | `password` | [`String`] | ✓ | User password |
///
/// See: <https://api.immich.app/models/SignUpDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SignUpDTO {
    /// User email (pattern: `` ^[\p{L}\p{M}\p{N}.!#$%&'*+/=?^_`{|}~-]+@[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?(?:\.[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?)*$ ``) (required)
    pub email: String,
    /// User name (required)
    pub name: String,
    /// User password (required)
    pub password: String,
}

impl SignUpDTO {
    /// Creates a new [`SignUpDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(email: impl Into<String>, name: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            email: email.into(),
            name: name.into(),
            password: password.into(),
        }
    }
    /// Sets `email`.
    #[must_use]
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = value.into();
        self
    }
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }
    /// Sets `password`.
    #[must_use]
    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.password = value.into();
        self
    }
}

/// `UserAdminResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `avatar_color` | [`UserAvatarColor`] | ✓ |  |
/// | `cluster_group_id` | [`ClusterGroupId`](crate::id::ClusterGroupId) | ✓ | Cluster group the user is a member of (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Creation date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `deleted_at` | [`DateTime`](::chrono::DateTime) | ✓ | Deletion date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `email` | [`String`] | ✓ | User email (pattern: `` ^[\p{L}\p{M}\p{N}.!#$%&'*+/=?^_`{\|}~-]+@[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?(?:\.[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?)*$ ``) |
/// | `id` | [`UserId`](crate::id::UserId) | ✓ | User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `is_admin` | `bool` | ✓ | Is admin user |
/// | `license` | [`UserLicense`] | ✓ |  |
/// | `name` | [`String`] | ✓ | User name |
/// | `oauth_id` | [`String`] | ✓ | `OAuth` ID |
/// | `profile_changed_at` | [`DateTime`](::chrono::DateTime) | ✓ | Profile change date |
/// | `profile_image_path` | [`String`] | ✓ | Profile image path |
/// | `quota_size_in_bytes` | `i64` | ✓ | Storage quota in bytes |
/// | `quota_usage_in_bytes` | `i64` | ✓ | Storage usage in bytes |
/// | `should_change_password` | `bool` | ✓ | Require password change on next login |
/// | `status` | [`UserStatus`] | ✓ |  |
/// | `storage_label` | [`String`] | ✓ | Storage label |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Last update date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
///
/// See: <https://api.immich.app/models/UserAdminResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct UserAdminResponseDTO {
    /// (required)
    #[serde(rename = "avatarColor")]
    pub avatar_color: UserAvatarColor,
    /// Cluster group the user is a member of (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "clusterGroupId")]
    pub cluster_group_id: ClusterGroupId,
    /// Creation date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Deletion date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "deletedAt")]
    pub deleted_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// User email (pattern: `` ^[\p{L}\p{M}\p{N}.!#$%&'*+/=?^_`{|}~-]+@[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?(?:\.[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?)*$ ``) (required)
    pub email: String,
    /// User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: UserId,
    /// Is admin user (required)
    #[serde(rename = "isAdmin")]
    pub is_admin: bool,
    /// (required)
    pub license: Option<UserLicense>,
    /// User name (required)
    pub name: String,
    /// `OAuth` ID (required)
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
    pub quota_size_in_bytes: Option<i64>,
    /// Storage usage in bytes (required)
    #[serde(rename = "quotaUsageInBytes")]
    pub quota_usage_in_bytes: Option<i64>,
    /// Require password change on next login (required)
    #[serde(rename = "shouldChangePassword")]
    pub should_change_password: bool,
    /// (required)
    pub status: UserStatus,
    /// Storage label (required)
    #[serde(rename = "storageLabel")]
    pub storage_label: Option<String>,
    /// Last update date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl UserAdminResponseDTO {
    /// Creates a new [`UserAdminResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        avatar_color: UserAvatarColor,
        cluster_group_id: impl Into<ClusterGroupId>,
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        email: impl Into<String>,
        id: impl Into<UserId>,
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
            cluster_group_id: cluster_group_id.into(),
            created_at,
            deleted_at: None,
            email: email.into(),
            id: id.into(),
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
    /// Sets `avatar_color`.
    #[must_use]
    pub fn avatar_color(mut self, value: UserAvatarColor) -> Self {
        self.avatar_color = value;
        self
    }
    /// Sets `cluster_group_id`.
    #[must_use]
    pub fn cluster_group_id(mut self, value: impl Into<ClusterGroupId>) -> Self {
        self.cluster_group_id = value.into();
        self
    }
    /// Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_at = value;
        self
    }
    /// Sets `deleted_at`.
    #[must_use]
    pub fn deleted_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.deleted_at = Some(value);
        self
    }
    /// Sets or clears `deleted_at`.
    #[must_use]
    pub fn set_deleted_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.deleted_at = value;
        self
    }
    /// Sets `email`.
    #[must_use]
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = value.into();
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<UserId>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `is_admin`.
    #[must_use]
    pub fn is_admin(mut self, value: bool) -> Self {
        self.is_admin = value;
        self
    }
    /// Sets `license`.
    #[must_use]
    pub fn license(mut self, value: UserLicense) -> Self {
        self.license = Some(value);
        self
    }
    /// Sets or clears `license`.
    #[must_use]
    pub fn set_license(mut self, value: Option<UserLicense>) -> Self {
        self.license = value;
        self
    }
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }
    /// Sets `oauth_id`.
    #[must_use]
    pub fn oauth_id(mut self, value: impl Into<String>) -> Self {
        self.oauth_id = value.into();
        self
    }
    /// Sets `profile_changed_at`.
    #[must_use]
    pub fn profile_changed_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.profile_changed_at = value;
        self
    }
    /// Sets `profile_image_path`.
    #[must_use]
    pub fn profile_image_path(mut self, value: impl Into<String>) -> Self {
        self.profile_image_path = value.into();
        self
    }
    /// Sets `quota_size_in_bytes`.
    #[must_use]
    pub fn quota_size_in_bytes(mut self, value: i64) -> Self {
        self.quota_size_in_bytes = Some(value);
        self
    }
    /// Sets or clears `quota_size_in_bytes`.
    #[must_use]
    pub fn set_quota_size_in_bytes(mut self, value: Option<i64>) -> Self {
        self.quota_size_in_bytes = value;
        self
    }
    /// Sets `quota_usage_in_bytes`.
    #[must_use]
    pub fn quota_usage_in_bytes(mut self, value: i64) -> Self {
        self.quota_usage_in_bytes = Some(value);
        self
    }
    /// Sets or clears `quota_usage_in_bytes`.
    #[must_use]
    pub fn set_quota_usage_in_bytes(mut self, value: Option<i64>) -> Self {
        self.quota_usage_in_bytes = value;
        self
    }
    /// Sets `should_change_password`.
    #[must_use]
    pub fn should_change_password(mut self, value: bool) -> Self {
        self.should_change_password = value;
        self
    }
    /// Sets `status`.
    #[must_use]
    pub fn status(mut self, value: UserStatus) -> Self {
        self.status = value;
        self
    }
    /// Sets `storage_label`.
    #[must_use]
    pub fn storage_label(mut self, value: impl Into<String>) -> Self {
        self.storage_label = Some(value.into());
        self
    }
    /// Sets or clears `storage_label`.
    #[must_use]
    pub fn set_storage_label(mut self, value: Option<String>) -> Self {
        self.storage_label = value;
        self
    }
    /// Sets `updated_at`.
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
/// | `activated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Activation date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `activation_key` | [`String`] | ✓ | Activation key |
/// | `license_key` | [`String`] | ✓ | License key (format: /^IM(SV\|CL)(-\[\dA-Za-z\]{4}){8}$/) (pattern: `^IM(SV\|CL)(-[\dA-Za-z]{4}){8}$`) |
///
/// See: <https://api.immich.app/models/UserLicense>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct UserLicense {
    /// Activation date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "activatedAt")]
    pub activated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Activation key (required)
    #[serde(rename = "activationKey")]
    pub activation_key: String,
    /// License key (format: /^IM(SV|CL)(-\[\dA-Za-z\]{4}){8}$/) (pattern: `^IM(SV|CL)(-[\dA-Za-z]{4}){8}$`) (required)
    #[serde(rename = "licenseKey")]
    pub license_key: String,
}

impl UserLicense {
    /// Creates a new [`UserLicense`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        activated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        activation_key: impl Into<String>,
        license_key: impl Into<String>,
    ) -> Self {
        Self {
            activated_at,
            activation_key: activation_key.into(),
            license_key: license_key.into(),
        }
    }
    /// Sets `activated_at`.
    #[must_use]
    pub fn activated_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.activated_at = value;
        self
    }
    /// Sets `activation_key`.
    #[must_use]
    pub fn activation_key(mut self, value: impl Into<String>) -> Self {
        self.activation_key = value.into();
        self
    }
    /// Sets `license_key`.
    #[must_use]
    pub fn license_key(mut self, value: impl Into<String>) -> Self {
        self.license_key = value.into();
        self
    }
}

/// User status
///
/// See: <https://api.immich.app/models/UserStatus>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum UserStatus {
    Active,
    Removing,
    Deleted,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl UserStatus {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Active => "active",
            Self::Removing => "removing",
            Self::Deleted => "deleted",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for UserStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for UserStatus {
    fn from(value: &str) -> Self {
        match value {
            "active" => Self::Active,
            "removing" => Self::Removing,
            "deleted" => Self::Deleted,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for UserStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "active" => Self::Active,
            "removing" => Self::Removing,
            "deleted" => Self::Deleted,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for UserStatus {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for UserStatus {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for UserStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for UserStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = UserStatus;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(UserStatus::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(UserStatus::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// `ValidateAccessTokenResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `auth_status` | `bool` | ✓ | Authentication status |
///
/// See: <https://api.immich.app/models/ValidateAccessTokenResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct ValidateAccessTokenResponseDTO {
    /// Authentication status (required)
    #[serde(rename = "authStatus")]
    pub auth_status: bool,
}

impl ValidateAccessTokenResponseDTO {
    /// Creates a new [`ValidateAccessTokenResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(auth_status: bool) -> Self {
        Self { auth_status }
    }
    /// Sets `auth_status`.
    #[must_use]
    pub fn auth_status(mut self, value: bool) -> Self {
        self.auth_status = value;
        self
    }
}
