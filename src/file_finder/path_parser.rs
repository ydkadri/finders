use anyhow::{Context, Result};
use std::io::{Error, ErrorKind};
use std::path::Path;

pub const DEFAULT_PATH: &str = ".";

pub fn parse(path: Option<&str>) -> Result<&Path> {
    // Parse a string into a path object and
    // validate that the path exists
    match path {
        Some(p) => {
            let new_path = Path::new(p);

            if new_path.exists() {
                Ok(new_path)
            } else {
                Err(Error::new(
                    ErrorKind::NotFound,
                    format!("path does not exist: '{}'", p),
                ))
                .context("validating path")?
            }
        }
        _ => Ok(Path::new(DEFAULT_PATH)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_path() -> Result<()> {
        // Parse a valid directory path
        let cwd: &str = ".";

        let observed_path = parse(Some(cwd))?;
        let expected_path = Path::new(cwd);

        assert_eq!(observed_path, expected_path);

        Ok(())
    }

    #[test]
    fn parse_empty_path() -> Result<()> {
        // Ensure if no path is provided we fall back
        // on the DEFAULT_PATH value (".")
        let observed_path = parse(None)?;
        let expected_path = Path::new(DEFAULT_PATH);

        assert_eq!(observed_path, expected_path);

        Ok(())
    }

    #[test]
    fn parse_invalid_path() -> Result<()> {
        let result = parse(Some("path/to/nowhere"));
        assert!(result.is_err());

        // Verify it's a NotFound error by downcasting
        if let Err(e) = result {
            let io_err = e
                .downcast_ref::<Error>()
                .expect("Error should be io::Error");
            assert_eq!(io_err.kind(), ErrorKind::NotFound);
        }

        Ok(())
    }
}
