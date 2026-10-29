// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! `OpenAPI` data models for `common`.

pub use crate::albums::{AlbumUserRole, AssetOrder};
pub use crate::authentication::{
    ChangePasswordNewPassword, LoginCredentialEmail, LoginResponseUserEmail, OAuthCallbackUrl, PinCodeChangeNewPinCode,
    PinCodeChangePinCode, PinCodeResetPinCode, PinCodeSetupPinCode, SignUpEmail,
};
pub use crate::libraries::{CreateLibraryName, UpdateLibraryName};
pub use crate::partners::PartnerResponseEmail;
pub use crate::people::{
    PeopleUpdateItemColor, PeopleUserResponseEmail, PersonCreateColor, PersonUpdateColor, PersonUsersDeleteDTO,
    PersonUsersDeleteItem, PersonUsersResponseDTO, PersonUsersResponseItem,
};
pub use crate::search::{MetadataSearchCursor, SearchOrderField};
pub use crate::server::LicenseKeyLicenseKey;
pub use crate::sessions::SessionUnlockPinCode;
pub use crate::system::config::{AdminConfigNightlyTasksStartTime, HlsVideoResolution};
pub use crate::tags::{TagCreateColor, TagCreateName, TagUpdateColor, TagUpdateName};
pub use crate::users::admin::{
    UserAdminCreateEmail, UserAdminCreatePinCode, UserAdminResponseEmail, UserAdminUpdateEmail, UserAdminUpdatePinCode,
};
pub use crate::users::{UserLicenseLicenseKey, UserResponseEmail, UserUpdateMeEmail};

/// Error types.
pub mod error {
    #[derive(Clone, PartialEq, Eq)]
    pub struct ConversionError(pub ::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}

impl From<AdminConfigNightlyTasksStartTime> for String {
    fn from(value: AdminConfigNightlyTasksStartTime) -> Self {
        value.0
    }
}

impl From<ChangePasswordNewPassword> for String {
    fn from(value: ChangePasswordNewPassword) -> Self {
        value.0
    }
}

/// `ClusterGroupRequestCreateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `user_id` | [`Uuid`](::uuid::Uuid) | ✓ | User to invite into the cluster group |
///
/// See: <https://api.immich.app/models/ClusterGroupRequestCreateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct ClusterGroupRequestCreateDTO {
    /// User to invite into the cluster group (required)
    #[serde(rename = "userId")]
    pub user_id: ::uuid::Uuid,
}

impl ClusterGroupRequestCreateDTO {
    ///Creates a new [`ClusterGroupRequestCreateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(user_id: ::uuid::Uuid) -> Self {
        Self { user_id }
    }

    ///Sets `user_id`.
    #[must_use]
    pub fn user_id(mut self, value: ::uuid::Uuid) -> Self {
        self.user_id = value;
        self
    }
}

/// `ClusterGroupRequestResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `cluster_group_id` | [`Uuid`](::uuid::Uuid) | ✓ | Cluster group the user is invited to join |
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Creation date |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Request ID |
/// | `user_id` | [`Uuid`](::uuid::Uuid) | ✓ | User the request was created for |
///
/// See: <https://api.immich.app/models/ClusterGroupRequestResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct ClusterGroupRequestResponseDTO {
    /// Cluster group the user is invited to join (required)
    #[serde(rename = "clusterGroupId")]
    pub cluster_group_id: ::uuid::Uuid,
    /// Creation date (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub id: ::uuid::Uuid,
    /// User the request was created for (required)
    #[serde(rename = "userId")]
    pub user_id: ::uuid::Uuid,
}

impl ClusterGroupRequestResponseDTO {
    ///Creates a new [`ClusterGroupRequestResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        cluster_group_id: ::uuid::Uuid,
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        id: ::uuid::Uuid,
        user_id: ::uuid::Uuid,
    ) -> Self {
        Self {
            cluster_group_id,
            created_at,
            id,
            user_id,
        }
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

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `user_id`.
    #[must_use]
    pub fn user_id(mut self, value: ::uuid::Uuid) -> Self {
        self.user_id = value;
        self
    }
}

///CQ mode
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum CqMode {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "cqp")]
    Cqp,
    #[serde(rename = "icq")]
    Icq,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for CqMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Cqp => f.write_str("cqp"),
            Self::Icq => f.write_str("icq"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for CqMode {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "cqp" => Ok(Self::Cqp),
            "icq" => Ok(Self::Icq),
            _ => Err(format!("invalid value for {}: {}", "CqMode", value).into()),
        }
    }
}

impl TryFrom<&str> for CqMode {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for CqMode {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for CqMode {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl From<CreateLibraryName> for String {
    fn from(value: CreateLibraryName) -> Self {
        value.0
    }
}

impl From<HlsVideoResolution> for i64 {
    fn from(value: HlsVideoResolution) -> Self {
        value.0
    }
}

impl From<LicenseKeyLicenseKey> for String {
    fn from(value: LicenseKeyLicenseKey) -> Self {
        value.0
    }
}

impl From<LoginCredentialEmail> for String {
    fn from(value: LoginCredentialEmail) -> Self {
        value.0
    }
}

impl From<LoginResponseUserEmail> for String {
    fn from(value: LoginResponseUserEmail) -> Self {
        value.0
    }
}

impl From<MetadataSearchCursor> for String {
    fn from(value: MetadataSearchCursor) -> Self {
        value.0
    }
}

impl From<OAuthCallbackUrl> for String {
    fn from(value: OAuthCallbackUrl) -> Self {
        value.0
    }
}

impl From<PartnerResponseEmail> for String {
    fn from(value: PartnerResponseEmail) -> Self {
        value.0
    }
}

impl From<PeopleUpdateItemColor> for String {
    fn from(value: PeopleUpdateItemColor) -> Self {
        value.0
    }
}

impl From<PeopleUserResponseEmail> for String {
    fn from(value: PeopleUserResponseEmail) -> Self {
        value.0
    }
}

impl From<PersonCreateColor> for String {
    fn from(value: PersonCreateColor) -> Self {
        value.0
    }
}

impl From<PersonUpdateColor> for String {
    fn from(value: PersonUpdateColor) -> Self {
        value.0
    }
}

impl From<PersonUsersDeleteDTO> for Vec<PersonUsersDeleteItem> {
    fn from(value: PersonUsersDeleteDTO) -> Self {
        value.0
    }
}

impl From<PersonUsersResponseDTO> for Vec<PersonUsersResponseItem> {
    fn from(value: PersonUsersResponseDTO) -> Self {
        value.0
    }
}

impl From<PinCodeChangeNewPinCode> for String {
    fn from(value: PinCodeChangeNewPinCode) -> Self {
        value.0
    }
}

impl From<PinCodeChangePinCode> for String {
    fn from(value: PinCodeChangePinCode) -> Self {
        value.0
    }
}

impl From<PinCodeResetPinCode> for String {
    fn from(value: PinCodeResetPinCode) -> Self {
        value.0
    }
}

impl From<PinCodeSetupPinCode> for String {
    fn from(value: PinCodeSetupPinCode) -> Self {
        value.0
    }
}

/// Configuration properties that are visible to everyone
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `oauth` | [`PublicConfigOAuthDTO`] | ✓ |  |
/// | `password_login` | [`PublicConfigPasswordLoginDTO`] | ✓ |  |
/// | `server` | [`PublicConfigServerDTO`] | ✓ |  |
/// | `theme` | [`PublicConfigThemeDTO`] | ✓ |  |
///
/// See: <https://api.immich.app/models/PublicConfigDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PublicConfigDTO {
    pub oauth: PublicConfigOAuthDTO,
    /// (required)
    #[serde(rename = "passwordLogin")]
    pub password_login: PublicConfigPasswordLoginDTO,
    pub server: PublicConfigServerDTO,
    pub theme: PublicConfigThemeDTO,
}

impl PublicConfigDTO {
    ///Creates a new [`PublicConfigDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        oauth: PublicConfigOAuthDTO,
        password_login: PublicConfigPasswordLoginDTO,
        server: PublicConfigServerDTO,
        theme: PublicConfigThemeDTO,
    ) -> Self {
        Self {
            oauth,
            password_login,
            server,
            theme,
        }
    }

    ///Sets `oauth`.
    #[must_use]
    pub fn oauth(mut self, value: PublicConfigOAuthDTO) -> Self {
        self.oauth = value;
        self
    }

    ///Sets `password_login`.
    #[must_use]
    pub fn password_login(mut self, value: PublicConfigPasswordLoginDTO) -> Self {
        self.password_login = value;
        self
    }

    ///Sets `server`.
    #[must_use]
    pub fn server(mut self, value: PublicConfigServerDTO) -> Self {
        self.server = value;
        self
    }

    ///Sets `theme`.
    #[must_use]
    pub fn theme(mut self, value: PublicConfigThemeDTO) -> Self {
        self.theme = value;
        self
    }
}

/// `PublicConfigOAuthDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `auto_launch` | `bool` | ✓ | Auto launch |
/// | `button_text` | [`String`] | ✓ | Button text |
/// | `enabled` | `bool` | ✓ | Enabled |
///
/// See: <https://api.immich.app/models/PublicConfigOAuthDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PublicConfigOAuthDTO {
    /// Auto launch (required)
    #[serde(rename = "autoLaunch")]
    pub auto_launch: bool,
    /// Button text (required)
    #[serde(rename = "buttonText")]
    pub button_text: String,
    pub enabled: bool,
}

impl PublicConfigOAuthDTO {
    ///Creates a new [`PublicConfigOAuthDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(auto_launch: bool, button_text: impl Into<String>, enabled: bool) -> Self {
        Self {
            auto_launch,
            button_text: button_text.into(),
            enabled,
        }
    }

    ///Sets `auto_launch`.
    #[must_use]
    pub fn auto_launch(mut self, value: bool) -> Self {
        self.auto_launch = value;
        self
    }

    ///Sets `button_text`.
    #[must_use]
    pub fn button_text(mut self, value: impl Into<String>) -> Self {
        self.button_text = value.into();
        self
    }

    ///Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
}

/// `PublicConfigPasswordLoginDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` | ✓ | Enabled |
///
/// See: <https://api.immich.app/models/PublicConfigPasswordLoginDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PublicConfigPasswordLoginDTO {
    pub enabled: bool,
}

impl PublicConfigPasswordLoginDTO {
    ///Creates a new [`PublicConfigPasswordLoginDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    ///Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
}

/// `PublicConfigServerDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `login_page_message` | [`String`] | ✓ | Login page message |
///
/// See: <https://api.immich.app/models/PublicConfigServerDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PublicConfigServerDTO {
    /// Login page message (required)
    #[serde(rename = "loginPageMessage")]
    pub login_page_message: String,
}

impl PublicConfigServerDTO {
    ///Creates a new [`PublicConfigServerDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(login_page_message: impl Into<String>) -> Self {
        Self {
            login_page_message: login_page_message.into(),
        }
    }

    ///Sets `login_page_message`.
    #[must_use]
    pub fn login_page_message(mut self, value: impl Into<String>) -> Self {
        self.login_page_message = value.into();
        self
    }
}

/// `PublicConfigThemeDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `custom_css` | [`String`] | ✓ | Custom CSS for theming |
///
/// See: <https://api.immich.app/models/PublicConfigThemeDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PublicConfigThemeDTO {
    /// Custom CSS for theming (required)
    #[serde(rename = "customCss")]
    pub custom_css: String,
}

impl PublicConfigThemeDTO {
    ///Creates a new [`PublicConfigThemeDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(custom_css: impl Into<String>) -> Self {
        Self {
            custom_css: custom_css.into(),
        }
    }

    ///Sets `custom_css`.
    #[must_use]
    pub fn custom_css(mut self, value: impl Into<String>) -> Self {
        self.custom_css = value.into();
        self
    }
}

impl From<SessionUnlockPinCode> for String {
    fn from(value: SessionUnlockPinCode) -> Self {
        value.0
    }
}

impl From<SignUpEmail> for String {
    fn from(value: SignUpEmail) -> Self {
        value.0
    }
}

#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct StringPatternFilterEndsWith(pub String);

impl ::std::ops::Deref for StringPatternFilterEndsWith {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl From<StringPatternFilterEndsWith> for String {
    fn from(value: StringPatternFilterEndsWith) -> Self {
        value.0
    }
}

impl ::std::str::FromStr for StringPatternFilterEndsWith {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        if value.is_empty() {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for StringPatternFilterEndsWith {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for StringPatternFilterEndsWith {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for StringPatternFilterEndsWith {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl<'de> ::serde::Deserialize<'de> for StringPatternFilterEndsWith {
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
pub struct StringPatternFilterLike(pub String);

impl ::std::ops::Deref for StringPatternFilterLike {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl From<StringPatternFilterLike> for String {
    fn from(value: StringPatternFilterLike) -> Self {
        value.0
    }
}

impl ::std::str::FromStr for StringPatternFilterLike {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        if value.is_empty() {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for StringPatternFilterLike {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for StringPatternFilterLike {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for StringPatternFilterLike {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl<'de> ::serde::Deserialize<'de> for StringPatternFilterLike {
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
pub struct StringPatternFilterNotLike(pub String);

impl ::std::ops::Deref for StringPatternFilterNotLike {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl From<StringPatternFilterNotLike> for String {
    fn from(value: StringPatternFilterNotLike) -> Self {
        value.0
    }
}

impl ::std::str::FromStr for StringPatternFilterNotLike {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        if value.is_empty() {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for StringPatternFilterNotLike {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for StringPatternFilterNotLike {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for StringPatternFilterNotLike {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl<'de> ::serde::Deserialize<'de> for StringPatternFilterNotLike {
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
pub struct StringPatternFilterStartsWith(pub String);

impl ::std::ops::Deref for StringPatternFilterStartsWith {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl From<StringPatternFilterStartsWith> for String {
    fn from(value: StringPatternFilterStartsWith) -> Self {
        value.0
    }
}

impl ::std::str::FromStr for StringPatternFilterStartsWith {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        if value.is_empty() {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for StringPatternFilterStartsWith {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for StringPatternFilterStartsWith {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for StringPatternFilterStartsWith {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl<'de> ::serde::Deserialize<'de> for StringPatternFilterStartsWith {
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
pub struct StringSimilarityFilterMatches(pub String);

impl ::std::ops::Deref for StringSimilarityFilterMatches {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl From<StringSimilarityFilterMatches> for String {
    fn from(value: StringSimilarityFilterMatches) -> Self {
        value.0
    }
}

impl ::std::str::FromStr for StringSimilarityFilterMatches {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        if value.is_empty() {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for StringSimilarityFilterMatches {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for StringSimilarityFilterMatches {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for StringSimilarityFilterMatches {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl<'de> ::serde::Deserialize<'de> for StringSimilarityFilterMatches {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: crate::common::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}

impl From<TagCreateColor> for String {
    fn from(value: TagCreateColor) -> Self {
        value.0
    }
}

impl From<TagCreateName> for String {
    fn from(value: TagCreateName) -> Self {
        value.0
    }
}

impl From<TagUpdateColor> for String {
    fn from(value: TagUpdateColor) -> Self {
        value.0
    }
}

impl From<TagUpdateName> for String {
    fn from(value: TagUpdateName) -> Self {
        value.0
    }
}

///Transcode hardware acceleration
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum TranscodeHwAccel {
    #[serde(rename = "nvenc")]
    Nvenc,
    #[serde(rename = "qsv")]
    Qsv,
    #[serde(rename = "vaapi")]
    Vaapi,
    #[serde(rename = "rkmpp")]
    Rkmpp,
    #[serde(rename = "disabled")]
    Disabled,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for TranscodeHwAccel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Nvenc => f.write_str("nvenc"),
            Self::Qsv => f.write_str("qsv"),
            Self::Vaapi => f.write_str("vaapi"),
            Self::Rkmpp => f.write_str("rkmpp"),
            Self::Disabled => f.write_str("disabled"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for TranscodeHwAccel {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "nvenc" => Ok(Self::Nvenc),
            "qsv" => Ok(Self::Qsv),
            "vaapi" => Ok(Self::Vaapi),
            "rkmpp" => Ok(Self::Rkmpp),
            "disabled" => Ok(Self::Disabled),
            _ => Err(format!("invalid value for {}: {}", "TranscodeHwAccel", value).into()),
        }
    }
}

impl TryFrom<&str> for TranscodeHwAccel {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for TranscodeHwAccel {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for TranscodeHwAccel {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl From<UpdateLibraryName> for String {
    fn from(value: UpdateLibraryName) -> Self {
        value.0
    }
}

impl From<UserAdminCreateEmail> for String {
    fn from(value: UserAdminCreateEmail) -> Self {
        value.0
    }
}

impl From<UserAdminCreatePinCode> for String {
    fn from(value: UserAdminCreatePinCode) -> Self {
        value.0
    }
}

impl From<UserAdminResponseEmail> for String {
    fn from(value: UserAdminResponseEmail) -> Self {
        value.0
    }
}

impl From<UserAdminUpdateEmail> for String {
    fn from(value: UserAdminUpdateEmail) -> Self {
        value.0
    }
}

impl From<UserAdminUpdatePinCode> for String {
    fn from(value: UserAdminUpdatePinCode) -> Self {
        value.0
    }
}

impl From<UserLicenseLicenseKey> for String {
    fn from(value: UserLicenseLicenseKey) -> Self {
        value.0
    }
}

impl From<UserResponseEmail> for String {
    fn from(value: UserResponseEmail) -> Self {
        value.0
    }
}

impl From<UserUpdateMeEmail> for String {
    fn from(value: UserUpdateMeEmail) -> Self {
        value.0
    }
}

/// Generation of default values for serde.
#[allow(clippy::missing_panics_doc)]
pub mod defaults {
    #[must_use]
    pub fn default_bool<const V: bool>() -> bool {
        V
    }
    #[must_use]
    pub fn default_nzu64<T, const V: u64>() -> T
    where
        T: TryFrom<::std::num::NonZeroU64>,
        <T as ::std::convert::TryFrom<::std::num::NonZeroU64>>::Error: ::std::fmt::Debug,
    {
        T::try_from(::std::num::NonZeroU64::try_from(V).unwrap()).unwrap()
    }
    #[must_use]
    pub fn album_user_add_dto_role() -> super::AlbumUserRole {
        super::AlbumUserRole::Editor
    }
    #[must_use]
    pub fn search_order_direction() -> super::AssetOrder {
        super::AssetOrder::Desc
    }
    #[must_use]
    pub fn search_order_field() -> super::SearchOrderField {
        super::SearchOrderField::FileCreatedAt
    }
}
