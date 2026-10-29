use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result};

/// Runs `cargo clippy --fix` and `cargo fmt` on the given crate.
///
/// # Errors
/// Returns an error if the `cargo` command fails.
pub fn format_crate(workspace_root: &Path, crate_name: &str) -> Result<()> {
    println!("[xtask] Running cargo fmt on {crate_name}...");
    let fmt_status = Command::new("cargo")
        .args(["fmt", "-p", crate_name])
        .current_dir(workspace_root)
        .status()
        .context("launching cargo fmt")?;

    if !fmt_status.success() {
        anyhow::bail!("cargo fmt failed for {crate_name}");
    }

    println!("[xtask] Running clippy auto-fix on {crate_name}...");
    let clippy_status = Command::new("cargo")
        .args([
            "clippy",
            "--fix",
            "-p",
            crate_name,
            "--lib",
            "--allow-dirty",
            "--allow-no-vcs",
        ])
        .current_dir(workspace_root)
        .status()
        .context("launching cargo clippy --fix")?;

    if !clippy_status.success() {
        anyhow::bail!("cargo clippy --fix failed for {crate_name}");
    }

    println!("[xtask] Running cargo fmt on {crate_name} (final)...");
    let final_fmt_status = Command::new("cargo")
        .args(["fmt", "-p", crate_name])
        .current_dir(workspace_root)
        .status()
        .context("launching final cargo fmt")?;
    if !final_fmt_status.success() {
        anyhow::bail!("final cargo fmt failed for {crate_name}");
    }

    Ok(())
}

/// Cleans up unneeded path prefixes and adds blank lines between items
pub fn post_process_formatting(code: &mut String) {
    // Add blank lines between top-level items
    *code = code.replace("}\npub struct", "}\n\npub struct");
    *code = code.replace("}\npub enum", "}\n\npub enum");
    *code = code.replace("}\nimpl", "}\n\nimpl");
    *code = code.replace(");\nimpl ", ");\n\nimpl ");
    *code = code.replace("}\n///", "}\n\n///");
    *code = code.replace("    }\n    ///", "    }\n\n    ///");
    *code = code.replace(";\n///", ";\n\n///");
    *code = code.replace(";\n#[", ";\n\n#[");
    *code = code.replace("}\n#[", "}\n\n#[");
    *code = code.replace("    }\n    #[", "    }\n\n    #[");
    *code = code.replace("}\nimpl ", "}\n\nimpl ");
    *code = code.replace("\n        let mut request = self", "\n\n        let mut request = self");
    *code = code.replace("\n        let request = self", "\n\n        let request = self");
    *code = code.replace("\n        let response = self", "\n\n        let response = self");

    // typify hardcodes some paths and patterns that need adjusting:
    *code = code.replace("self::error::ConversionError", "crate::common::error::ConversionError");
    *code = code.replace("\"defaults::", "\"crate::common::defaults::");
    *code = code.replace("pub(super) ", "pub ");
    *code = code.replace("::regress::Regex", "::regex::Regex");
    *code = code.replace("1usize as u64", "1_u64");
    *code = code.replace("value.chars().count() < 1usize", "value.is_empty()");
}
