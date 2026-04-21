---
title: Use anyhow for Error Handling
date: 2026-04-21
status: accepted
---

# Use anyhow for Error Handling

## Context

Our CLI tool has multiple points of failure:
- File system operations (opening files, reading directories)
- Path validation
- Regex compilation
- File content reading (UTF-8 decoding)

Previously, we used raw `std::io::Error` and `Result<T, Error>` types throughout. This had several issues:

1. **Poor error context**: When a file couldn't be opened, users only saw "No such file or directory" without knowing which operation failed or what path was being accessed.

2. **Verbose error handling**: Every function needed to import `std::io::Error` and manage error types explicitly.

3. **Limited error composition**: Chaining context through multiple layers was difficult and verbose.

4. **Inconsistent patterns**: Some code used `unwrap()` or `expect()` for convenience, which could panic rather than providing helpful errors.

Example of previous error experience:
```
Error: No such file or directory (os error 2)
```

Desired error experience:
```
Error: initializing file finder

Caused by:
    0: validating root path
    1: validating path
    2: path does not exist: '/invalid/path'
```

## Decision

We will use the `anyhow` crate for all error handling throughout the application (both library and binary).

**Why anyhow rather than thiserror:**
- This is an application/tool, not a library providing APIs to other Rust code
- The library portion (`finders` crate) is internal-only, used exclusively by the CLI binary
- We need flexible error context chains more than we need specific error types
- anyhow's `.context()` method makes it trivial to add helpful context at every layer

**Implementation approach:**
1. Replace all `Result<T, std::io::Error>` with `anyhow::Result<T>`
2. Add context to every error-returning operation using `.context()`
3. Use error downcasting where we need to check specific error types (e.g., `InvalidData` for encoding errors)
4. Update tests to check for context messages rather than raw error types

## Consequences

### Positive

**Better user experience:**
- Users see exactly what operation failed and why
- Error chains show the full context from high-level operation down to low-level cause
- Context messages include relevant details (file paths, patterns, etc.)

**Simpler code:**
- Single error type (`anyhow::Error`) throughout
- `.context()` method is ergonomic and self-documenting
- Less boilerplate in error handling

**Easier debugging:**
- Error messages include full context chains
- When users report issues, we can see exactly what failed
- Test failures are more informative

**Consistent patterns:**
- All error paths use the same pattern: `operation.context("what it was doing")?`
- No more `unwrap()` or `expect()` (except in tests)

### Negative

**Library/binary distinction blurred:**
- Technically `finders` is a library, but we're using application-focused error handling
- If we ever wanted to expose this as a public library, we'd need to switch to thiserror
- **Mitigation**: This is acceptable because the library is internal-only

**Error type opacity:**
- `anyhow::Error` is a trait object, so we can't match on error types directly
- Need to use downcasting (`downcast_ref::<T>()`) to check specific error kinds
- **Mitigation**: We only need type checking in one place (detecting encoding errors to continue vs. propagate)

**Binary size:**
- anyhow adds ~50KB to binary size
- **Mitigation**: Negligible for a CLI tool (our binary is already several MB)

**Learning curve:**
- Team needs to understand context chaining and when to add context
- **Mitigation**: Pattern is simple: add context to every `?` operation

## Alternatives Considered

### Alternative 1: Keep using raw std errors
- **Pros**: No dependencies, minimal binary size
- **Cons**: Poor user experience, verbose code, no context chaining
- **Rejected**: User experience is too important

### Alternative 2: Use thiserror
- **Pros**: Type-safe error handling, good for libraries
- **Cons**: More boilerplate, less flexible context, designed for library APIs
- **Rejected**: We're not a public library, and we need flexible context more than type safety

### Alternative 3: Custom error type with manual context
- **Pros**: No dependencies, complete control
- **Cons**: Significant implementation work, likely to reinvent anyhow poorly
- **Rejected**: Not worth the maintenance burden when anyhow exists

## Implementation Notes

**Pattern for adding context:**
```rust
operation
    .context(format!("doing something with '{}'", detail))?
```

**Pattern for checking specific error types:**
```rust
if let Some(io_err) = e.downcast_ref::<std::io::Error>()
    && io_err.kind() == ErrorKind::InvalidData
{
    // Handle this specific error
}
```

**Functions that changed:**
- `src/bin/finder.rs`: `main()` - context on Finder::new and ReSearcher::new
- `src/file_finder/mod.rs`: `Finder::new()` - context on path parsing
- `src/file_finder/path_parser.rs`: `parse()` - context on validation
- `src/searcher.rs`: `ReSearcher::new()` - context on regex compilation
- `src/lib.rs`: `search_files()`, `search_file()` - context on file operations
- Tests updated to check for context messages

**Coverage impact:**
- Coverage improved from 90.43% to 90.92%
- New integration tests specifically verify error context chains
- All existing tests still pass

## References

- [anyhow documentation](https://docs.rs/anyhow)
- [thiserror vs anyhow guidance](https://nick.groenen.me/posts/rust-error-handling/#thiserror-and-anyhow)
- Rust Error Handling best practices: `~/Documents/ydkadri/claude/languages/rust.md`
