use std::io::{self, Write};
use std::path::Path;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

use super::{ColourMode, Outputs, SearchMatch};

/// Count output mode (like grep -c)
/// Outputs match count per file in format: path:count
pub struct CountOutput {
    stdout: StandardStream,
    path_colour: ColorSpec,
    current_file: Option<String>,
    current_count: usize,
}

impl CountOutput {
    pub fn new(colour_mode: ColourMode) -> Self {
        let stdout = StandardStream::stdout(colour_mode.to_color_choice());

        let mut path_colour = ColorSpec::new();
        path_colour.set_fg(Some(Color::Green));

        CountOutput {
            stdout,
            path_colour,
            current_file: None,
            current_count: 0,
        }
    }

    fn write_coloured_path(&mut self, path: &Path) -> io::Result<()> {
        self.stdout.set_color(&self.path_colour)?;
        write!(self.stdout, "{}", path.to_string_lossy())?;
        self.stdout.reset()?;
        Ok(())
    }

    fn flush_current_file(&mut self) {
        if let Some(ref path) = self.current_file
            && self.current_count > 0
        {
            // Output path:count
            let path_buf = std::path::PathBuf::from(path);
            let _ = self.write_coloured_path(&path_buf);
            let _ = write!(self.stdout, ":{}", self.current_count);
            let _ = writeln!(self.stdout);
            let _ = self.stdout.flush();
        }
    }
}

impl Default for CountOutput {
    fn default() -> Self {
        Self::new(ColourMode::Auto)
    }
}

impl Outputs for CountOutput {
    fn write_match(&mut self, match_result: &SearchMatch) {
        let path_str = match_result.path.to_string_lossy().to_string();

        // If we're starting a new file, flush the previous file's count
        if self.current_file.as_ref() != Some(&path_str) {
            self.flush_current_file();
            self.current_file = Some(path_str);
            self.current_count = 1;
        } else {
            self.current_count += 1;
        }
    }

    fn write_file(&mut self, path: &Path) {
        // For file-only mode, just output path with count 0
        let _ = self.write_coloured_path(path);
        let _ = write!(self.stdout, ":0");
        let _ = writeln!(self.stdout);
        let _ = self.stdout.flush();
    }

    fn finalize(&mut self) {
        // Output the last file's count
        self.flush_current_file();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_count_aggregation() {
        let mut output = CountOutput::new(ColourMode::Never);
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

        // Verify count is accumulated
        assert_eq!(output.current_file, Some("test.txt".to_string()));
        assert_eq!(output.current_count, 3);
    }

    #[test]
    fn test_count_multiple_files() {
        let mut output = CountOutput::new(ColourMode::Never);
        let path1 = PathBuf::from("file1.txt");
        let path2 = PathBuf::from("file2.txt");

        // Add matches to first file
        output.write_match(&SearchMatch {
            path: &path1,
            line_number: 1,
            content: "match",
            match_positions: &[(0, 5)],
        });

        output.write_match(&SearchMatch {
            path: &path1,
            line_number: 2,
            content: "match",
            match_positions: &[(0, 5)],
        });

        // Switch to second file
        output.write_match(&SearchMatch {
            path: &path2,
            line_number: 1,
            content: "match",
            match_positions: &[(0, 5)],
        });

        // Second file should be current with count 1
        assert_eq!(output.current_file, Some("file2.txt".to_string()));
        assert_eq!(output.current_count, 1);
    }
}
