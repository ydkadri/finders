use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::PathBuf;

pub mod file_finder;
pub mod searcher;

const CHUNK_SIZE: usize = 8192; // 8KB chunks for reading files

pub fn search_files(
    searcher: impl searcher::Searches,
    paths: Vec<PathBuf>,
    verbose: bool,
) -> Result<(), Error> {
    // Process files one at a time in a streaming fashion
    for path in paths {
        if let Err(e) = search_file(&searcher, &path, verbose) {
            if e.kind() == ErrorKind::InvalidData {
                if verbose {
                    println!("Cannot read file: {:?}", path);
                }
                continue;
            } else {
                return Err(e);
            }
        }
    }

    Ok(())
}

fn search_file(
    searcher: &impl searcher::Searches,
    path: &PathBuf,
    verbose: bool,
) -> Result<(), Error> {
    // Open file and create buffered reader for efficient streaming
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            if verbose {
                println!("Cannot open file: {:?} - {}", path, e);
            }
            return Err(e);
        }
    };

    let reader = BufReader::with_capacity(CHUNK_SIZE, file);
    let mut rownum = 1;

    // Stream through file line by line, processing as we go
    for line in reader.lines() {
        match line {
            Ok(content) => {
                // Search this single line
                if let Some(result) = searcher.search_line(&content, rownum) {
                    println!(
                        "{:>4}: {:<56} {}",
                        result.rownum,
                        path.as_path().to_str().unwrap(),
                        result.line
                    );
                }
                rownum += 1;
            }
            Err(e) => {
                if e.kind() == ErrorKind::InvalidData {
                    if verbose {
                        println!("Cannot read line {} in file: {:?}", rownum, path);
                    }
                    // Continue to next line on encoding errors
                    rownum += 1;
                    continue;
                } else {
                    return Err(e);
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
    fn test_search_files_streaming() -> Result<(), Error> {
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

        // This should find 2 matches (lines 1 and 3)
        let result = search_files(searcher, paths, false);

        // Clean up
        fs::remove_file(&test_file)?;

        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn test_search_line_functionality() -> Result<(), Error> {
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
    fn test_chunked_reading() -> Result<(), Error> {
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

        let result = search_files(searcher, paths, false);

        // Clean up
        fs::remove_file(&test_file)?;

        assert!(result.is_ok());
        Ok(())
    }
}
