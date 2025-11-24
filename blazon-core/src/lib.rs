//! blazon-core: badge generation for Rust project metrics

pub mod debug;

use std::collections::HashSet;
use std::process::Command;

/// Project metrics
pub struct Metrics {
    pub dep_count: usize,
    pub binary_size_bytes: u64,
}

/// Count unique dependencies using cargo tree
pub fn count_dependencies() -> Result<usize, String> {
    let output = Command::new("cargo")
        .args(["tree", "--edges", "normal", "--prefix", "none"])
        .output()
        .map_err(|e| format!("Failed to run cargo tree: {}", e))?;

    if !output.status.success() {
        return Err("cargo tree failed".to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut unique_deps: HashSet<String> = HashSet::new();

    for line in stdout.lines() {
        if let Some(dep) = line.trim().split_whitespace().next() {
            if !dep.is_empty() {
                unique_deps.insert(dep.to_string());
            }
        }
    }

    Ok(unique_deps.len())
}

/// Get the main binary name from Cargo.toml
pub fn get_binary_name() -> Result<String, String> {
    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .output()
        .map_err(|e| format!("Failed to run cargo metadata: {}", e))?;

    if !output.status.success() {
        return Err("cargo metadata failed".to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Simple string search for binary target
    for line in stdout.lines() {
        if line.contains(r#""kind":["bin"]"#) || line.contains(r#""kind": ["bin"]"#) {
            if let Some(name_line) = stdout
                .lines()
                .skip_while(|l| {
                    !l.contains(r#""kind":["bin"]"#) && !l.contains(r#""kind": ["bin"]"#)
                })
                .find(|l| l.contains(r#""name":"#))
            {
                if let Some(start) = name_line.find(r#""name":""#) {
                    let name_start = start + r#""name":""#.len();
                    if let Some(end) = name_line[name_start..].find('"') {
                        return Ok(name_line[name_start..name_start + end].to_string());
                    }
                }
            }
        }
    }

    Err("No binary target found".to_string())
}

/// Build in release mode
pub fn build_release() -> Result<(), String> {
    let status = Command::new("cargo")
        .args(["build", "--release", "--quiet"])
        .status()
        .map_err(|e| format!("Failed to run cargo build: {}", e))?;

    if !status.success() {
        return Err("cargo build failed".to_string());
    }

    Ok(())
}

/// Get binary size in bytes
pub fn get_binary_size(binary_name: &str) -> Result<u64, String> {
    let path = format!("target/release/{}", binary_name);
    std::fs::metadata(&path)
        .map(|m| m.len())
        .map_err(|e| format!("Failed to get size for {}: {}", path, e))
}

/// Collect all metrics
pub fn collect_metrics(binary_name: &str, should_build: bool) -> Result<Metrics, String> {
    if should_build {
        build_release()?;
    }

    let dep_count = count_dependencies()?;
    let binary_size_bytes = get_binary_size(binary_name)?;

    Ok(Metrics {
        dep_count,
        binary_size_bytes,
    })
}

/// Format bytes as human-readable string
pub fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{}B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1}K", bytes as f64 / 1024.0)
    } else {
        format!("{:.1}M", bytes as f64 / (1024.0 * 1024.0))
    }
}

/// Generate shields.io badge URLs and markdown
pub fn generate_badges(metrics: &Metrics, crate_name: &str) -> String {
    let crates_io_url = format!("https://crates.io/crates/{}", crate_name);
    let size_formatted = format_size(metrics.binary_size_bytes);

    let deps_badge = format!(
        "[![Dependencies: {}](https://img.shields.io/badge/cargo%20tree-{}-blue)]({})",
        metrics.dep_count, metrics.dep_count, crates_io_url
    );

    let size_badge = format!(
        "[![Binary Size: {}](https://img.shields.io/badge/build%20size-{}-green)]({})",
        size_formatted, size_formatted, crates_io_url
    );

    format!("{}\n{}", deps_badge, size_badge)
}

/// Update README file with generated badges using textum
pub fn update_readme(readme_path: &str, badge_content: &str) -> Result<(), String> {
    use textum::{Boundary, BoundaryMode, Patch, Snippet, Target};

    let start = Boundary::new(
        Target::Literal("<!-- auto-generated badges -->".to_string()),
        BoundaryMode::Exclude,
    );
    let end = Boundary::new(
        Target::Literal("<!-- /auto-generated badges -->".to_string()),
        BoundaryMode::Exclude,
    );

    let snippet = Snippet::Between { start, end };

    let patch = Patch {
        file: Some(readme_path.to_string()),
        snippet,
        replacement: format!("\n{}\n", badge_content),
    };

    // Read the file content
    let content = std::fs::read_to_string(readme_path)
        .map_err(|e| format!("Failed to read {}: {}", readme_path, e))?;

    // Apply patch to string
    let updated = patch
        .apply_to_string(&content)
        .map_err(|e| format!("Failed to apply patch: {:?}", e))?;

    // Write back
    std::fs::write(readme_path, updated)
        .map_err(|e| format!("Failed to write {}: {}", readme_path, e))?;

    Ok(())
}

/// Macro for debug output
#[macro_export]
macro_rules! blazon_debug {
    ($($arg:tt)*) => {
        if $crate::debug::is_enabled() {
            eprintln!("[BLAZON DEBUG] {}", format!($($arg)*));
        }
    };
}
