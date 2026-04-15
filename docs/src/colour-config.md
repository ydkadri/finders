# Colour Configuration

FindeRS provides coloured output for better readability. This page explains how to configure and control colours.

## Default Behaviour

Colours are **automatic** by default:
- **On** when outputting to a terminal (TTY)
- **Off** when piping to another command or file

This "just works" for most use cases without any configuration.

## Colour Scheme

When colours are enabled:
- **File paths**: Green
- **Line numbers**: Cyan
- **Match highlights**: Bold white on blue background

## Force Colours On

Use `--colour` to force colours on, even when piping:

```bash
finder -s "pattern" --colour | less -R
```

The `-R` flag tells `less` to display colours correctly.

## Force Colours Off

Use `--no-colour` to force colours off:

```bash
finder -s "pattern" --no-colour
```

Useful for:
- Saving output to files
- Ensuring plain text in scripts
- Terminal compatibility issues

## Environment Variables

FindeRS respects standard colour environment variables.

### `NO_COLOR`

Disables all colours when set (any value):

```bash
NO_COLOR=1 finder -s "pattern"
```

Learn more at [no-colour.org](https://no-colour.org/).

### `CLICOLOR`

Controls colour support:

```bash
CLICOLOR=0 finder -s "pattern"  # Disable colours
CLICOLOR=1 finder -s "pattern"  # Enable colours (with TTY detection)
```

### `CLICOLOR_FORCE`

Forces colours on, even when not outputting to a terminal:

```bash
CLICOLOR_FORCE=1 finder -s "pattern"
```

Learn more at [bixense.com/clicolours](https://bixense.com/clicolours/).

## Priority Order

When multiple settings conflict, FindeRS uses this priority:

1. **CLI flags** (`--colour` or `--no-colour`)
2. **NO_COLOR** environment variable
3. **CLICOLOR_FORCE** environment variable  
4. **CLICOLOR** environment variable
5. **Auto-detection** (default)

## Examples

### Force colours for paging

```bash
finder -s "TODO" --colour | less -R
```

### Disable colours for file output

```bash
finder -s "error" --no-colour > errors.txt
```

### Respect NO_COLOR in scripts

```bash
#!/bin/bash
export NO_COLOR=1
finder -s "pattern"  # No colours
```

## Troubleshooting

### Colors don't work in my terminal

- Check your terminal supports ANSI colours
- Try forcing colours: `finder --colour`
- Check if `NO_COLOR` is set: `echo $NO_COLOR`

### Colors show as weird characters

Your terminal doesn't support ANSI escape codes. Use `--no-colour`.

### Colors persist after piping

This is expected behavior. Use `--no-colour` if needed.

## See Also

- [Output Modes](./output-modes.md) - Different output formats
- [CLI Reference](./cli-reference.md) - All command-line options
