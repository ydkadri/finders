use std::io::{Error, ErrorKind};
use std::path::Path;

pub const DEFAULT_PATH: &str = ".";

pub fn parse(path: Option<&str>) -> Result<&Path, Error> {
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
                    format!("No such path: {}", p),
                ))
            }
        }
        _ => Ok(Path::new(DEFAULT_PATH)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_path() -> Result<(), Error> {
        // Parse a valid directory path
        let cwd: &str = ".";

        let observed_path = parse(Some(cwd))?;
        let expected_path = Path::new(cwd);

        assert_eq!(observed_path, expected_path);

        Ok(())
    }

    #[test]
    fn parse_empty_path() -> Result<(), Error> {
        // Ensure if no path is provided we fall back
        // on the DEFAULT_PATH value (".")
        let observed_path = parse(None)?;
        let expected_path = Path::new(DEFAULT_PATH);

        assert_eq!(observed_path, expected_path);

        Ok(())
    }

    #[test]
    fn parse_invalid_path() -> Result<(), Error> {
        let observed_error = parse(Some("path/to/nowhere")).map_err(|e| e.kind());
        let expected_error = Err(ErrorKind::NotFound);

        assert_eq!(observed_error, expected_error);

        Ok(())
    }
}
