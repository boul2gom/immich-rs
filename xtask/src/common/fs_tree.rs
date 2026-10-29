use std::collections::HashMap;
use std::fmt::Write;
use std::path::Path;

use anyhow::{Context, Result};

#[derive(Debug)]
pub struct VirtualFs {
    files: HashMap<String, String>,
    custom_module_content: HashMap<String, String>,
    module_docs: HashMap<String, String>,
    root_filename: String,
}

impl VirtualFs {
    #[must_use]
    pub fn new(root_filename: &str) -> Self {
        Self {
            files: HashMap::new(),
            custom_module_content: HashMap::new(),
            module_docs: HashMap::new(),
            root_filename: root_filename.to_string(),
        }
    }

    pub fn add_file(&mut self, module_path: &str, content: String) {
        self.files.insert(module_path.to_string(), content);
    }

    pub fn add_module_content(&mut self, module_path: &str, content: String) {
        self.custom_module_content.insert(module_path.to_string(), content);
    }

    pub fn set_module_doc(&mut self, module_path: &str, doc: String) {
        self.module_docs.insert(module_path.to_string(), doc);
    }

    /// Write the virtual filesystem to disk.
    ///
    /// # Errors
    /// Returns an error if directory creation or file writing fails.
    ///
    /// # Panics
    /// Panics if the generated file path has no parent (which should never happen since it's inside `out_dir`).
    pub fn write_to_disk(&self, out_dir: &Path) -> Result<()> {
        let mut children_of: HashMap<String, std::collections::BTreeSet<String>> = HashMap::new();

        // Build the directory tree
        for module_path in self.files.keys().chain(self.custom_module_content.keys()) {
            if module_path.is_empty() {
                continue;
            }
            let parts: Vec<&str> = module_path.split('/').collect();
            for depth in 0..parts.len() {
                let parent = parts[..depth].join("/");
                let child = parts[depth].to_string();
                children_of.entry(parent.clone()).or_default().insert(child);
            }
        }

        std::fs::create_dir_all(out_dir)?;

        // Write individual files
        for (module_path, content) in &self.files {
            if module_path.is_empty() {
                continue;
            }

            let mut file_path = out_dir.to_path_buf();
            let has_children = children_of.contains_key(module_path);

            if has_children {
                for part in module_path.split('/') {
                    file_path = file_path.join(part);
                }
                std::fs::create_dir_all(&file_path)?;
                file_path = file_path.join("mod.rs");
            } else {
                let parts: Vec<&str> = module_path.split('/').collect();
                for (i, part) in parts.iter().enumerate() {
                    if i == parts.len() - 1 {
                        file_path = file_path.join(format!("{part}.rs"));
                    } else {
                        file_path = file_path.join(part);
                    }
                }
                if let Some(parent) = file_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
            }

            let mut final_content = String::new();

            if let Some(doc) = self.module_docs.get(module_path) {
                final_content.push_str(doc);
                final_content.push('\n');
            }

            if has_children {
                if let Some(children) = children_of.get(module_path) {
                    for child in children {
                        let _ = writeln!(final_content, "pub mod {child};");
                    }
                }
                final_content.push('\n');
            }

            final_content.push_str(content);

            if let Some(custom) = self.custom_module_content.get(module_path) {
                final_content.push('\n');
                final_content.push_str(custom);
            }

            let final_content = crate::common::ast_utils::inject_unified_header(&final_content);
            std::fs::write(&file_path, final_content).with_context(|| format!("writing {}", file_path.display()))?;
        }

        // Write directories that don't have a specific file registered in self.files
        for dir in children_of.keys() {
            let mut file_path = out_dir.to_path_buf();
            if dir.is_empty() {
                file_path = file_path.join(&self.root_filename);
            } else {
                for part in dir.split('/') {
                    file_path = file_path.join(part);
                }
                std::fs::create_dir_all(&file_path)?;
                file_path = file_path.join("mod.rs");
            }

            // Only write if it wasn't already written as a file with children
            if dir.is_empty() || !self.files.contains_key(dir) {
                let mut content = String::new();
                if let Some(doc) = self.module_docs.get(dir) {
                    content.push_str(doc);
                    content.push('\n');
                }
                if let Some(children) = children_of.get(dir) {
                    for child in children {
                        let _ = writeln!(content, "pub mod {child};");
                    }
                }
                if let Some(custom) = self.custom_module_content.get(dir) {
                    content.push('\n');
                    content.push_str(custom);
                }

                let final_content = crate::common::ast_utils::inject_unified_header(&content);
                std::fs::write(&file_path, final_content)
                    .with_context(|| format!("writing {}", file_path.display()))?;
            }
        }

        Ok(())
    }
}
