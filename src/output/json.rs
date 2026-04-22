use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use super::{Outputs, SearchMatch};

/// JSON match representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonMatch {
    pub line: usize,
    pub content: String,
}

/// JSON file representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonFile {
    pub path: String,
    pub matches: Vec<JsonMatch>,
}

/// JSON output mode
/// Outputs structured JSON for machine processing
pub struct JsonOutput {
    files: HashMap<String, Vec<JsonMatch>>,
}

impl JsonOutput {
    pub fn new() -> Self {
        JsonOutput {
            files: HashMap::new(),
        }
    }
}

impl Default for JsonOutput {
    fn default() -> Self {
        Self::new()
    }
}

impl Outputs for JsonOutput {
    fn write_match(&mut self, match_result: &SearchMatch) {
        let path_str = match_result.path.to_string_lossy().to_string();

        let json_match = JsonMatch {
            line: match_result.line_number,
            content: match_result.content.to_string(),
        };

        self.files.entry(path_str).or_default().push(json_match);
    }

    fn write_file(&mut self, path: &Path) {
        // For file-only mode, add path with no matches
        let path_str = path.to_string_lossy().to_string();
        self.files.entry(path_str).or_default();
    }

    fn finalize(&mut self) {
        // Convert HashMap to Vec of JsonFile and serialise
        let mut output: Vec<JsonFile> = self
            .files
            .iter()
            .map(|(path, matches)| JsonFile {
                path: path.clone(),
                matches: matches.clone(),
            })
            .collect();

        // Sort by path for consistent output
        output.sort_by(|a, b| a.path.cmp(&b.path));

        // Serialise and print
        match serde_json::to_string_pretty(&output) {
            Ok(json) => println!("{}", json),
            Err(e) => eprintln!("Error serialising JSON: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_json_match_serialisation() {
        let json_match = JsonMatch {
            line: 42,
            content: "test content".to_string(),
        };

        let json = serde_json::to_string(&json_match).unwrap();
        assert!(json.contains("\"line\":42"));
        assert!(json.contains("\"content\":\"test content\""));
    }

    #[test]
    fn test_json_file_serialisation() {
        let json_file = JsonFile {
            path: "src/test.rs".to_string(),
            matches: vec![
                JsonMatch {
                    line: 1,
                    content: "first line".to_string(),
                },
                JsonMatch {
                    line: 5,
                    content: "fifth line".to_string(),
                },
            ],
        };

        let json = serde_json::to_string(&json_file).unwrap();
        assert!(json.contains("\"path\":\"src/test.rs\""));
        assert!(json.contains("\"line\":1"));
        assert!(json.contains("\"line\":5"));
    }

    #[test]
    fn test_json_output_accumulates_matches() {
        let mut output = JsonOutput::new();
        let path = PathBuf::from("test.txt");

        // Add multiple matches from same file
        output.write_match(&SearchMatch {
            path: &path,
            line_number: 1,
            content: "first match",
            match_positions: &[(0, 5)],
        });

        output.write_match(&SearchMatch {
            path: &path,
            line_number: 3,
            content: "second match",
            match_positions: &[(0, 6)],
        });

        // Verify matches are stored
        assert_eq!(output.files.len(), 1);
        assert_eq!(output.files.get("test.txt").unwrap().len(), 2);
    }

    #[test]
    fn test_json_output_write_file() {
        let mut output = JsonOutput::new();
        let path = PathBuf::from("empty.txt");

        // Add file with no matches
        output.write_file(&path);

        // Verify file is stored with empty matches
        assert_eq!(output.files.len(), 1);
        assert_eq!(output.files.get("empty.txt").unwrap().len(), 0);
    }
}
