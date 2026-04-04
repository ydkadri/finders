use std::process::Command;

/// Test that invalid regex patterns display user-friendly errors
#[test]
fn test_invalid_regex_pattern_cli() {
    let output = Command::new(env!("CARGO_BIN_EXE_finder"))
        .arg(".")
        .arg("--regex-pattern")
        .arg("[invalid(")
        .output()
        .expect("Failed to execute finder");

    // Should exit with error
    assert!(!output.status.success());

    // Should show error message
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Invalid regex pattern"),
        "Expected error message, got: {}",
        stderr
    );
    assert!(
        stderr.contains("unclosed character class"),
        "Expected regex error details, got: {}",
        stderr
    );
}

/// Test that another invalid regex pattern is caught
#[test]
fn test_unclosed_group_regex_cli() {
    let output = Command::new(env!("CARGO_BIN_EXE_finder"))
        .arg(".")
        .arg("--regex-pattern")
        .arg("(unclosed")
        .output()
        .expect("Failed to execute finder");

    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Invalid regex pattern"),
        "Expected error message, got: {}",
        stderr
    );
}

/// Test that valid regex patterns still work
#[test]
fn test_valid_regex_pattern_cli() {
    let output = Command::new(env!("CARGO_BIN_EXE_finder"))
        .arg(".")
        .arg("--file-pattern")
        .arg("Cargo.toml")
        .arg("--regex-pattern")
        .arg("version")
        .output()
        .expect("Failed to execute finder");

    // Should succeed
    assert!(
        output.status.success(),
        "Command should succeed with valid regex. Stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Should find Cargo.toml with version field
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Cargo.toml"),
        "Should find Cargo.toml, got: {}",
        stdout
    );
}
