use std::io::{self, Write};
use std::path::Path;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

use super::{ColourMode, Outputs, SearchMatch};

/// Standard output format with coloured output
pub struct StandardOutput {
    stdout: StandardStream,
    // Colour specs for different elements
    path_colour: ColorSpec,
    line_number_colour: ColorSpec,
    match_colour: ColorSpec,
}

impl StandardOutput {
    pub fn new(colour_mode: ColourMode) -> Self {
        let stdout = StandardStream::stdout(colour_mode.to_color_choice());

        let mut path_colour = ColorSpec::new();
        path_colour.set_fg(Some(Color::Green));

        let mut line_number_colour = ColorSpec::new();
        line_number_colour.set_fg(Some(Color::Cyan));

        let mut match_colour = ColorSpec::new();
        match_colour.set_fg(Some(Color::White));
        match_colour.set_bg(Some(Color::Blue));
        match_colour.set_bold(true);

        StandardOutput {
            stdout,
            path_colour,
            line_number_colour,
            match_colour,
        }
    }

    /// Write coloured path
    fn write_coloured_path(&mut self, path: &Path) -> io::Result<()> {
        self.stdout.set_color(&self.path_colour)?;
        write!(self.stdout, "{}", path.to_string_lossy())?;
        self.stdout.reset()?;
        Ok(())
    }

    /// Write coloured line number
    fn write_coloured_line_number(&mut self, line_num: usize) -> io::Result<()> {
        self.stdout.set_color(&self.line_number_colour)?;
        write!(self.stdout, "{}", line_num)?;
        self.stdout.reset()?;
        Ok(())
    }

    /// Write content with highlighted matches
    fn write_highlighted_content(
        &mut self,
        content: &str,
        match_positions: &[(usize, usize)],
    ) -> io::Result<()> {
        if match_positions.is_empty() {
            // No matches to highlight, just write content
            write!(self.stdout, "{}", content)?;
            return Ok(());
        }

        let mut last_end = 0;
        for (start, end) in match_positions {
            // Write text before match
            write!(self.stdout, "{}", &content[last_end..*start])?;
            // Write highlighted match
            self.stdout.set_color(&self.match_colour)?;
            write!(self.stdout, "{}", &content[*start..*end])?;
            self.stdout.reset()?;
            last_end = *end;
        }
        // Write remaining text after last match
        write!(self.stdout, "{}", &content[last_end..])?;
        Ok(())
    }
}

impl Default for StandardOutput {
    fn default() -> Self {
        Self::new(ColourMode::Auto)
    }
}

impl Outputs for StandardOutput {
    fn write_match(&mut self, match_result: &SearchMatch) {
        // v3.0.0 standard format: "path:line: content"
        // Matches grep/ripgrep conventions
        // With colours:
        // - Path: green
        // - Line number: cyan
        // - Separators: default
        // - Content: default with highlighted matches (bold white on blue)

        // Note: Ignoring errors for now since println! doesn't expose errors either
        let _ = self.write_coloured_path(match_result.path);
        let _ = write!(self.stdout, ":");
        let _ = self.write_coloured_line_number(match_result.line_number);
        let _ = write!(self.stdout, ": ");
        let _ = self.write_highlighted_content(match_result.content, match_result.match_positions);
        let _ = writeln!(self.stdout);
        let _ = self.stdout.flush();
    }

    fn write_file(&mut self, path: &Path) {
        // v3.0.0 format: plain path without quotes
        // With colours: path in green
        let _ = self.write_coloured_path(path);
        let _ = writeln!(self.stdout);
        let _ = self.stdout.flush();
    }
}
