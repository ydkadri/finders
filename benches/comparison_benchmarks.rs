use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::path::{Path, PathBuf};
use std::process::Command;

mod fixtures;

use fixtures::generator::{self, FixtureConfig, RARE_PATTERN};

/// Check if a command-line tool is available
fn is_tool_available(tool: &str) -> bool {
    Command::new("which")
        .arg(tool)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Find the finder binary for benchmarking
fn find_finder_binary() -> Option<PathBuf> {
    // Try multiple locations in order of preference
    let candidates = vec![
        // 1. Release build (preferred for benchmarks)
        std::env::current_dir()
            .ok()
            .map(|p| p.join("target").join("release").join("finder")),
        // 2. Debug build (fallback)
        std::env::current_dir()
            .ok()
            .map(|p| p.join("target").join("debug").join("finder")),
        // 3. Relative to benchmark executable
        std::env::current_exe().ok().and_then(|p| {
            p.parent()
                .and_then(|p| p.parent())
                .map(|p| p.join("release").join("finder"))
        }),
        // 4. In PATH
        Some(PathBuf::from("finder")),
    ];

    // Find first existing binary
    for candidate in candidates.into_iter().flatten() {
        if candidate.exists() {
            return Some(candidate);
        }
    }

    // Check if finder is in PATH
    if is_tool_available("finder") {
        return Some(PathBuf::from("finder"));
    }

    None
}

/// Validate that a binary exists and is executable
fn validate_binary(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("Binary not found at: {}", path.display()));
    }

    // Try to get version to verify it's executable
    let output = Command::new(path)
        .arg("--version")
        .output()
        .map_err(|e| format!("Failed to execute {}: {}", path.display(), e))?;

    if !output.status.success() {
        return Err(format!(
            "Binary at {} is not executable or returned error",
            path.display()
        ));
    }

    Ok(())
}

/// Run find + grep command
fn run_find_grep(dir: &PathBuf, pattern: &str) -> std::io::Result<Vec<String>> {
    let output = Command::new("find")
        .arg(dir)
        .arg("-type")
        .arg("f")
        .arg("-name")
        .arg("*.rs")
        .arg("-exec")
        .arg("grep")
        .arg("-l")
        .arg(pattern)
        .arg("{}")
        .arg(";")
        .output()?;

    if !output.status.success() {
        return Ok(Vec::new());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.lines().map(|s| s.to_string()).collect())
}

/// Run ripgrep command
fn run_ripgrep(dir: &PathBuf, pattern: &str) -> std::io::Result<Vec<String>> {
    let output = Command::new("rg")
        .arg("-l")
        .arg("--glob")
        .arg("*.rs")
        .arg(pattern)
        .arg(dir)
        .output()?;

    if !output.status.success() {
        return Ok(Vec::new());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.lines().map(|s| s.to_string()).collect())
}

/// Run finder command
fn run_finder(dir: &PathBuf, pattern: &str, finder_bin: &Path) -> std::io::Result<Vec<String>> {
    let output = Command::new(finder_bin)
        .arg(dir)
        .arg("--file-pattern")
        .arg(".rs") // finder uses contains(), not globs, so ".rs" matches "*.rs"
        .arg("--search-pattern")
        .arg(pattern)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!(
            "⚠️  finder command failed:\n  Binary: {}\n  Stderr: {}",
            finder_bin.display(),
            stderr
        );
        return Ok(Vec::new());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    // finder outputs: "   4: /path/to/file.rs content..."
    // Format: <line_num>: <path> <content>
    // Extract unique file paths (all paths end with .rs in this benchmark)
    let mut files = std::collections::HashSet::new();
    for line in stdout.lines() {
        // Find first colon (after line number)
        if let Some(colon_pos) = line.find(": ") {
            let after_colon = &line[colon_pos + 2..];
            // Path ends with .rs - find the last occurrence
            if let Some(rs_pos) = after_colon.rfind(".rs") {
                // Include ".rs" in the path
                let file_path = &after_colon[..rs_pos + 3];
                files.insert(file_path.trim().to_string());
            }
        }
    }

    Ok(files.into_iter().collect())
}

/// Benchmark configuration for a test scenario
struct BenchmarkScenario {
    name: &'static str,
    config: FixtureConfig,
    pattern: &'static str,
    pattern_type: &'static str,
}

impl BenchmarkScenario {
    const SCENARIOS: &'static [Self] = &[
        Self {
            name: "small_common",
            config: FixtureConfig::SMALL,
            pattern: "function",
            pattern_type: "common",
        },
        Self {
            name: "small_rare",
            config: FixtureConfig::SMALL,
            pattern: RARE_PATTERN,
            pattern_type: "rare",
        },
        Self {
            name: "medium_common",
            config: FixtureConfig::MEDIUM,
            pattern: "function",
            pattern_type: "common",
        },
        Self {
            name: "medium_rare",
            config: FixtureConfig::MEDIUM,
            pattern: RARE_PATTERN,
            pattern_type: "rare",
        },
        Self {
            name: "large_common",
            config: FixtureConfig::LARGE,
            pattern: "function",
            pattern_type: "common",
        },
        Self {
            name: "large_rare",
            config: FixtureConfig::LARGE,
            pattern: RARE_PATTERN,
            pattern_type: "rare",
        },
    ];
}

fn setup_fixtures() -> PathBuf {
    let base_dir = std::env::temp_dir().join("finders_comparison_bench");

    // Generate all fixture sizes
    generator::generate_fixtures(&base_dir, &FixtureConfig::SMALL)
        .expect("Failed to generate small fixtures");
    generator::generate_fixtures(&base_dir, &FixtureConfig::MEDIUM)
        .expect("Failed to generate medium fixtures");
    generator::generate_fixtures(&base_dir, &FixtureConfig::LARGE)
        .expect("Failed to generate large fixtures");

    base_dir
}

fn bench_comparison(c: &mut Criterion) {
    // Check tool availability
    let has_rg = is_tool_available("rg");

    if !has_rg {
        eprintln!("\n⚠️  Warning: ripgrep (rg) not found in PATH");
        eprintln!("   Install with: cargo install ripgrep");
        eprintln!("   Skipping ripgrep benchmarks...\n");
    }

    // Find and validate finder binary
    let finder_bin = match find_finder_binary() {
        Some(path) => {
            println!("✓ Found finder binary at: {}", path.display());
            if let Err(e) = validate_binary(&path) {
                panic!("❌ Finder binary validation failed: {}\n   Build finder first with: cargo build --release", e);
            }
            path
        }
        None => {
            panic!(
                "❌ Could not find finder binary!\n   Build it first with: cargo build --release"
            );
        }
    };

    // Setup fixtures once
    println!("Generating benchmark fixtures...");
    let base_dir = setup_fixtures();
    println!("Fixtures ready at: {}", base_dir.display());

    // Sanity check: verify all tools produce similar results on small dataset
    println!("\nValidating tools produce similar results...");
    let small_dir = base_dir.join("small");
    let test_pattern = "function";

    let find_grep_results = run_find_grep(&small_dir, test_pattern)
        .expect("find+grep sanity check failed");
    let finder_results = run_finder(&small_dir, test_pattern, &finder_bin)
        .expect("finder sanity check failed");

    println!("  find+grep found: {} files", find_grep_results.len());
    println!("  finder found:    {} files", finder_results.len());

    if has_rg {
        let rg_results = run_ripgrep(&small_dir, test_pattern)
            .expect("ripgrep sanity check failed");
        println!("  ripgrep found:   {} files", rg_results.len());

        // Verify all tools found similar number of files (within 10%)
        let max_count = find_grep_results.len().max(finder_results.len()).max(rg_results.len());
        let min_count = find_grep_results.len().min(finder_results.len()).min(rg_results.len());

        if max_count > 0 && (max_count - min_count) as f64 / max_count as f64 > 0.1 {
            panic!(
                "❌ Tools found significantly different results!\n\
                 This suggests a bug in one of the implementations.\n\
                 find+grep: {}, ripgrep: {}, finder: {}",
                find_grep_results.len(),
                rg_results.len(),
                finder_results.len()
            );
        }
    } else {
        // Without ripgrep, just verify finder and find+grep match
        let max_count = find_grep_results.len().max(finder_results.len());
        let min_count = find_grep_results.len().min(finder_results.len());

        if max_count > 0 && (max_count - min_count) as f64 / max_count as f64 > 0.1 {
            panic!(
                "❌ finder and find+grep found significantly different results!\n\
                 This suggests a bug in the implementation.\n\
                 find+grep: {}, finder: {}",
                find_grep_results.len(),
                finder_results.len()
            );
        }
    }

    println!("✓ All tools validated - results match within tolerance\n");

    // Run benchmarks for each scenario
    for scenario in BenchmarkScenario::SCENARIOS {
        let fixture_dir = base_dir.join(scenario.config.name);
        let mut group = c.benchmark_group(scenario.name);

        // Configure benchmark settings for faster CI runs
        group.sample_size(50);

        // Benchmark find + grep
        group.bench_function(BenchmarkId::new("find_grep", scenario.pattern_type), |b| {
            b.iter(|| run_find_grep(&fixture_dir, scenario.pattern))
        });

        // Benchmark ripgrep (if available)
        if has_rg {
            group.bench_function(BenchmarkId::new("ripgrep", scenario.pattern_type), |b| {
                b.iter(|| run_ripgrep(&fixture_dir, scenario.pattern))
            });
        }

        // Benchmark finder
        let finder_bin_clone = finder_bin.clone();
        group.bench_function(BenchmarkId::new("finder", scenario.pattern_type), |b| {
            b.iter(|| run_finder(&fixture_dir, scenario.pattern, &finder_bin_clone))
        });

        group.finish();
    }

    // Note: We don't cleanup fixtures here to allow multiple benchmark runs
    // They'll be regenerated on next run if needed
}

criterion_group!(benches, bench_comparison);
criterion_main!(benches);
