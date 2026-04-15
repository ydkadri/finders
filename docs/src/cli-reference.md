# CLI Reference

Complete reference for all FindeRS command-line options.

## Synopsis

```
finder [OPTIONS] [PATH]
```

## Arguments

### `[PATH]`

Optional path to operate on. Defaults to the current working directory.

**Examples:**
```bash
finder              # Search in current directory
finder src/         # Search in src/ directory
finder ../project/  # Search in ../project/ directory
```

## Options

### File Filtering

#### `-f, --file-pattern <PATTERN>`

Filter files by pattern in filename.

**Examples:**
```bash
finder -f ".rs"     # All Rust files
finder -f "test"    # Files containing "test" in name
finder -f ".config" # All config files
```

### Content Searching

#### `-s, --search-pattern <PATTERN>`

Search for a literal string pattern in file contents.

**Examples:**
```bash
finder -s "TODO"            # Find all TODOs
finder -s "function main"   # Find main functions
finder -s "api_key"         # Find API key references
```

#### `-r, --regex-pattern <PATTERN>`

Search using a regular expression pattern.

**Examples:**
```bash
finder -r "TODO|FIXME"          # Find TODOs or FIXMEs
finder -r "fn \w+\("            # Find Rust functions
finder -r "[0-9]{3}-[0-9]{4}"   # Find phone numbers
```

#### `-i, --case-insensitive`

Make search case-insensitive.

**Examples:**
```bash
finder -s "error" -i     # Matches "Error", "ERROR", "error"
finder -r "todo" -i      # Case-insensitive regex search
```

### Output Control

#### `-l, --files-with-matches`

Output only file paths that contain matches (like `grep -l`).

**Example:**
```bash
finder -s "TODO" -l
```

Output:
```
src/lib.rs
src/main.rs
```

#### `-c, --count`

Output match count per file (like `grep -c`).

**Example:**
```bash
finder -s "error" -c
```

Output:
```
src/lib.rs:5
src/main.rs:2
```

#### `--json`

Output results in JSON format for programmatic processing.

**Example:**
```bash
finder -s "error" --json
```

Output:
```json
[
  {
    "path": "src/lib.rs",
    "matches": [
      {"line": 42, "content": "handle error"}
    ]
  }
]
```

### Color Control

#### `--colour`

Force colored output on (useful when piping to `less -R`).

**Example:**
```bash
finder -s "pattern" --colour | less -R
```

#### `--no-colour`

Force colored output off.

**Example:**
```bash
finder -s "pattern" --no-colour > results.txt
```

**Note:** Colors auto-detect by default - on for terminals, off for pipes.

### Verbosity

#### `-v, --verbose`

Show verbose output including files that couldn't be read.

**Example:**
```bash
finder -s "pattern" -v
```

### Information

#### `-h, --help`

Print help information and exit.

```bash
finder --help
```

#### `-V, --version`

Print version information and exit.

```bash
finder --version
```

## Examples

### Basic Usage

```bash
# Find Python files
finder -f ".py"

# Search for TODO in all files
finder -s "TODO"

# Find TODOs in Python files
finder -f ".py" -s "TODO"
```

### Advanced Usage

```bash
# Case-insensitive regex search in TypeScript files
finder -f ".ts" -r "interface \w+" -i

# Get JSON output for processing
finder -s "error" --json | jq '.[] | .path'

# Count all TODOs across the project
finder -s "TODO" -c | awk -F: '{sum+=$2} END {print sum}'

# Find files and open in vim
vim $(finder -f "config" -l)
```

### Working with Directories

```bash
# Search in specific directory
finder src/ -s "TODO"

# Search in multiple patterns
finder -f ".rs" -s "TODO" && finder -f ".md" -s "TODO"

# Search specific directory with file type
finder src/ -f ".rs" -s "TODO"
```

## Environment Variables

FindeRS respects standard color environment variables:

- `NO_COLOR` - Disable colors entirely
- `CLICOLOR` - Enable/disable color support
- `CLICOLOR_FORCE` - Force colors on

See [Color Configuration](./color-config.md) for details.

## Exit Codes

- `0` - Success, matches found
- `1` - Error occurred
- `0` - No matches found (not an error)

## See Also

- [Quick Start](./quick-start.md) - Getting started guide
- [Output Modes](./output-modes.md) - Detailed output format examples
- [Color Configuration](./color-config.md) - Color customization
- [Examples](./examples/common-use-cases.md) - Real-world usage patterns
