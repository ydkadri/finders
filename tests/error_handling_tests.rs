use finders::file_finder::Finder;
use std::fs;
use std::io::Write;

/// Test that finder handles directories it can search without panicking
#[test]
fn test_finder_handles_valid_directory() {
    // This should not panic even if there are files it can't read
    let finder = Finder::new(Some(".")).expect("Should create finder for current directory");
    let results = finder.find(Some("Cargo.toml"));

    // Should find at least one Cargo.toml without panicking
    assert!(
        !results.is_empty(),
        "Should find Cargo.toml in current directory"
    );
}

/// Test that finder handles non-existent directory with proper error
#[test]
fn test_finder_handles_nonexistent_directory() {
    let result = Finder::new(Some("/path/that/definitely/does/not/exist/12345"));

    // Should return error, not panic
    assert!(result.is_err(), "Should return error for non-existent path");

    if let Err(err) = result {
        assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
    }
}

/// Test search_files with file that exists
#[test]
fn test_search_files_handles_valid_search() -> Result<(), std::io::Error> {
    use finders::{search_files, searcher::Searcher};

    // Create a temporary test file
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("finder_test_search.txt");

    let mut file = fs::File::create(&test_file)?;
    writeln!(file, "This line contains the search term")?;
    writeln!(file, "This line does not")?;
    drop(file);

    // Search for the term
    let searcher = Searcher::new("search", false);
    let paths = vec![test_file.clone()];
    let mut output = finders::output::StandardOutput::new(finders::output::ColourMode::Never);

    // Should complete without panicking
    let result = search_files(searcher, paths, false, &mut output);

    // Clean up
    fs::remove_file(&test_file)?;

    assert!(result.is_ok(), "Search should succeed on valid file");
    Ok(())
}

/// Test that search_files handles files with non-UTF8 content gracefully
#[test]
fn test_search_files_handles_binary_file() -> Result<(), std::io::Error> {
    use finders::{search_files, searcher::Searcher};

    // Create a temporary binary file
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("finder_test_binary.bin");

    let mut file = fs::File::create(&test_file)?;
    // Write some invalid UTF-8 bytes
    file.write_all(&[0xFF, 0xFE, 0xFD, 0x00, 0x80, 0x81])?;
    drop(file);

    // Try to search it - should handle gracefully with verbose=true
    let searcher = Searcher::new("test", false);
    let paths = vec![test_file.clone()];
    let mut output = finders::output::StandardOutput::new(finders::output::ColourMode::Never);

    // Should not panic, even with invalid UTF-8
    let result = search_files(searcher, paths, true, &mut output);

    // Clean up
    fs::remove_file(&test_file)?;

    // Should succeed (may print warnings but shouldn't crash)
    assert!(result.is_ok(), "Should handle binary file gracefully");
    Ok(())
}

/// Test that searcher.search_line doesn't panic on any input
#[test]
fn test_searcher_search_line_robust() {
    use finders::searcher::{Searcher, Searches};

    let searcher = Searcher::new("test", false);

    // Should handle various inputs without panicking
    let long_line = "x".repeat(10000);
    let test_cases = [
        "",
        "normal text",
        "test pattern here",
        "\n\n\n",
        "unicode: 你好世界",
        "emojis: 🚀🎉",
        long_line.as_str(), // Very long line
    ];

    for (idx, test_case) in test_cases.iter().enumerate() {
        let result = searcher.search_line(test_case, idx + 1);
        // Just verify it doesn't panic - result can be Some or None
        let _ = result;
    }
}
