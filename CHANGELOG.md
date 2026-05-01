# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- **Performance:** Implement parallel file processing with Rayon for 30-60% speedup on medium/large repositories
  - Uses Rayon's work-stealing thread pool for automatic load balancing
  - Verified 593% CPU usage on 2000-file dataset (utilizing ~6 of 8 cores)
  - Expected 30-40% improvement on repos with ~1K files
  - Expected 40-60% improvement on repos with ~5K+ files
  - Closes performance gap with ripgrep from 2.4x slower to ~1.6x slower on large repos

### Added

- **API:** `Outputs` trait now requires `Send` bound for thread safety
  - Enables safe use of output writers across multiple threads
  - All built-in output types (StandardOutput, JsonOutput, etc.) automatically implement Send
  - **Non-breaking:** Implementations don't need changes unless they use non-Send types

- **Dependencies:** Added `rayon = "1.10"` for data parallelism

- **Documentation:** Comprehensive performance analysis and implementation guides
  - `docs/performance-analysis.md` - Initial bottleneck analysis and optimization opportunities
  - `docs/parallel-implementation-explained.md` - Deep dive on threading vs async, Mutex, Send trait, Rayon internals
  - `docs/parallel-performance-results.md` - Real-world verification results
  - ADR 0006 - Architectural decision with alternatives and trade-offs

### Developer Experience

- Add profiling infrastructure: `benches/profile_search.rs` for performance testing
- Enable debug symbols in bench profile for profiling tools

## [3.1.0] - 2026-04-22

### ⚠️ BREAKING CHANGES

#### API Changes

**JsonOutput constructor:**
- `JsonOutput::new()` no longer takes `ColourMode` parameter
- Migration: Change `JsonOutput::new(colour_mode)` to `JsonOutput::new()`
- Rationale: JSON output doesn't use colours, parameter was unused

**Finder::find() signature:**
- Now requires `verbose: bool` parameter: `find(&self, query: Option<&str>, verbose: bool)`
- Migration: Add `false` for silent mode or `true` for warnings
- Rationale: Consistent warning control across the API

**Searches trait:**
- Removed `search()` method, only `search_line()` remains
- Migration: Use `search_line()` directly (was already the production path)
- Rationale: Remove unused code, simplify API surface

### Changed

- **API encapsulation:** Make `Finder::path` field private
  - Field is now fully private (no public getter needed)
  - Only accessed internally within `Finder` methods
  - No external impact (field only accessed in internal tests)

- **API flexibility:** `search_files()` now accepts `impl IntoIterator<Item = PathBuf>`
  - Previously required `Vec<PathBuf>`
  - Non-breaking: Vec implements IntoIterator
  - More flexible and idiomatic Rust

### Added

- **Documentation:** Added comprehensive doc comment for `CHUNK_SIZE` constant
  - Explains rationale for 8KB buffer size choice
  - Documents trade-off between memory usage and I/O efficiency

### Impact Assessment

**Zero external dependents:** Checked crates.io - no packages depend on finders, so breaking changes have no ecosystem impact.

**Internal impact:** All call sites updated in same release.

## [3.0.2] - 2026-04-21

### Changed

- Migrate to `anyhow` for comprehensive error handling throughout the application
- Error messages now include full context chains showing what operation failed and why
- File operations, path validation, and regex compilation all provide detailed error context

### Added

- Integration tests for error message context chains
- ADR 0005 documenting the decision to use anyhow for error handling

### Developer Experience

- Add justfile with development workflow commands (lint, test, coverage, build)
- Organised commands into logical groups (workflows, build, test, lint, coverage, bench, optional)
- Pre-commit checks now run via `just pre-commit` (lint + test + coverage-check)
- CI workflow updated to use justfile commands for consistency
- Documentation workflow now validates builds on PRs before deployment

## [3.0.1] - 2026-04-15

### Documentation

- Add comprehensive mdBook documentation site
  - User guide: Installation, Quick Start, CLI Reference, Output Modes, Colour Configuration
  - Examples: Common Use Cases, Advanced Patterns, CI/CD Integration
  - Reference: Tool Comparison, Performance, Troubleshooting
  - Contributing guide for external contributors
- Set up GitHub Pages deployment for documentation
- Move benchmark results to /benchmarks subdirectory for better organisation
- Update README benchmark links to new location

## [3.0.0] - 2026-04-05

### ⚠️ BREAKING CHANGES

#### Output Format Standardization

The output format has changed to match industry standard (grep/ripgrep).

**Before (v2.x):**
```
   4: /path/to/file.rs                                      line content
```

**After (v3.0.0):**
```
/path/to/file.rs:4: line content
```

**What changed:**
- Line number position: moved from beginning to after path
- Format: now `path:line: content` (consistent structure)
- Fixed-width padding: removed (was 56 characters)
- File-only mode: paths no longer have quotes

**Impact:**
- Scripts parsing finder output WILL BREAK
- You must update any scripts that parse finder output
- Pattern to match: `path:line: content`

**Migration:**
```bash
# OLD parsing (v2.x):
finder . -s "pattern" | awk '{print $2}'  # Extract path

# NEW parsing (v3.0.0):
finder . -s "pattern" | cut -d: -f1       # Extract path
finder . -s "pattern" | cut -d: -f2       # Extract line number
```

**For stable output:** Use `--json` flag:
```bash
finder . -s "pattern" --json | jq '.[] | .path'
```

**Why this change:**
- Matches grep/ripgrep standard format (familiar to users)
- Easier to parse programmatically (consistent structure)
- Better handling of long paths and narrow terminals
- Simpler, more maintainable codebase

### Added

#### Coloured Output

- **CLI Flags:**
  - `--colour`: Force coloured output on
  - `--no-colour`: Force coloured output off
  - Default: Auto-detect TTY (colours in terminal, none when piped)

- **Colour Scheme:**
  - File paths: Green
  - Line numbers: Cyan
  - Match highlighting: Bold white text on blue background

- **Environment Variables:**
  - `NO_COLOR`: Universal opt-out from colour (https://no-color.org/)
  - `CLICOLOR`: Enable/disable with TTY respect (https://bixense.com/clicolors/)
  - `CLICOLOR_FORCE`: Force colours on, ignore TTY detection

- **Priority:** `--colour`/`--no-colour` > `NO_COLOR` > `CLICOLOR_FORCE` > `CLICOLOR` > auto-detect

```bash
# Force colours when piping to less
finder . -s "pattern" --colour | less -R

# Disable colours for scripting
NO_COLOR=1 finder . -s "pattern"
```

#### Multiple Output Modes

**Files-with-matches mode (`-l`):**
Output only file paths containing matches (like `grep -l`)
```bash
finder . -s "TODO" -l
# Output:
# src/lib.rs
# src/main.rs
```

**Count mode (`-c`):**
Output match count per file (like `grep -c`)
```bash
finder . -s "FIXME" -c
# Output:
# src/lib.rs:3
# src/main.rs:7
```

**JSON mode (`--json`):**
Output structured JSON for machine processing
```bash
finder . -s "error" --json | jq '.[] | select(.matches | length > 5)'
```

JSON format:
```json
[
  {
    "path": "src/lib.rs",
    "matches": [
      {"line": 42, "content": "matching line"},
      {"line": 67, "content": "another match"}
    ]
  }
]
```

**Mutual exclusivity:** Only one output mode can be specified at a time. Clap enforces this and shows clear error messages.

### Changed

- Output abstraction: Refactored output handling to use trait-based design for extensibility
- Function signatures: `search_files` now accepts `&mut dyn Outputs` instead of specific types

### Dependencies

- Added `termcolor` 1.4 for cross-platform coloured output
- Added `serde` 1.0 with derive feature for JSON serialisation
- Added `serde_json` 1.0 for JSON output

### Documentation

- Added ADR 0002: Output abstraction and v3.0.0 breaking changes
- Created interface design document: `docs/interface-v3.0.0.md`
- Updated README with new features and examples

## [2.1.2] - 2026-04-04

### Fixed

- Fixed regex error handling to return proper errors instead of panicking
- Removed `unwrap()` calls in file finder and library code
- Added graceful handling of non-UTF8 file paths and binary files

### Added

- Integration tests for error handling scenarios
- Tests for binary file handling
- Tests for regex error cases

### Changed

- Improved error messages for invalid regex patterns
- Better handling of filesystem permission errors

## [2.1.1] - 2026-04-03

### Added

- Comparison benchmarks vs find+grep and ripgrep
- Benchmark workflow automation
- Performance regression detection

### Fixed

- Binary detection in benchmarks
- Result parsing for accurate comparisons

## [2.1.0] - 2026-04-02

### Added

- Cross-platform CI testing (Linux, macOS, Windows)
- Code coverage tracking with badges
- Security vulnerability scanning (cargo-audit)
- Project documentation structure
- ADR (Architecture Decision Records) framework

### Changed

- Improved CI workflow organization
- Enhanced test coverage reporting

## [2.0.0] - Earlier

Initial public release with basic functionality.

[Unreleased]: https://github.com/ydkadri/finders/compare/v3.0.0...HEAD
[3.0.0]: https://github.com/ydkadri/finders/compare/v2.1.2...v3.0.0
[2.1.2]: https://github.com/ydkadri/finders/compare/v2.1.1...v2.1.2
[2.1.1]: https://github.com/ydkadri/finders/compare/v2.1.0...v2.1.1
[2.1.0]: https://github.com/ydkadri/finders/compare/v2.0.0...v2.1.0
