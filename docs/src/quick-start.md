# Quick Start

Get started with FindeRS in minutes. This guide covers the most common usage patterns.

## Basic Concepts

FindeRS has two main operations:

1. **Finding files** - Filter by filename pattern with `-f`
2. **Searching content** - Search for text patterns with `-s`

You can use them separately or together.

## Find Files by Pattern

Find all Python files in the current directory:

```bash
finder -f ".py"
```

Find all Markdown files:

```bash
finder -f ".md"
```

Find files in a specific directory:

```bash
finder src/ -f ".rs"
```

## Search Content

Search for "TODO" in all files:

```bash
finder -s "TODO"
```

Case-insensitive search:

```bash
finder -s "error" -i
```

## Combine Both

Find "TODO" comments in Python files:

```bash
finder -f ".py" -s "TODO"
```

Find "FIXME" in Rust files within the src directory:

```bash
finder src/ -f ".rs" -s "FIXME"
```

## Output Modes

### Standard Output (Default)

Shows file path, line number, and matching content:

```bash
finder -s "TODO"
```

Output:
```
src/lib.rs:42: // TODO: implement this feature
src/main.rs:15: // TODO: add better error handling
```

### Files Only (`-l`)

List only the file paths (like `grep -l`):

```bash
finder -s "TODO" -l
```

Output:
```
src/lib.rs
src/main.rs
```

### Count Matches (`-c`)

Show match count per file (like `grep -c`):

```bash
finder -s "TODO" -c
```

Output:
```
src/lib.rs:3
src/main.rs:2
```

### JSON Output (`--json`)

Structured output for scripts:

```bash
finder -s "error" --json
```

Output:
```json
[
  {
    "path": "src/lib.rs",
    "matches": [
      {"line": 42, "content": "handle error cases"}
    ]
  }
]
```

## Working with Colors

Colors are automatic - on by default when outputting to a terminal, off when piping.

Force colors on (useful with `less -R`):

```bash
finder -s "pattern" --colour | less -R
```

Force colors off:

```bash
finder -s "pattern" --no-colour
```

Respect `NO_COLOR` environment variable:

```bash
NO_COLOR=1 finder -s "pattern"
```

## Using with Other Tools

### Pipe to jq

```bash
finder -s "error" --json | jq '.[] | .path'
```

### Count total matches

```bash
finder -s "TODO" -c | awk -F: '{sum+=$2} END {print sum}'
```

### Find and open in editor

```bash
vim $(finder -f "config" -l)
```

## Common Patterns

### Find configuration files

```bash
finder -f "config"
```

### Search for API keys (be careful!)

```bash
finder -s "api_key" -i
```

### Find large log files

```bash
finder -f ".log" | xargs ls -lh
```

### Search in specific file types

```bash
finder -f ".ts" -s "interface"  # TypeScript
finder -f ".go" -s "func"        # Go
finder -f ".py" -s "class"       # Python
```

## Next Steps

- Explore [common use cases](./examples/common-use-cases.md) in detail
- Check the [CLI Reference](./cli-reference.md) for all options
- Learn about [color configuration](./color-config.md)
- See [advanced patterns](./examples/advanced-patterns.md)
