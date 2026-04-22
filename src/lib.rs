use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind};
use std::path::PathBuf;

pub mod file_finder;
pub mod output;
pub mod searcher;

// Re-export commonly used types for convenience
pub use file_finder::Finder;
pub use output::{
    ColourMode, CountOutput, FilesOnlyOutput, JsonOutput, Outputs, SearchMatch, StandardOutput,
};
pub use searcher::{ReSearcher, SearchResult, Searcher, Searches};

/// Buffer size for reading files (8KB)
///
/// This size is chosen as a balance between memory usage and I/O efficiency.
/// The BufReader uses this capacity to minimize system calls while keeping
/// memory footprint reasonable for processing many files.
const CHUNK_SIZE: usize = 8192;

pub fn search_files(
    searcher: impl searcher::Searches,
    paths: impl IntoIterator<Item = PathBuf>,
    verbose: bool,
    output: &mut dyn output::Outputs,
) -> Result<()> {
    // Process files one at a time in a streaming fashion
    for path in paths {
        if let Err(e) = search_file(&searcher, &path, verbose, output)
            .context(format!("searching in '{}'", path.display()))
        {
            // Check if it's an encoding error (can continue)
            if let Some(io_err) = e.downcast_ref::<std::io::Error>()
                && io_err.kind() == ErrorKind::InvalidData
            {
                if verbose {
                    eprintln!("Warning: Cannot read file '{}': {}", path.display(), io_err);
                }
                continue;
            }
            // Other errors are fatal
            return Err(e);
        }
    }

    output.finalize();
    Ok(())
}

fn search_file(
    searcher: &impl searcher::Searches,
    path: &PathBuf,
    verbose: bool,
    output: &mut dyn output::Outputs,
) -> Result<()> {
    // Open file and create buffered reader for efficient streaming
    let file = File::open(path).context(format!("failed to open '{}'", path.display()))?;

    let reader = BufReader::with_capacity(CHUNK_SIZE, file);
    let mut rownum = 1;

    // Stream through file line by line, processing as we go
    for line in reader.lines() {
        match line {
            Ok(content) => {
                // Search this single line
                if let Some(result) = searcher.search_line(&content, rownum) {
                    let search_match = output::SearchMatch {
                        path: path.as_path(),
                        line_number: result.rownum,
                        content: &result.line,
                        match_positions: &result.match_positions,
                    };
                    output.write_match(&search_match);
                }
                rownum += 1;
            }
            Err(e) => {
                if e.kind() == ErrorKind::InvalidData {
                    if verbose {
                        eprintln!(
                            "Warning: Cannot read line {} in file '{}': {}",
                            rownum,
                            path.display(),
                            e
                        );
                    }
                    // Continue to next line on encoding errors
                    rownum += 1;
                    continue;
                } else {
                    return Err(e).context(format!(
                        "reading line {} in '{}'",
                        rownum,
                        path.display()
                    ));
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::searcher::Searches;
    use super::*;
    use std::fs;
    use std::io::Write;

    #[test]
    fn test_search_files_streaming() -> Result<()> {
        // Create a temporary test file
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_streaming.txt");

        let mut file = fs::File::create(&test_file)?;
        writeln!(file, "line one")?;
        writeln!(file, "LINE TWO")?;
        writeln!(file, "line three")?;

        // Test searching with case-sensitive searcher
        let searcher = searcher::Searcher::new("line", false);
        let paths = vec![test_file.clone()];
        let mut output = output::StandardOutput::new(output::ColourMode::Never);

        // This should find 2 matches (lines 1 and 3)
        let result = search_files(searcher, paths, false, &mut output);

        // Clean up
        fs::remove_file(&test_file)?;

        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn test_search_line_functionality() -> Result<()> {
        // Test that search_line works correctly
        let searcher = searcher::Searcher::new("test", false);

        let result = searcher.search_line("this is a test line", 1);
        assert!(result.is_some());
        assert_eq!(result.unwrap().rownum, 1);

        let no_match = searcher.search_line("this line has no match", 2);
        assert!(no_match.is_none());

        Ok(())
    }

    #[test]
    fn test_chunked_reading() -> Result<()> {
        // Create a large test file to test chunking
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_chunked.txt");

        let mut file = fs::File::create(&test_file)?;
        // Write lines that will exceed CHUNK_SIZE
        for i in 0..1000 {
            writeln!(file, "This is line {} with some content to search", i)?;
        }

        let searcher = searcher::Searcher::new("line 500", false);
        let paths = vec![test_file.clone()];
        let mut output = output::StandardOutput::new(output::ColourMode::Never);

        let result = search_files(searcher, paths, false, &mut output);

        // Clean up
        fs::remove_file(&test_file)?;

        assert!(result.is_ok());
        Ok(())
    }
}
