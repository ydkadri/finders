# Justfile Guide

This project uses [just](https://github.com/casey/just) as a command runner for common development tasks. This ensures consistency between local development and CI workflows.

## Installation

### Install just

**macOS:**
```bash
brew install just
```

**Linux:**
```bash
# Via cargo
cargo install just

# Or via package manager (Ubuntu/Debian)
curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to /usr/local/bin
```

**Windows:**
```powershell
# Via cargo
cargo install just

# Or via scoop
scoop install just
```

### Optional Tools

Some commands require additional tools. Check what you have:

```bash
just check-tools
```

**Install missing tools:**
```bash
# For coverage commands (coverage, coverage-check)
cargo install cargo-llvm-cov

# For watch mode (watch, watch-test)
cargo install cargo-watch

# For security audit (audit)
cargo install cargo-audit
```

**Note:** CI installs these automatically, so they're only needed for local development.

## Available Commands

Run `just` or `just --list` to see all available commands.

### Quick Development Workflow

```bash
# Fast feedback loop - format, lint, and test library
just dev

# Format code before committing
just fmt

# Run all lints (format check + clippy)
just lint

# Run all tests (with serialized execution for colour tests)
just test
```

### Testing

```bash
# Run all tests (default: serialized execution for colour tests)
just test

# Run tests with parallel execution (faster, but may cause colour test failures)
just test 4

# Run only library tests
just test-lib

# Run library tests with 4 threads
just test-lib 4

# Run only integration tests
just test-integration
```

**Note:** Tests default to `--test-threads=1` (serialized) because colour output tests have a race condition. For faster tests during development, you can override with `just test 4` or similar.

**Watch mode** (requires `cargo-watch`):

Watch mode automatically re-runs tests when you save files, providing instant feedback during development.

```bash
# Watch all tests
just watch

# Watch with parallel execution (faster feedback)
just watch 4

# Watch specific test (focused development)
just watch-test test_regex_pattern

# Watch specific test with parallel execution
just watch-test searcher 4
```

**When to use watch mode:**
- 🎯 Working on a specific feature - use `just watch-test feature_name`
- 🔄 Making incremental changes - immediate feedback without manual test runs
- 🐛 Debugging failing tests - see results instantly as you fix
- 💡 Test-driven development - write test, watch it fail, implement, watch it pass

### Code Quality

```bash
# Check if code compiles
just check

# Format code
just fmt

# Check formatting without modifying files
just fmt-check

# Run clippy lints
just clippy

# Run all lints (fmt-check + clippy)
just lint
```

### Coverage

```bash
# Generate HTML coverage report
just coverage
# Opens in browser at target/llvm-cov/html/index.html

# Generate JSON coverage and check 70% threshold (used in CI)
just coverage-check
```

### Benchmarks

```bash
# Run internal search benchmarks
just bench

# Run comparison benchmarks (requires ripgrep)
just bench-compare

# Run all benchmarks
just bench-all
```

### Building

```bash
# Build debug binary
just build

# Build release binary
just build-release

# Build and install locally
just install
```

### Pre-Commit & CI

```bash
# Run all pre-commit checks (lint + test + coverage-check)
just pre-commit

# Run all CI checks locally (check + lint + test + coverage + audit + bench)
just ci
```

### Optional Commands

These commands are grouped under `[optional]` as they're not needed for typical development:

```bash
# Security audit on dependencies (CI does this automatically)
just audit

# Generate rustdoc documentation (useful for library API work)
just docs

# Build and install locally (most people use 'cargo run' instead)
just install

# Run comparison benchmarks (requires ripgrep installed)
just bench-compare
```

### Utilities

```bash
# Clean build artifacts
just clean
```

## CI Integration

The CI workflows (`.github/workflows/ci.yml`) use just commands to ensure local and CI environments are consistent:

- **Check job**: `just check`
- **Test job**: `just test`
- **Lints job**: `just lint`
- **Coverage job**: `just coverage-check`
- **Benchmarks job**: `just bench`
- **Security job**: `just audit`

This means you can run the exact same commands locally that CI runs, making it easier to debug CI failures.

## Tips

- **Before pushing**: Run `just pre-commit` to catch issues early
- **Quick iteration**: Use `just dev` for fast feedback during development
- **Watch mode**: Use `just watch` to automatically run tests as you code (requires `cargo install cargo-watch`)
- **Local CI**: Run `just ci` to simulate all CI checks before creating a PR

## Customization

The justfile is located at the repository root. You can add project-specific commands or modify existing ones as needed. See [just documentation](https://github.com/casey/just) for syntax and features.
