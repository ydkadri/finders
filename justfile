# FindeRS Justfile
# Commands for building, testing, and linting
#
# Prerequisites:
#   - cargo-llvm-cov: Install with 'cargo install cargo-llvm-cov' (for coverage commands)
#   - cargo-watch: Install with 'cargo install cargo-watch' (for watch commands)
#   - cargo-audit: Install with 'cargo install cargo-audit' (for audit command)

# Show available commands
default:
    @just --list

# Check if required tools are installed
[group('workflows')]
check-tools:
    #!/usr/bin/env bash
    echo "Checking for optional tools..."
    command -v cargo-llvm-cov >/dev/null 2>&1 && echo "✅ cargo-llvm-cov" || echo "❌ cargo-llvm-cov (install: cargo install cargo-llvm-cov)"
    command -v cargo-watch >/dev/null 2>&1 && echo "✅ cargo-watch" || echo "❌ cargo-watch (install: cargo install cargo-watch)"
    command -v cargo-audit >/dev/null 2>&1 && echo "✅ cargo-audit" || echo "❌ cargo-audit (install: cargo install cargo-audit)"

# Quick development check (fast feedback loop)
[group('workflows')]
dev: fmt clippy (test-lib "1")
    @echo "✅ Quick checks passed!"

# Run all pre-commit checks (lint + test + coverage-check)
[group('workflows')]
pre-commit: lint (test "1") coverage-check
    @echo "✅ All pre-commit checks passed!"

# Run all CI checks locally
[group('workflows')]
ci: check lint (test "1") coverage-check audit bench
    @echo "✅ All CI checks passed!"

# Check if code compiles
[group('build')]
check:
    cargo check --all-targets

# Build debug binary
[group('build')]
build:
    cargo build

# Build release binary
[group('build')]
build-release:
    cargo build --release


# Clean build artifacts
[group('build')]
clean:
    cargo clean

# Run all tests (default: serialized for colour tests, override with THREADS=N)
[group('test')]
test THREADS="1":
    cargo test --all-features -- --test-threads={{THREADS}}

# Run only library tests (default: serialized, override with THREADS=N)
[group('test')]
test-lib THREADS="1":
    cargo test --lib -- --test-threads={{THREADS}}

# Run only integration tests (default: serialized, override with THREADS=N)
[group('test')]
test-integration THREADS="1":
    cargo test --test '*' -- --test-threads={{THREADS}}

# Run only benchmarks tests (not actual benchmarking)
[group('test')]
test-bench:
    cargo test --benches

# Watch mode - run tests on file changes (requires cargo-watch)
[group('test')]
watch THREADS="1":
    cargo watch -x 'test --all-features -- --test-threads={{THREADS}}'

# Watch mode for specific test
[group('test')]
watch-test TEST THREADS="1":
    cargo watch -x 'test {{TEST}} -- --test-threads={{THREADS}}'

# Format code
[group('lint')]
fmt:
    cargo fmt --all

# Check code formatting without modifying files
[group('lint')]
fmt-check:
    cargo fmt --all -- --check

# Run clippy lints
[group('lint')]
clippy:
    cargo clippy --all-targets --all-features -- -D warnings

# Run all lints (fmt + clippy)
[group('lint')]
lint: fmt-check clippy

# Generate code coverage report
[group('coverage')]
coverage:
    cargo llvm-cov --all-features --workspace --html -- --test-threads=1
    @echo "Coverage report generated at: target/llvm-cov/html/index.html"

# Generate JSON coverage report and check threshold
[group('coverage')]
coverage-check:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo llvm-cov --all-features --workspace --json --output-path coverage.json -- --test-threads=1
    COVERAGE=$(jq -r '.data[0].totals.lines.percent' coverage.json)
    echo "Coverage: ${COVERAGE}%"
    if (( $(echo "${COVERAGE} < 70" | bc -l) )); then
        echo "❌ Coverage ${COVERAGE}% is below 70% threshold"
        exit 1
    fi
    echo "✅ Coverage ${COVERAGE}% meets 70% threshold"

# Run benchmarks (internal search benchmarks)
[group('bench')]
bench:
    cargo bench --bench search_benchmarks

# Run all benchmarks
[group('bench')]
bench-all:
    cargo bench

# Run security audit on dependencies
[group('optional')]
audit:
    cargo audit

# Generate rustdoc documentation
[group('optional')]
docs:
    cargo doc --no-deps --open

# Build and install locally
[group('optional')]
install:
    cargo install --path .

# Run comparison benchmarks (requires ripgrep installed)
[group('optional')]
bench-compare:
    cargo bench --bench comparison_benchmarks
