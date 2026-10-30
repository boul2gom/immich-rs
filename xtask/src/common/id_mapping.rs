//! Canonical mapping of Immich `OpenAPI` UUID fields and parameters to strongly typed `immich_models::id` wrappers.

/// Complete list of strongly typed identifier newtypes generated into `immich-models/src/id.rs`.
pub static ALL_TYPED_IDS: &[(&str, &str)] = &[
    ("ActivityId", "A strongly typed identifier for an Immich activity."),
    ("AlbumId", "A strongly typed identifier for an Immich album."),
    ("ApiKeyId", "A strongly typed identifier for an Immich API key."),
    (
        "AssetFileId",
        "A strongly typed identifier for an Immich auxiliary asset file.",
    ),
    ("AssetId", "A strongly typed identifier for an Immich media asset."),
    (
        "ClusterGroupId",
        "A strongly typed identifier for an Immich facial-recognition cluster group.",
    ),
    (
        "DuplicateId",
        "A strongly typed identifier for an Immich duplicate asset group.",
    ),
    (
        "FaceId",
        "A strongly typed identifier for a detected face bounding box on an asset.",
    ),
    ("LibraryId", "A strongly typed identifier for an Immich library."),
    ("MemoryId", "A strongly typed identifier for an Immich memory."),
    (
        "NotificationId",
        "A strongly typed identifier for an Immich user notification.",
    ),
    (
        "PartnerId",
        "A strongly typed identifier for an Immich partner sharing relationship.",
    ),
    ("PersonId", "A strongly typed identifier for an Immich person."),
    ("PluginId", "A strongly typed identifier for an Immich plugin."),
    (
        "SessionId",
        "A strongly typed identifier for an Immich authentication or streaming session.",
    ),
    ("SharedLinkId", "A strongly typed identifier for an Immich shared link."),
    ("StackId", "A strongly typed identifier for an Immich asset stack."),
    ("TagId", "A strongly typed identifier for an Immich tag."),
    ("UserId", "A strongly typed identifier for an Immich user."),
    (
        "WorkflowId",
        "A strongly typed identifier for an Immich automation workflow.",
    ),
];

/// Returns `true` if `ident` is one of the generated `immich_models::id` newtypes.
#[must_use]
pub fn is_typed_id_name(ident: &str) -> bool {
    ALL_TYPED_IDS.iter().any(|(name, _)| *name == ident)
}

/// Resolves an `OpenAPI` UUID field on a `Rust` struct (`struct_name`, `field_snake`) to a typed ID name.
#[must_use]
pub fn resolve_struct_field_id(struct_name: &str, field_snake: &str) -> Option<&'static str> {
    if let Some(unambiguous) = resolve_unambiguous_field_name(field_snake) {
        return Some(unambiguous);
    }

    match (struct_name, field_snake) {
        ("AssetCopyDTO", "source_id" | "target_id")
        | ("AssetBulkDeleteDTO" | "AssetBulkUpdateDTO", "ids")
        | (
            "AssetResponseDTO"
            | "AssetMediaResponseDTO"
            | "MapMarkerResponseDTO"
            | "SyncAssetV1"
            | "SyncAssetV2"
            | "MetadataSearchDTO",
            "id",
        ) => Some("AssetId"),
        ("AssetFileResponseDTO", "id") => Some("AssetFileId"),
        ("AlbumResponseDTO" | "SyncAlbumV1" | "SyncAlbumV2", "id") => Some("AlbumId"),
        (
            "UserResponseDTO"
            | "UserAdminResponseDTO"
            | "PeopleUserResponseDTO"
            | "PartnerResponseDTO"
            | "SyncUserV1"
            | "SyncAuthUserV1"
            | "SyncAuthUserV2",
            "id",
        ) => Some("UserId"),
        ("TagCreateDTO", "parent_id") | ("TagResponseDTO", "id") => Some("TagId"),
        ("LibraryResponseDTO", "id") => Some("LibraryId"),
        ("ApiKeyResponseDTO" | "ApiKeyCreateResponseDTO", "id") => Some("ApiKeyId"),
        ("PersonResponseDTO" | "PeopleUpdateItem" | "SyncPersonV1", "id")
        | ("MergePersonDTO" | "PeopleDeleteDTO", "ids") => Some("PersonId"),
        ("AssetFaceResponseDTO" | "FaceDTO" | "SyncAssetFaceV1" | "SyncAssetFaceV3", "id") => Some("FaceId"),
        ("MemoryResponseDTO" | "SyncMemoryV1" | "SyncMemoryV2", "id") => Some("MemoryId"),
        ("StackResponseDTO" | "AssetStackResponseDTO" | "SyncStackV1", "id") => Some("StackId"),
        ("ActivityResponseDTO", "id") => Some("ActivityId"),
        ("WorkflowResponseDTO", "id") => Some("WorkflowId"),
        ("SessionResponseDTO" | "SessionCreateResponseDTO", "id") => Some("SessionId"),
        ("SharedLinkResponseDTO", "id") => Some("SharedLinkId"),
        ("NotificationDTO", "id") | ("NotificationDeleteAllDTO" | "NotificationUpdateAllDTO", "ids") => {
            Some("NotificationId")
        }
        ("PluginResponseDTO", "id") => Some("PluginId"),
        _ => None,
    }
}

/// Resolves an `OpenAPI` UUID endpoint parameter (`operation_id`, `module_path`, `param_snake`) to a typed ID name.
#[must_use]
pub fn resolve_endpoint_param_id(operation_id: &str, module_path: &str, param_snake: &str) -> Option<&'static str> {
    if let Some(unambiguous) = resolve_unambiguous_field_name(param_snake) {
        return Some(unambiguous);
    }

    if param_snake != "id" {
        return None;
    }

    // Special cases where `id` refers to a sub-resource UUID rather than the module's primary entity ID
    if matches!(
        operation_id,
        "searchPluginMethods" | "deleteClusterGroupRequest" | "acceptClusterGroupRequest"
    ) {
        return None;
    }

    match module_path {
        "activities" => Some("ActivityId"),
        "albums" => Some("AlbumId"),
        "api_keys" => Some("ApiKeyId"),
        "assets" => Some("AssetId"),
        "assets/faces" => Some("FaceId"),
        "assets/files" => Some("AssetFileId"),
        "assets/stacks" => Some("StackId"),
        "clusters" => Some("ClusterGroupId"),
        "duplicates" => Some("DuplicateId"),
        "libraries" => Some("LibraryId"),
        "memories" => Some("MemoryId"),
        "notifications" => Some("NotificationId"),
        "partners" | "users" | "users/admin" => Some("UserId"),
        "people" => Some("PersonId"),
        "plugins" => Some("PluginId"),
        "sessions" => Some("SessionId"),
        "shared_links" => Some("SharedLinkId"),
        "tags" => Some("TagId"),
        "workflows" => Some("WorkflowId"),
        _ => None,
    }
}

fn resolve_unambiguous_field_name(field_snake: &str) -> Option<&'static str> {
    match field_snake {
        "album_id" | "album_ids" => Some("AlbumId"),
        "asset_id"
        | "asset_ids"
        | "primary_asset_id"
        | "album_thumbnail_asset_id"
        | "live_photo_video_id"
        | "feature_face_asset_id"
        | "query_asset_id"
        | "closest_asset_id"
        | "keep_asset_ids"
        | "trash_asset_ids"
        | "suggested_keep_asset_ids" => Some("AssetId"),
        "user_id" | "owner_id" | "shared_with_id" | "shared_with_ids" | "shared_by_id" => Some("UserId"),
        "tag_id" | "tag_ids" => Some("TagId"),
        "library_id" => Some("LibraryId"),
        "api_key_id" | "key_id" => Some("ApiKeyId"),
        "person_id" | "person_ids" | "closest_person_id" => Some("PersonId"),
        "asset_face_id" => Some("FaceId"),
        "memory_id" => Some("MemoryId"),
        "stack_id" => Some("StackId"),
        "activity_id" => Some("ActivityId"),
        "partner_id" => Some("PartnerId"),
        "workflow_id" => Some("WorkflowId"),
        "session_id" => Some("SessionId"),
        "cluster_group_id" => Some("ClusterGroupId"),
        "duplicate_id" => Some("DuplicateId"),
        _ => None,
    }
}
