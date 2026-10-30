use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result};

/// Runs `cargo +nightly fmt` (falling back to stable `cargo fmt` if nightly `rustfmt` is unavailable)
/// on the given crate, matching `rustfmt.toml`.
///
/// # Errors
/// Returns an error if formatting fails.
pub fn format_crate(workspace_root: &Path, crate_name: &str) -> Result<()> {
    println!("[xtask] Running cargo +nightly fmt on {crate_name}...");
    let nightly_status = Command::new("cargo")
        .args(["+nightly", "fmt", "-p", crate_name])
        .current_dir(workspace_root)
        .status();

    match nightly_status {
        Ok(s) if s.success() => return Ok(()),
        _ => {
            println!("[xtask] Nightly rustfmt unavailable, falling back to stable cargo fmt on {crate_name}...");
        }
    }

    let fallback_status = Command::new("cargo")
        .args(["fmt", "-p", crate_name])
        .current_dir(workspace_root)
        .status()
        .context("launching cargo fmt")?;

    if !fallback_status.success() {
        anyhow::bail!("cargo fmt failed for {crate_name}");
    }

    Ok(())
}

/// Renders a `syn::File` with `prettyplease`, separating top-level items with a blank line.
#[must_use]
pub fn unparse_with_spacing(file: &syn::File) -> String {
    let mut blocks = Vec::with_capacity(file.items.len());

    if !file.attrs.is_empty() {
        let header_only = syn::File {
            shebang: None,
            frontmatter: None,
            attrs: file.attrs.clone(),
            items: Vec::new(),
        };
        let rendered = prettyplease::unparse(&header_only);
        if !rendered.trim().is_empty() {
            blocks.push(rendered.trim_end().to_string());
        }
    }

    let mut use_items = Vec::new();
    for item in &file.items {
        if matches!(item, syn::Item::Use(_)) {
            use_items.push(item.clone());
            continue;
        }
        if !use_items.is_empty() {
            let use_file = syn::File {
                shebang: None,
                frontmatter: None,
                attrs: Vec::new(),
                items: std::mem::take(&mut use_items),
            };
            blocks.push(prettyplease::unparse(&use_file).trim_end().to_string());
        }

        let single = syn::File {
            shebang: None,
            frontmatter: None,
            attrs: Vec::new(),
            items: vec![item.clone()],
        };
        blocks.push(prettyplease::unparse(&single).trim_end().to_string());
    }

    if !use_items.is_empty() {
        let use_file = syn::File {
            shebang: None,
            frontmatter: None,
            attrs: Vec::new(),
            items: use_items,
        };
        blocks.push(prettyplease::unparse(&use_file).trim_end().to_string());
    }

    let mut out = blocks.join("\n\n");
    out.push('\n');
    out
}
