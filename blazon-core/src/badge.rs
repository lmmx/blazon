//! SVG badge generation for Rust project metrics

use std::process::Command;

/// Generate an SVG badge
pub fn create_badge(label: &str, value: &str, color: &str) -> String {
    let label_width = label.len() * 7 + 10;
    let value_width = value.len() * 7 + 10;
    let total_width = label_width + value_width;

    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{total_width}" height="20">
  <linearGradient id="b" x2="0" y2="100%">
    <stop offset="0" stop-color="#bbb" stop-opacity=".1"/>
    <stop offset="1" stop-opacity=".1"/>
  </linearGradient>
  <rect rx="3" width="{total_width}" height="20" fill="#555"/>
  <rect rx="3" x="{label_width}" width="{value_width}" height="20" fill="{color}"/>
  <rect rx="3" width="{total_width}" height="20" fill="url(#b)"/>
  <g fill="#fff" text-anchor="middle" font-family="DejaVu Sans,Verdana,Geneva,sans-serif" font-size="11">
    <text x="{}" y="15" fill="#010101" fill-opacity=".3">{label}</text>
    <text x="{}" y="14">{label}</text>
    <text x="{}" y="15" fill="#010101" fill-opacity=".3">{value}</text>
    <text x="{}" y="14">{value}</text>
  </g>
</svg>"#,
        label_width / 2,
        label_width / 2,
        label_width + value_width / 2,
        label_width + value_width / 2
    )
}

/// Count dependencies using cargo tree
pub fn count_dependencies() -> Result<usize, String> {
    let output = Command::new("cargo")
        .args(["tree", "--edges", "normal", "--prefix", "none"])
        .output()
        .map_err(|e| format!("Failed to run cargo tree: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "cargo tree failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut unique_deps: std::collections::HashSet<&str> = std::collections::HashSet::new();
    
    for line in stdout.lines() {
        let dep = line.trim().split_whitespace().next().unwrap_or("");
        if !dep.is_empty() {
            unique_deps.insert(dep);
        }
    }

    Ok(unique_deps.len())
}

/// Get binary size in bytes
pub fn get_binary_size(binary_name: &str) -> Result<u64, String> {
    let path = format!("target/release/{}", binary_name);
    std::fs::metadata(&path)
        .map(|m| m.len())
        .map_err(|e| format!("Failed to get binary size for {}: {}", path, e))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_badge() {
        let badge = create_badge("test", "123", "blue");
        assert!(badge.contains("<svg"));
        assert!(badge.contains("test"));
        assert!(badge.contains("123"));
    }

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(500), "500B");
        assert_eq!(format_size(2048), "2.0K");
        assert_eq!(format_size(2 * 1024 * 1024), "2.0M");
    }
}
