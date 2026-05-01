// Simple benchmark for profiling finder's search performance
// This creates a realistic test scenario and runs it many times
// so the profiler has enough samples to work with

use std::fs;
use std::path::PathBuf;
use finders::{Finder, search_files, StandardOutput, Searcher, ColourMode};

fn main() {
    // Create a test directory with realistic structure
    let temp_dir = setup_test_directory();

    println!("Profiling finder search on {} files...", count_files(&temp_dir));
    println!("Running 100 iterations for profiling...");

    // Run the search many times so profiler can collect samples
    for i in 0..100 {
        if i % 10 == 0 {
            println!("Iteration {}/100", i);
        }

        // This mirrors what the CLI does
        let temp_dir_str = temp_dir.to_str().expect("Invalid path");
        let finder = Finder::new(Some(temp_dir_str)).expect("Failed to create finder");
        let files = finder.find(Some(".rs"), false);

        // Search for a common pattern (appears in ~50% of files)
        let searcher = Searcher::new("TODO", false);
        let mut output = StandardOutput::new(ColourMode::Never); // No colours for profiling

        search_files(searcher, files, false, &mut output).expect("Failed to search");
    }

    println!("Profiling complete!");

    // Cleanup
    let _ = fs::remove_dir_all(&temp_dir);
}

fn setup_test_directory() -> PathBuf {
    let temp_dir = std::env::temp_dir().join("finder_profile_test");

    // Clean up if exists
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");

    // Create ~1000 files with various patterns
    // This mirrors the "medium" benchmark scenario
    for dir_idx in 0..10 {
        let dir_path = temp_dir.join(format!("module_{}", dir_idx));
        fs::create_dir_all(&dir_path).expect("Failed to create directory");

        for file_idx in 0..100 {
            let file_path = dir_path.join(format!("file_{}.rs", file_idx));

            // Create file content with various patterns
            let has_todo = file_idx % 2 == 0; // TODO in 50% of files
            let has_fixme = file_idx % 5 == 0; // FIXME in 20% of files

            let mut content = format!("// File {} in module {}\n", file_idx, dir_idx);
            content.push_str("pub struct MyStruct {\n");
            content.push_str("    field: String,\n");
            content.push_str("}\n\n");

            if has_todo {
                content.push_str("// TODO: implement this\n");
            }

            content.push_str("impl MyStruct {\n");
            content.push_str("    pub fn new() -> Self {\n");
            content.push_str("        Self { field: String::new() }\n");
            content.push_str("    }\n");

            if has_fixme {
                content.push_str("    // FIXME: optimize this\n");
            }

            content.push_str("}\n");

            fs::write(&file_path, content).expect("Failed to write file");
        }
    }

    temp_dir
}

fn count_files(dir: &PathBuf) -> usize {
    walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .count()
}
