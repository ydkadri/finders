# Contributing

Thanks for your interest in contributing to FindeRS! This is a personal project that grew into my daily command-line companion, and I'm happy to have others involved.

## Getting Started

FindeRS is written in Rust. You'll need:

- Rust 1.70 or later (`rustup update`)
- Git
- A GitHub account (for pull requests)

Clone and build:

```bash
git clone https://github.com/ydkadri/finders.git
cd finders
cargo build
cargo test
```

## Areas for Contribution

I'm particularly interested in:

- **Bug fixes** - If something doesn't work as documented, please fix it!
- **Documentation** - Improvements to examples, clarifications, typo fixes
- **Performance** - Benchmarks, profiling, optimization ideas
- **Testing** - More test cases, especially edge cases
- **Examples** - Real-world usage patterns and integration examples

## Before You Start

For anything beyond typos and documentation:

1. **Open an issue first** to discuss the change
2. Wait for feedback before investing significant time
3. Keep changes focused and atomic

This helps ensure your effort aligns with the project direction.

## Development Workflow

### Making Changes

1. Fork the repository
2. Create a feature branch (`git checkout -b fix/your-fix`)
3. Make your changes
4. Write or update tests
5. Run the checks (see below)
6. Commit with clear messages
7. Push and open a pull request

### Quality Checks

Before submitting:

```bash
# Format code
cargo fmt

# Lint
cargo clippy -- -D warnings

# Test
cargo test

# Benchmarks (if you changed performance-sensitive code)
cargo bench
```

All checks must pass before your PR can be merged.

### Writing Tests

- Add unit tests in the same file as the code
- Add integration tests in `tests/` for end-to-end scenarios
- Test both happy paths and error cases

Example:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_your_feature() {
        // Arrange
        let input = setup_test_data();
        
        // Act
        let result = your_function(input);
        
        // Assert
        assert_eq!(result, expected);
    }
}
```

### Commit Messages

Keep them clear and descriptive:

```
Fix regex escaping in file patterns

Filenames with dots were incorrectly treated as regex
metacharacters. Now properly escape special characters
before compiling regex patterns.

Fixes #42
```

## Pull Request Process

1. **Create PR early** - Mark as draft if not ready for review
2. **Describe the change** - What problem does it solve? Why this approach?
3. **Link to issue** - Reference any related issues
4. **Be responsive** - Reply to feedback and questions
5. **Squash commits** - Before marking ready, rebase into logical commits

### PR Checklist

- [ ] Tests added/updated and passing
- [ ] Documentation updated (if needed)
- [ ] `cargo fmt` and `cargo clippy` pass
- [ ] Commit messages are clear
- [ ] CHANGELOG.md updated (for user-facing changes)

## Code Style

Follow Rust conventions:

- Use `rustfmt` (run `cargo fmt`)
- Follow `clippy` suggestions (run `cargo clippy`)
- Prefer `Result` and `Option` over panics
- Document public APIs with `///` comments
- Keep functions focused and small

## Documentation

User-facing documentation is in `docs/src/`:

- `quick-start.md` - Getting started guide
- `cli-reference.md` - Complete CLI documentation
- `examples/` - Usage examples
- `reference/` - Technical details

When adding features:

1. Update relevant documentation
2. Add examples showing how to use it
3. Update CHANGELOG.md

## Testing Philosophy

- **Unit tests** for individual functions and modules
- **Integration tests** for command-line interface
- **Benchmarks** for performance-critical code

Test real scenarios that users will encounter.

## Performance Considerations

FindeRS aims to be "fast enough" - quick for daily use but prioritizing simplicity:

- Benchmark significant changes with `cargo bench`
- Profile with Instruments (macOS) or perf (Linux) if needed
- Don't sacrifice readability for premature optimization
- Document performance trade-offs

## Release Process

Releases are handled by maintainers:

1. Version bump in `Cargo.toml`
2. Update CHANGELOG.md
3. Tag release (GitHub Actions handles the rest)
4. Publish to crates.io
5. GitHub release with binaries

Contributors don't need to worry about this - focus on the fix or feature!

## Communication

- **GitHub Issues** - For bugs, features, and questions
- **Pull Requests** - For code review and discussion
- **Email** - youcef.kadri@example.com for private concerns

## Questions?

Don't hesitate to ask! Open an issue with your question, even if it's just "how do I...?" - I'm happy to help.

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (see LICENSE file).

## Code of Conduct

Be respectful and constructive. This is a small project - let's keep it friendly and collaborative.

---

Thank you for contributing to FindeRS! 🦀
