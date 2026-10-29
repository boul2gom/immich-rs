use super::{HashMap, HashSet, ImportCollector, Path, Result};
use crate::common::assign::item_ident;

pub fn clear_generated_dir(out_dir: &Path) -> Result<()> {
    if !out_dir.exists() {
        return Ok(());
    }
    for entry in std::fs::read_dir(out_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            std::fs::remove_dir_all(&path)?;
        } else if path.extension().is_some_and(|e| e == "rs") {
            std::fs::remove_file(&path)?;
        }
    }
    Ok(())
}

pub fn write_modules(out_dir: &Path, module_items: &HashMap<String, Vec<syn::Item>>, version: &str) -> Result<()> {
    // Map all generated types to their modules
    let mut type_to_module: HashMap<String, String> = HashMap::new();
    for (module_path, items) in module_items {
        for item in items {
            if let Some(ident) = item_ident(item) {
                type_to_module.insert(ident, module_path.clone());
            }
        }
    }

    let mut vfs = crate::common::fs_tree::VirtualFs::new("lib.rs");

    for (module_path, items) in module_items {
        if items.is_empty() {
            continue;
        }

        let last_part = module_path.split('/').next_back().unwrap_or(module_path);
        vfs.set_module_doc(module_path, format!("//! OpenAPI data models for `{last_part}`.\n"));

        let mut all_items = items.clone();

        // Collect and inject explicit imports for types from other modules
        let mut collector = ImportCollector {
            type_to_module: &type_to_module,
            current_module: module_path,
            used_imports: HashSet::new(),
        };
        for item in &mut all_items {
            syn::visit_mut::VisitMut::visit_item_mut(&mut collector, item);
        }

        let mut sorted_imports: Vec<_> = collector.used_imports.into_iter().collect();
        sorted_imports.sort();

        let mut use_items = Vec::new();
        for (mod_path, ident_str) in sorted_imports {
            let mod_rs = mod_path.replace('/', "::");
            let use_stmt: syn::Item = syn::parse_str(&format!("pub use crate::{mod_rs}::{ident_str};")).unwrap();
            use_items.push(use_stmt);
        }

        use_items.append(&mut all_items);
        all_items = use_items;

        let generated_file = syn::File {
            frontmatter: None,
            shebang: None,
            attrs: vec![],
            items: all_items,
        };

        let mut code = prettyplease::unparse(&generated_file);
        crate::common::fmt::post_process_formatting(&mut code);

        vfs.add_file(module_path, code);
    }

    vfs.set_module_doc(
        "",
        "//! Generated OpenAPI data models for the Immich photo and video server.\n".to_string(),
    );
    vfs.add_file("id", ID_RS_CONTENT.to_string());
    let root_content = format!(
        "pub use id::*;\n\n/// Target Immich OpenAPI version from which these models were generated.\n\
         pub const TARGET_IMMICH_VERSION: &str = \"{version}\";\n"
    );
    vfs.add_module_content("", root_content);

    clear_generated_dir(out_dir)?;
    vfs.write_to_disk(out_dir)?;

    Ok(())
}

const ID_RS_CONTENT: &str = r#"//! Strongly typed entity identifiers for Immich.
//!
//! Provides type-safe wrappers around [`uuid::Uuid`] for domain entities such as
//! [`AssetId`], [`AlbumId`], [`UserId`], [`TagId`], and others.
//!
//! Each identifier is `#[serde(transparent)]` and implements [`std::ops::Deref<Target = uuid::Uuid>`],
//! allowing direct access to UUID methods while preventing accidental ID mixing at compile time.

use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

macro_rules! define_id {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub Uuid);

        impl $name {
            /// Creates a new strongly typed identifier from a [`Uuid`].
            #[inline]
            #[must_use]
            pub const fn new(uuid: Uuid) -> Self {
                Self(uuid)
            }

            /// Unwraps and returns the inner [`Uuid`].
            #[inline]
            #[must_use]
            pub const fn into_inner(self) -> Uuid {
                self.0
            }

            /// Returns a reference to the inner [`Uuid`].
            #[inline]
            #[must_use]
            pub const fn as_uuid(&self) -> &Uuid {
                &self.0
            }

            /// Parses an identifier from a string slice representing a UUID.
            ///
            /// # Errors
            /// Returns an error if the string is not a valid UUID.
            #[inline]
            pub fn parse_str(s: &str) -> std::result::Result<Self, uuid::Error> {
                Uuid::parse_str(s).map(Self)
            }
        }

        impl Deref for $name {
            type Target = Uuid;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl AsRef<Uuid> for $name {
            #[inline]
            fn as_ref(&self) -> &Uuid {
                &self.0
            }
        }

        impl From<Uuid> for $name {
            #[inline]
            fn from(uuid: Uuid) -> Self {
                Self(uuid)
            }
        }

        impl From<&Uuid> for $name {
            #[inline]
            fn from(uuid: &Uuid) -> Self {
                Self(*uuid)
            }
        }

        impl From<$name> for Uuid {
            #[inline]
            fn from(id: $name) -> Self {
                id.0
            }
        }

        impl TryFrom<&str> for $name {
            type Error = uuid::Error;

            #[inline]
            fn try_from(s: &str) -> std::result::Result<Self, Self::Error> {
                Self::parse_str(s)
            }
        }

        impl TryFrom<String> for $name {
            type Error = uuid::Error;

            #[inline]
            fn try_from(s: String) -> std::result::Result<Self, Self::Error> {
                Self::parse_str(&s)
            }
        }

        impl fmt::Display for $name {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(&self.0, f)
            }
        }

        impl FromStr for $name {
            type Err = uuid::Error;

            #[inline]
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::parse_str(s)
            }
        }
    };
}

define_id!(AssetId, "A strongly typed identifier for an Immich asset.");
define_id!(AlbumId, "A strongly typed identifier for an Immich album.");
define_id!(UserId, "A strongly typed identifier for an Immich user.");
define_id!(TagId, "A strongly typed identifier for an Immich tag.");
define_id!(LibraryId, "A strongly typed identifier for an Immich library.");
define_id!(ApiKeyId, "A strongly typed identifier for an Immich API key.");
define_id!(PersonId, "A strongly typed identifier for an Immich person or recognized face.");
define_id!(MemoryId, "A strongly typed identifier for an Immich memory.");
define_id!(StackId, "A strongly typed identifier for an Immich asset stack.");
define_id!(ActivityId, "A strongly typed identifier for an Immich activity.");
define_id!(PartnerId, "A strongly typed identifier for an Immich partner sharing relationship.");
define_id!(WorkflowId, "A strongly typed identifier for an Immich automation workflow.");
"#;
