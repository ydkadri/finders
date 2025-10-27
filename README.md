# FindeRS

[![CI](https://github.com/octo-youcef/finders/actions/workflows/pr.yml/badge.svg)](https://github.com/octo-youcef/finders/actions/workflows/pr.yml)
[![Benchmarks](https://github.com/octo-youcef/finders/actions/workflows/benchmark.yml/badge.svg)](https://github.com/octo-youcef/finders/actions/workflows/benchmark.yml)
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
  -h, --help                             Print help
  -V, --version                          Print version
```

### Performance

FindeRS is designed for performance with:
- Streaming file processing using 8KB chunks
- Line-by-line searching to minimize memory usage
- Efficient pattern matching with both simple string search and regex support

Benchmarks are run automatically on every pull request and merge to main. View the latest benchmark results in the [Actions tab](https://github.com/octo-youcef/finders/actions/workflows/benchmark.yml).

Key benchmark categories:
- **searcher_search_line**: Tests case-sensitive and case-insensitive string matching
- **regex_searcher_search_line**: Tests regex pattern matching performance
- **searcher_search_content**: Tests multi-line content searching
- **file_finder**: Tests file discovery with and without patterns

### Development

To run benchmarks locally:
```shell
cargo bench
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
