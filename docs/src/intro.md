# Introduction

**A simpler way to find files and search content from the command line.**

I live in the terminal and I got sick of typing `find . -type f -name "*.py" -exec grep -iH "..." {} \;` every day. This is what I use these days and you should get involved.

## What is FindeRS?

FindeRS (`finder`) is a command-line tool that simplifies file searching. It combines the power of `find` and `grep` with a cleaner, more intuitive interface.

Instead of this:
```bash
find . -type f -name "*.py" -exec grep -iH "TODO" {} \;
```

You write this:
```bash
finder -f ".py" -s "TODO"
```

## Key Features

- **Simple interface**: `-f` finds files, `-s` searches content
- **Colored output**: Matches highlighted, easy to scan
- **Multiple output modes**: JSON, count-only, files-only
- **Fast and efficient**: Streaming file processing
- **Single binary**: No dependencies, just download and run

## Quick Example

Search for "TODO" comments in all Python files:

```bash
finder -f ".py" -s "TODO"
```

Output:
```
src/lib.rs:42: // TODO: implement this feature
src/main.rs:15: // TODO: add better error handling
```

## Why Choose FindeRS?

- **Learnable**: Simple flags, no complex syntax
- **Practical**: Built for daily use, not edge cases
- **Predictable**: Sensible defaults, colored output when needed
- **Fast enough**: Streaming processing, efficient search algorithms

## Next Steps

- [Install FindeRS](./installation.md) on your system
- Follow the [Quick Start](./quick-start.md) guide
- Explore [common use cases](./examples/common-use-cases.md)

## Getting Help

- [GitHub Issues](https://github.com/ydkadri/finders/issues) - Bug reports and feature requests
- [Contributing Guide](./contributing.md) - Help make it better
- [Troubleshooting](./reference/troubleshooting.md) - Common problems and solutions
