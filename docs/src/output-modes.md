# Output Modes

FindeRS supports multiple output modes to suit different use cases.

## Standard Output (Default)

Shows file path, line number, and matching content.

```bash
finder -s "TODO"
```

Output:
```
src/lib.rs:42: // TODO: implement this
src/main.rs:15: // TODO: add error handling
```

## Files Only Mode (`-l`)

Lists only file paths containing matches, similar to `grep -l`.

```bash
finder -s "TODO" -l
```

Output:
```
src/lib.rs
src/main.rs
```

## Count Mode (`-c`)

Shows the number of matches per file, similar to `grep -c`.

```bash
finder -s "TODO" -c
```

Output:
```
src/lib.rs:3
src/main.rs:2
```

## JSON Mode (`--json`)

Structured JSON output for programmatic processing.

```bash
finder -s "error" --json
```

Output:
```json
[
  {
    "path": "src/lib.rs",
    "matches": [
      {"line": 42, "content": "handle error cases"},
      {"line": 87, "content": "return error result"}
    ]
  }
]
```

## Combining Modes

Output modes are mutually exclusive. Use one at a time:

```bash
# ✓ Valid
finder -s "TODO" -l
finder -s "TODO" -c
finder -s "TODO" --json

# ✗ Invalid (last one wins)
finder -s "TODO" -l -c
```

## Use Cases

- **Standard**: Interactive terminal use, reading results
- **Files only (`-l`)**: Piping to other commands, opening in editor
- **Count (`-c`)**: Statistics, understanding distribution
- **JSON (`--json`)**: Scripting, integration with other tools

## Next Steps

- [Color Configuration](./color-config.md) - Customize output colors
- [Integration Examples](./examples/integration.md) - Use with other tools
