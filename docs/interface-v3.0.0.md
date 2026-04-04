# Interface Design: v3.0.0 UX Enhancements

**Features:** #17 (Colored Output) + #22 (Multiple Output Modes)  
**Version:** 3.0.0 (MAJOR - Breaking Changes)  
**Status:** Design Document

---

## Overview

Adds coloured output and multiple output modes to improve user experience and enable better tooling integration.

---

## CLI Interface

### New Flags

#### Color Control
```bash
--colour          Enable coloured output (force on)
--no-colour       Disable coloured output (force off)
```

**Default behavior:** Auto-detect TTY
- Colors enabled when outputting to terminal
- Colors disabled when piped or redirected
- Respects `NO_COLOR` environment variable (disables if set)
- Respects `CLICOLOR` environment variable (enable/disable with TTY respect)
- Respects `CLICOLOR_FORCE` environment variable (force enable, ignore TTY)

**Priority:** `--colour`/`--no-colour` flag > `NO_COLOR` > `CLICOLOR_FORCE` > `CLICOLOR` > auto-detect

**Notes:**
- UK spelling throughout CLI
- No short flag (avoid confusion with `-c` for count)

#### Output Modes
```bash
-l, --files-with-matches    Output only file paths with matches
-c, --count                 Output match count per file (format: path:count)
--json                      Output structured JSON
```

**Mutual exclusivity:** Only one output mode can be specified at a time.

**Error behavior:** Exit with error if multiple output mode flags given.

---

## Output Formats

### Default Output (with search pattern)
```
src/searcher.rs:42: This is a line with matching text in it
src/lib.rs:67: Another line with a match here
```

**Format:** `path:line: content`

**With colours:**
- **Path**: Green (`\033[32m`)
- **Line number**: Cyan (`\033[36m`)
- **Separator** (`:`): Default (no color)
- **Matches**: White text on Magenta background (`\033[37m\033[45m`)
- **Reset**: After each colored segment (`\033[0m`)

**Example colored:**
```
[GREEN]src/searcher.rs[RESET]:[CYAN]42[RESET]: This is a line with [WHITE][BG_MAGENTA]matching text[RESET] in it
```

### Files-Only Output (no search pattern)
```
src/searcher.rs
src/lib.rs
tests/integration.rs
```

**Format:** `path` (one per line, no quotes)

**When triggered:**
- No search pattern given: `finder .`
- Search pattern with `-l` flag: `finder . -s "text" -l`

**Note:** Current behavior preserved - no breaking changes.

### Files-With-Matches Output (`-l`)
```
src/searcher.rs
src/lib.rs
```

**Format:** `path` (one per line)

**Behavior:** Only files that contain matches (like grep -l)

**With colours:** Paths in green (if colors enabled)

### Count Output (`-c`)
```
src/searcher.rs:3
src/lib.rs:7
tests/integration.rs:1
```

**Format:** `path:count`

**With colours:** 
- Path in green
- Separator in default
- Count in default

### JSON Output (`--json`)
```json
[
  {
    "path": "src/searcher.rs",
    "matches": [
      {"line": 42, "content": "This is a line with matching text"},
      {"line": 67, "content": "Another matching line"}
    ]
  },
  {
    "path": "src/lib.rs",
    "matches": [
      {"line": 15, "content": "A match here"}
    ]
  }
]
```

**Format:** Array of objects, one per file with matches

**Fields:**
- `path` (string): File path
- `matches` (array): Array of match objects
  - `line` (number): Line number (1-indexed)
  - `content` (string): Full line content

**Notes:**
- No colors in JSON output (always machine-readable)
- Valid JSON array output
- Empty array `[]` if no matches found

---

## Backwards Compatibility

### Preserved Behaviors
1. **No search pattern** → List all files matching file pattern
   ```bash
   finder . --file-pattern ".rs"
   # Still outputs file paths, no change
   ```

2. **Search pattern output** → Format changes from custom alignment to standard
   ```bash
   # OLD: "   4: /path/to/file.rs                                      line content"
   # NEW: "/path/to/file.rs:4: line content"
   ```
   **Impact:** Breaking change to output format, but more standard (matches grep/ripgrep)

3. **Piped output** → No colors by default (already behavior, now explicit)
   ```bash
   finder . -s "text" | less
   # No colors unless --colour specified
   ```

### Breaking Changes

⚠️ **This is a MAJOR release (v3.0.0) due to breaking changes in output format.**

#### Output Format Change (BREAKING)

**What changed:**
```bash
# v2.x.x output:
   4: /path/to/file.rs                                      line content

# v3.0.0 output:
/path/to/file.rs:4: line content
```

**Impact:**
- Scripts parsing finder output WILL BREAK
- Line number moved from beginning to after filename
- Fixed-width padding removed
- Path format changed (no quotes in file-only mode)

**Why this change:**
- Matches grep/ripgrep standard format (industry convention)
- Easier to parse programmatically (consistent `path:line:content` structure)
- Better handling of long paths and narrow terminals
- More predictable output for scripting

**Migration:**
- Update scripts to parse new `path:line: content` format
- Use `--json` mode for stable, machine-readable output
- Test scripts with v3.0.0 before deploying to production

---

## Environment Variables

Finder respects community standards for terminal color control.

### NO_COLOR
- **Type:** Presence check (any value disables colors)
- **Effect:** Disables coloured output when set
- **Standard:** [no-color.org](https://no-color.org/) - Universal opt-out from color
- **Example:** `NO_COLOR=1 finder . -s "text"`

### CLICOLOR
- **Type:** Value check (zero disables, non-zero enables)
- **Effect:** 
  - `CLICOLOR=0` - Disable colors (even to TTY)
  - `CLICOLOR=1` (or non-zero) - Enable colors IF outputting to terminal (respects TTY detection)
- **Standard:** [CLICOLORS](https://bixense.com/clicolors/) - BSD/macOS color control convention
- **Example:** 
  ```bash
  CLICOLOR=1 finder . -s "text"           # Colors ON (if TTY)
  CLICOLOR=1 finder . -s "text" | cat     # Colors OFF (not TTY)
  CLICOLOR=0 finder . -s "text"           # Colors OFF (forced)
  ```

### CLICOLOR_FORCE
- **Type:** Value check (non-zero enables colors)
- **Effect:** Forces coloured output when set to non-zero, EVEN when not outputting to terminal
- **Standard:** [CLICOLORS](https://bixense.com/clicolors/) - BSD/macOS color control convention
- **Example:** `CLICOLOR_FORCE=1 finder . -s "text" | less -R`
- **Use case:** Piping coloured output to pagers that support ANSI codes

**References:**
- **NO_COLOR specification:** https://no-color.org/
- **CLICOLORS specification:** https://bixense.com/clicolors/

**Priority (highest to lowest):**
1. CLI flags (`--colour` or `--no-colour`) - Explicit user choice
2. `NO_COLOR` - Universal opt-out (overrides everything else)
3. `CLICOLOR_FORCE` - Force colors on, ignore TTY detection
4. `CLICOLOR` - Enable/disable with TTY respect
5. Auto-detect TTY - Default behavior if no flags/env vars set

---

## Error Handling

### Multiple Output Modes
```bash
$ finder . -s "text" --json --count
Error: Cannot specify multiple output modes (--json, --count)
```

**Exit code:** 1

### Invalid Flag Combinations
```bash
$ finder . --files-with-matches
Error: --files-with-matches requires a search pattern
```

**Exit code:** 1

---

## Usage Examples

### Basic colored search
```bash
finder . -s "function"
# Outputs with colors (auto-detected TTY)
```

### Disable colors for scripting
```bash
finder . -s "TODO" --no-colour > todos.txt
# Plain text output
```

### Get files to process
```bash
for file in $(finder . -s "deprecated" -l); do
  echo "Processing $file"
done
```

### Count matches
```bash
finder . -s "FIXME" -c
# Shows: path:count for each file
```

### JSON for tooling
```bash
finder . -s "error" --json | jq '.[] | select(.matches | length > 5)'
# Filter files with >5 matches
```

### Colors in pager
```bash
# Method 1: Use --colour flag
finder . -s "fn" --colour | less -R

# Method 2: Use CLICOLOR_FORCE environment variable
CLICOLOR_FORCE=1 finder . -s "fn" | less -R
```

### Disable colors globally
```bash
# Disable for all commands that respect NO_COLOR
export NO_COLOR=1
finder . -s "text"
# No colors in output
```

### Enable colors with TTY respect
```bash
# Enable colors only when outputting to terminal
export CLICOLOR=1
finder . -s "text"           # Colors shown (TTY)
finder . -s "text" | cat     # No colors (piped)
```

---

## Implementation Dependencies

**Crates:**
- `termcolor` - ANSI color output with TTY detection
- `atty` - TTY detection for auto-color mode
- `serde` + `serde_json` - JSON serialization

**CLI changes:**
- Add flags to `clap` parser
- Add mutual exclusivity validation
- Environment variable checking

**Output refactoring:**
- Centralize output formatting
- Separate logic for each output mode
- Color application based on flags/env/TTY

---

**Last Updated:** 2026-04-05
