// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! `OpenAPI` data models for `api_keys`.

/// `ApiKeyCreateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `name` | [`String`] |  | API key name |
/// | `permissions` | [`Vec<Permission>`](crate::api_keys::Permission) | ✓ | List of permissions |
///
/// See: <https://api.immich.app/models/ApiKeyCreateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct ApiKeyCreateDTO {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: Option<String>,
    pub permissions: Vec<Permission>,
}

impl ApiKeyCreateDTO {
    ///Creates a new [`ApiKeyCreateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(permissions: Vec<Permission>) -> Self {
        Self {
            name: None,
            permissions,
        }
    }

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    ///Sets or clears `name`.
    #[must_use]
    pub fn set_name(mut self, value: Option<String>) -> Self {
        self.name = value;
        self
    }

    ///Sets `permissions`.
    #[must_use]
    pub fn permissions(mut self, value: Vec<Permission>) -> Self {
        self.permissions = value;
        self
    }
}

/// `ApiKeyCreateResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `api_key` | [`ApiKeyResponseDTO`] | ✓ |  |
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Creation date |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | API key ID |
/// | `name` | [`String`] | ✓ | API key name |
/// | `permissions` | [`Vec<Permission>`](crate::api_keys::Permission) | ✓ | List of permissions |
/// | `secret` | [`String`] | ✓ | API key secret (only shown once) |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Last update date |
///
/// See: <https://api.immich.app/models/ApiKeyCreateResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct ApiKeyCreateResponseDTO {
    /// (required)
    #[serde(rename = "apiKey")]
    pub api_key: ApiKeyResponseDTO,
    /// Creation date (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub id: ::uuid::Uuid,
    pub name: String,
    pub permissions: Vec<Permission>,
    pub secret: String,
    /// Last update date (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl ApiKeyCreateResponseDTO {
    ///Creates a new [`ApiKeyCreateResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        api_key: ApiKeyResponseDTO,
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        id: ::uuid::Uuid,
        name: impl Into<String>,
        permissions: Vec<Permission>,
        secret: impl Into<String>,
        updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ) -> Self {
        Self {
            api_key,
            created_at,
            id,
            name: name.into(),
            permissions,
            secret: secret.into(),
            updated_at,
        }
    }

    ///Sets `api_key`.
    #[must_use]
    pub fn api_key(mut self, value: ApiKeyResponseDTO) -> Self {
        self.api_key = value;
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

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }

    ///Sets `permissions`.
    #[must_use]
    pub fn permissions(mut self, value: Vec<Permission>) -> Self {
        self.permissions = value;
        self
    }

    ///Sets `secret`.
    #[must_use]
    pub fn secret(mut self, value: impl Into<String>) -> Self {
        self.secret = value.into();
        self
    }

    ///Sets `updated_at`.
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
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Creation date |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | API key ID |
/// | `name` | [`String`] | ✓ | API key name |
/// | `permissions` | [`Vec<Permission>`](crate::api_keys::Permission) | ✓ | List of permissions |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Last update date |
///
/// See: <https://api.immich.app/models/ApiKeyResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct ApiKeyResponseDTO {
    /// Creation date (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub id: ::uuid::Uuid,
    pub name: String,
    pub permissions: Vec<Permission>,
    /// Last update date (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl ApiKeyResponseDTO {
    ///Creates a new [`ApiKeyResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        id: ::uuid::Uuid,
        name: impl Into<String>,
        permissions: Vec<Permission>,
        updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ) -> Self {
        Self {
            created_at,
            id,
            name: name.into(),
            permissions,
            updated_at,
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

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }

    ///Sets `permissions`.
    #[must_use]
    pub fn permissions(mut self, value: Vec<Permission>) -> Self {
        self.permissions = value;
        self
    }

    ///Sets `updated_at`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct ApiKeyUpdateDTO {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub permissions: Vec<Permission>,
}

impl ApiKeyUpdateDTO {
    ///Creates a new [`ApiKeyUpdateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            name: None,
            permissions: Vec::new(),
        }
    }

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    ///Sets or clears `name`.
    #[must_use]
    pub fn set_name(mut self, value: Option<String>) -> Self {
        self.name = value;
        self
    }

    ///Sets `permissions`.
    #[must_use]
    pub fn permissions(mut self, value: Vec<Permission>) -> Self {
        self.permissions = value;
        self
    }
}

/// List of permissions
///
/// See: <https://api.immich.app/models/Permission>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum Permission {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "activity.create")]
    ActivityCreate,
    #[serde(rename = "activity.read")]
    ActivityRead,
    #[serde(rename = "activity.update")]
    ActivityUpdate,
    #[serde(rename = "activity.delete")]
    ActivityDelete,
    #[serde(rename = "activity.statistics")]
    ActivityStatistics,
    #[serde(rename = "apiKey.create")]
    ApiKeyCreate,
    #[serde(rename = "apiKey.read")]
    ApiKeyRead,
    #[serde(rename = "apiKey.update")]
    ApiKeyUpdate,
    #[serde(rename = "apiKey.delete")]
    ApiKeyDelete,
    #[serde(rename = "apiKey.rotate")]
    ApiKeyRotate,
    #[serde(rename = "asset.read")]
    AssetRead,
    #[serde(rename = "asset.update")]
    AssetUpdate,
    #[serde(rename = "asset.delete")]
    AssetDelete,
    #[serde(rename = "asset.statistics")]
    AssetStatistics,
    #[serde(rename = "asset.share")]
    AssetShare,
    #[serde(rename = "asset.view")]
    AssetView,
    #[serde(rename = "asset.download")]
    AssetDownload,
    #[serde(rename = "asset.upload")]
    AssetUpload,
    #[serde(rename = "asset.copy")]
    AssetCopy,
    #[serde(rename = "asset.derive")]
    AssetDerive,
    #[serde(rename = "assetFile.read")]
    AssetFileRead,
    #[serde(rename = "assetFile.delete")]
    AssetFileDelete,
    #[serde(rename = "assetFile.download")]
    AssetFileDownload,
    #[serde(rename = "asset.edit.get")]
    AssetEditGet,
    #[serde(rename = "asset.edit.create")]
    AssetEditCreate,
    #[serde(rename = "asset.edit.delete")]
    AssetEditDelete,
    #[serde(rename = "album.create")]
    AlbumCreate,
    #[serde(rename = "album.read")]
    AlbumRead,
    #[serde(rename = "album.update")]
    AlbumUpdate,
    #[serde(rename = "album.delete")]
    AlbumDelete,
    #[serde(rename = "album.statistics")]
    AlbumStatistics,
    #[serde(rename = "album.share")]
    AlbumShare,
    #[serde(rename = "album.download")]
    AlbumDownload,
    #[serde(rename = "albumAsset.create")]
    AlbumAssetCreate,
    #[serde(rename = "albumAsset.delete")]
    AlbumAssetDelete,
    #[serde(rename = "albumUser.create")]
    AlbumUserCreate,
    #[serde(rename = "albumUser.update")]
    AlbumUserUpdate,
    #[serde(rename = "albumUser.delete")]
    AlbumUserDelete,
    #[serde(rename = "auth.changePassword")]
    AuthChangePassword,
    #[serde(rename = "authDevice.delete")]
    AuthDeviceDelete,
    #[serde(rename = "archive.read")]
    ArchiveRead,
    #[serde(rename = "backup.list")]
    BackupList,
    #[serde(rename = "backup.download")]
    BackupDownload,
    #[serde(rename = "backup.upload")]
    BackupUpload,
    #[serde(rename = "backup.delete")]
    BackupDelete,
    #[serde(rename = "clusterGroup.read")]
    ClusterGroupRead,
    #[serde(rename = "clusterGroup.leave")]
    ClusterGroupLeave,
    #[serde(rename = "clusterGroupRequest.create")]
    ClusterGroupRequestCreate,
    #[serde(rename = "clusterGroupRequest.read")]
    ClusterGroupRequestRead,
    #[serde(rename = "clusterGroupRequest.delete")]
    ClusterGroupRequestDelete,
    #[serde(rename = "adminConfig.read")]
    AdminConfigRead,
    #[serde(rename = "adminConfig.update")]
    AdminConfigUpdate,
    #[serde(rename = "userConfig.read")]
    UserConfigRead,
    #[serde(rename = "duplicate.read")]
    DuplicateRead,
    #[serde(rename = "duplicate.delete")]
    DuplicateDelete,
    #[serde(rename = "face.create")]
    FaceCreate,
    #[serde(rename = "face.read")]
    FaceRead,
    #[serde(rename = "face.update")]
    FaceUpdate,
    #[serde(rename = "face.delete")]
    FaceDelete,
    #[serde(rename = "folder.read")]
    FolderRead,
    #[serde(rename = "job.create")]
    JobCreate,
    #[serde(rename = "job.read")]
    JobRead,
    #[serde(rename = "library.create")]
    LibraryCreate,
    #[serde(rename = "library.read")]
    LibraryRead,
    #[serde(rename = "library.update")]
    LibraryUpdate,
    #[serde(rename = "library.delete")]
    LibraryDelete,
    #[serde(rename = "library.statistics")]
    LibraryStatistics,
    #[serde(rename = "timeline.read")]
    TimelineRead,
    #[serde(rename = "timeline.download")]
    TimelineDownload,
    #[serde(rename = "maintenance")]
    Maintenance,
    #[serde(rename = "map.read")]
    MapRead,
    #[serde(rename = "map.search")]
    MapSearch,
    #[serde(rename = "memory.create")]
    MemoryCreate,
    #[serde(rename = "memory.read")]
    MemoryRead,
    #[serde(rename = "memory.update")]
    MemoryUpdate,
    #[serde(rename = "memory.delete")]
    MemoryDelete,
    #[serde(rename = "memory.statistics")]
    MemoryStatistics,
    #[serde(rename = "memoryAsset.create")]
    MemoryAssetCreate,
    #[serde(rename = "memoryAsset.delete")]
    MemoryAssetDelete,
    #[serde(rename = "notification.create")]
    NotificationCreate,
    #[serde(rename = "notification.read")]
    NotificationRead,
    #[serde(rename = "notification.update")]
    NotificationUpdate,
    #[serde(rename = "notification.delete")]
    NotificationDelete,
    #[serde(rename = "partner.create")]
    PartnerCreate,
    #[serde(rename = "partner.read")]
    PartnerRead,
    #[serde(rename = "partner.update")]
    PartnerUpdate,
    #[serde(rename = "partner.delete")]
    PartnerDelete,
    #[serde(rename = "person.create")]
    PersonCreate,
    #[serde(rename = "person.read")]
    PersonRead,
    #[serde(rename = "person.update")]
    PersonUpdate,
    #[serde(rename = "person.delete")]
    PersonDelete,
    #[serde(rename = "person.statistics")]
    PersonStatistics,
    #[serde(rename = "person.merge")]
    PersonMerge,
    #[serde(rename = "person.reassign")]
    PersonReassign,
    #[serde(rename = "pinCode.create")]
    PinCodeCreate,
    #[serde(rename = "pinCode.update")]
    PinCodeUpdate,
    #[serde(rename = "pinCode.delete")]
    PinCodeDelete,
    #[serde(rename = "plugin.create")]
    PluginCreate,
    #[serde(rename = "plugin.read")]
    PluginRead,
    #[serde(rename = "plugin.update")]
    PluginUpdate,
    #[serde(rename = "plugin.delete")]
    PluginDelete,
    #[serde(rename = "server.about")]
    ServerAbout,
    #[serde(rename = "server.apkLinks")]
    ServerApkLinks,
    #[serde(rename = "server.storage")]
    ServerStorage,
    #[serde(rename = "server.statistics")]
    ServerStatistics,
    #[serde(rename = "server.versionCheck")]
    ServerVersionCheck,
    #[serde(rename = "serverLicense.read")]
    ServerLicenseRead,
    #[serde(rename = "serverLicense.update")]
    ServerLicenseUpdate,
    #[serde(rename = "serverLicense.delete")]
    ServerLicenseDelete,
    #[serde(rename = "session.create")]
    SessionCreate,
    #[serde(rename = "session.read")]
    SessionRead,
    #[serde(rename = "session.update")]
    SessionUpdate,
    #[serde(rename = "session.delete")]
    SessionDelete,
    #[serde(rename = "session.lock")]
    SessionLock,
    #[serde(rename = "sharedLink.create")]
    SharedLinkCreate,
    #[serde(rename = "sharedLink.read")]
    SharedLinkRead,
    #[serde(rename = "sharedLink.update")]
    SharedLinkUpdate,
    #[serde(rename = "sharedLink.delete")]
    SharedLinkDelete,
    #[serde(rename = "stack.create")]
    StackCreate,
    #[serde(rename = "stack.read")]
    StackRead,
    #[serde(rename = "stack.update")]
    StackUpdate,
    #[serde(rename = "stack.delete")]
    StackDelete,
    #[serde(rename = "sync.stream")]
    SyncStream,
    #[serde(rename = "syncCheckpoint.read")]
    SyncCheckpointRead,
    #[serde(rename = "syncCheckpoint.update")]
    SyncCheckpointUpdate,
    #[serde(rename = "syncCheckpoint.delete")]
    SyncCheckpointDelete,
    #[serde(rename = "systemConfig.read")]
    ConfigRead,
    #[serde(rename = "systemConfig.update")]
    ConfigUpdate,
    #[serde(rename = "systemMetadata.read")]
    SystemMetadataRead,
    #[serde(rename = "systemMetadata.update")]
    SystemMetadataUpdate,
    #[serde(rename = "tag.create")]
    TagCreate,
    #[serde(rename = "tag.read")]
    TagRead,
    #[serde(rename = "tag.update")]
    TagUpdate,
    #[serde(rename = "tag.delete")]
    TagDelete,
    #[serde(rename = "tag.asset")]
    TagAsset,
    #[serde(rename = "user.read")]
    UserRead,
    #[serde(rename = "user.update")]
    UserUpdate,
    #[serde(rename = "userLicense.create")]
    UserLicenseCreate,
    #[serde(rename = "userLicense.read")]
    UserLicenseRead,
    #[serde(rename = "userLicense.update")]
    UserLicenseUpdate,
    #[serde(rename = "userLicense.delete")]
    UserLicenseDelete,
    #[serde(rename = "userOnboarding.read")]
    UserOnboardingRead,
    #[serde(rename = "userOnboarding.update")]
    UserOnboardingUpdate,
    #[serde(rename = "userOnboarding.delete")]
    UserOnboardingDelete,
    #[serde(rename = "userPreference.read")]
    UserPreferenceRead,
    #[serde(rename = "userPreference.update")]
    UserPreferenceUpdate,
    #[serde(rename = "userProfileImage.create")]
    UserProfileImageCreate,
    #[serde(rename = "userProfileImage.read")]
    UserProfileImageRead,
    #[serde(rename = "userProfileImage.update")]
    UserProfileImageUpdate,
    #[serde(rename = "userProfileImage.delete")]
    UserProfileImageDelete,
    #[serde(rename = "queue.read")]
    QueueRead,
    #[serde(rename = "queue.update")]
    QueueUpdate,
    #[serde(rename = "queueJob.create")]
    QueueJobCreate,
    #[serde(rename = "queueJob.read")]
    QueueJobRead,
    #[serde(rename = "queueJob.update")]
    QueueJobUpdate,
    #[serde(rename = "queueJob.delete")]
    QueueJobDelete,
    #[serde(rename = "workflow.create")]
    WorkflowCreate,
    #[serde(rename = "workflow.read")]
    WorkflowRead,
    #[serde(rename = "workflow.update")]
    WorkflowUpdate,
    #[serde(rename = "workflow.delete")]
    WorkflowDelete,
    #[serde(rename = "workflow.logs")]
    WorkflowLogs,
    #[serde(rename = "adminUser.create")]
    AdminUserCreate,
    #[serde(rename = "adminUser.read")]
    AdminUserRead,
    #[serde(rename = "adminUser.update")]
    AdminUserUpdate,
    #[serde(rename = "adminUser.delete")]
    AdminUserDelete,
    #[serde(rename = "adminSession.read")]
    AdminSessionRead,
    #[serde(rename = "adminAuth.unlinkAll")]
    AdminAuthUnlinkAll,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for Permission {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::All => f.write_str("all"),
            Self::ActivityCreate => f.write_str("activity.create"),
            Self::ActivityRead => f.write_str("activity.read"),
            Self::ActivityUpdate => f.write_str("activity.update"),
            Self::ActivityDelete => f.write_str("activity.delete"),
            Self::ActivityStatistics => f.write_str("activity.statistics"),
            Self::ApiKeyCreate => f.write_str("apiKey.create"),
            Self::ApiKeyRead => f.write_str("apiKey.read"),
            Self::ApiKeyUpdate => f.write_str("apiKey.update"),
            Self::ApiKeyDelete => f.write_str("apiKey.delete"),
            Self::ApiKeyRotate => f.write_str("apiKey.rotate"),
            Self::AssetRead => f.write_str("asset.read"),
            Self::AssetUpdate => f.write_str("asset.update"),
            Self::AssetDelete => f.write_str("asset.delete"),
            Self::AssetStatistics => f.write_str("asset.statistics"),
            Self::AssetShare => f.write_str("asset.share"),
            Self::AssetView => f.write_str("asset.view"),
            Self::AssetDownload => f.write_str("asset.download"),
            Self::AssetUpload => f.write_str("asset.upload"),
            Self::AssetCopy => f.write_str("asset.copy"),
            Self::AssetDerive => f.write_str("asset.derive"),
            Self::AssetFileRead => f.write_str("assetFile.read"),
            Self::AssetFileDelete => f.write_str("assetFile.delete"),
            Self::AssetFileDownload => f.write_str("assetFile.download"),
            Self::AssetEditGet => f.write_str("asset.edit.get"),
            Self::AssetEditCreate => f.write_str("asset.edit.create"),
            Self::AssetEditDelete => f.write_str("asset.edit.delete"),
            Self::AlbumCreate => f.write_str("album.create"),
            Self::AlbumRead => f.write_str("album.read"),
            Self::AlbumUpdate => f.write_str("album.update"),
            Self::AlbumDelete => f.write_str("album.delete"),
            Self::AlbumStatistics => f.write_str("album.statistics"),
            Self::AlbumShare => f.write_str("album.share"),
            Self::AlbumDownload => f.write_str("album.download"),
            Self::AlbumAssetCreate => f.write_str("albumAsset.create"),
            Self::AlbumAssetDelete => f.write_str("albumAsset.delete"),
            Self::AlbumUserCreate => f.write_str("albumUser.create"),
            Self::AlbumUserUpdate => f.write_str("albumUser.update"),
            Self::AlbumUserDelete => f.write_str("albumUser.delete"),
            Self::AuthChangePassword => f.write_str("auth.changePassword"),
            Self::AuthDeviceDelete => f.write_str("authDevice.delete"),
            Self::ArchiveRead => f.write_str("archive.read"),
            Self::BackupList => f.write_str("backup.list"),
            Self::BackupDownload => f.write_str("backup.download"),
            Self::BackupUpload => f.write_str("backup.upload"),
            Self::BackupDelete => f.write_str("backup.delete"),
            Self::ClusterGroupRead => f.write_str("clusterGroup.read"),
            Self::ClusterGroupLeave => f.write_str("clusterGroup.leave"),
            Self::ClusterGroupRequestCreate => f.write_str("clusterGroupRequest.create"),
            Self::ClusterGroupRequestRead => f.write_str("clusterGroupRequest.read"),
            Self::ClusterGroupRequestDelete => f.write_str("clusterGroupRequest.delete"),
            Self::AdminConfigRead => f.write_str("adminConfig.read"),
            Self::AdminConfigUpdate => f.write_str("adminConfig.update"),
            Self::UserConfigRead => f.write_str("userConfig.read"),
            Self::DuplicateRead => f.write_str("duplicate.read"),
            Self::DuplicateDelete => f.write_str("duplicate.delete"),
            Self::FaceCreate => f.write_str("face.create"),
            Self::FaceRead => f.write_str("face.read"),
            Self::FaceUpdate => f.write_str("face.update"),
            Self::FaceDelete => f.write_str("face.delete"),
            Self::FolderRead => f.write_str("folder.read"),
            Self::JobCreate => f.write_str("job.create"),
            Self::JobRead => f.write_str("job.read"),
            Self::LibraryCreate => f.write_str("library.create"),
            Self::LibraryRead => f.write_str("library.read"),
            Self::LibraryUpdate => f.write_str("library.update"),
            Self::LibraryDelete => f.write_str("library.delete"),
            Self::LibraryStatistics => f.write_str("library.statistics"),
            Self::TimelineRead => f.write_str("timeline.read"),
            Self::TimelineDownload => f.write_str("timeline.download"),
            Self::Maintenance => f.write_str("maintenance"),
            Self::MapRead => f.write_str("map.read"),
            Self::MapSearch => f.write_str("map.search"),
            Self::MemoryCreate => f.write_str("memory.create"),
            Self::MemoryRead => f.write_str("memory.read"),
            Self::MemoryUpdate => f.write_str("memory.update"),
            Self::MemoryDelete => f.write_str("memory.delete"),
            Self::MemoryStatistics => f.write_str("memory.statistics"),
            Self::MemoryAssetCreate => f.write_str("memoryAsset.create"),
            Self::MemoryAssetDelete => f.write_str("memoryAsset.delete"),
            Self::NotificationCreate => f.write_str("notification.create"),
            Self::NotificationRead => f.write_str("notification.read"),
            Self::NotificationUpdate => f.write_str("notification.update"),
            Self::NotificationDelete => f.write_str("notification.delete"),
            Self::PartnerCreate => f.write_str("partner.create"),
            Self::PartnerRead => f.write_str("partner.read"),
            Self::PartnerUpdate => f.write_str("partner.update"),
            Self::PartnerDelete => f.write_str("partner.delete"),
            Self::PersonCreate => f.write_str("person.create"),
            Self::PersonRead => f.write_str("person.read"),
            Self::PersonUpdate => f.write_str("person.update"),
            Self::PersonDelete => f.write_str("person.delete"),
            Self::PersonStatistics => f.write_str("person.statistics"),
            Self::PersonMerge => f.write_str("person.merge"),
            Self::PersonReassign => f.write_str("person.reassign"),
            Self::PinCodeCreate => f.write_str("pinCode.create"),
            Self::PinCodeUpdate => f.write_str("pinCode.update"),
            Self::PinCodeDelete => f.write_str("pinCode.delete"),
            Self::PluginCreate => f.write_str("plugin.create"),
            Self::PluginRead => f.write_str("plugin.read"),
            Self::PluginUpdate => f.write_str("plugin.update"),
            Self::PluginDelete => f.write_str("plugin.delete"),
            Self::ServerAbout => f.write_str("server.about"),
            Self::ServerApkLinks => f.write_str("server.apkLinks"),
            Self::ServerStorage => f.write_str("server.storage"),
            Self::ServerStatistics => f.write_str("server.statistics"),
            Self::ServerVersionCheck => f.write_str("server.versionCheck"),
            Self::ServerLicenseRead => f.write_str("serverLicense.read"),
            Self::ServerLicenseUpdate => f.write_str("serverLicense.update"),
            Self::ServerLicenseDelete => f.write_str("serverLicense.delete"),
            Self::SessionCreate => f.write_str("session.create"),
            Self::SessionRead => f.write_str("session.read"),
            Self::SessionUpdate => f.write_str("session.update"),
            Self::SessionDelete => f.write_str("session.delete"),
            Self::SessionLock => f.write_str("session.lock"),
            Self::SharedLinkCreate => f.write_str("sharedLink.create"),
            Self::SharedLinkRead => f.write_str("sharedLink.read"),
            Self::SharedLinkUpdate => f.write_str("sharedLink.update"),
            Self::SharedLinkDelete => f.write_str("sharedLink.delete"),
            Self::StackCreate => f.write_str("stack.create"),
            Self::StackRead => f.write_str("stack.read"),
            Self::StackUpdate => f.write_str("stack.update"),
            Self::StackDelete => f.write_str("stack.delete"),
            Self::SyncStream => f.write_str("sync.stream"),
            Self::SyncCheckpointRead => f.write_str("syncCheckpoint.read"),
            Self::SyncCheckpointUpdate => f.write_str("syncCheckpoint.update"),
            Self::SyncCheckpointDelete => f.write_str("syncCheckpoint.delete"),
            Self::ConfigRead => f.write_str("systemConfig.read"),
            Self::ConfigUpdate => f.write_str("systemConfig.update"),
            Self::SystemMetadataRead => f.write_str("systemMetadata.read"),
            Self::SystemMetadataUpdate => f.write_str("systemMetadata.update"),
            Self::TagCreate => f.write_str("tag.create"),
            Self::TagRead => f.write_str("tag.read"),
            Self::TagUpdate => f.write_str("tag.update"),
            Self::TagDelete => f.write_str("tag.delete"),
            Self::TagAsset => f.write_str("tag.asset"),
            Self::UserRead => f.write_str("user.read"),
            Self::UserUpdate => f.write_str("user.update"),
            Self::UserLicenseCreate => f.write_str("userLicense.create"),
            Self::UserLicenseRead => f.write_str("userLicense.read"),
            Self::UserLicenseUpdate => f.write_str("userLicense.update"),
            Self::UserLicenseDelete => f.write_str("userLicense.delete"),
            Self::UserOnboardingRead => f.write_str("userOnboarding.read"),
            Self::UserOnboardingUpdate => f.write_str("userOnboarding.update"),
            Self::UserOnboardingDelete => f.write_str("userOnboarding.delete"),
            Self::UserPreferenceRead => f.write_str("userPreference.read"),
            Self::UserPreferenceUpdate => f.write_str("userPreference.update"),
            Self::UserProfileImageCreate => f.write_str("userProfileImage.create"),
            Self::UserProfileImageRead => f.write_str("userProfileImage.read"),
            Self::UserProfileImageUpdate => f.write_str("userProfileImage.update"),
            Self::UserProfileImageDelete => f.write_str("userProfileImage.delete"),
            Self::QueueRead => f.write_str("queue.read"),
            Self::QueueUpdate => f.write_str("queue.update"),
            Self::QueueJobCreate => f.write_str("queueJob.create"),
            Self::QueueJobRead => f.write_str("queueJob.read"),
            Self::QueueJobUpdate => f.write_str("queueJob.update"),
            Self::QueueJobDelete => f.write_str("queueJob.delete"),
            Self::WorkflowCreate => f.write_str("workflow.create"),
            Self::WorkflowRead => f.write_str("workflow.read"),
            Self::WorkflowUpdate => f.write_str("workflow.update"),
            Self::WorkflowDelete => f.write_str("workflow.delete"),
            Self::WorkflowLogs => f.write_str("workflow.logs"),
            Self::AdminUserCreate => f.write_str("adminUser.create"),
            Self::AdminUserRead => f.write_str("adminUser.read"),
            Self::AdminUserUpdate => f.write_str("adminUser.update"),
            Self::AdminUserDelete => f.write_str("adminUser.delete"),
            Self::AdminSessionRead => f.write_str("adminSession.read"),
            Self::AdminAuthUnlinkAll => f.write_str("adminAuth.unlinkAll"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for Permission {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "all" => Ok(Self::All),
            "activity.create" => Ok(Self::ActivityCreate),
            "activity.read" => Ok(Self::ActivityRead),
            "activity.update" => Ok(Self::ActivityUpdate),
            "activity.delete" => Ok(Self::ActivityDelete),
            "activity.statistics" => Ok(Self::ActivityStatistics),
            "apiKey.create" => Ok(Self::ApiKeyCreate),
            "apiKey.read" => Ok(Self::ApiKeyRead),
            "apiKey.update" => Ok(Self::ApiKeyUpdate),
            "apiKey.delete" => Ok(Self::ApiKeyDelete),
            "apiKey.rotate" => Ok(Self::ApiKeyRotate),
            "asset.read" => Ok(Self::AssetRead),
            "asset.update" => Ok(Self::AssetUpdate),
            "asset.delete" => Ok(Self::AssetDelete),
            "asset.statistics" => Ok(Self::AssetStatistics),
            "asset.share" => Ok(Self::AssetShare),
            "asset.view" => Ok(Self::AssetView),
            "asset.download" => Ok(Self::AssetDownload),
            "asset.upload" => Ok(Self::AssetUpload),
            "asset.copy" => Ok(Self::AssetCopy),
            "asset.derive" => Ok(Self::AssetDerive),
            "assetFile.read" => Ok(Self::AssetFileRead),
            "assetFile.delete" => Ok(Self::AssetFileDelete),
            "assetFile.download" => Ok(Self::AssetFileDownload),
            "asset.edit.get" => Ok(Self::AssetEditGet),
            "asset.edit.create" => Ok(Self::AssetEditCreate),
            "asset.edit.delete" => Ok(Self::AssetEditDelete),
            "album.create" => Ok(Self::AlbumCreate),
            "album.read" => Ok(Self::AlbumRead),
            "album.update" => Ok(Self::AlbumUpdate),
            "album.delete" => Ok(Self::AlbumDelete),
            "album.statistics" => Ok(Self::AlbumStatistics),
            "album.share" => Ok(Self::AlbumShare),
            "album.download" => Ok(Self::AlbumDownload),
            "albumAsset.create" => Ok(Self::AlbumAssetCreate),
            "albumAsset.delete" => Ok(Self::AlbumAssetDelete),
            "albumUser.create" => Ok(Self::AlbumUserCreate),
            "albumUser.update" => Ok(Self::AlbumUserUpdate),
            "albumUser.delete" => Ok(Self::AlbumUserDelete),
            "auth.changePassword" => Ok(Self::AuthChangePassword),
            "authDevice.delete" => Ok(Self::AuthDeviceDelete),
            "archive.read" => Ok(Self::ArchiveRead),
            "backup.list" => Ok(Self::BackupList),
            "backup.download" => Ok(Self::BackupDownload),
            "backup.upload" => Ok(Self::BackupUpload),
            "backup.delete" => Ok(Self::BackupDelete),
            "clusterGroup.read" => Ok(Self::ClusterGroupRead),
            "clusterGroup.leave" => Ok(Self::ClusterGroupLeave),
            "clusterGroupRequest.create" => Ok(Self::ClusterGroupRequestCreate),
            "clusterGroupRequest.read" => Ok(Self::ClusterGroupRequestRead),
            "clusterGroupRequest.delete" => Ok(Self::ClusterGroupRequestDelete),
            "adminConfig.read" => Ok(Self::AdminConfigRead),
            "adminConfig.update" => Ok(Self::AdminConfigUpdate),
            "userConfig.read" => Ok(Self::UserConfigRead),
            "duplicate.read" => Ok(Self::DuplicateRead),
            "duplicate.delete" => Ok(Self::DuplicateDelete),
            "face.create" => Ok(Self::FaceCreate),
            "face.read" => Ok(Self::FaceRead),
            "face.update" => Ok(Self::FaceUpdate),
            "face.delete" => Ok(Self::FaceDelete),
            "folder.read" => Ok(Self::FolderRead),
            "job.create" => Ok(Self::JobCreate),
            "job.read" => Ok(Self::JobRead),
            "library.create" => Ok(Self::LibraryCreate),
            "library.read" => Ok(Self::LibraryRead),
            "library.update" => Ok(Self::LibraryUpdate),
            "library.delete" => Ok(Self::LibraryDelete),
            "library.statistics" => Ok(Self::LibraryStatistics),
            "timeline.read" => Ok(Self::TimelineRead),
            "timeline.download" => Ok(Self::TimelineDownload),
            "maintenance" => Ok(Self::Maintenance),
            "map.read" => Ok(Self::MapRead),
            "map.search" => Ok(Self::MapSearch),
            "memory.create" => Ok(Self::MemoryCreate),
            "memory.read" => Ok(Self::MemoryRead),
            "memory.update" => Ok(Self::MemoryUpdate),
            "memory.delete" => Ok(Self::MemoryDelete),
            "memory.statistics" => Ok(Self::MemoryStatistics),
            "memoryAsset.create" => Ok(Self::MemoryAssetCreate),
            "memoryAsset.delete" => Ok(Self::MemoryAssetDelete),
            "notification.create" => Ok(Self::NotificationCreate),
            "notification.read" => Ok(Self::NotificationRead),
            "notification.update" => Ok(Self::NotificationUpdate),
            "notification.delete" => Ok(Self::NotificationDelete),
            "partner.create" => Ok(Self::PartnerCreate),
            "partner.read" => Ok(Self::PartnerRead),
            "partner.update" => Ok(Self::PartnerUpdate),
            "partner.delete" => Ok(Self::PartnerDelete),
            "person.create" => Ok(Self::PersonCreate),
            "person.read" => Ok(Self::PersonRead),
            "person.update" => Ok(Self::PersonUpdate),
            "person.delete" => Ok(Self::PersonDelete),
            "person.statistics" => Ok(Self::PersonStatistics),
            "person.merge" => Ok(Self::PersonMerge),
            "person.reassign" => Ok(Self::PersonReassign),
            "pinCode.create" => Ok(Self::PinCodeCreate),
            "pinCode.update" => Ok(Self::PinCodeUpdate),
            "pinCode.delete" => Ok(Self::PinCodeDelete),
            "plugin.create" => Ok(Self::PluginCreate),
            "plugin.read" => Ok(Self::PluginRead),
            "plugin.update" => Ok(Self::PluginUpdate),
            "plugin.delete" => Ok(Self::PluginDelete),
            "server.about" => Ok(Self::ServerAbout),
            "server.apkLinks" => Ok(Self::ServerApkLinks),
            "server.storage" => Ok(Self::ServerStorage),
            "server.statistics" => Ok(Self::ServerStatistics),
            "server.versionCheck" => Ok(Self::ServerVersionCheck),
            "serverLicense.read" => Ok(Self::ServerLicenseRead),
            "serverLicense.update" => Ok(Self::ServerLicenseUpdate),
            "serverLicense.delete" => Ok(Self::ServerLicenseDelete),
            "session.create" => Ok(Self::SessionCreate),
            "session.read" => Ok(Self::SessionRead),
            "session.update" => Ok(Self::SessionUpdate),
            "session.delete" => Ok(Self::SessionDelete),
            "session.lock" => Ok(Self::SessionLock),
            "sharedLink.create" => Ok(Self::SharedLinkCreate),
            "sharedLink.read" => Ok(Self::SharedLinkRead),
            "sharedLink.update" => Ok(Self::SharedLinkUpdate),
            "sharedLink.delete" => Ok(Self::SharedLinkDelete),
            "stack.create" => Ok(Self::StackCreate),
            "stack.read" => Ok(Self::StackRead),
            "stack.update" => Ok(Self::StackUpdate),
            "stack.delete" => Ok(Self::StackDelete),
            "sync.stream" => Ok(Self::SyncStream),
            "syncCheckpoint.read" => Ok(Self::SyncCheckpointRead),
            "syncCheckpoint.update" => Ok(Self::SyncCheckpointUpdate),
            "syncCheckpoint.delete" => Ok(Self::SyncCheckpointDelete),
            "systemConfig.read" => Ok(Self::ConfigRead),
            "systemConfig.update" => Ok(Self::ConfigUpdate),
            "systemMetadata.read" => Ok(Self::SystemMetadataRead),
            "systemMetadata.update" => Ok(Self::SystemMetadataUpdate),
            "tag.create" => Ok(Self::TagCreate),
            "tag.read" => Ok(Self::TagRead),
            "tag.update" => Ok(Self::TagUpdate),
            "tag.delete" => Ok(Self::TagDelete),
            "tag.asset" => Ok(Self::TagAsset),
            "user.read" => Ok(Self::UserRead),
            "user.update" => Ok(Self::UserUpdate),
            "userLicense.create" => Ok(Self::UserLicenseCreate),
            "userLicense.read" => Ok(Self::UserLicenseRead),
            "userLicense.update" => Ok(Self::UserLicenseUpdate),
            "userLicense.delete" => Ok(Self::UserLicenseDelete),
            "userOnboarding.read" => Ok(Self::UserOnboardingRead),
            "userOnboarding.update" => Ok(Self::UserOnboardingUpdate),
            "userOnboarding.delete" => Ok(Self::UserOnboardingDelete),
            "userPreference.read" => Ok(Self::UserPreferenceRead),
            "userPreference.update" => Ok(Self::UserPreferenceUpdate),
            "userProfileImage.create" => Ok(Self::UserProfileImageCreate),
            "userProfileImage.read" => Ok(Self::UserProfileImageRead),
            "userProfileImage.update" => Ok(Self::UserProfileImageUpdate),
            "userProfileImage.delete" => Ok(Self::UserProfileImageDelete),
            "queue.read" => Ok(Self::QueueRead),
            "queue.update" => Ok(Self::QueueUpdate),
            "queueJob.create" => Ok(Self::QueueJobCreate),
            "queueJob.read" => Ok(Self::QueueJobRead),
            "queueJob.update" => Ok(Self::QueueJobUpdate),
            "queueJob.delete" => Ok(Self::QueueJobDelete),
            "workflow.create" => Ok(Self::WorkflowCreate),
            "workflow.read" => Ok(Self::WorkflowRead),
            "workflow.update" => Ok(Self::WorkflowUpdate),
            "workflow.delete" => Ok(Self::WorkflowDelete),
            "workflow.logs" => Ok(Self::WorkflowLogs),
            "adminUser.create" => Ok(Self::AdminUserCreate),
            "adminUser.read" => Ok(Self::AdminUserRead),
            "adminUser.update" => Ok(Self::AdminUserUpdate),
            "adminUser.delete" => Ok(Self::AdminUserDelete),
            "adminSession.read" => Ok(Self::AdminSessionRead),
            "adminAuth.unlinkAll" => Ok(Self::AdminAuthUnlinkAll),
            _ => Err(format!("invalid value for {}: {}", "Permission", value).into()),
        }
    }
}

impl TryFrom<&str> for Permission {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for Permission {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for Permission {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}
