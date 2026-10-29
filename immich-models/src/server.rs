// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! `OpenAPI` data models for `server`.

pub use crate::authentication::UserLicense;

/// `LicenseKeyDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `activation_key` | [`String`] | ✓ | Activation key |
/// | `license_key` | [`String`] | ✓ | License key (format: /^IM(SV|CL)(-[\dA-Za-z]{4}){8}$/) |
///
/// See: <https://api.immich.app/models/LicenseKeyDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct LicenseKeyDTO {
    /// Activation key (required)
    #[serde(rename = "activationKey")]
    pub activation_key: String,
    /// License key (format: /^IM(SV|CL)(-[\dA-Za-z]{4}){8}$/) (required)
    #[serde(rename = "licenseKey")]
    pub license_key: LicenseKeyLicenseKey,
}

impl LicenseKeyDTO {
    ///Creates a new [`LicenseKeyDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(activation_key: impl Into<String>, license_key: LicenseKeyLicenseKey) -> Self {
        Self {
            activation_key: activation_key.into(),
            license_key,
        }
    }

    ///Sets `activation_key`.
    #[must_use]
    pub fn activation_key(mut self, value: impl Into<String>) -> Self {
        self.activation_key = value.into();
        self
    }

    ///Sets `license_key`.
    #[must_use]
    pub fn license_key(mut self, value: LicenseKeyLicenseKey) -> Self {
        self.license_key = value;
        self
    }
}

#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LicenseKeyLicenseKey(pub String);

impl ::std::ops::Deref for LicenseKeyLicenseKey {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::str::FromStr for LicenseKeyLicenseKey {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regex::Regex> =
            ::std::sync::LazyLock::new(|| ::regex::Regex::new("^IM(SV|CL)(-[\\dA-Za-z]{4}){8}$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^IM(SV|CL)(-[\\dA-Za-z]{4}){8}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for LicenseKeyLicenseKey {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for LicenseKeyLicenseKey {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for LicenseKeyLicenseKey {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl<'de> ::serde::Deserialize<'de> for LicenseKeyLicenseKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: crate::common::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}

/// `LicenseResponseDTO`
///
/// See: <https://api.immich.app/models/LicenseResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct LicenseResponseDTO(pub UserLicense);

impl ::std::ops::Deref for LicenseResponseDTO {
    type Target = UserLicense;
    fn deref(&self) -> &UserLicense {
        &self.0
    }
}

impl From<UserLicense> for LicenseResponseDTO {
    fn from(value: UserLicense) -> Self {
        Self(value)
    }
}

/// `ReleaseEventV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `checked_at` | [`String`] | ✓ | When the server last checked for a latest version. As an ISO timestamp |
/// | `is_available` | `bool` | ✓ | Whether a new version is available |
/// | `release_version` | [`ServerVersionResponseDTO`] | ✓ |  |
/// | `server_version` | [`ServerVersionResponseDTO`] | ✓ |  |
/// | `type` | [`ReleaseType`] | ✓ | Release type |
///
/// See: <https://api.immich.app/models/ReleaseEventV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct ReleaseEventV1 {
    /// When the server last checked for a latest version. As an ISO timestamp (required)
    #[serde(rename = "checkedAt")]
    pub checked_at: String,
    /// Whether a new version is available (required)
    #[serde(rename = "isAvailable")]
    pub is_available: bool,
    /// (required)
    #[serde(rename = "releaseVersion")]
    pub release_version: ServerVersionResponseDTO,
    /// (required)
    #[serde(rename = "serverVersion")]
    pub server_version: ServerVersionResponseDTO,
    /// Release type (required)
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<ReleaseType>,
}

impl ReleaseEventV1 {
    ///Creates a new [`ReleaseEventV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        checked_at: impl Into<String>,
        is_available: bool,
        release_version: ServerVersionResponseDTO,
        server_version: ServerVersionResponseDTO,
    ) -> Self {
        Self {
            checked_at: checked_at.into(),
            is_available,
            release_version,
            server_version,
            type_: None,
        }
    }

    ///Sets `checked_at`.
    #[must_use]
    pub fn checked_at(mut self, value: impl Into<String>) -> Self {
        self.checked_at = value.into();
        self
    }

    ///Sets `is_available`.
    #[must_use]
    pub fn is_available(mut self, value: bool) -> Self {
        self.is_available = value;
        self
    }

    ///Sets `release_version`.
    #[must_use]
    pub fn release_version(mut self, value: ServerVersionResponseDTO) -> Self {
        self.release_version = value;
        self
    }

    ///Sets `server_version`.
    #[must_use]
    pub fn server_version(mut self, value: ServerVersionResponseDTO) -> Self {
        self.server_version = value;
        self
    }

    ///Sets `type_`.
    #[must_use]
    pub fn type_(mut self, value: ReleaseType) -> Self {
        self.type_ = Some(value);
        self
    }

    ///Sets or clears `type_`.
    #[must_use]
    pub fn set_type_(mut self, value: Option<ReleaseType>) -> Self {
        self.type_ = value;
        self
    }
}

///
/// See: <https://api.immich.app/models/ReleaseType>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ReleaseType {
    #[serde(rename = "major")]
    Major,
    #[serde(rename = "premajor")]
    Premajor,
    #[serde(rename = "minor")]
    Minor,
    #[serde(rename = "preminor")]
    Preminor,
    #[serde(rename = "patch")]
    Patch,
    #[serde(rename = "prepatch")]
    Prepatch,
    #[serde(rename = "prerelease")]
    Prerelease,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for ReleaseType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Major => f.write_str("major"),
            Self::Premajor => f.write_str("premajor"),
            Self::Minor => f.write_str("minor"),
            Self::Preminor => f.write_str("preminor"),
            Self::Patch => f.write_str("patch"),
            Self::Prepatch => f.write_str("prepatch"),
            Self::Prerelease => f.write_str("prerelease"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for ReleaseType {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "major" => Ok(Self::Major),
            "premajor" => Ok(Self::Premajor),
            "minor" => Ok(Self::Minor),
            "preminor" => Ok(Self::Preminor),
            "patch" => Ok(Self::Patch),
            "prepatch" => Ok(Self::Prepatch),
            "prerelease" => Ok(Self::Prerelease),
            _ => Err(format!("invalid value for {}: {}", "ReleaseType", value).into()),
        }
    }
}

impl TryFrom<&str> for ReleaseType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for ReleaseType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for ReleaseType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// `ServerAboutResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `build` | [`String`] |  | Build identifier |
/// | `build_image` | [`String`] |  | Build image name |
/// | `build_image_url` | [`String`] |  | Build image URL |
/// | `build_url` | [`String`] |  | Build URL |
/// | `exiftool` | [`String`] |  | `ExifTool` version |
/// | `ffmpeg` | [`String`] |  | `FFmpeg` version |
/// | `imagemagick` | [`String`] |  | `ImageMagick` version |
/// | `libvips` | [`String`] |  | libvips version |
/// | `licensed` | `bool` | ✓ | Whether the server is licensed |
/// | `nodejs` | [`String`] |  | Node.js version |
/// | `repository` | [`String`] |  | Repository name |
/// | `repository_url` | [`String`] |  | Repository URL |
/// | `source_commit` | [`String`] |  | Source commit hash |
/// | `source_ref` | [`String`] |  | Source reference (branch/tag) |
/// | `source_url` | [`String`] |  | Source URL |
/// | `third_party_bug_feature_url` | [`String`] |  | Third-party bug/feature URL |
/// | `third_party_documentation_url` | [`String`] |  | Third-party documentation URL |
/// | `third_party_source_url` | [`String`] |  | Third-party source URL |
/// | `third_party_support_url` | [`String`] |  | Third-party support URL |
/// | `version` | [`String`] | ✓ | Server version |
/// | `version_url` | [`String`] | ✓ | URL to version information |
///
/// See: <https://api.immich.app/models/ServerAboutResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct ServerAboutResponseDTO {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub build: Option<String>,
    /// Build image name
    #[serde(
        rename = "buildImage",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub build_image: Option<String>,
    /// Build image URL
    #[serde(
        rename = "buildImageUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub build_image_url: Option<String>,
    /// Build URL
    #[serde(rename = "buildUrl", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub build_url: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub exiftool: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ffmpeg: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub imagemagick: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub libvips: Option<String>,
    pub licensed: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub nodejs: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub repository: Option<String>,
    /// Repository URL
    #[serde(
        rename = "repositoryUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub repository_url: Option<String>,
    /// Source commit hash
    #[serde(
        rename = "sourceCommit",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub source_commit: Option<String>,
    /// Source reference (branch/tag)
    #[serde(
        rename = "sourceRef",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub source_ref: Option<String>,
    /// Source URL
    #[serde(
        rename = "sourceUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub source_url: Option<String>,
    /// Third-party bug/feature URL
    #[serde(
        rename = "thirdPartyBugFeatureUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub third_party_bug_feature_url: Option<String>,
    /// Third-party documentation URL
    #[serde(
        rename = "thirdPartyDocumentationUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub third_party_documentation_url: Option<String>,
    /// Third-party source URL
    #[serde(
        rename = "thirdPartySourceUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub third_party_source_url: Option<String>,
    /// Third-party support URL
    #[serde(
        rename = "thirdPartySupportUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub third_party_support_url: Option<String>,
    pub version: String,
    /// URL to version information (required)
    #[serde(rename = "versionUrl")]
    pub version_url: String,
}

impl ServerAboutResponseDTO {
    ///Creates a new [`ServerAboutResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(licensed: bool, version: impl Into<String>, version_url: impl Into<String>) -> Self {
        Self {
            build: None,
            build_image: None,
            build_image_url: None,
            build_url: None,
            exiftool: None,
            ffmpeg: None,
            imagemagick: None,
            libvips: None,
            licensed,
            nodejs: None,
            repository: None,
            repository_url: None,
            source_commit: None,
            source_ref: None,
            source_url: None,
            third_party_bug_feature_url: None,
            third_party_documentation_url: None,
            third_party_source_url: None,
            third_party_support_url: None,
            version: version.into(),
            version_url: version_url.into(),
        }
    }

    ///Sets `build`.
    #[must_use]
    pub fn build(mut self, value: impl Into<String>) -> Self {
        self.build = Some(value.into());
        self
    }

    ///Sets or clears `build`.
    #[must_use]
    pub fn set_build(mut self, value: Option<String>) -> Self {
        self.build = value;
        self
    }

    ///Sets `build_image`.
    #[must_use]
    pub fn build_image(mut self, value: impl Into<String>) -> Self {
        self.build_image = Some(value.into());
        self
    }

    ///Sets or clears `build_image`.
    #[must_use]
    pub fn set_build_image(mut self, value: Option<String>) -> Self {
        self.build_image = value;
        self
    }

    ///Sets `build_image_url`.
    #[must_use]
    pub fn build_image_url(mut self, value: impl Into<String>) -> Self {
        self.build_image_url = Some(value.into());
        self
    }

    ///Sets or clears `build_image_url`.
    #[must_use]
    pub fn set_build_image_url(mut self, value: Option<String>) -> Self {
        self.build_image_url = value;
        self
    }

    ///Sets `build_url`.
    #[must_use]
    pub fn build_url(mut self, value: impl Into<String>) -> Self {
        self.build_url = Some(value.into());
        self
    }

    ///Sets or clears `build_url`.
    #[must_use]
    pub fn set_build_url(mut self, value: Option<String>) -> Self {
        self.build_url = value;
        self
    }

    ///Sets `exiftool`.
    #[must_use]
    pub fn exiftool(mut self, value: impl Into<String>) -> Self {
        self.exiftool = Some(value.into());
        self
    }

    ///Sets or clears `exiftool`.
    #[must_use]
    pub fn set_exiftool(mut self, value: Option<String>) -> Self {
        self.exiftool = value;
        self
    }

    ///Sets `ffmpeg`.
    #[must_use]
    pub fn ffmpeg(mut self, value: impl Into<String>) -> Self {
        self.ffmpeg = Some(value.into());
        self
    }

    ///Sets or clears `ffmpeg`.
    #[must_use]
    pub fn set_ffmpeg(mut self, value: Option<String>) -> Self {
        self.ffmpeg = value;
        self
    }

    ///Sets `imagemagick`.
    #[must_use]
    pub fn imagemagick(mut self, value: impl Into<String>) -> Self {
        self.imagemagick = Some(value.into());
        self
    }

    ///Sets or clears `imagemagick`.
    #[must_use]
    pub fn set_imagemagick(mut self, value: Option<String>) -> Self {
        self.imagemagick = value;
        self
    }

    ///Sets `libvips`.
    #[must_use]
    pub fn libvips(mut self, value: impl Into<String>) -> Self {
        self.libvips = Some(value.into());
        self
    }

    ///Sets or clears `libvips`.
    #[must_use]
    pub fn set_libvips(mut self, value: Option<String>) -> Self {
        self.libvips = value;
        self
    }

    ///Sets `licensed`.
    #[must_use]
    pub fn licensed(mut self, value: bool) -> Self {
        self.licensed = value;
        self
    }

    ///Sets `nodejs`.
    #[must_use]
    pub fn nodejs(mut self, value: impl Into<String>) -> Self {
        self.nodejs = Some(value.into());
        self
    }

    ///Sets or clears `nodejs`.
    #[must_use]
    pub fn set_nodejs(mut self, value: Option<String>) -> Self {
        self.nodejs = value;
        self
    }

    ///Sets `repository`.
    #[must_use]
    pub fn repository(mut self, value: impl Into<String>) -> Self {
        self.repository = Some(value.into());
        self
    }

    ///Sets or clears `repository`.
    #[must_use]
    pub fn set_repository(mut self, value: Option<String>) -> Self {
        self.repository = value;
        self
    }

    ///Sets `repository_url`.
    #[must_use]
    pub fn repository_url(mut self, value: impl Into<String>) -> Self {
        self.repository_url = Some(value.into());
        self
    }

    ///Sets or clears `repository_url`.
    #[must_use]
    pub fn set_repository_url(mut self, value: Option<String>) -> Self {
        self.repository_url = value;
        self
    }

    ///Sets `source_commit`.
    #[must_use]
    pub fn source_commit(mut self, value: impl Into<String>) -> Self {
        self.source_commit = Some(value.into());
        self
    }

    ///Sets or clears `source_commit`.
    #[must_use]
    pub fn set_source_commit(mut self, value: Option<String>) -> Self {
        self.source_commit = value;
        self
    }

    ///Sets `source_ref`.
    #[must_use]
    pub fn source_ref(mut self, value: impl Into<String>) -> Self {
        self.source_ref = Some(value.into());
        self
    }

    ///Sets or clears `source_ref`.
    #[must_use]
    pub fn set_source_ref(mut self, value: Option<String>) -> Self {
        self.source_ref = value;
        self
    }

    ///Sets `source_url`.
    #[must_use]
    pub fn source_url(mut self, value: impl Into<String>) -> Self {
        self.source_url = Some(value.into());
        self
    }

    ///Sets or clears `source_url`.
    #[must_use]
    pub fn set_source_url(mut self, value: Option<String>) -> Self {
        self.source_url = value;
        self
    }

    ///Sets `third_party_bug_feature_url`.
    #[must_use]
    pub fn third_party_bug_feature_url(mut self, value: impl Into<String>) -> Self {
        self.third_party_bug_feature_url = Some(value.into());
        self
    }

    ///Sets or clears `third_party_bug_feature_url`.
    #[must_use]
    pub fn set_third_party_bug_feature_url(mut self, value: Option<String>) -> Self {
        self.third_party_bug_feature_url = value;
        self
    }

    ///Sets `third_party_documentation_url`.
    #[must_use]
    pub fn third_party_documentation_url(mut self, value: impl Into<String>) -> Self {
        self.third_party_documentation_url = Some(value.into());
        self
    }

    ///Sets or clears `third_party_documentation_url`.
    #[must_use]
    pub fn set_third_party_documentation_url(mut self, value: Option<String>) -> Self {
        self.third_party_documentation_url = value;
        self
    }

    ///Sets `third_party_source_url`.
    #[must_use]
    pub fn third_party_source_url(mut self, value: impl Into<String>) -> Self {
        self.third_party_source_url = Some(value.into());
        self
    }

    ///Sets or clears `third_party_source_url`.
    #[must_use]
    pub fn set_third_party_source_url(mut self, value: Option<String>) -> Self {
        self.third_party_source_url = value;
        self
    }

    ///Sets `third_party_support_url`.
    #[must_use]
    pub fn third_party_support_url(mut self, value: impl Into<String>) -> Self {
        self.third_party_support_url = Some(value.into());
        self
    }

    ///Sets or clears `third_party_support_url`.
    #[must_use]
    pub fn set_third_party_support_url(mut self, value: Option<String>) -> Self {
        self.third_party_support_url = value;
        self
    }

    ///Sets `version`.
    #[must_use]
    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = value.into();
        self
    }

    ///Sets `version_url`.
    #[must_use]
    pub fn version_url(mut self, value: impl Into<String>) -> Self {
        self.version_url = value.into();
        self
    }
}

/// `ServerApkLinksDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `arm64v8a` | [`String`] | ✓ | APK download link for ARM64 v8a architecture |
/// | `armeabiv7a` | [`String`] | ✓ | APK download link for ARM EABI v7a architecture |
/// | `universal` | [`String`] | ✓ | APK download link for universal architecture |
/// | `x86_64` | [`String`] | ✓ | APK download link for `x86_64` architecture |
///
/// See: <https://api.immich.app/models/ServerApkLinksDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct ServerApkLinksDTO {
    pub arm64v8a: String,
    pub armeabiv7a: String,
    pub universal: String,
    pub x86_64: String,
}

impl ServerApkLinksDTO {
    ///Creates a new [`ServerApkLinksDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        arm64v8a: impl Into<String>,
        armeabiv7a: impl Into<String>,
        universal: impl Into<String>,
        x86_64: impl Into<String>,
    ) -> Self {
        Self {
            arm64v8a: arm64v8a.into(),
            armeabiv7a: armeabiv7a.into(),
            universal: universal.into(),
            x86_64: x86_64.into(),
        }
    }

    ///Sets `arm64v8a`.
    #[must_use]
    pub fn arm64v8a(mut self, value: impl Into<String>) -> Self {
        self.arm64v8a = value.into();
        self
    }

    ///Sets `armeabiv7a`.
    #[must_use]
    pub fn armeabiv7a(mut self, value: impl Into<String>) -> Self {
        self.armeabiv7a = value.into();
        self
    }

    ///Sets `universal`.
    #[must_use]
    pub fn universal(mut self, value: impl Into<String>) -> Self {
        self.universal = value.into();
        self
    }

    ///Sets `x86_64`.
    #[must_use]
    pub fn x86_64(mut self, value: impl Into<String>) -> Self {
        self.x86_64 = value.into();
        self
    }
}

/// `ServerConfigDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `external_domain` | [`String`] | ✓ | External domain URL |
/// | `is_initialized` | `bool` | ✓ | Whether the server has been initialized |
/// | `is_onboarded` | `bool` | ✓ | Whether the admin has completed onboarding |
/// | `login_page_message` | [`String`] | ✓ | Login page message |
/// | `maintenance_mode` | `bool` | ✓ | Whether maintenance mode is active |
/// | `map_dark_style_url` | [`String`] | ✓ | Map dark style URL |
/// | `map_light_style_url` | [`String`] | ✓ | Map light style URL |
/// | `min_faces` | `i64` | ✓ | People min faces server default |
/// | `oauth_account_management_url` | [`String`] |  | OAuth account management URL |
/// | `oauth_button_text` | [`String`] | ✓ | OAuth button text |
/// | `public_users` | `bool` | ✓ | Whether public user registration is enabled |
/// | `trash_days` | `i64` | ✓ | Number of days before trashed assets are permanently deleted |
/// | `user_delete_delay` | `i64` | ✓ | Delay in days before deleted users are permanently removed |
///
/// See: <https://api.immich.app/models/ServerConfigDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct ServerConfigDTO {
    /// External domain URL (required)
    #[serde(rename = "externalDomain")]
    pub external_domain: String,
    /// Whether the server has been initialized (required)
    #[serde(rename = "isInitialized")]
    pub is_initialized: bool,
    /// Whether the admin has completed onboarding (required)
    #[serde(rename = "isOnboarded")]
    pub is_onboarded: bool,
    /// Login page message (required)
    #[serde(rename = "loginPageMessage")]
    pub login_page_message: String,
    /// Whether maintenance mode is active (required)
    #[serde(rename = "maintenanceMode")]
    pub maintenance_mode: bool,
    /// Map dark style URL (required)
    #[serde(rename = "mapDarkStyleUrl")]
    pub map_dark_style_url: String,
    /// Map light style URL (required)
    #[serde(rename = "mapLightStyleUrl")]
    pub map_light_style_url: String,
    /// People min faces server default (required)
    #[serde(rename = "minFaces")]
    pub min_faces: i64,
    /// OAuth account management URL
    #[serde(rename = "oauthAccountManagementUrl", default)]
    pub oauth_account_management_url: String,
    /// OAuth button text (required)
    #[serde(rename = "oauthButtonText")]
    pub oauth_button_text: String,
    /// Whether public user registration is enabled (required)
    #[serde(rename = "publicUsers")]
    pub public_users: bool,
    /// Number of days before trashed assets are permanently deleted (required)
    #[serde(rename = "trashDays")]
    pub trash_days: i64,
    /// Delay in days before deleted users are permanently removed (required)
    #[serde(rename = "userDeleteDelay")]
    pub user_delete_delay: i64,
}

impl ServerConfigDTO {
    ///Creates a new [`ServerConfigDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        external_domain: impl Into<String>,
        is_initialized: bool,
        is_onboarded: bool,
        login_page_message: impl Into<String>,
        maintenance_mode: bool,
        map_dark_style_url: impl Into<String>,
        map_light_style_url: impl Into<String>,
        min_faces: i64,
        oauth_button_text: impl Into<String>,
        public_users: bool,
        trash_days: i64,
        user_delete_delay: i64,
    ) -> Self {
        Self {
            external_domain: external_domain.into(),
            is_initialized,
            is_onboarded,
            login_page_message: login_page_message.into(),
            maintenance_mode,
            map_dark_style_url: map_dark_style_url.into(),
            map_light_style_url: map_light_style_url.into(),
            min_faces,
            oauth_account_management_url: <String>::default(),
            oauth_button_text: oauth_button_text.into(),
            public_users,
            trash_days,
            user_delete_delay,
        }
    }

    ///Sets `external_domain`.
    #[must_use]
    pub fn external_domain(mut self, value: impl Into<String>) -> Self {
        self.external_domain = value.into();
        self
    }

    ///Sets `is_initialized`.
    #[must_use]
    pub fn is_initialized(mut self, value: bool) -> Self {
        self.is_initialized = value;
        self
    }

    ///Sets `is_onboarded`.
    #[must_use]
    pub fn is_onboarded(mut self, value: bool) -> Self {
        self.is_onboarded = value;
        self
    }

    ///Sets `login_page_message`.
    #[must_use]
    pub fn login_page_message(mut self, value: impl Into<String>) -> Self {
        self.login_page_message = value.into();
        self
    }

    ///Sets `maintenance_mode`.
    #[must_use]
    pub fn maintenance_mode(mut self, value: bool) -> Self {
        self.maintenance_mode = value;
        self
    }

    ///Sets `map_dark_style_url`.
    #[must_use]
    pub fn map_dark_style_url(mut self, value: impl Into<String>) -> Self {
        self.map_dark_style_url = value.into();
        self
    }

    ///Sets `map_light_style_url`.
    #[must_use]
    pub fn map_light_style_url(mut self, value: impl Into<String>) -> Self {
        self.map_light_style_url = value.into();
        self
    }

    ///Sets `min_faces`.
    #[must_use]
    pub fn min_faces(mut self, value: i64) -> Self {
        self.min_faces = value;
        self
    }

    ///Sets `oauth_account_management_url`.
    #[must_use]
    pub fn oauth_account_management_url(mut self, value: impl Into<String>) -> Self {
        self.oauth_account_management_url = value.into();
        self
    }

    ///Sets `oauth_button_text`.
    #[must_use]
    pub fn oauth_button_text(mut self, value: impl Into<String>) -> Self {
        self.oauth_button_text = value.into();
        self
    }

    ///Sets `public_users`.
    #[must_use]
    pub fn public_users(mut self, value: bool) -> Self {
        self.public_users = value;
        self
    }

    ///Sets `trash_days`.
    #[must_use]
    pub fn trash_days(mut self, value: i64) -> Self {
        self.trash_days = value;
        self
    }

    ///Sets `user_delete_delay`.
    #[must_use]
    pub fn user_delete_delay(mut self, value: i64) -> Self {
        self.user_delete_delay = value;
        self
    }
}

/// `ServerFeaturesDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `config_file` | `bool` | ✓ | Whether config file is available |
/// | `duplicate_detection` | `bool` | ✓ | Whether duplicate detection is enabled |
/// | `email` | `bool` | ✓ | Whether email notifications are enabled |
/// | `facial_recognition` | `bool` | ✓ | Whether facial recognition is enabled |
/// | `import_faces` | `bool` | ✓ | Whether face import is enabled |
/// | `map` | `bool` | ✓ | Whether map feature is enabled |
/// | `oauth` | `bool` | ✓ | Whether OAuth is enabled |
/// | `oauth_auto_launch` | `bool` | ✓ | Whether OAuth auto-launch is enabled |
/// | `ocr` | `bool` | ✓ | Whether OCR is enabled |
/// | `password_login` | `bool` | ✓ | Whether password login is enabled |
/// | `realtime_transcoding` | `bool` | ✓ | Whether real-time transcoding is enabled |
/// | `reverse_geocoding` | `bool` | ✓ | Whether reverse geocoding is enabled |
/// | `search` | `bool` | ✓ | Whether search is enabled |
/// | `sidecar` | `bool` | ✓ | Whether sidecar files are supported |
/// | `smart_search` | `bool` | ✓ | Whether smart search is enabled |
/// | `trash` | `bool` | ✓ | Whether trash feature is enabled |
///
/// See: <https://api.immich.app/models/ServerFeaturesDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct ServerFeaturesDTO {
    /// Whether config file is available (required)
    #[serde(rename = "configFile")]
    pub config_file: bool,
    /// Whether duplicate detection is enabled (required)
    #[serde(rename = "duplicateDetection")]
    pub duplicate_detection: bool,
    pub email: bool,
    /// Whether facial recognition is enabled (required)
    #[serde(rename = "facialRecognition")]
    pub facial_recognition: bool,
    /// Whether face import is enabled (required)
    #[serde(rename = "importFaces")]
    pub import_faces: bool,
    pub map: bool,
    pub oauth: bool,
    /// Whether OAuth auto-launch is enabled (required)
    #[serde(rename = "oauthAutoLaunch")]
    pub oauth_auto_launch: bool,
    pub ocr: bool,
    /// Whether password login is enabled (required)
    #[serde(rename = "passwordLogin")]
    pub password_login: bool,
    /// Whether real-time transcoding is enabled (required)
    #[serde(rename = "realtimeTranscoding")]
    pub realtime_transcoding: bool,
    /// Whether reverse geocoding is enabled (required)
    #[serde(rename = "reverseGeocoding")]
    pub reverse_geocoding: bool,
    pub search: bool,
    pub sidecar: bool,
    /// Whether smart search is enabled (required)
    #[serde(rename = "smartSearch")]
    pub smart_search: bool,
    pub trash: bool,
}

impl ServerFeaturesDTO {
    ///Creates a new [`ServerFeaturesDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        config_file: bool,
        duplicate_detection: bool,
        email: bool,
        facial_recognition: bool,
        import_faces: bool,
        map: bool,
        oauth: bool,
        oauth_auto_launch: bool,
        ocr: bool,
        password_login: bool,
        realtime_transcoding: bool,
        reverse_geocoding: bool,
        search: bool,
        sidecar: bool,
        smart_search: bool,
        trash: bool,
    ) -> Self {
        Self {
            config_file,
            duplicate_detection,
            email,
            facial_recognition,
            import_faces,
            map,
            oauth,
            oauth_auto_launch,
            ocr,
            password_login,
            realtime_transcoding,
            reverse_geocoding,
            search,
            sidecar,
            smart_search,
            trash,
        }
    }

    ///Sets `config_file`.
    #[must_use]
    pub fn config_file(mut self, value: bool) -> Self {
        self.config_file = value;
        self
    }

    ///Sets `duplicate_detection`.
    #[must_use]
    pub fn duplicate_detection(mut self, value: bool) -> Self {
        self.duplicate_detection = value;
        self
    }

    ///Sets `email`.
    #[must_use]
    pub fn email(mut self, value: bool) -> Self {
        self.email = value;
        self
    }

    ///Sets `facial_recognition`.
    #[must_use]
    pub fn facial_recognition(mut self, value: bool) -> Self {
        self.facial_recognition = value;
        self
    }

    ///Sets `import_faces`.
    #[must_use]
    pub fn import_faces(mut self, value: bool) -> Self {
        self.import_faces = value;
        self
    }

    ///Sets `map`.
    #[must_use]
    pub fn map(mut self, value: bool) -> Self {
        self.map = value;
        self
    }

    ///Sets `oauth`.
    #[must_use]
    pub fn oauth(mut self, value: bool) -> Self {
        self.oauth = value;
        self
    }

    ///Sets `oauth_auto_launch`.
    #[must_use]
    pub fn oauth_auto_launch(mut self, value: bool) -> Self {
        self.oauth_auto_launch = value;
        self
    }

    ///Sets `ocr`.
    #[must_use]
    pub fn ocr(mut self, value: bool) -> Self {
        self.ocr = value;
        self
    }

    ///Sets `password_login`.
    #[must_use]
    pub fn password_login(mut self, value: bool) -> Self {
        self.password_login = value;
        self
    }

    ///Sets `realtime_transcoding`.
    #[must_use]
    pub fn realtime_transcoding(mut self, value: bool) -> Self {
        self.realtime_transcoding = value;
        self
    }

    ///Sets `reverse_geocoding`.
    #[must_use]
    pub fn reverse_geocoding(mut self, value: bool) -> Self {
        self.reverse_geocoding = value;
        self
    }

    ///Sets `search`.
    #[must_use]
    pub fn search(mut self, value: bool) -> Self {
        self.search = value;
        self
    }

    ///Sets `sidecar`.
    #[must_use]
    pub fn sidecar(mut self, value: bool) -> Self {
        self.sidecar = value;
        self
    }

    ///Sets `smart_search`.
    #[must_use]
    pub fn smart_search(mut self, value: bool) -> Self {
        self.smart_search = value;
        self
    }

    ///Sets `trash`.
    #[must_use]
    pub fn trash(mut self, value: bool) -> Self {
        self.trash = value;
        self
    }
}

/// `ServerMediaTypesResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `image` | [`Vec<String>`](::std::vec::Vec) | ✓ | Supported image MIME types |
/// | `sidecar` | [`Vec<String>`](::std::vec::Vec) | ✓ | Supported sidecar MIME types |
/// | `video` | [`Vec<String>`](::std::vec::Vec) | ✓ | Supported video MIME types |
///
/// See: <https://api.immich.app/models/ServerMediaTypesResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct ServerMediaTypesResponseDTO {
    pub image: Vec<String>,
    pub sidecar: Vec<String>,
    pub video: Vec<String>,
}

impl ServerMediaTypesResponseDTO {
    ///Creates a new [`ServerMediaTypesResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(image: Vec<String>, sidecar: Vec<String>, video: Vec<String>) -> Self {
        Self { image, sidecar, video }
    }

    ///Sets `image`.
    #[must_use]
    pub fn image(mut self, value: Vec<String>) -> Self {
        self.image = value;
        self
    }

    ///Sets `sidecar`.
    #[must_use]
    pub fn sidecar(mut self, value: Vec<String>) -> Self {
        self.sidecar = value;
        self
    }

    ///Sets `video`.
    #[must_use]
    pub fn video(mut self, value: Vec<String>) -> Self {
        self.video = value;
        self
    }
}

/// `ServerPingResponse`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `res` | [`String`] | ✓ |  |
///
/// See: <https://api.immich.app/models/ServerPingResponse>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct ServerPingResponse {
    pub res: String,
}

impl ServerPingResponse {
    ///Creates a new [`ServerPingResponse`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(res: impl Into<String>) -> Self {
        Self { res: res.into() }
    }

    ///Sets `res`.
    #[must_use]
    pub fn res(mut self, value: impl Into<String>) -> Self {
        self.res = value.into();
        self
    }
}

/// `ServerStatsResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `photos` | `i64` | ✓ | Total number of photos |
/// | `usage` | `i64` | ✓ | Total storage usage in bytes |
/// | `usage_by_user` | [`Vec<UsageByUserDTO>`](crate::server::UsageByUserDTO) | ✓ | Array of usage for each user |
/// | `usage_photos` | `i64` | ✓ | Storage usage for photos in bytes |
/// | `usage_videos` | `i64` | ✓ | Storage usage for videos in bytes |
/// | `videos` | `i64` | ✓ | Total number of videos |
///
/// See: <https://api.immich.app/models/ServerStatsResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct ServerStatsResponseDTO {
    pub photos: i64,
    pub usage: i64,
    /// Array of usage for each user (required)
    #[serde(rename = "usageByUser")]
    pub usage_by_user: Vec<UsageByUserDTO>,
    /// Storage usage for photos in bytes (required)
    #[serde(rename = "usagePhotos")]
    pub usage_photos: i64,
    /// Storage usage for videos in bytes (required)
    #[serde(rename = "usageVideos")]
    pub usage_videos: i64,
    pub videos: i64,
}

impl ServerStatsResponseDTO {
    ///Creates a new [`ServerStatsResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        photos: i64,
        usage: i64,
        usage_by_user: Vec<UsageByUserDTO>,
        usage_photos: i64,
        usage_videos: i64,
        videos: i64,
    ) -> Self {
        Self {
            photos,
            usage,
            usage_by_user,
            usage_photos,
            usage_videos,
            videos,
        }
    }

    ///Sets `photos`.
    #[must_use]
    pub fn photos(mut self, value: i64) -> Self {
        self.photos = value;
        self
    }

    ///Sets `usage`.
    #[must_use]
    pub fn usage(mut self, value: i64) -> Self {
        self.usage = value;
        self
    }

    ///Sets `usage_by_user`.
    #[must_use]
    pub fn usage_by_user(mut self, value: Vec<UsageByUserDTO>) -> Self {
        self.usage_by_user = value;
        self
    }

    ///Sets `usage_photos`.
    #[must_use]
    pub fn usage_photos(mut self, value: i64) -> Self {
        self.usage_photos = value;
        self
    }

    ///Sets `usage_videos`.
    #[must_use]
    pub fn usage_videos(mut self, value: i64) -> Self {
        self.usage_videos = value;
        self
    }

    ///Sets `videos`.
    #[must_use]
    pub fn videos(mut self, value: i64) -> Self {
        self.videos = value;
        self
    }
}

/// `ServerStorageResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `disk_available` | [`String`] | ✓ | Available disk space (human-readable format) |
/// | `disk_available_raw` | `i64` | ✓ | Available disk space in bytes |
/// | `disk_size` | [`String`] | ✓ | Total disk size (human-readable format) |
/// | `disk_size_raw` | `i64` | ✓ | Total disk size in bytes |
/// | `disk_usage_percentage` | `f64` | ✓ | Disk usage percentage (0-100) |
/// | `disk_use` | [`String`] | ✓ | Used disk space (human-readable format) |
/// | `disk_use_raw` | `i64` | ✓ | Used disk space in bytes |
///
/// See: <https://api.immich.app/models/ServerStorageResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct ServerStorageResponseDTO {
    /// Available disk space (human-readable format) (required)
    #[serde(rename = "diskAvailable")]
    pub disk_available: String,
    /// Available disk space in bytes (required)
    #[serde(rename = "diskAvailableRaw")]
    pub disk_available_raw: i64,
    /// Total disk size (human-readable format) (required)
    #[serde(rename = "diskSize")]
    pub disk_size: String,
    /// Total disk size in bytes (required)
    #[serde(rename = "diskSizeRaw")]
    pub disk_size_raw: i64,
    /// Disk usage percentage (0-100) (required)
    #[serde(rename = "diskUsagePercentage")]
    pub disk_usage_percentage: f64,
    /// Used disk space (human-readable format) (required)
    #[serde(rename = "diskUse")]
    pub disk_use: String,
    /// Used disk space in bytes (required)
    #[serde(rename = "diskUseRaw")]
    pub disk_use_raw: i64,
}

impl ServerStorageResponseDTO {
    ///Creates a new [`ServerStorageResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        disk_available: impl Into<String>,
        disk_available_raw: i64,
        disk_size: impl Into<String>,
        disk_size_raw: i64,
        disk_usage_percentage: f64,
        disk_use: impl Into<String>,
        disk_use_raw: i64,
    ) -> Self {
        Self {
            disk_available: disk_available.into(),
            disk_available_raw,
            disk_size: disk_size.into(),
            disk_size_raw,
            disk_usage_percentage,
            disk_use: disk_use.into(),
            disk_use_raw,
        }
    }

    ///Sets `disk_available`.
    #[must_use]
    pub fn disk_available(mut self, value: impl Into<String>) -> Self {
        self.disk_available = value.into();
        self
    }

    ///Sets `disk_available_raw`.
    #[must_use]
    pub fn disk_available_raw(mut self, value: i64) -> Self {
        self.disk_available_raw = value;
        self
    }

    ///Sets `disk_size`.
    #[must_use]
    pub fn disk_size(mut self, value: impl Into<String>) -> Self {
        self.disk_size = value.into();
        self
    }

    ///Sets `disk_size_raw`.
    #[must_use]
    pub fn disk_size_raw(mut self, value: i64) -> Self {
        self.disk_size_raw = value;
        self
    }

    ///Sets `disk_usage_percentage`.
    #[must_use]
    pub fn disk_usage_percentage(mut self, value: f64) -> Self {
        self.disk_usage_percentage = value;
        self
    }

    ///Sets `disk_use`.
    #[must_use]
    pub fn disk_use(mut self, value: impl Into<String>) -> Self {
        self.disk_use = value.into();
        self
    }

    ///Sets `disk_use_raw`.
    #[must_use]
    pub fn disk_use_raw(mut self, value: i64) -> Self {
        self.disk_use_raw = value;
        self
    }
}

/// `ServerVersionHistoryResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | When this version was first seen |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Version history entry ID |
/// | `version` | [`String`] | ✓ | Version string |
///
/// See: <https://api.immich.app/models/ServerVersionHistoryResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct ServerVersionHistoryResponseDTO {
    /// When this version was first seen (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub id: ::uuid::Uuid,
    pub version: String,
}

impl ServerVersionHistoryResponseDTO {
    ///Creates a new [`ServerVersionHistoryResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        id: ::uuid::Uuid,
        version: impl Into<String>,
    ) -> Self {
        Self {
            created_at,
            id,
            version: version.into(),
        }
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

    ///Sets `version`.
    #[must_use]
    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = value.into();
        self
    }
}

/// `ServerVersionResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `major` | `i64` | ✓ | Major version number |
/// | `minor` | `i64` | ✓ | Minor version number |
/// | `patch` | `i64` | ✓ | Patch version number |
/// | `prerelease` | `i64` | ✓ | Pre-release version number |
///
/// See: <https://api.immich.app/models/ServerVersionResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct ServerVersionResponseDTO {
    pub major: i64,
    pub minor: i64,
    pub patch: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prerelease: Option<i64>,
}

impl ServerVersionResponseDTO {
    ///Creates a new [`ServerVersionResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(major: i64, minor: i64, patch: i64) -> Self {
        Self {
            major,
            minor,
            patch,
            prerelease: None,
        }
    }

    ///Sets `major`.
    #[must_use]
    pub fn major(mut self, value: i64) -> Self {
        self.major = value;
        self
    }

    ///Sets `minor`.
    #[must_use]
    pub fn minor(mut self, value: i64) -> Self {
        self.minor = value;
        self
    }

    ///Sets `patch`.
    #[must_use]
    pub fn patch(mut self, value: i64) -> Self {
        self.patch = value;
        self
    }

    ///Sets `prerelease`.
    #[must_use]
    pub fn prerelease(mut self, value: i64) -> Self {
        self.prerelease = Some(value);
        self
    }

    ///Sets or clears `prerelease`.
    #[must_use]
    pub fn set_prerelease(mut self, value: Option<i64>) -> Self {
        self.prerelease = value;
        self
    }
}

/// `UsageByUserDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `photos` | `i64` | ✓ | Number of photos |
/// | `quota_size_in_bytes` | `i64` | ✓ | User quota size in bytes (null if unlimited) |
/// | `usage` | `i64` | ✓ | Total storage usage in bytes |
/// | `usage_photos` | `i64` | ✓ | Storage usage for photos in bytes |
/// | `usage_videos` | `i64` | ✓ | Storage usage for videos in bytes |
/// | `user_id` | [`Uuid`](::uuid::Uuid) | ✓ | User ID |
/// | `user_name` | [`String`] | ✓ | User name |
/// | `videos` | `i64` | ✓ | Number of videos |
///
/// See: <https://api.immich.app/models/UsageByUserDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct UsageByUserDTO {
    pub photos: i64,
    /// User quota size in bytes (null if unlimited) (required)
    #[serde(rename = "quotaSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_size_in_bytes: Option<i64>,
    pub usage: i64,
    /// Storage usage for photos in bytes (required)
    #[serde(rename = "usagePhotos")]
    pub usage_photos: i64,
    /// Storage usage for videos in bytes (required)
    #[serde(rename = "usageVideos")]
    pub usage_videos: i64,
    /// User ID (required)
    #[serde(rename = "userId")]
    pub user_id: ::uuid::Uuid,
    /// User name (required)
    #[serde(rename = "userName")]
    pub user_name: String,
    pub videos: i64,
}

impl UsageByUserDTO {
    ///Creates a new [`UsageByUserDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        photos: i64,
        usage: i64,
        usage_photos: i64,
        usage_videos: i64,
        user_id: ::uuid::Uuid,
        user_name: impl Into<String>,
        videos: i64,
    ) -> Self {
        Self {
            photos,
            quota_size_in_bytes: None,
            usage,
            usage_photos,
            usage_videos,
            user_id,
            user_name: user_name.into(),
            videos,
        }
    }

    ///Sets `photos`.
    #[must_use]
    pub fn photos(mut self, value: i64) -> Self {
        self.photos = value;
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

    ///Sets `usage`.
    #[must_use]
    pub fn usage(mut self, value: i64) -> Self {
        self.usage = value;
        self
    }

    ///Sets `usage_photos`.
    #[must_use]
    pub fn usage_photos(mut self, value: i64) -> Self {
        self.usage_photos = value;
        self
    }

    ///Sets `usage_videos`.
    #[must_use]
    pub fn usage_videos(mut self, value: i64) -> Self {
        self.usage_videos = value;
        self
    }

    ///Sets `user_id`.
    #[must_use]
    pub fn user_id(mut self, value: ::uuid::Uuid) -> Self {
        self.user_id = value;
        self
    }

    ///Sets `user_name`.
    #[must_use]
    pub fn user_name(mut self, value: impl Into<String>) -> Self {
        self.user_name = value.into();
        self
    }

    ///Sets `videos`.
    #[must_use]
    pub fn videos(mut self, value: i64) -> Self {
        self.videos = value;
        self
    }
}

/// `VersionCheckStateResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `checked_at` | [`String`] | ✓ | Last check timestamp |
/// | `release_version` | [`String`] | ✓ | Release version |
///
/// See: <https://api.immich.app/models/VersionCheckStateResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct VersionCheckStateResponseDTO {
    /// Last check timestamp (required)
    #[serde(rename = "checkedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked_at: Option<String>,
    /// Release version (required)
    #[serde(rename = "releaseVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_version: Option<String>,
}

impl VersionCheckStateResponseDTO {
    ///Creates a new [`VersionCheckStateResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            checked_at: None,
            release_version: None,
        }
    }

    ///Sets `checked_at`.
    #[must_use]
    pub fn checked_at(mut self, value: impl Into<String>) -> Self {
        self.checked_at = Some(value.into());
        self
    }

    ///Sets or clears `checked_at`.
    #[must_use]
    pub fn set_checked_at(mut self, value: Option<String>) -> Self {
        self.checked_at = value;
        self
    }

    ///Sets `release_version`.
    #[must_use]
    pub fn release_version(mut self, value: impl Into<String>) -> Self {
        self.release_version = Some(value.into());
        self
    }

    ///Sets or clears `release_version`.
    #[must_use]
    pub fn set_release_version(mut self, value: Option<String>) -> Self {
        self.release_version = value;
        self
    }
}

impl Default for VersionCheckStateResponseDTO {
    fn default() -> Self {
        Self::new()
    }
}
