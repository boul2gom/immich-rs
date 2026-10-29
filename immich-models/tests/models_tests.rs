use immich_models::activities::{ReactionLevel, ReactionType};
use immich_models::albums::{AlbumUserRole, AssetOrder, CreateAlbumDTO};
use pretty_assertions::assert_eq;

#[test]
fn enum_forward_compatibility_deserializes_unknown_server_variants_without_failure() {
    // Immich server may introduce new reaction types or roles in future releases
    let unknown_reaction: ReactionType =
        serde_json::from_str("\"super_like_future_feature\"").expect("unknown enum value must not fail");
    assert_eq!(unknown_reaction, ReactionType::UnknownValue);

    let unknown_role: AlbumUserRole = serde_json::from_str("\"moderator\"").expect("unknown role must not fail");
    assert_eq!(unknown_role, AlbumUserRole::UnknownValue);

    let unknown_order: AssetOrder = serde_json::from_str("\"random_shuffled\"").expect("unknown order must not fail");
    assert_eq!(unknown_order, AssetOrder::UnknownValue);
}

#[test]
fn enum_display_and_round_trip_matches_expected_string() {
    let role = AlbumUserRole::Editor;
    let json = serde_json::to_string(&role).expect("serialize role");
    assert_eq!(json, "\"editor\"");

    let deserialized: AlbumUserRole = serde_json::from_str(&json).expect("deserialize role");
    assert_eq!(deserialized, role);

    // Display for UnknownValue outputs "unknown"
    assert_eq!(format!("{}", AlbumUserRole::UnknownValue), "unknown");
    assert_eq!(format!("{}", ReactionLevel::UnknownValue), "unknown");
}

#[test]
fn dto_serialization_skips_none_fields_omitting_null_properties() {
    let album = CreateAlbumDTO::new("Vacation 2026");
    let json = serde_json::to_string(&album).expect("serialize CreateAlbumDTO");

    // The output must contain "albumName" but NOT "description" or other None fields as null
    assert!(json.contains("\"albumName\":\"Vacation 2026\""));
    assert!(!json.contains("\"description\""));
    assert!(!json.contains("null"));
}

#[test]
fn dto_constructor_and_builders_construct_valid_structure() {
    let album = CreateAlbumDTO::new("Summer Roadtrip")
        .description("Across Europe")
        .asset_ids(vec![uuid::Uuid::nil()]);

    assert_eq!(album.album_name, "Summer Roadtrip");
    assert_eq!(album.description.as_deref(), Some("Across Europe"));
    assert_eq!(album.asset_ids, vec![uuid::Uuid::nil()]);

    let cleared = album.set_description(None);
    assert_eq!(cleared.description, None);
}

#[test]
fn tag_create_dto_serialization_round_trip() {
    use immich_models::tags::{TagCreateColor, TagCreateDTO, TagCreateName};

    let tag = TagCreateDTO::new(TagCreateName("Family".into())).color(TagCreateColor("#FF5733".into()));
    let json = serde_json::to_string(&tag).expect("serialize TagCreateDTO");

    assert!(json.contains("\"name\":\"Family\""));
    assert!(json.contains("\"color\":\"#FF5733\""));

    let deserialized: TagCreateDTO = serde_json::from_str(&json).expect("deserialize TagCreateDTO");
    assert_eq!(deserialized.name.0, "Family");
    assert_eq!(deserialized.color.as_ref().map(|c| c.0.as_str()), Some("#FF5733"));
}

#[test]
fn user_admin_create_dto_constructor_and_builders() {
    use immich_models::users::admin::{UserAdminCreateDTO, UserAdminCreateEmail};

    let user = UserAdminCreateDTO::new(
        UserAdminCreateEmail("user@example.com".into()),
        "Alex",
        "securePassword123!",
    )
    .should_change_password(false)
    .quota_size_in_bytes(10_000_000_000i64);

    assert_eq!(user.email.0, "user@example.com");
    assert_eq!(user.name, "Alex");
    assert_eq!(user.password, "securePassword123!");
    assert_eq!(user.should_change_password, Some(false));
    assert_eq!(user.quota_size_in_bytes, Some(10_000_000_000));
}

#[test]
fn id_newtypes_serde_transparent_round_trip_and_traits() {
    use std::str::FromStr;

    use immich_models::id::{ActivityId, AlbumId, ApiKeyId, AssetId, LibraryId, PersonId, TagId, UserId};

    let raw_uuid = uuid::Uuid::parse_str("a1b2c3d4-e5f6-4a1b-8c2d-3e4f5a6b7c8d").unwrap();

    // Construction and conversions
    let asset_id = AssetId::new(raw_uuid);
    assert_eq!(*asset_id, raw_uuid);
    assert_eq!(asset_id.into_inner(), raw_uuid);
    assert_eq!(asset_id.as_uuid(), &raw_uuid);
    assert_eq!(AssetId::from(raw_uuid), asset_id);
    let back_to_uuid: uuid::Uuid = asset_id.into();
    assert_eq!(back_to_uuid, raw_uuid);

    // Display & FromStr
    assert_eq!(asset_id.to_string(), "a1b2c3d4-e5f6-4a1b-8c2d-3e4f5a6b7c8d");
    let parsed = AssetId::from_str("a1b2c3d4-e5f6-4a1b-8c2d-3e4f5a6b7c8d").unwrap();
    assert_eq!(parsed, asset_id);
    let parsed_str = AssetId::parse_str("a1b2c3d4-e5f6-4a1b-8c2d-3e4f5a6b7c8d").unwrap();
    assert_eq!(parsed_str, asset_id);

    // TryFrom &str and String
    let try_from_str: AssetId = "a1b2c3d4-e5f6-4a1b-8c2d-3e4f5a6b7c8d".try_into().unwrap();
    assert_eq!(try_from_str, asset_id);
    let try_from_string: AssetId = "a1b2c3d4-e5f6-4a1b-8c2d-3e4f5a6b7c8d".to_string().try_into().unwrap();
    assert_eq!(try_from_string, asset_id);

    // Serde transparency: serialize must be identical to raw UUID string
    let json_asset = serde_json::to_string(&asset_id).unwrap();
    assert_eq!(json_asset, "\"a1b2c3d4-e5f6-4a1b-8c2d-3e4f5a6b7c8d\"");
    let deserialized_asset: AssetId = serde_json::from_str(&json_asset).unwrap();
    assert_eq!(deserialized_asset, asset_id);

    // Verify all other IDs behave identically
    let album_id: AlbumId = raw_uuid.into();
    let user_id: UserId = raw_uuid.into();
    let tag_id: TagId = raw_uuid.into();
    let library_id: LibraryId = raw_uuid.into();
    let api_key_id: ApiKeyId = raw_uuid.into();
    let person_id: PersonId = raw_uuid.into();
    let activity_id: ActivityId = raw_uuid.into();

    assert_eq!(*album_id, raw_uuid);
    assert_eq!(*user_id, raw_uuid);
    assert_eq!(*tag_id, raw_uuid);
    assert_eq!(*library_id, raw_uuid);
    assert_eq!(*api_key_id, raw_uuid);
    assert_eq!(*person_id, raw_uuid);
    assert_eq!(*activity_id, raw_uuid);

    assert_eq!(serde_json::to_string(&album_id).unwrap(), json_asset);
    assert_eq!(serde_json::to_string(&user_id).unwrap(), json_asset);
    assert_eq!(serde_json::to_string(&tag_id).unwrap(), json_asset);
}
