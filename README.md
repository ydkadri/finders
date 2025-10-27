# FindeRS
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
  -h, --help                             Print help
  -V, --version                          Print version
```

### Benchmarking
FindeRS includes comprehensive benchmarks using Criterion. To run the benchmarks:
```shell
cargo bench
```

The benchmarks cover:
- File finding operations (all files and filtered by pattern)
- Search operations (case-sensitive, case-insensitive, and regex)
- Streaming file processing with various file counts and sizes
- Line-by-line search performance

Benchmark reports are generated in `target/criterion/` directory with detailed HTML reports.

### Development

#### Running Tests
```shell
cargo test
```

#### Linting and Formatting
```shell
cargo fmt --all -- --check  # Check formatting
cargo clippy --all-features -- -D warnings  # Run linter
```

#### Continuous Integration
The project uses GitHub Actions for CI/CD:
- **PR Checks**: Runs on every pull request to validate formatting, linting, tests, and benchmarks
- **Release Pipeline**: Manual workflow for creating releases and publishing to crates.io

### Releasing
To create a new release:
1. Go to GitHub Actions → Release workflow
2. Click "Run workflow"
3. Enter the version number (e.g., `2.0.3`)
4. Optionally run a dry-run first to validate
5. The workflow will:
   - Update version in Cargo.toml
   - Create a git tag
   - Build release binary
   - Create GitHub release with release notes
   - Publish to crates.io (requires `CARGO_TOKEN` secret)

### TODO
 - [ ] release pipeline
 - [ ] validate lockfile before merge (build will update this but causes publish issues)

### References
 - [Semantic Versioning][sem_ver]


[finders_crate]: https://crates.io/crates/finders
[sem_ver]: https://doc.rust-lang.org/cargo/reference/semver.html
