use std::path::Path;

mod colour;
mod count;
mod files_only;
mod json;
mod standard;

pub use colour::ColourMode;
pub use count::CountOutput;
pub use files_only::FilesOnlyOutput;
pub use json::{JsonFile, JsonMatch, JsonOutput};
pub use standard::StandardOutput;

/// Result of a search match containing line number and content
pub struct SearchMatch<'a> {
    pub path: &'a Path,
    pub line_number: usize,
    pub content: &'a str,
    pub match_positions: &'a [(usize, usize)], // (start, end) byte positions
}

/// Trait for different output formats
pub trait Outputs {
    /// Output a single search match
    fn write_match(&mut self, match_result: &SearchMatch);

    /// Output a file path (for file-only mode)
    fn write_file(&mut self, path: &Path);

    /// Finalise output (e.g., close JSON array)
    fn finalize(&mut self) {}
}
