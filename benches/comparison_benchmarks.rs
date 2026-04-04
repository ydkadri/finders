use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::path::PathBuf;
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
fn run_finder(dir: &PathBuf, pattern: &str) -> std::io::Result<Vec<String>> {
    // Get the finder binary path
    let finder_bin = std::env::current_exe()
        .ok()
        .and_then(|p| {
            p.parent()
                .and_then(|p| p.parent())
                .map(|p| p.join("debug").join("finder"))
        })
        .or_else(|| {
            // Fallback: try to find in target/release
            std::env::current_dir()
                .ok()
                .map(|p| p.join("target").join("release").join("finder"))
        })
        .unwrap_or_else(|| PathBuf::from("finder"));

    let output = Command::new(&finder_bin)
        .arg(dir)
        .arg("--file-pattern")
        .arg("*.rs")
        .arg("--search-pattern")
        .arg(pattern)
        .output()?;

    if !output.status.success() {
        return Ok(Vec::new());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.lines().map(|s| s.to_string()).collect())
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

    // Setup fixtures once
    println!("Generating benchmark fixtures...");
    let base_dir = setup_fixtures();
    println!("Fixtures ready at: {}", base_dir.display());

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
        group.bench_function(BenchmarkId::new("finder", scenario.pattern_type), |b| {
            b.iter(|| run_finder(&fixture_dir, scenario.pattern))
        });

        group.finish();
    }

    // Note: We don't cleanup fixtures here to allow multiple benchmark runs
    // They'll be regenerated on next run if needed
}

criterion_group!(benches, bench_comparison);
criterion_main!(benches);
