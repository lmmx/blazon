use blazon_core::{Metrics, format_size, generate_badges};

#[test]
fn test_format_size() {
    assert_eq!(format_size(500), "500B");
    assert_eq!(format_size(1024), "1.0K");
    assert_eq!(format_size(2048), "2.0K");
    assert_eq!(format_size(1024 * 1024), "1.0M");
    assert_eq!(format_size(2 * 1024 * 1024), "2.0M");
    assert_eq!(format_size(1536 * 1024), "1.5M");
}

#[test]
fn test_generate_badges() {
    let metrics = Metrics {
        dep_count: 42,
        binary_size_bytes: 2 * 1024 * 1024, // 2MB
    };

    let badges = generate_badges(&metrics, "test-crate");

    // Check the badges contain expected elements
    assert!(badges.contains("Dependencies: 42"));
    assert!(badges.contains("Binary Size: 2.0M"));
    assert!(badges.contains("https://img.shields.io/badge/cargo%20tree-42-blue"));
    assert!(badges.contains("https://img.shields.io/badge/build%20size-2.0M-green"));
    assert!(badges.contains("https://crates.io/crates/test-crate"));
}

#[test]
fn test_badge_format() {
    let metrics = Metrics {
        dep_count: 123,
        binary_size_bytes: 1500000, // ~1.4M
    };

    let badges = generate_badges(&metrics, "my-crate");

    // Should have two badge lines
    let lines: Vec<&str> = badges.lines().collect();
    assert_eq!(lines.len(), 2);

    // Each line should be a complete markdown badge
    assert!(lines[0].starts_with("[![Dependencies:"));
    assert!(lines[1].starts_with("[![Binary Size:"));
}
