# FindeRS

[![CI](https://github.com/ydkadri/finders/actions/workflows/pr.yml/badge.svg)](https://github.com/ydkadri/finders/actions/workflows/pr.yml)
![Tests](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/ydkadri/5616380737bada94e84764d02b816b38/raw/finders-tests.json)
![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/ydkadri/5616380737bada94e84764d02b816b38/raw/finders-coverage.json)
[![Benchmarks](https://github.com/ydkadri/finders/actions/workflows/benchmark.yml/badge.svg)](https://github.com/ydkadri/finders/actions/workflows/benchmark.yml)
[![Crates.io](https://img.shields.io/crates/v/finders.svg)](https://crates.io/crates/finders)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A tool to replace the complex bash `find` logic which searches for files (optionally) containing some string or regular expression pattern.

### The challenge
Finding files containing some string is a common use case in the shell, however the command is cumbersome:
```shell
# Bash command
find <dir> \
    -type f \
    -name <file pattern> \
    -exec grep -iH <search pattern> {} \;
```

Instead, `finders` provides a lightweight wrapper for this common command.

### Installation
FindeRS can be found on [crates.io][finders_crate] and as such can be installed with
```shell
cargo install finders
```

### Usage
```shell
Usage: finder [OPTIONS] [PATH]

Arguments:
  [PATH]  Optional path to operate on, defaults to CWD

Options:
  -f, --file-pattern <FILE_PATTERN>      File pattern to filter results
  -s, --search-pattern <SEARCH_PATTERN>  Search pattern to match in result files
  -r, --regex-pattern <REGEX_PATTERN>    Regex pattern to match in result files
  -i, --case-insensitive                 Flag for case insensitive search
  -v, --verbose                          Verbose output details unreadable files
  -l, --files-with-matches               Output only file paths with matches (like grep -l)
  -c, --count                            Output match count per file (like grep -c)
      --json                             Output results as JSON
      --colour                           Enable coloured output (force on)
      --no-colour                        Disable coloured output (force off)
  -h, --help                             Print help
  -V, --version                          Print version
```

### Features

#### Coloured Output
FindeRS supports coloured output for better readability:
- **File paths**: Green
- **Line numbers**: Cyan  
- **Matches**: Bold white on blue background

Colours auto-detect when outputting to a terminal and disable when piped. Control with:
```shell
# Force colours on (useful for piping to less)
finder . -s "pattern" --colour | less -R

# Force colours off
finder . -s "pattern" --no-colour

# Respect NO_COLOR environment variable
NO_COLOR=1 finder . -s "pattern"
```

Respects [NO_COLOR](https://no-color.org/) and [CLICOLORS](https://bixense.com/clicolors/) standards.

#### Multiple Output Modes

**Standard output (default):**
```shell
finder . -s "TODO"
# Output: path:line: content
# src/lib.rs:42: // TODO: implement this
```

**Files-only mode (`-l`):**
```shell
finder . -s "TODO" -l
# Output: file paths only
# src/lib.rs
# src/main.rs
```

**Count mode (`-c`):**
```shell
finder . -s "FIXME" -c
# Output: path:count
# src/lib.rs:3
# src/main.rs:7
```

**JSON mode (`--json`):**
```shell
finder . -s "error" --json | jq
# Output: structured JSON
[
  {
    "path": "src/lib.rs",
    "matches": [
      {"line": 42, "content": "error handling"}
    ]
  }
]
```

### Performance

FindeRS is designed for performance with:
- Streaming file processing using 8KB chunks
- Line-by-line searching to minimize memory usage
- Efficient pattern matching with both simple string search and regex support

**Internal Benchmarks**: Run automatically on every pull request and merge to main. View the latest results in the [Actions tab](https://github.com/ydkadri/finders/actions/workflows/benchmark.yml).

Key benchmark categories:
- **searcher_search_line**: Tests case-sensitive and case-insensitive string matching
- **regex_searcher_search_line**: Tests regex pattern matching performance
- **searcher_search_content**: Tests multi-line content searching
- **file_finder**: Tests file discovery with and without patterns

**Comparison Benchmarks**: See how `finder` performs against `find+grep` and `ripgrep` at [ydkadri.github.io/finders](https://ydkadri.github.io/finders/)

Comparison benchmarks test 6 scenarios:
- 3 repository sizes: small (~100 files), medium (~1K files), large (~5K files)
- 2 search patterns: common (found in ~50% of files), rare (found in 1 file)

Benchmarks run automatically on new releases and can be triggered manually via GitHub Actions.

### Development

To run internal benchmarks locally:
```shell
cargo bench
```

To run comparison benchmarks (requires `ripgrep` installed):
```shell
cargo build --release  # Build finder first
cargo bench --bench comparison_benchmarks
```

To run tests:
```shell
cargo test
```

To check code quality:
```shell
cargo fmt --all -- --check
cargo clippy -- -D warnings
```

### Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Release Process

Releases are automated through GitHub Actions:
1. Create and push a new tag with semantic versioning: `git tag v2.0.3 && git push origin v2.0.3`
2. The release workflow will automatically:
   - Run all tests
   - Build the release binary
   - Create a GitHub release with release notes
   - Publish the new version to crates.io

### References
 - [Semantic Versioning][sem_ver]


[finders_crate]: https://crates.io/crates/finders
[sem_ver]: https://doc.rust-lang.org/cargo/reference/semver.html
