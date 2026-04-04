use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Configuration for fixture generation
pub struct FixtureConfig {
    pub name: &'static str,
    pub num_files: usize,
    pub common_pattern_ratio: f32, // Ratio of files containing "function" keyword
}

impl FixtureConfig {
    pub const SMALL: Self = Self {
        name: "small",
        num_files: 100,
        common_pattern_ratio: 0.5,
    };

    pub const MEDIUM: Self = Self {
        name: "medium",
        num_files: 1000,
        common_pattern_ratio: 0.5,
    };

    pub const LARGE: Self = Self {
        name: "large",
        num_files: 10000,
        common_pattern_ratio: 0.5,
    };
}

/// The rare pattern that appears in exactly one file
pub const RARE_PATTERN: &str = "BENCHMARK_MARKER_UNIQUE_XYZ123";

/// Generate realistic Rust source files for benchmarking
pub fn generate_fixtures(base_dir: &Path, config: &FixtureConfig) -> std::io::Result<PathBuf> {
    let fixture_dir = base_dir.join(config.name);

    // Skip if already generated
    if fixture_dir.exists() {
        return Ok(fixture_dir);
    }

    fs::create_dir_all(&fixture_dir)?;

    let num_with_functions = (config.num_files as f32 * config.common_pattern_ratio) as usize;
    let rare_file_index = config.num_files / 2; // Put rare pattern in middle file

    for i in 0..config.num_files {
        let file_path = fixture_dir.join(format!("file_{:05}.rs", i));
        let mut file = fs::File::create(&file_path)?;

        // Determine file structure
        let has_functions = i < num_with_functions;
        let has_rare_marker = i == rare_file_index;

        generate_rust_file(&mut file, i, has_functions, has_rare_marker)?;
    }

    Ok(fixture_dir)
}

/// Generate a single realistic Rust source file
fn generate_rust_file(
    file: &mut fs::File,
    index: usize,
    has_functions: bool,
    has_rare_marker: bool,
) -> std::io::Result<()> {
    // File header with module docs
    writeln!(file, "//! Module file_{:05}", index)?;
    writeln!(file, "//!")?;
    writeln!(
        file,
        "//! This module contains generated code for benchmarking."
    )?;
    writeln!(file)?;

    // Imports
    writeln!(file, "use std::collections::HashMap;")?;
    writeln!(file, "use std::io::Result;")?;
    writeln!(file)?;

    // Struct definition (varies by index)
    if index.is_multiple_of(3) {
        writeln!(file, "/// Data structure for module {}", index)?;
        writeln!(file, "#[derive(Debug, Clone)]")?;
        writeln!(file, "pub struct Data{} {{", index)?;
        writeln!(file, "    id: usize,")?;
        writeln!(file, "    name: String,")?;
        writeln!(file, "    values: Vec<i32>,")?;
        writeln!(file, "}}")?;
        writeln!(file)?;
    }

    // Functions (if configured)
    if has_functions {
        // Simple function
        writeln!(file, "/// Processes data for module {}", index)?;
        writeln!(
            file,
            "pub fn process_data(input: &str) -> Result<String> {{"
        )?;
        writeln!(file, "    // Implementation details")?;
        writeln!(file, "    let result = input.to_uppercase();")?;
        writeln!(file, "    Ok(result)")?;
        writeln!(file, "}}")?;
        writeln!(file)?;

        // Complex function with more code
        writeln!(file, "/// Analyzes data and returns statistics")?;
        writeln!(
            file,
            "pub fn analyze_data(values: &[i32]) -> HashMap<String, f64> {{"
        )?;
        writeln!(file, "    let mut stats = HashMap::new();")?;
        writeln!(file)?;
        writeln!(file, "    if values.is_empty() {{")?;
        writeln!(file, "        return stats;")?;
        writeln!(file, "    }}")?;
        writeln!(file)?;
        writeln!(file, "    let sum: i32 = values.iter().sum();")?;
        writeln!(file, "    let avg = sum as f64 / values.len() as f64;")?;
        writeln!(file)?;
        writeln!(file, "    stats.insert(\"mean\".to_string(), avg);")?;
        writeln!(
            file,
            "    stats.insert(\"count\".to_string(), values.len() as f64);"
        )?;
        writeln!(file)?;
        writeln!(file, "    stats")?;
        writeln!(file, "}}")?;
        writeln!(file)?;
    } else {
        // Files without functions - just constants and types
        writeln!(file, "/// Configuration constant")?;
        writeln!(file, "pub const CONFIG_VALUE: usize = {};", index)?;
        writeln!(file)?;
        writeln!(file, "/// Type alias for convenience")?;
        writeln!(file, "pub type DataMap = HashMap<usize, String>;")?;
        writeln!(file)?;
    }

    // Rare marker (in exactly one file)
    if has_rare_marker {
        writeln!(file, "// Special marker: {}", RARE_PATTERN)?;
        writeln!(file, "const _MARKER: &str = \"{}\";", RARE_PATTERN)?;
        writeln!(file)?;
    }

    // Tests module (in some files)
    if index.is_multiple_of(5) {
        writeln!(file, "#[cfg(test)]")?;
        writeln!(file, "mod tests {{")?;
        writeln!(file, "    use super::*;")?;
        writeln!(file)?;
        writeln!(file, "    #[test]")?;
        writeln!(file, "    fn test_basic() {{")?;
        writeln!(file, "        assert_eq!(2 + 2, 4);")?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}")?;
    }

    Ok(())
}

/// Remove all generated fixtures
#[allow(dead_code)]
pub fn cleanup_fixtures(base_dir: &Path) -> std::io::Result<()> {
    if base_dir.exists() {
        fs::remove_dir_all(base_dir)?;
    }
    Ok(())
}
