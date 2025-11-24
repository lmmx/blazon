use std::fs;
use tempfile::TempDir;

#[test]
fn test_update_readme() {
    let temp_dir = TempDir::new().unwrap();
    let readme_path = temp_dir.path().join("README.md");

    // Create a README with markers
    let initial_content = r#"# My Project

<!-- auto-generated badges -->
old badge content here
<!-- /auto-generated badges -->

Some other content.
"#;

    fs::write(&readme_path, initial_content).unwrap();

    // Update with new badges
    let new_badges = "[![test](https://example.com)](https://example.com)";
    blazon_core::update_readme(readme_path.to_str().unwrap(), new_badges).unwrap();

    // Read back and verify
    let updated = fs::read_to_string(&readme_path).unwrap();

    assert!(updated.contains("# My Project"));
    assert!(updated.contains("Some other content."));
    assert!(updated.contains(new_badges));
    assert!(!updated.contains("old badge content here"));
}

#[test]
fn test_update_readme_preserves_structure() {
    let temp_dir = TempDir::new().unwrap();
    let readme_path = temp_dir.path().join("README.md");

    let content = r#"# Title

Before badges.

<!-- auto-generated badges -->
<!-- /auto-generated badges -->

After badges.
"#;

    fs::write(&readme_path, content).unwrap();

    blazon_core::update_readme(readme_path.to_str().unwrap(), "NEW").unwrap();

    let updated = fs::read_to_string(&readme_path).unwrap();

    // Check structure is preserved
    assert!(updated.contains("Before badges."));
    assert!(updated.contains("After badges."));
    assert!(updated.contains("NEW"));

    // Check markers are still there
    assert!(updated.contains("<!-- auto-generated badges -->"));
    assert!(updated.contains("<!-- /auto-generated badges -->"));
}
