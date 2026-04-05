# FindeRS

```
___________.__            .___    __________  _________
\_   _____/|__| ____    __| _/____\______   \/   _____/
 |    __)  |  |/    \  / __ |/ __ \|       _/\_____  \
 |     \   |  |   |  \/ /_/ \  ___/|    |   \/        \
 \___  /   |__|___|  /\____ |\___  >____|_  /_______  /
     \/            \/      \/    \/       \/        \/
```

[![CI](https://github.com/ydkadri/finders/actions/workflows/ci.yml/badge.svg)](https://github.com/ydkadri/finders/actions/workflows/ci.yml)
![Tests](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/ydkadri/5616380737bada94e84764d02b816b38/raw/finders-tests.json)
![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/ydkadri/5616380737bada94e84764d02b816b38/raw/finders-coverage.json)
[![Crates.io](https://img.shields.io/crates/v/finders.svg)](https://crates.io/crates/finders)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**A simpler way to find files and search content from the command line.**

I live in the terminal and I got sick of typing `find . -type f -name "*.py" -exec grep -iH "..." {} \;` every day. This is what I use these days and you should get involved.

---

## 📚 Quick Links

- **[Performance Benchmarks](https://ydkadri.github.io/finders/)** - See how it compares
- **[Contributing Guide](CONTRIBUTING.md)** - Help make it better
- **[Changelog](CHANGELOG.md)** - What's new
- **[Architecture Decisions](docs/adr/)** - Why things work the way they do

---

## Why I Built This

I wanted to learn Rust. I live in the command line. I needed a problem to solve. While I was typing the cumbersome find command above for maybe the 1,000,000th time, I decided I could do better.

This tool is faster and simpler:

```bash
# The old way
find . -type f -name "*.py" -exec grep -iH "TODO" {} \;

# The new way
finder . -f ".py" -s "TODO"
```

**What you get:**
- Simple interface: `-f` finds files, `-s` searches content
- Colored output by default (matches highlighted, easy to scan)
- Multiple output modes (JSON, count-only, files-only)
- Fast enough for daily use (streaming, efficient)
- Single binary, no dependencies

---

## Quick Start

**Find all Python files:**
```bash
finder -f ".py"
```

**Search for "TODO" in all files:**
```bash
finder -s "TODO"
```

**Find TODOs in Python files in a library:**
```bash
finder src/ -f ".py" -s "TODO"
```

**Case-insensitive search:**
```bash
finder -s "error" -i
```

**Get just the file paths (like `grep -l`):**
```bash
finder -s "TODO" -l
```

**Output as JSON for scripting:**
```bash
finder -s "error" --json | jq '.[] | .path'
```

That's it. The output is colored by default when you're in a terminal, and switches to plain text when piped. You can force colors with `--colour` or disable them with `--no-colour`.

---

## Installation

### From Binaries (Recommended)

Download the latest release for your platform from [GitHub Releases](https://github.com/ydkadri/finders/releases):

```bash
wget https://github.com/ydkadri/finders/releases/latest/download/finder-<version>-<arch>.tar.gz
```

**Available architectures:**
- `x86_64-linux` - Linux (x86_64)
- `aarch64-macos` - macOS (Apple Silicon)
- `x86_64-macos` - macOS (Intel)
- `x86_64-windows` - Windows (use `.zip` instead of `.tar.gz`)

**Extract and install:**
```bash
tar -xzf finder-<version>-<arch>.tar.gz  # or unzip for Windows
sudo mv finder /usr/local/bin/           # or add to PATH on Windows
```

**Verify checksum (optional):**
```bash
wget https://github.com/ydkadri/finders/releases/latest/download/finder-<version>-<arch>.tar.gz.sha256
sha256sum -c finder-<version>-<arch>.tar.gz.sha256
```

### From Source (via Cargo)

If you have Rust installed:
```bash
cargo install finders
```

### Verify Installation

```bash
finder --version
```

---

## Usage

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
      --colour                           Enable coloured output (force on)
      --no-colour                        Disable coloured output (force off)
  -l, --files-with-matches               Output only file paths with matches (like grep -l)
  -c, --count                            Output match count per file (like grep -c)
      --json                             Output results as JSON
  -h, --help                             Print help
  -V, --version                          Print version
```

---

## Features

### Colored Output

FindeRS supports colored output for better readability:
- **File paths**: Green
- **Line numbers**: Cyan  
- **Matches**: Bold white on blue background

Colors auto-detect when outputting to a terminal and disable when piped. Control with:
```shell
# Force colors on (useful for piping to less)
finder . -s "pattern" --colour | less -R

# Force colors off
finder . -s "pattern" --no-colour

# Respect NO_COLOR environment variable
NO_COLOR=1 finder . -s "pattern"
```

Respects [NO_COLOR](https://no-color.org/) and [CLICOLORS](https://bixense.com/clicolors/) standards.

### Multiple Output Modes

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

---

## What's New in v3.0.0

**⚠️ Breaking Changes:**
- Output format changed from `   4: /path/file.rs    content` to `/path/file.rs:4: content`
- This matches industry standards (grep/ripgrep) for better tool integration
- **If you have scripts parsing finder output, they'll need updates**

**New Features:**
- ✨ Colored output with match highlighting
- ✨ Multiple output modes (`-l`, `-c`, `--json`)
- ✨ Environment variable support for color control

See [CHANGELOG.md](CHANGELOG.md) for full details and migration guide.

---

## Performance

FindeRS is designed for performance with:
- Streaming file processing using 8KB chunks
- Line-by-line searching to minimize memory usage
- Efficient pattern matching with both simple string search and regex support

**Internal Benchmarks**: Run automatically on every pull request and merge to main. View the latest results in the [Actions tab](https://github.com/ydkadri/finders/actions/workflows/ci.yml).

Key benchmark categories:
- **searcher_search_line**: Tests case-sensitive and case-insensitive string matching
- **regex_searcher_search_line**: Tests regex pattern matching performance
- **searcher_search_content**: Tests multi-line content searching
- **file_finder**: Tests file discovery with and without patterns

**Comparison Benchmarks**: See how `finder` performs against `find+grep` and `ripgrep` at [ydkadri.github.io/finders](https://ydkadri.github.io/finders/)

### Performance Summary (as of 2026-04-05)

| Scenario | Repository Size | finder | find+grep | ripgrep |
|----------|----------------|--------|-----------|---------|
| Common pattern | Small (~100 files) | N/A | N/A | N/A |
| Common pattern | Medium (~1K files) | N/A | N/A | N/A |
| Common pattern | Large (~5K files) | N/A | N/A | N/A |
| Rare pattern | Small (~100 files) | N/A | N/A | N/A |
| Rare pattern | Medium (~1K files) | N/A | N/A | N/A |
| Rare pattern | Large (~5K files) | N/A | N/A | N/A |

Comparison benchmarks test 6 scenarios:
- 3 repository sizes: small (~100 files), medium (~1K files), large (~5K files)
- 2 search patterns: common (found in ~50% of files), rare (found in 1 file)

Benchmarks run automatically on new releases and can be triggered manually via GitHub Actions.

---

## Development

### Running Tests and Benchmarks

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

### Code Quality

To check code quality:
```shell
cargo fmt --all -- --check
cargo clippy -- -D warnings
```

---

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Key points:
- Experiment and iterate - try things, see what works, throw away what doesn't
- Explain **why** changes matter, not just what changed
- Follow Rust best practices and conventions

---

## Release Process

Releases are automated through GitHub Actions:
1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Merge to main - the auto-tag workflow creates a git tag
4. The release workflow automatically:
   - Builds binaries for all platforms
   - Creates a GitHub release
   - Publishes to crates.io

---

## License

MIT License - see [LICENSE](LICENSE) for details.
