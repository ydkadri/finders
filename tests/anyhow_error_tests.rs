use std::fs;
use std::io::Write;

/// Test that anyhow error context chains work properly
#[test]
fn test_file_not_found_context() {
    use finders::file_finder::Finder;

    let result = Finder::new(Some("/path/that/does/not/exist"));
    assert!(result.is_err(), "Should fail for non-existent path");

    if let Err(err) = result {
        let err_str = format!("{:?}", err);

        // Should contain context chain
        assert!(
            err_str.contains("initializing file finder")
                || err_str.contains("validating root path")
                || err_str.contains("validating path"),
            "Error should contain context chain. Got: {}",
            err_str
        );
    }
}

/// Test that regex compilation errors have helpful context
#[test]
fn test_regex_error_context() {
    use finders::searcher::ReSearcher;

    let result = ReSearcher::new("[invalid(");
    assert!(result.is_err(), "Should fail for invalid regex");

    if let Err(err) = result {
        let err_str = format!("{:?}", err);

        // Should contain both the pattern and context
        assert!(
            err_str.contains("compiling regex pattern"),
            "Error should contain context. Got: {}",
            err_str
        );
        assert!(
            err_str.contains("[invalid("),
            "Error should contain the invalid pattern. Got: {}",
            err_str
        );
    }
}

/// Test that file reading errors have helpful context
#[test]
fn test_file_reading_error_context() -> Result<(), std::io::Error> {
    use finders::{search_files, searcher::Searcher};

    // Create a temporary file then make it unreadable
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("finder_test_unreadable.txt");

    let mut file = fs::File::create(&test_file)?;
    writeln!(file, "test content")?;
    drop(file);

    // On Unix, we can make it unreadable with permissions
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&test_file)?.permissions();
        perms.set_mode(0o000); // No permissions
        fs::set_permissions(&test_file, perms)?;
    }

    let searcher = Searcher::new("test", false);
    let paths = vec![test_file.clone()];
    let mut output = finders::output::StandardOutput::new(finders::output::ColourMode::Never);

    let result = search_files(searcher, paths, false, &mut output);

    // Restore permissions before cleanup
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = fs::metadata(&test_file) {
            let mut perms = metadata.permissions();
            perms.set_mode(0o644);
            let _ = fs::set_permissions(&test_file, perms);
        }
    }

    // Clean up
    let _ = fs::remove_file(&test_file);

    #[cfg(unix)]
    {
        assert!(result.is_err(), "Should fail for unreadable file");

        let err = result.unwrap_err();
        let err_str = format!("{:?}", err);

        // Should contain context about which file failed
        assert!(
            err_str.contains("searching in") || err_str.contains("failed to open"),
            "Error should contain file context. Got: {}",
            err_str
        );
    }

    #[cfg(not(unix))]
    {
        // On non-Unix systems, we can't easily make files unreadable
        // Just verify the test file was created and cleaned up
        let _ = result;
    }

    Ok(())
}

/// Test that binary file warnings work with verbose mode
#[test]
fn test_binary_file_verbose_warning() -> Result<(), std::io::Error> {
    use finders::{search_files, searcher::Searcher};

    // Create a temporary binary file
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("finder_test_binary_verbose.bin");

    let mut file = fs::File::create(&test_file)?;
    // Write some invalid UTF-8 bytes
    file.write_all(&[0xFF, 0xFE, 0xFD, 0x00, 0x80, 0x81])?;
    drop(file);

    let searcher = Searcher::new("test", false);
    let paths = vec![test_file.clone()];
    let mut output = finders::output::StandardOutput::new(finders::output::ColourMode::Never);

    // Should succeed with verbose=true (prints warnings but doesn't fail)
    let result = search_files(searcher, paths, true, &mut output);

    // Clean up
    fs::remove_file(&test_file)?;

    assert!(
        result.is_ok(),
        "Should handle binary file gracefully with verbose mode"
    );

    Ok(())
}
