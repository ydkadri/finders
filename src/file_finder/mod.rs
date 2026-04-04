use std::io::Error;
use std::path::{Path, PathBuf};
use walkdir::{IntoIter, WalkDir};
mod path_parser;

pub struct Finder<'a> {
    pub path: &'a Path,
}

impl Finder<'_> {
    pub fn new(root: Option<&str>) -> Result<Finder<'_>, Error> {
        let path = path_parser::parse(root)?;

        Ok(Finder { path })
    }

    fn find_internal(&self) -> IntoIter {
        WalkDir::new(self.path).follow_links(true).into_iter()
    }

    fn unfiltered_find(&self) -> Vec<PathBuf> {
        // Recursively find files from the finder root
        let mut results = Vec::new();
        let filepath_iterator = self.find_internal().filter_map(|e| e.ok()).filter_map(|e| {
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

        for entry in filepath_iterator {
            results.push(entry.into_path());
        }

        results
    }

    fn filtered_find(&self, query: &str) -> Vec<PathBuf> {
        // Recursively find files from the finder root
        // filtering to filenames containing some value
        let mut results = Vec::new();
        let filepath_iterator = self
            .find_internal()
            .filter_map(|e| e.ok())
            .filter_map(|e| {
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
            })
            .filter_map(|e| {
                match e.file_name().to_str() {
                    Some(name) if name.contains(query) => Some(e),
                    Some(_) => None, // Filename doesn't match query
                    None => {
                        eprintln!(
                            "Warning: Skipping file with non-UTF8 name: {}",
                            e.path().display()
                        );
                        None
                    }
                }
            });

        for entry in filepath_iterator {
            results.push(entry.into_path());
        }

        results
    }

    pub fn find(&self, query: Option<&str>) -> Vec<PathBuf> {
        match query {
            Some(q) => self.filtered_find(q),
            None => self.unfiltered_find(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::ErrorKind;

    // TODO: test find (mock WalkDir return value)

    #[test]
    fn initialise_finder_with_dir() -> Result<(), Error> {
        let cwd = ".";
        let finder = Finder::new(Some(cwd))?;

        let expected_finder_path = Path::new(cwd);

        assert_eq!(finder.path, expected_finder_path);

        Ok(())
    }

    #[test]
    fn initialise_finder_default_dir() -> Result<(), Error> {
        let finder = Finder::new(None)?;

        let expected_finder_path = Path::new(path_parser::DEFAULT_PATH);

        assert_eq!(finder.path, expected_finder_path);

        Ok(())
    }

    #[test]
    fn invalid_path_error_propagates() -> Result<(), Error> {
        if let Err(e) = Finder::new(Some("path/to/nowhere")) {
            assert!(e.kind() == ErrorKind::NotFound)
        } else {
            panic!("Expected error for invalid path")
        }

        Ok(())
    }
}
