use std::mem::size_of;
use std::str::FromStr;

use immich_models::activities::{ReactionLevel, ReactionType};
use immich_models::albums::{AlbumUserRole, AssetOrder, BulkIdErrorReason, CreateAlbumDTO};
use immich_models::assets::AssetEditActionItemParameters;
use immich_models::id::{
    ActivityId, AlbumId, ApiKeyId, AssetFileId, AssetId, ClusterGroupId, DuplicateId, FaceId, LibraryId, MemoryId,
    NotificationId, PartnerId, PersonId, PluginId, SessionId, SharedLinkId, StackId, TagId, UserId, WorkflowId,
};
use immich_models::libraries::{CreateLibraryDTO, LibraryResponseDTO};
use immich_models::tags::TagCreateDTO;
use immich_models::users::admin::UserAdminCreateDTO;
use pretty_assertions::assert_eq;

#[test]
fn enum_forward_compatibility_preserves_unknown_server_variants_losslessly() {
    let unknown_reaction: ReactionType =
        serde_json::from_str("\"super_like_future_feature\"").expect("unknown enum value must deserialize");
    assert_eq!(
        unknown_reaction,
        ReactionType::Unknown("super_like_future_feature".to_string())
    );
    assert_eq!(unknown_reaction.as_str(), "super_like_future_feature");
    assert_eq!(unknown_reaction.to_string(), "super_like_future_feature");
    assert_eq!(
        serde_json::to_string(&unknown_reaction).expect("re-serialize unknown reaction"),
        "\"super_like_future_feature\""
    );

    let unknown_role: AlbumUserRole = serde_json::from_str("\"moderator\"").expect("unknown role must deserialize");
    assert_eq!(unknown_role, AlbumUserRole::Unknown("moderator".to_string()));
    assert_eq!(
        serde_json::to_string(&unknown_role).expect("re-serialize unknown role"),
        "\"moderator\""
    );

    let unknown_order: AssetOrder =
        serde_json::from_str("\"random_shuffled\"").expect("unknown order must deserialize");
    assert_eq!(unknown_order, AssetOrder::Unknown("random_shuffled".to_string()));
}

#[test]
fn enum_with_existing_unknown_variant_uses_unknown_value_fallback_without_collision() {
    let explicit_unknown: BulkIdErrorReason =
        serde_json::from_str("\"unknown\"").expect("known 'unknown' variant must deserialize");
    assert_eq!(explicit_unknown, BulkIdErrorReason::Unknown);
    assert_eq!(explicit_unknown.as_str(), "unknown");

    let novel_reason: BulkIdErrorReason =
        serde_json::from_str("\"quota_reached\"").expect("novel server reason must deserialize");
    assert_eq!(
        novel_reason,
        BulkIdErrorReason::UnknownValue("quota_reached".to_string())
    );
    assert_eq!(
        serde_json::to_string(&novel_reason).expect("re-serialize novel reason"),
        "\"quota_reached\""
    );
}

#[test]
fn untagged_oneof_union_deserializes_unknown_payload_without_failing() {
    let payload = serde_json::json!({
        "futureFilter": "sepia",
        "intensity": 42
    });
    let serialized = serde_json::to_string(&payload).expect("serialize json value");
    let parsed: AssetEditActionItemParameters =
        serde_json::from_str(&serialized).expect("unknown untagged union variant must fall back to Unknown(Value)");

    assert_eq!(parsed, AssetEditActionItemParameters::Unknown(payload.clone()));
    let round_trip: serde_json::Value =
        serde_json::from_str(&serde_json::to_string(&parsed).expect("re-serialize union")).expect("parse json");
    assert_eq!(round_trip, payload);
}

#[test]
fn enum_display_and_from_str_round_trip_known_and_unknown_strings() {
    let role = AlbumUserRole::Editor;
    let json = serde_json::to_string(&role).expect("serialize role");
    assert_eq!(json, "\"editor\"");

    let deserialized: AlbumUserRole = serde_json::from_str(&json).expect("deserialize role");
    assert_eq!(deserialized, role);

    assert_eq!("editor".parse::<AlbumUserRole>().unwrap(), AlbumUserRole::Editor);
    assert_eq!(
        "custom_role".parse::<AlbumUserRole>().unwrap(),
        AlbumUserRole::Unknown("custom_role".to_string())
    );
    assert_eq!(ReactionLevel::Album.to_string(), "album");
}

#[test]
fn optional_non_required_fields_are_omitted_while_required_nullable_fields_serialize_null() {
    let create_lib = CreateLibraryDTO::new(uuid::Uuid::nil());
    let create_json: serde_json::Value = serde_json::to_value(&create_lib).expect("serialize CreateLibraryDTO");
    assert_eq!(
        create_json,
        serde_json::json!({ "ownerId": "00000000-0000-0000-0000-000000000000" })
    );

    let ts = chrono::DateTime::parse_from_rfc3339("2026-01-01T00:00:00Z")
        .expect("valid timestamp")
        .with_timezone(&chrono::Utc);
    let lib_resp = LibraryResponseDTO::new(
        10,
        ts,
        ["*.tmp"],
        uuid::Uuid::nil(),
        ["/photos"],
        "Main Library",
        uuid::Uuid::nil(),
        ts,
    );
    let resp_json: serde_json::Value = serde_json::to_value(&lib_resp).expect("serialize LibraryResponseDTO");
    assert_eq!(resp_json["refreshedAt"], serde_json::Value::Null);
}

#[test]
fn dto_constructor_and_builders_accept_typed_ids_and_slices() {
    let raw_id = uuid::Uuid::nil();
    let album = CreateAlbumDTO::new("Summer Roadtrip")
        .description("Across Europe")
        .asset_ids([raw_id]);

    assert_eq!(album.album_name, "Summer Roadtrip");
    assert_eq!(album.description.as_deref(), Some("Across Europe"));
    assert_eq!(album.asset_ids, vec![AssetId::from(raw_id)]);

    let cleared = album.set_description(None);
    assert_eq!(cleared.description, None);
}

#[test]
fn tag_and_user_dtos_accept_plain_strings_including_local_self_hosted_domains() {
    let tag = TagCreateDTO::new("Family").color("#FF5733");
    let json = serde_json::to_string(&tag).expect("serialize TagCreateDTO");
    let deserialized: TagCreateDTO = serde_json::from_str(&json).expect("deserialize TagCreateDTO");
    assert_eq!(deserialized, tag);

    // Self-hosted instances frequently use non-RFC `.local` or bare-hostname addresses
    let local_admin = UserAdminCreateDTO::new("admin@localhost", "Local Admin", "securePassword123!")
        .should_change_password(false)
        .quota_size_in_bytes(10_000_000_000_i64);

    let admin_json = serde_json::to_string(&local_admin).expect("serialize UserAdminCreateDTO");
    let round_trip: UserAdminCreateDTO = serde_json::from_str(&admin_json).expect("deserialize non-RFC local email");
    assert_eq!(round_trip, local_admin);
}

#[test]
fn all_twenty_id_newtypes_are_repr_transparent_and_preserve_uuid_wire_format() {
    assert_eq!(size_of::<AssetId>(), size_of::<uuid::Uuid>());
    assert_eq!(size_of::<Option<AssetId>>(), size_of::<Option<uuid::Uuid>>());

    let raw_uuid = uuid::Uuid::parse_str("a1b2c3d4-e5f6-4a1b-8c2d-3e4f5a6b7c8d").unwrap();
    let asset_id = AssetId::new(raw_uuid);

    assert_eq!(*asset_id, raw_uuid);
    assert_eq!(asset_id.into_inner(), raw_uuid);
    assert_eq!(asset_id.as_uuid(), &raw_uuid);
    assert_eq!(AssetId::from(raw_uuid), asset_id);
    assert_eq!(uuid::Uuid::from(asset_id), raw_uuid);
    assert_eq!(asset_id.to_string(), "a1b2c3d4-e5f6-4a1b-8c2d-3e4f5a6b7c8d");
    assert_eq!(
        AssetId::from_str("a1b2c3d4-e5f6-4a1b-8c2d-3e4f5a6b7c8d").unwrap(),
        asset_id
    );
    assert_eq!(
        AssetId::parse_str("a1b2c3d4-e5f6-4a1b-8c2d-3e4f5a6b7c8d").unwrap(),
        asset_id
    );
    assert!("not-a-uuid".parse::<AssetId>().is_err());

    let json_asset = serde_json::to_string(&asset_id).unwrap();
    assert_eq!(json_asset, "\"a1b2c3d4-e5f6-4a1b-8c2d-3e4f5a6b7c8d\"");
    assert_eq!(serde_json::from_str::<AssetId>(&json_asset).unwrap(), asset_id);

    // Exhaustively verify all 20 domain ID newtypes construct from Uuid and serialize identically
    let serialized_all = [
        serde_json::to_string(&ActivityId::from(raw_uuid)).unwrap(),
        serde_json::to_string(&AlbumId::from(raw_uuid)).unwrap(),
        serde_json::to_string(&ApiKeyId::from(raw_uuid)).unwrap(),
        serde_json::to_string(&AssetFileId::from(raw_uuid)).unwrap(),
        serde_json::to_string(&AssetId::from(raw_uuid)).unwrap(),
        serde_json::to_string(&ClusterGroupId::from(raw_uuid)).unwrap(),
        serde_json::to_string(&DuplicateId::from(raw_uuid)).unwrap(),
        serde_json::to_string(&FaceId::from(raw_uuid)).unwrap(),
        serde_json::to_string(&LibraryId::from(raw_uuid)).unwrap(),
        serde_json::to_string(&MemoryId::from(raw_uuid)).unwrap(),
        serde_json::to_string(&NotificationId::from(raw_uuid)).unwrap(),
        serde_json::to_string(&PartnerId::from(raw_uuid)).unwrap(),
        serde_json::to_string(&PersonId::from(raw_uuid)).unwrap(),
        serde_json::to_string(&PluginId::from(raw_uuid)).unwrap(),
        serde_json::to_string(&SessionId::from(raw_uuid)).unwrap(),
        serde_json::to_string(&SharedLinkId::from(raw_uuid)).unwrap(),
        serde_json::to_string(&StackId::from(raw_uuid)).unwrap(),
        serde_json::to_string(&TagId::from(raw_uuid)).unwrap(),
        serde_json::to_string(&UserId::from(raw_uuid)).unwrap(),
        serde_json::to_string(&WorkflowId::from(raw_uuid)).unwrap(),
    ];

    for wire in serialized_all {
        assert_eq!(wire, json_asset);
    }
}
