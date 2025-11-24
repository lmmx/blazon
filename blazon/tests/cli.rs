#![cfg(feature = "cli")]
use assert_cmd::Command;
use assert_cmd::cargo;
use insta::assert_snapshot;
use std::fs;
use tempfile::TempDir;

/// Helper to create a minimal Cargo.toml for testing
fn create_test_cargo_toml(temp_dir: &TempDir) {
    let cargo_toml = r#"[package]
name = "test-crate"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "blazon"
path = "src/main.rs"
"#;
    fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml).unwrap();
}

/// Helper to create a fake binary for testing
fn create_fake_binary(temp_dir: &TempDir, name: &str) {
    let target_dir = temp_dir.path().join("target/release");
    fs::create_dir_all(&target_dir).unwrap();

    let binary_path = target_dir.join(name);
    fs::write(&binary_path, "fake binary content").unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&binary_path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&binary_path, perms).unwrap();
    }
}

#[test]
fn test_help_flag() {
    let output = Command::new(cargo::cargo_bin!("blazon"))
        .arg("--help")
        .output()
        .expect("failed to run command");

    assert!(output.status.success(), "command did not exit successfully");
    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    assert_snapshot!(stdout);
}

#[test]
fn test_missing_markers() {
    let temp_dir = TempDir::new().unwrap();
    create_test_cargo_toml(&temp_dir);
    let readme_path = temp_dir.path().join("README.md");

    fs::write(&readme_path, "# No markers here\n").unwrap();
    create_fake_binary(&temp_dir, "blazon");

    Command::new(cargo::cargo_bin!("blazon"))
        .current_dir(temp_dir.path())
        .arg(readme_path.to_str().unwrap())
        .arg("--no-build")
        .arg("--binary")
        .arg("blazon")
        .assert()
        .failure()
        .stderr(predicates::str::contains("Failed to apply patch"));
}

#[test]
fn test_update_readme_success() {
    let temp_dir = TempDir::new().unwrap();
    create_test_cargo_toml(&temp_dir);
    let readme_path = temp_dir.path().join("README.md");

    let initial = r#"# Test Project

<!-- auto-generated badges -->
old content
<!-- /auto-generated badges -->

More content.
"#;

    fs::write(&readme_path, initial).unwrap();
    create_fake_binary(&temp_dir, "blazon");

    Command::new(cargo::cargo_bin!("blazon"))
        .current_dir(temp_dir.path())
        .arg(readme_path.to_str().unwrap())
        .arg("--no-build")
        .arg("--binary")
        .arg("blazon")
        .arg("--crate-name")
        .arg("test-crate")
        .assert()
        .success()
        .stderr(predicates::str::contains("Successfully updated"));

    let updated = fs::read_to_string(&readme_path).unwrap();
    assert!(updated.contains("Dependencies:"));
    assert!(updated.contains("Binary Size:"));
    assert!(!updated.contains("old content"));
}

#[test]
fn test_verbose_output() {
    let temp_dir = TempDir::new().unwrap();
    create_test_cargo_toml(&temp_dir);
    let readme_path = temp_dir.path().join("README.md");

    fs::write(
        &readme_path,
        r#"
<!-- auto-generated badges -->
<!-- /auto-generated badges -->
"#,
    )
    .unwrap();
    create_fake_binary(&temp_dir, "blazon");

    Command::new(cargo::cargo_bin!("blazon"))
        .current_dir(temp_dir.path())
        .arg(readme_path.to_str().unwrap())
        .arg("--no-build")
        .arg("--binary")
        .arg("blazon")
        .arg("--verbose")
        .assert()
        .success()
        .stderr(predicates::str::contains("Binary:"))
        .stderr(predicates::str::contains("Dependencies:"))
        .stderr(predicates::str::contains("Generated badges:"));
}

#[test]
fn test_infer_binary_name() {
    let temp_dir = TempDir::new().unwrap();
    create_test_cargo_toml(&temp_dir);
    let readme_path = temp_dir.path().join("README.md");

    fs::write(
        &readme_path,
        r#"
<!-- auto-generated badges -->
<!-- /auto-generated badges -->
"#,
    )
    .unwrap();
    create_fake_binary(&temp_dir, "blazon");
    create_fake_binary(&temp_dir, "test-crate");

    Command::new(cargo::cargo_bin!("blazon"))
        .current_dir(temp_dir.path())
        .arg(readme_path.to_str().unwrap())
        .arg("--no-build")
        .assert()
        .success();
}

#[test]
fn test_missing_binary_error() {
    let temp_dir = TempDir::new().unwrap();
    create_test_cargo_toml(&temp_dir);
    let readme_path = temp_dir.path().join("README.md");

    fs::write(&readme_path, "test").unwrap();

    Command::new(cargo::cargo_bin!("blazon"))
        .current_dir(temp_dir.path())
        .arg(readme_path.to_str().unwrap())
        .arg("--no-build")
        .arg("--binary")
        .arg("nonexistent-binary")
        .assert()
        .failure()
        .stderr(predicates::str::contains("Failed to get size"));
}
