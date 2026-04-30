# Contributing to FindeRS

Thanks for considering contributing! This is a pet project that grew into a standard part of my workflow. It keeps growing and changing and I'm happy to see contributions to make it better.

## Philosophy

This project follows an experiment-and-iterate approach: try things, see what works, throw away what doesn't quickly.

I'm still learning the Rust language, so if you're proposing changes, please explain **why** the change matters, not just what you changed. Context helps.

## Getting Started

### Prerequisites

- Rust (stable) - [Install from rustup.rs](https://rustup.rs/)
- Git
- [just](https://github.com/casey/just) - Command runner (optional but recommended)

### Setup

```bash
# Clone the repository
git clone https://github.com/ydkadri/finders.git
cd finders

# Build and run (using just - matches CI)
just build
just run . -s "TODO"

# Or use cargo directly
cargo build
cargo run -- . -s "TODO"

# Run tests (using just ensures same config as CI)
just test

# Run benchmarks
just bench
```

## How to Contribute

### Reporting Bugs

Open an issue with:
- What you were trying to do
- What happened instead
- Steps to reproduce
- Your OS and Rust version (`rustc --version`)

### Suggesting Features

When suggesting features:
- Explain the **problem** you're trying to solve
- Describe your **use case**
- Consider whether it fits the project's scope (simple daily-use CLI tool)

### Pull Requests

**Before starting work on a PR:**
1. Check existing issues and PRs to avoid duplicate work
2. For significant changes, open an issue first to discuss the approach

**PR Guidelines:**
1. Write tests for new functionality
2. Ensure all CI checks pass (use `just pre-commit` to run same checks as CI)
3. Keep commits focused and logical
4. Write clear commit messages explaining **why**, not just what

## Code Standards

### Style

- Run `just fmt` before committing (or `cargo fmt` directly)
- Run `just lint` and fix all warnings (or `cargo clippy -- -D warnings` directly)
- Follow Rust naming conventions (see [CLAUDE.md](CLAUDE.md) for details)
- Use `just` commands when possible to ensure consistency with CI

### Testing

- Unit tests in `#[cfg(test)]` modules
- Integration tests in `tests/` directory
- Benchmarks in `benches/` directory

### Documentation

- Document all public APIs with `///` comments
- Include examples in documentation
- Update README.md for user-facing changes
- Update CHANGELOG.md with your changes

### British English

This project uses British English spelling (colour, behaviour, etc.) in documentation, comments, and user-facing text. Exceptions:
- External crate names
- Standard environment variables
- Code identifiers where required by convention

## Architecture

Key documents:
- **[docs/adr/](docs/adr/)** - Architecture Decision Records

## Questions?

Open an issue or reach out on [GitHub](https://github.com/ydkadri) or [LinkedIn](https://www.linkedin.com/in/youcefk/).

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
