use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use walkdir::{IntoIter, WalkDir};
mod path_parser;

pub struct Finder<'a> {
    pub path: &'a Path,
}

impl Finder<'_> {
    pub fn new(root: Option<&str>) -> Result<Finder<'_>> {
        let path = path_parser::parse(root).context("validating root path")?;

        Ok(Finder { path })
    }

    pub fn find(&self, query: Option<&str>) -> Vec<PathBuf> {
        // Common file metadata handling for all cases
        let file_iterator = self.find_internal().filter_map(|e| e.ok()).filter_map(|e| {
            match e.metadata() {
                Ok(metadata) if metadata.is_file() => Some(e),
                Ok(_) => None, // Not a file (directory, symlink, etc.)
                Err(err) => {
                    eprintln!(
                        "Warning: Cannot read metadata: {} ({})",
                        e.path().display(),
                        err
                    );
                    None
                }
            }
        });

        // Apply filename filter if provided, otherwise return all files
        if let Some(pattern) = query {
            file_iterator
                .filter_map(|e| {
                    match e.file_name().to_str() {
                        Some(name) if name.contains(pattern) => Some(e),
                        Some(_) => None, // Filename doesn't match pattern
                        None => {
                            eprintln!(
                                "Warning: Skipping file with non-UTF8 name: {}",
                                e.path().display()
                            );
                            None
                        }
                    }
                })
                .map(|e| e.into_path())
                .collect()
        } else {
            file_iterator.map(|e| e.into_path()).collect()
        }
    }

    fn find_internal(&self) -> IntoIter {
        WalkDir::new(self.path).follow_links(true).into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::ErrorKind;

    // TODO: test find (mock WalkDir return value)

    #[test]
    fn initialise_finder_with_dir() -> Result<()> {
        let cwd = ".";
        let finder = Finder::new(Some(cwd))?;

        let expected_finder_path = Path::new(cwd);

        assert_eq!(finder.path, expected_finder_path);

        Ok(())
    }

    #[test]
    fn initialise_finder_default_dir() -> Result<()> {
        let finder = Finder::new(None)?;

        let expected_finder_path = Path::new(path_parser::DEFAULT_PATH);

        assert_eq!(finder.path, expected_finder_path);

        Ok(())
    }

    #[test]
    fn invalid_path_error_propagates() -> Result<()> {
        if let Err(e) = Finder::new(Some("path/to/nowhere")) {
            // Downcast to io::Error to check kind
            let io_err = e
                .downcast_ref::<std::io::Error>()
                .expect("Error should be io::Error");
            assert!(io_err.kind() == ErrorKind::NotFound)
        } else {
            panic!("Expected error for invalid path")
        }

        Ok(())
    }
}
