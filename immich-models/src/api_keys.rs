// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! `OpenAPI` data models for `api_keys`.

pub use crate::id::ApiKeyId;

/// `ApiKeyCreateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `name` | [`String`] |  | API key name |
/// | `permissions` | [`Vec<Permission>`](crate::api_keys::Permission) | ✓ | List of permissions |
///
/// See: <https://api.immich.app/models/ApiKeyCreateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct ApiKeyCreateDTO {
    /// API key name
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: Option<String>,
    /// List of permissions (required)
    pub permissions: Vec<Permission>,
}

impl ApiKeyCreateDTO {
    /// Creates a new [`ApiKeyCreateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(permissions: impl IntoIterator<Item = Permission>) -> Self {
        Self {
            name: None,
            permissions: permissions.into_iter().collect(),
        }
    }
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }
    /// Sets or clears `name`.
    #[must_use]
    pub fn set_name(mut self, value: Option<String>) -> Self {
        self.name = value;
        self
    }
    /// Sets `permissions`.
    #[must_use]
    pub fn permissions(mut self, value: impl IntoIterator<Item = Permission>) -> Self {
        self.permissions = value.into_iter().collect();
        self
    }
}

/// `ApiKeyCreateResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `api_key` | [`ApiKeyResponseDTO`] | ✓ |  |
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Creation date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `id` | [`ApiKeyId`](crate::id::ApiKeyId) | ✓ | API key ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `name` | [`String`] | ✓ | API key name |
/// | `permissions` | [`Vec<Permission>`](crate::api_keys::Permission) | ✓ | List of permissions |
/// | `secret` | [`String`] | ✓ | API key secret (only shown once) |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Last update date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
///
/// See: <https://api.immich.app/models/ApiKeyCreateResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct ApiKeyCreateResponseDTO {
    /// (required)
    #[serde(rename = "apiKey")]
    pub api_key: ApiKeyResponseDTO,
    /// Creation date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// API key ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: ApiKeyId,
    /// API key name (required)
    pub name: String,
    /// List of permissions (required)
    pub permissions: Vec<Permission>,
    /// API key secret (only shown once) (required)
    pub secret: String,
    /// Last update date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl ApiKeyCreateResponseDTO {
    /// Creates a new [`ApiKeyCreateResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        api_key: ApiKeyResponseDTO,
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        id: impl Into<ApiKeyId>,
        name: impl Into<String>,
        permissions: impl IntoIterator<Item = Permission>,
        secret: impl Into<String>,
        updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ) -> Self {
        Self {
            api_key,
            created_at,
            id: id.into(),
            name: name.into(),
            permissions: permissions.into_iter().collect(),
            secret: secret.into(),
            updated_at,
        }
    }
    /// Sets `api_key`.
    #[must_use]
    pub fn api_key(mut self, value: ApiKeyResponseDTO) -> Self {
        self.api_key = value;
        self
    }
    /// Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_at = value;
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<ApiKeyId>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }
    /// Sets `permissions`.
    #[must_use]
    pub fn permissions(mut self, value: impl IntoIterator<Item = Permission>) -> Self {
        self.permissions = value.into_iter().collect();
        self
    }
    /// Sets `secret`.
    #[must_use]
    pub fn secret(mut self, value: impl Into<String>) -> Self {
        self.secret = value.into();
        self
    }
    /// Sets `updated_at`.
    #[must_use]
    pub fn updated_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.updated_at = value;
        self
    }
}

/// `ApiKeyResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Creation date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `id` | [`ApiKeyId`](crate::id::ApiKeyId) | ✓ | API key ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `name` | [`String`] | ✓ | API key name |
/// | `permissions` | [`Vec<Permission>`](crate::api_keys::Permission) | ✓ | List of permissions |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Last update date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
///
/// See: <https://api.immich.app/models/ApiKeyResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct ApiKeyResponseDTO {
    /// Creation date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// API key ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: ApiKeyId,
    /// API key name (required)
    pub name: String,
    /// List of permissions (required)
    pub permissions: Vec<Permission>,
    /// Last update date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl ApiKeyResponseDTO {
    /// Creates a new [`ApiKeyResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        id: impl Into<ApiKeyId>,
        name: impl Into<String>,
        permissions: impl IntoIterator<Item = Permission>,
        updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ) -> Self {
        Self {
            created_at,
            id: id.into(),
            name: name.into(),
            permissions: permissions.into_iter().collect(),
            updated_at,
        }
    }
    /// Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_at = value;
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<ApiKeyId>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }
    /// Sets `permissions`.
    #[must_use]
    pub fn permissions(mut self, value: impl IntoIterator<Item = Permission>) -> Self {
        self.permissions = value.into_iter().collect();
        self
    }
    /// Sets `updated_at`.
    #[must_use]
    pub fn updated_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.updated_at = value;
        self
    }
}

/// `ApiKeyUpdateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `name` | [`String`] |  | API key name |
/// | `permissions` | [`Vec<Permission>`](crate::api_keys::Permission) |  | List of permissions |
///
/// See: <https://api.immich.app/models/ApiKeyUpdateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct ApiKeyUpdateDTO {
    /// API key name
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: Option<String>,
    /// List of permissions
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub permissions: Vec<Permission>,
}

impl ApiKeyUpdateDTO {
    /// Creates a new [`ApiKeyUpdateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            name: None,
            permissions: Vec::new(),
        }
    }
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }
    /// Sets or clears `name`.
    #[must_use]
    pub fn set_name(mut self, value: Option<String>) -> Self {
        self.name = value;
        self
    }
    /// Sets `permissions`.
    #[must_use]
    pub fn permissions(mut self, value: impl IntoIterator<Item = Permission>) -> Self {
        self.permissions = value.into_iter().collect();
        self
    }
}

/// List of permissions
///
/// See: <https://api.immich.app/models/Permission>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum Permission {
    All,
    ActivityCreate,
    ActivityRead,
    ActivityUpdate,
    ActivityDelete,
    ActivityStatistics,
    ApiKeyCreate,
    ApiKeyRead,
    ApiKeyUpdate,
    ApiKeyDelete,
    ApiKeyRotate,
    AssetRead,
    AssetUpdate,
    AssetDelete,
    AssetStatistics,
    AssetShare,
    AssetView,
    AssetDownload,
    AssetUpload,
    AssetCopy,
    AssetDerive,
    AssetFileRead,
    AssetFileDelete,
    AssetFileDownload,
    AssetEditGet,
    AssetEditCreate,
    AssetEditDelete,
    AlbumCreate,
    AlbumRead,
    AlbumUpdate,
    AlbumDelete,
    AlbumStatistics,
    AlbumShare,
    AlbumDownload,
    AlbumAssetCreate,
    AlbumAssetDelete,
    AlbumUserCreate,
    AlbumUserUpdate,
    AlbumUserDelete,
    AuthChangePassword,
    AuthDeviceDelete,
    ArchiveRead,
    BackupList,
    BackupDownload,
    BackupUpload,
    BackupDelete,
    ClusterGroupRead,
    ClusterGroupLeave,
    ClusterGroupRequestCreate,
    ClusterGroupRequestRead,
    ClusterGroupRequestDelete,
    AdminConfigRead,
    AdminConfigUpdate,
    UserConfigRead,
    DuplicateRead,
    DuplicateDelete,
    FaceCreate,
    FaceRead,
    FaceUpdate,
    FaceDelete,
    FolderRead,
    JobCreate,
    JobRead,
    LibraryCreate,
    LibraryRead,
    LibraryUpdate,
    LibraryDelete,
    LibraryStatistics,
    TimelineRead,
    TimelineDownload,
    Maintenance,
    MapRead,
    MapSearch,
    MemoryCreate,
    MemoryRead,
    MemoryUpdate,
    MemoryDelete,
    MemoryStatistics,
    MemoryAssetCreate,
    MemoryAssetDelete,
    NotificationCreate,
    NotificationRead,
    NotificationUpdate,
    NotificationDelete,
    PartnerCreate,
    PartnerRead,
    PartnerUpdate,
    PartnerDelete,
    PersonCreate,
    PersonRead,
    PersonUpdate,
    PersonDelete,
    PersonStatistics,
    PersonMerge,
    PersonReassign,
    PinCodeCreate,
    PinCodeUpdate,
    PinCodeDelete,
    PluginCreate,
    PluginRead,
    PluginUpdate,
    PluginDelete,
    ServerAbout,
    ServerApkLinks,
    ServerStorage,
    ServerStatistics,
    ServerVersionCheck,
    ServerLicenseRead,
    ServerLicenseUpdate,
    ServerLicenseDelete,
    SessionCreate,
    SessionRead,
    SessionUpdate,
    SessionDelete,
    SessionLock,
    SharedLinkCreate,
    SharedLinkRead,
    SharedLinkUpdate,
    SharedLinkDelete,
    StackCreate,
    StackRead,
    StackUpdate,
    StackDelete,
    SyncStream,
    SyncCheckpointRead,
    SyncCheckpointUpdate,
    SyncCheckpointDelete,
    ConfigRead,
    ConfigUpdate,
    SystemMetadataRead,
    SystemMetadataUpdate,
    TagCreate,
    TagRead,
    TagUpdate,
    TagDelete,
    TagAsset,
    UserRead,
    UserUpdate,
    UserLicenseCreate,
    UserLicenseRead,
    UserLicenseUpdate,
    UserLicenseDelete,
    UserOnboardingRead,
    UserOnboardingUpdate,
    UserOnboardingDelete,
    UserPreferenceRead,
    UserPreferenceUpdate,
    UserProfileImageCreate,
    UserProfileImageRead,
    UserProfileImageUpdate,
    UserProfileImageDelete,
    QueueRead,
    QueueUpdate,
    QueueJobCreate,
    QueueJobRead,
    QueueJobUpdate,
    QueueJobDelete,
    WorkflowCreate,
    WorkflowRead,
    WorkflowUpdate,
    WorkflowDelete,
    WorkflowLogs,
    AdminUserCreate,
    AdminUserRead,
    AdminUserUpdate,
    AdminUserDelete,
    AdminSessionRead,
    AdminAuthUnlinkAll,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl Permission {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::All => "all",
            Self::ActivityCreate => "activity.create",
            Self::ActivityRead => "activity.read",
            Self::ActivityUpdate => "activity.update",
            Self::ActivityDelete => "activity.delete",
            Self::ActivityStatistics => "activity.statistics",
            Self::ApiKeyCreate => "apiKey.create",
            Self::ApiKeyRead => "apiKey.read",
            Self::ApiKeyUpdate => "apiKey.update",
            Self::ApiKeyDelete => "apiKey.delete",
            Self::ApiKeyRotate => "apiKey.rotate",
            Self::AssetRead => "asset.read",
            Self::AssetUpdate => "asset.update",
            Self::AssetDelete => "asset.delete",
            Self::AssetStatistics => "asset.statistics",
            Self::AssetShare => "asset.share",
            Self::AssetView => "asset.view",
            Self::AssetDownload => "asset.download",
            Self::AssetUpload => "asset.upload",
            Self::AssetCopy => "asset.copy",
            Self::AssetDerive => "asset.derive",
            Self::AssetFileRead => "assetFile.read",
            Self::AssetFileDelete => "assetFile.delete",
            Self::AssetFileDownload => "assetFile.download",
            Self::AssetEditGet => "asset.edit.get",
            Self::AssetEditCreate => "asset.edit.create",
            Self::AssetEditDelete => "asset.edit.delete",
            Self::AlbumCreate => "album.create",
            Self::AlbumRead => "album.read",
            Self::AlbumUpdate => "album.update",
            Self::AlbumDelete => "album.delete",
            Self::AlbumStatistics => "album.statistics",
            Self::AlbumShare => "album.share",
            Self::AlbumDownload => "album.download",
            Self::AlbumAssetCreate => "albumAsset.create",
            Self::AlbumAssetDelete => "albumAsset.delete",
            Self::AlbumUserCreate => "albumUser.create",
            Self::AlbumUserUpdate => "albumUser.update",
            Self::AlbumUserDelete => "albumUser.delete",
            Self::AuthChangePassword => "auth.changePassword",
            Self::AuthDeviceDelete => "authDevice.delete",
            Self::ArchiveRead => "archive.read",
            Self::BackupList => "backup.list",
            Self::BackupDownload => "backup.download",
            Self::BackupUpload => "backup.upload",
            Self::BackupDelete => "backup.delete",
            Self::ClusterGroupRead => "clusterGroup.read",
            Self::ClusterGroupLeave => "clusterGroup.leave",
            Self::ClusterGroupRequestCreate => "clusterGroupRequest.create",
            Self::ClusterGroupRequestRead => "clusterGroupRequest.read",
            Self::ClusterGroupRequestDelete => "clusterGroupRequest.delete",
            Self::AdminConfigRead => "adminConfig.read",
            Self::AdminConfigUpdate => "adminConfig.update",
            Self::UserConfigRead => "userConfig.read",
            Self::DuplicateRead => "duplicate.read",
            Self::DuplicateDelete => "duplicate.delete",
            Self::FaceCreate => "face.create",
            Self::FaceRead => "face.read",
            Self::FaceUpdate => "face.update",
            Self::FaceDelete => "face.delete",
            Self::FolderRead => "folder.read",
            Self::JobCreate => "job.create",
            Self::JobRead => "job.read",
            Self::LibraryCreate => "library.create",
            Self::LibraryRead => "library.read",
            Self::LibraryUpdate => "library.update",
            Self::LibraryDelete => "library.delete",
            Self::LibraryStatistics => "library.statistics",
            Self::TimelineRead => "timeline.read",
            Self::TimelineDownload => "timeline.download",
            Self::Maintenance => "maintenance",
            Self::MapRead => "map.read",
            Self::MapSearch => "map.search",
            Self::MemoryCreate => "memory.create",
            Self::MemoryRead => "memory.read",
            Self::MemoryUpdate => "memory.update",
            Self::MemoryDelete => "memory.delete",
            Self::MemoryStatistics => "memory.statistics",
            Self::MemoryAssetCreate => "memoryAsset.create",
            Self::MemoryAssetDelete => "memoryAsset.delete",
            Self::NotificationCreate => "notification.create",
            Self::NotificationRead => "notification.read",
            Self::NotificationUpdate => "notification.update",
            Self::NotificationDelete => "notification.delete",
            Self::PartnerCreate => "partner.create",
            Self::PartnerRead => "partner.read",
            Self::PartnerUpdate => "partner.update",
            Self::PartnerDelete => "partner.delete",
            Self::PersonCreate => "person.create",
            Self::PersonRead => "person.read",
            Self::PersonUpdate => "person.update",
            Self::PersonDelete => "person.delete",
            Self::PersonStatistics => "person.statistics",
            Self::PersonMerge => "person.merge",
            Self::PersonReassign => "person.reassign",
            Self::PinCodeCreate => "pinCode.create",
            Self::PinCodeUpdate => "pinCode.update",
            Self::PinCodeDelete => "pinCode.delete",
            Self::PluginCreate => "plugin.create",
            Self::PluginRead => "plugin.read",
            Self::PluginUpdate => "plugin.update",
            Self::PluginDelete => "plugin.delete",
            Self::ServerAbout => "server.about",
            Self::ServerApkLinks => "server.apkLinks",
            Self::ServerStorage => "server.storage",
            Self::ServerStatistics => "server.statistics",
            Self::ServerVersionCheck => "server.versionCheck",
            Self::ServerLicenseRead => "serverLicense.read",
            Self::ServerLicenseUpdate => "serverLicense.update",
            Self::ServerLicenseDelete => "serverLicense.delete",
            Self::SessionCreate => "session.create",
            Self::SessionRead => "session.read",
            Self::SessionUpdate => "session.update",
            Self::SessionDelete => "session.delete",
            Self::SessionLock => "session.lock",
            Self::SharedLinkCreate => "sharedLink.create",
            Self::SharedLinkRead => "sharedLink.read",
            Self::SharedLinkUpdate => "sharedLink.update",
            Self::SharedLinkDelete => "sharedLink.delete",
            Self::StackCreate => "stack.create",
            Self::StackRead => "stack.read",
            Self::StackUpdate => "stack.update",
            Self::StackDelete => "stack.delete",
            Self::SyncStream => "sync.stream",
            Self::SyncCheckpointRead => "syncCheckpoint.read",
            Self::SyncCheckpointUpdate => "syncCheckpoint.update",
            Self::SyncCheckpointDelete => "syncCheckpoint.delete",
            Self::ConfigRead => "systemConfig.read",
            Self::ConfigUpdate => "systemConfig.update",
            Self::SystemMetadataRead => "systemMetadata.read",
            Self::SystemMetadataUpdate => "systemMetadata.update",
            Self::TagCreate => "tag.create",
            Self::TagRead => "tag.read",
            Self::TagUpdate => "tag.update",
            Self::TagDelete => "tag.delete",
            Self::TagAsset => "tag.asset",
            Self::UserRead => "user.read",
            Self::UserUpdate => "user.update",
            Self::UserLicenseCreate => "userLicense.create",
            Self::UserLicenseRead => "userLicense.read",
            Self::UserLicenseUpdate => "userLicense.update",
            Self::UserLicenseDelete => "userLicense.delete",
            Self::UserOnboardingRead => "userOnboarding.read",
            Self::UserOnboardingUpdate => "userOnboarding.update",
            Self::UserOnboardingDelete => "userOnboarding.delete",
            Self::UserPreferenceRead => "userPreference.read",
            Self::UserPreferenceUpdate => "userPreference.update",
            Self::UserProfileImageCreate => "userProfileImage.create",
            Self::UserProfileImageRead => "userProfileImage.read",
            Self::UserProfileImageUpdate => "userProfileImage.update",
            Self::UserProfileImageDelete => "userProfileImage.delete",
            Self::QueueRead => "queue.read",
            Self::QueueUpdate => "queue.update",
            Self::QueueJobCreate => "queueJob.create",
            Self::QueueJobRead => "queueJob.read",
            Self::QueueJobUpdate => "queueJob.update",
            Self::QueueJobDelete => "queueJob.delete",
            Self::WorkflowCreate => "workflow.create",
            Self::WorkflowRead => "workflow.read",
            Self::WorkflowUpdate => "workflow.update",
            Self::WorkflowDelete => "workflow.delete",
            Self::WorkflowLogs => "workflow.logs",
            Self::AdminUserCreate => "adminUser.create",
            Self::AdminUserRead => "adminUser.read",
            Self::AdminUserUpdate => "adminUser.update",
            Self::AdminUserDelete => "adminUser.delete",
            Self::AdminSessionRead => "adminSession.read",
            Self::AdminAuthUnlinkAll => "adminAuth.unlinkAll",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for Permission {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for Permission {
    fn from(value: &str) -> Self {
        match value {
            "all" => Self::All,
            "activity.create" => Self::ActivityCreate,
            "activity.read" => Self::ActivityRead,
            "activity.update" => Self::ActivityUpdate,
            "activity.delete" => Self::ActivityDelete,
            "activity.statistics" => Self::ActivityStatistics,
            "apiKey.create" => Self::ApiKeyCreate,
            "apiKey.read" => Self::ApiKeyRead,
            "apiKey.update" => Self::ApiKeyUpdate,
            "apiKey.delete" => Self::ApiKeyDelete,
            "apiKey.rotate" => Self::ApiKeyRotate,
            "asset.read" => Self::AssetRead,
            "asset.update" => Self::AssetUpdate,
            "asset.delete" => Self::AssetDelete,
            "asset.statistics" => Self::AssetStatistics,
            "asset.share" => Self::AssetShare,
            "asset.view" => Self::AssetView,
            "asset.download" => Self::AssetDownload,
            "asset.upload" => Self::AssetUpload,
            "asset.copy" => Self::AssetCopy,
            "asset.derive" => Self::AssetDerive,
            "assetFile.read" => Self::AssetFileRead,
            "assetFile.delete" => Self::AssetFileDelete,
            "assetFile.download" => Self::AssetFileDownload,
            "asset.edit.get" => Self::AssetEditGet,
            "asset.edit.create" => Self::AssetEditCreate,
            "asset.edit.delete" => Self::AssetEditDelete,
            "album.create" => Self::AlbumCreate,
            "album.read" => Self::AlbumRead,
            "album.update" => Self::AlbumUpdate,
            "album.delete" => Self::AlbumDelete,
            "album.statistics" => Self::AlbumStatistics,
            "album.share" => Self::AlbumShare,
            "album.download" => Self::AlbumDownload,
            "albumAsset.create" => Self::AlbumAssetCreate,
            "albumAsset.delete" => Self::AlbumAssetDelete,
            "albumUser.create" => Self::AlbumUserCreate,
            "albumUser.update" => Self::AlbumUserUpdate,
            "albumUser.delete" => Self::AlbumUserDelete,
            "auth.changePassword" => Self::AuthChangePassword,
            "authDevice.delete" => Self::AuthDeviceDelete,
            "archive.read" => Self::ArchiveRead,
            "backup.list" => Self::BackupList,
            "backup.download" => Self::BackupDownload,
            "backup.upload" => Self::BackupUpload,
            "backup.delete" => Self::BackupDelete,
            "clusterGroup.read" => Self::ClusterGroupRead,
            "clusterGroup.leave" => Self::ClusterGroupLeave,
            "clusterGroupRequest.create" => Self::ClusterGroupRequestCreate,
            "clusterGroupRequest.read" => Self::ClusterGroupRequestRead,
            "clusterGroupRequest.delete" => Self::ClusterGroupRequestDelete,
            "adminConfig.read" => Self::AdminConfigRead,
            "adminConfig.update" => Self::AdminConfigUpdate,
            "userConfig.read" => Self::UserConfigRead,
            "duplicate.read" => Self::DuplicateRead,
            "duplicate.delete" => Self::DuplicateDelete,
            "face.create" => Self::FaceCreate,
            "face.read" => Self::FaceRead,
            "face.update" => Self::FaceUpdate,
            "face.delete" => Self::FaceDelete,
            "folder.read" => Self::FolderRead,
            "job.create" => Self::JobCreate,
            "job.read" => Self::JobRead,
            "library.create" => Self::LibraryCreate,
            "library.read" => Self::LibraryRead,
            "library.update" => Self::LibraryUpdate,
            "library.delete" => Self::LibraryDelete,
            "library.statistics" => Self::LibraryStatistics,
            "timeline.read" => Self::TimelineRead,
            "timeline.download" => Self::TimelineDownload,
            "maintenance" => Self::Maintenance,
            "map.read" => Self::MapRead,
            "map.search" => Self::MapSearch,
            "memory.create" => Self::MemoryCreate,
            "memory.read" => Self::MemoryRead,
            "memory.update" => Self::MemoryUpdate,
            "memory.delete" => Self::MemoryDelete,
            "memory.statistics" => Self::MemoryStatistics,
            "memoryAsset.create" => Self::MemoryAssetCreate,
            "memoryAsset.delete" => Self::MemoryAssetDelete,
            "notification.create" => Self::NotificationCreate,
            "notification.read" => Self::NotificationRead,
            "notification.update" => Self::NotificationUpdate,
            "notification.delete" => Self::NotificationDelete,
            "partner.create" => Self::PartnerCreate,
            "partner.read" => Self::PartnerRead,
            "partner.update" => Self::PartnerUpdate,
            "partner.delete" => Self::PartnerDelete,
            "person.create" => Self::PersonCreate,
            "person.read" => Self::PersonRead,
            "person.update" => Self::PersonUpdate,
            "person.delete" => Self::PersonDelete,
            "person.statistics" => Self::PersonStatistics,
            "person.merge" => Self::PersonMerge,
            "person.reassign" => Self::PersonReassign,
            "pinCode.create" => Self::PinCodeCreate,
            "pinCode.update" => Self::PinCodeUpdate,
            "pinCode.delete" => Self::PinCodeDelete,
            "plugin.create" => Self::PluginCreate,
            "plugin.read" => Self::PluginRead,
            "plugin.update" => Self::PluginUpdate,
            "plugin.delete" => Self::PluginDelete,
            "server.about" => Self::ServerAbout,
            "server.apkLinks" => Self::ServerApkLinks,
            "server.storage" => Self::ServerStorage,
            "server.statistics" => Self::ServerStatistics,
            "server.versionCheck" => Self::ServerVersionCheck,
            "serverLicense.read" => Self::ServerLicenseRead,
            "serverLicense.update" => Self::ServerLicenseUpdate,
            "serverLicense.delete" => Self::ServerLicenseDelete,
            "session.create" => Self::SessionCreate,
            "session.read" => Self::SessionRead,
            "session.update" => Self::SessionUpdate,
            "session.delete" => Self::SessionDelete,
            "session.lock" => Self::SessionLock,
            "sharedLink.create" => Self::SharedLinkCreate,
            "sharedLink.read" => Self::SharedLinkRead,
            "sharedLink.update" => Self::SharedLinkUpdate,
            "sharedLink.delete" => Self::SharedLinkDelete,
            "stack.create" => Self::StackCreate,
            "stack.read" => Self::StackRead,
            "stack.update" => Self::StackUpdate,
            "stack.delete" => Self::StackDelete,
            "sync.stream" => Self::SyncStream,
            "syncCheckpoint.read" => Self::SyncCheckpointRead,
            "syncCheckpoint.update" => Self::SyncCheckpointUpdate,
            "syncCheckpoint.delete" => Self::SyncCheckpointDelete,
            "systemConfig.read" => Self::ConfigRead,
            "systemConfig.update" => Self::ConfigUpdate,
            "systemMetadata.read" => Self::SystemMetadataRead,
            "systemMetadata.update" => Self::SystemMetadataUpdate,
            "tag.create" => Self::TagCreate,
            "tag.read" => Self::TagRead,
            "tag.update" => Self::TagUpdate,
            "tag.delete" => Self::TagDelete,
            "tag.asset" => Self::TagAsset,
            "user.read" => Self::UserRead,
            "user.update" => Self::UserUpdate,
            "userLicense.create" => Self::UserLicenseCreate,
            "userLicense.read" => Self::UserLicenseRead,
            "userLicense.update" => Self::UserLicenseUpdate,
            "userLicense.delete" => Self::UserLicenseDelete,
            "userOnboarding.read" => Self::UserOnboardingRead,
            "userOnboarding.update" => Self::UserOnboardingUpdate,
            "userOnboarding.delete" => Self::UserOnboardingDelete,
            "userPreference.read" => Self::UserPreferenceRead,
            "userPreference.update" => Self::UserPreferenceUpdate,
            "userProfileImage.create" => Self::UserProfileImageCreate,
            "userProfileImage.read" => Self::UserProfileImageRead,
            "userProfileImage.update" => Self::UserProfileImageUpdate,
            "userProfileImage.delete" => Self::UserProfileImageDelete,
            "queue.read" => Self::QueueRead,
            "queue.update" => Self::QueueUpdate,
            "queueJob.create" => Self::QueueJobCreate,
            "queueJob.read" => Self::QueueJobRead,
            "queueJob.update" => Self::QueueJobUpdate,
            "queueJob.delete" => Self::QueueJobDelete,
            "workflow.create" => Self::WorkflowCreate,
            "workflow.read" => Self::WorkflowRead,
            "workflow.update" => Self::WorkflowUpdate,
            "workflow.delete" => Self::WorkflowDelete,
            "workflow.logs" => Self::WorkflowLogs,
            "adminUser.create" => Self::AdminUserCreate,
            "adminUser.read" => Self::AdminUserRead,
            "adminUser.update" => Self::AdminUserUpdate,
            "adminUser.delete" => Self::AdminUserDelete,
            "adminSession.read" => Self::AdminSessionRead,
            "adminAuth.unlinkAll" => Self::AdminAuthUnlinkAll,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for Permission {
    fn from(value: String) -> Self {
        match value.as_str() {
            "all" => Self::All,
            "activity.create" => Self::ActivityCreate,
            "activity.read" => Self::ActivityRead,
            "activity.update" => Self::ActivityUpdate,
            "activity.delete" => Self::ActivityDelete,
            "activity.statistics" => Self::ActivityStatistics,
            "apiKey.create" => Self::ApiKeyCreate,
            "apiKey.read" => Self::ApiKeyRead,
            "apiKey.update" => Self::ApiKeyUpdate,
            "apiKey.delete" => Self::ApiKeyDelete,
            "apiKey.rotate" => Self::ApiKeyRotate,
            "asset.read" => Self::AssetRead,
            "asset.update" => Self::AssetUpdate,
            "asset.delete" => Self::AssetDelete,
            "asset.statistics" => Self::AssetStatistics,
            "asset.share" => Self::AssetShare,
            "asset.view" => Self::AssetView,
            "asset.download" => Self::AssetDownload,
            "asset.upload" => Self::AssetUpload,
            "asset.copy" => Self::AssetCopy,
            "asset.derive" => Self::AssetDerive,
            "assetFile.read" => Self::AssetFileRead,
            "assetFile.delete" => Self::AssetFileDelete,
            "assetFile.download" => Self::AssetFileDownload,
            "asset.edit.get" => Self::AssetEditGet,
            "asset.edit.create" => Self::AssetEditCreate,
            "asset.edit.delete" => Self::AssetEditDelete,
            "album.create" => Self::AlbumCreate,
            "album.read" => Self::AlbumRead,
            "album.update" => Self::AlbumUpdate,
            "album.delete" => Self::AlbumDelete,
            "album.statistics" => Self::AlbumStatistics,
            "album.share" => Self::AlbumShare,
            "album.download" => Self::AlbumDownload,
            "albumAsset.create" => Self::AlbumAssetCreate,
            "albumAsset.delete" => Self::AlbumAssetDelete,
            "albumUser.create" => Self::AlbumUserCreate,
            "albumUser.update" => Self::AlbumUserUpdate,
            "albumUser.delete" => Self::AlbumUserDelete,
            "auth.changePassword" => Self::AuthChangePassword,
            "authDevice.delete" => Self::AuthDeviceDelete,
            "archive.read" => Self::ArchiveRead,
            "backup.list" => Self::BackupList,
            "backup.download" => Self::BackupDownload,
            "backup.upload" => Self::BackupUpload,
            "backup.delete" => Self::BackupDelete,
            "clusterGroup.read" => Self::ClusterGroupRead,
            "clusterGroup.leave" => Self::ClusterGroupLeave,
            "clusterGroupRequest.create" => Self::ClusterGroupRequestCreate,
            "clusterGroupRequest.read" => Self::ClusterGroupRequestRead,
            "clusterGroupRequest.delete" => Self::ClusterGroupRequestDelete,
            "adminConfig.read" => Self::AdminConfigRead,
            "adminConfig.update" => Self::AdminConfigUpdate,
            "userConfig.read" => Self::UserConfigRead,
            "duplicate.read" => Self::DuplicateRead,
            "duplicate.delete" => Self::DuplicateDelete,
            "face.create" => Self::FaceCreate,
            "face.read" => Self::FaceRead,
            "face.update" => Self::FaceUpdate,
            "face.delete" => Self::FaceDelete,
            "folder.read" => Self::FolderRead,
            "job.create" => Self::JobCreate,
            "job.read" => Self::JobRead,
            "library.create" => Self::LibraryCreate,
            "library.read" => Self::LibraryRead,
            "library.update" => Self::LibraryUpdate,
            "library.delete" => Self::LibraryDelete,
            "library.statistics" => Self::LibraryStatistics,
            "timeline.read" => Self::TimelineRead,
            "timeline.download" => Self::TimelineDownload,
            "maintenance" => Self::Maintenance,
            "map.read" => Self::MapRead,
            "map.search" => Self::MapSearch,
            "memory.create" => Self::MemoryCreate,
            "memory.read" => Self::MemoryRead,
            "memory.update" => Self::MemoryUpdate,
            "memory.delete" => Self::MemoryDelete,
            "memory.statistics" => Self::MemoryStatistics,
            "memoryAsset.create" => Self::MemoryAssetCreate,
            "memoryAsset.delete" => Self::MemoryAssetDelete,
            "notification.create" => Self::NotificationCreate,
            "notification.read" => Self::NotificationRead,
            "notification.update" => Self::NotificationUpdate,
            "notification.delete" => Self::NotificationDelete,
            "partner.create" => Self::PartnerCreate,
            "partner.read" => Self::PartnerRead,
            "partner.update" => Self::PartnerUpdate,
            "partner.delete" => Self::PartnerDelete,
            "person.create" => Self::PersonCreate,
            "person.read" => Self::PersonRead,
            "person.update" => Self::PersonUpdate,
            "person.delete" => Self::PersonDelete,
            "person.statistics" => Self::PersonStatistics,
            "person.merge" => Self::PersonMerge,
            "person.reassign" => Self::PersonReassign,
            "pinCode.create" => Self::PinCodeCreate,
            "pinCode.update" => Self::PinCodeUpdate,
            "pinCode.delete" => Self::PinCodeDelete,
            "plugin.create" => Self::PluginCreate,
            "plugin.read" => Self::PluginRead,
            "plugin.update" => Self::PluginUpdate,
            "plugin.delete" => Self::PluginDelete,
            "server.about" => Self::ServerAbout,
            "server.apkLinks" => Self::ServerApkLinks,
            "server.storage" => Self::ServerStorage,
            "server.statistics" => Self::ServerStatistics,
            "server.versionCheck" => Self::ServerVersionCheck,
            "serverLicense.read" => Self::ServerLicenseRead,
            "serverLicense.update" => Self::ServerLicenseUpdate,
            "serverLicense.delete" => Self::ServerLicenseDelete,
            "session.create" => Self::SessionCreate,
            "session.read" => Self::SessionRead,
            "session.update" => Self::SessionUpdate,
            "session.delete" => Self::SessionDelete,
            "session.lock" => Self::SessionLock,
            "sharedLink.create" => Self::SharedLinkCreate,
            "sharedLink.read" => Self::SharedLinkRead,
            "sharedLink.update" => Self::SharedLinkUpdate,
            "sharedLink.delete" => Self::SharedLinkDelete,
            "stack.create" => Self::StackCreate,
            "stack.read" => Self::StackRead,
            "stack.update" => Self::StackUpdate,
            "stack.delete" => Self::StackDelete,
            "sync.stream" => Self::SyncStream,
            "syncCheckpoint.read" => Self::SyncCheckpointRead,
            "syncCheckpoint.update" => Self::SyncCheckpointUpdate,
            "syncCheckpoint.delete" => Self::SyncCheckpointDelete,
            "systemConfig.read" => Self::ConfigRead,
            "systemConfig.update" => Self::ConfigUpdate,
            "systemMetadata.read" => Self::SystemMetadataRead,
            "systemMetadata.update" => Self::SystemMetadataUpdate,
            "tag.create" => Self::TagCreate,
            "tag.read" => Self::TagRead,
            "tag.update" => Self::TagUpdate,
            "tag.delete" => Self::TagDelete,
            "tag.asset" => Self::TagAsset,
            "user.read" => Self::UserRead,
            "user.update" => Self::UserUpdate,
            "userLicense.create" => Self::UserLicenseCreate,
            "userLicense.read" => Self::UserLicenseRead,
            "userLicense.update" => Self::UserLicenseUpdate,
            "userLicense.delete" => Self::UserLicenseDelete,
            "userOnboarding.read" => Self::UserOnboardingRead,
            "userOnboarding.update" => Self::UserOnboardingUpdate,
            "userOnboarding.delete" => Self::UserOnboardingDelete,
            "userPreference.read" => Self::UserPreferenceRead,
            "userPreference.update" => Self::UserPreferenceUpdate,
            "userProfileImage.create" => Self::UserProfileImageCreate,
            "userProfileImage.read" => Self::UserProfileImageRead,
            "userProfileImage.update" => Self::UserProfileImageUpdate,
            "userProfileImage.delete" => Self::UserProfileImageDelete,
            "queue.read" => Self::QueueRead,
            "queue.update" => Self::QueueUpdate,
            "queueJob.create" => Self::QueueJobCreate,
            "queueJob.read" => Self::QueueJobRead,
            "queueJob.update" => Self::QueueJobUpdate,
            "queueJob.delete" => Self::QueueJobDelete,
            "workflow.create" => Self::WorkflowCreate,
            "workflow.read" => Self::WorkflowRead,
            "workflow.update" => Self::WorkflowUpdate,
            "workflow.delete" => Self::WorkflowDelete,
            "workflow.logs" => Self::WorkflowLogs,
            "adminUser.create" => Self::AdminUserCreate,
            "adminUser.read" => Self::AdminUserRead,
            "adminUser.update" => Self::AdminUserUpdate,
            "adminUser.delete" => Self::AdminUserDelete,
            "adminSession.read" => Self::AdminSessionRead,
            "adminAuth.unlinkAll" => Self::AdminAuthUnlinkAll,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for Permission {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for Permission {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for Permission {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for Permission {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = Permission;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(Permission::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(Permission::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}
