// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! `OpenAPI` data models for `common`.

pub use crate::albums::{AlbumUserRole, AssetOrder};
pub use crate::people::{PersonUsersDeleteDTO, PersonUsersDeleteItem, PersonUsersResponseDTO, PersonUsersResponseItem};
pub use crate::search::SearchOrderField;

/// Error types.
pub mod error {
    /// Error from a `TryFrom` or `FromStr` implementation.
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// CQ mode
#[non_exhaustive]
pub enum CqMode {
    Auto,
    Cqp,
    Icq,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl CqMode {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Auto => "auto",
            Self::Cqp => "cqp",
            Self::Icq => "icq",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for CqMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for CqMode {
    fn from(value: &str) -> Self {
        match value {
            "auto" => Self::Auto,
            "cqp" => Self::Cqp,
            "icq" => Self::Icq,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for CqMode {
    fn from(value: String) -> Self {
        match value.as_str() {
            "auto" => Self::Auto,
            "cqp" => Self::Cqp,
            "icq" => Self::Icq,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for CqMode {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for CqMode {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for CqMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for CqMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = CqMode;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(CqMode::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(CqMode::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Transcode hardware acceleration
#[non_exhaustive]
pub enum TranscodeHwAccel {
    Nvenc,
    Qsv,
    Vaapi,
    Rkmpp,
    Disabled,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl TranscodeHwAccel {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Nvenc => "nvenc",
            Self::Qsv => "qsv",
            Self::Vaapi => "vaapi",
            Self::Rkmpp => "rkmpp",
            Self::Disabled => "disabled",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for TranscodeHwAccel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for TranscodeHwAccel {
    fn from(value: &str) -> Self {
        match value {
            "nvenc" => Self::Nvenc,
            "qsv" => Self::Qsv,
            "vaapi" => Self::Vaapi,
            "rkmpp" => Self::Rkmpp,
            "disabled" => Self::Disabled,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for TranscodeHwAccel {
    fn from(value: String) -> Self {
        match value.as_str() {
            "nvenc" => Self::Nvenc,
            "qsv" => Self::Qsv,
            "vaapi" => Self::Vaapi,
            "rkmpp" => Self::Rkmpp,
            "disabled" => Self::Disabled,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for TranscodeHwAccel {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for TranscodeHwAccel {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for TranscodeHwAccel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for TranscodeHwAccel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = TranscodeHwAccel;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(TranscodeHwAccel::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(TranscodeHwAccel::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
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
