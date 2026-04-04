use std::io::{self, Write};
use std::path::Path;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

use super::{ColourMode, Outputs, SearchMatch};

/// Files-only output mode (like grep -l)
/// Outputs only file paths that contain matches, one per line
pub struct FilesOnlyOutput {
    stdout: StandardStream,
    path_colour: ColorSpec,
    current_file: Option<String>,
}

impl FilesOnlyOutput {
    pub fn new(colour_mode: ColourMode) -> Self {
        let stdout = StandardStream::stdout(colour_mode.to_color_choice());

        let mut path_colour = ColorSpec::new();
        path_colour.set_fg(Some(Color::Green));

        FilesOnlyOutput {
            stdout,
            path_colour,
            current_file: None,
        }
    }

    fn write_coloured_path(&mut self, path: &Path) -> io::Result<()> {
        self.stdout.set_color(&self.path_colour)?;
        write!(self.stdout, "{}", path.to_string_lossy())?;
        self.stdout.reset()?;
        Ok(())
    }
}

impl Default for FilesOnlyOutput {
    fn default() -> Self {
        Self::new(ColourMode::Auto)
    }
}

impl Outputs for FilesOnlyOutput {
    fn write_match(&mut self, match_result: &SearchMatch) {
        // Only output each file path once (first match)
        let path_str = match_result.path.to_string_lossy().to_string();
        if self.current_file.as_ref() != Some(&path_str) {
            let _ = self.write_coloured_path(match_result.path);
            let _ = writeln!(self.stdout);
            let _ = self.stdout.flush();
            self.current_file = Some(path_str);
        }
    }

    fn write_file(&mut self, path: &Path) {
        // Same as StandardOutput for file-only mode
        let _ = self.write_coloured_path(path);
        let _ = writeln!(self.stdout);
        let _ = self.stdout.flush();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_files_only_deduplication() {
        let mut output = FilesOnlyOutput::new(ColourMode::Never);
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

        output.write_match(&SearchMatch {
            path: &path,
            line_number: 7,
            content: "third match",
            match_positions: &[(0, 5)],
        });

        // Verify file is tracked (only output once)
        assert_eq!(output.current_file, Some("test.txt".to_string()));
    }

    #[test]
    fn test_files_only_multiple_files() {
        let mut output = FilesOnlyOutput::new(ColourMode::Never);
        let path1 = PathBuf::from("file1.txt");
        let path2 = PathBuf::from("file2.txt");

        // Add matches to different files
        output.write_match(&SearchMatch {
            path: &path1,
            line_number: 1,
            content: "match",
            match_positions: &[(0, 5)],
        });

        output.write_match(&SearchMatch {
            path: &path2,
            line_number: 1,
            content: "match",
            match_positions: &[(0, 5)],
        });

        // Should track the second file
        assert_eq!(output.current_file, Some("file2.txt".to_string()));
    }
}
