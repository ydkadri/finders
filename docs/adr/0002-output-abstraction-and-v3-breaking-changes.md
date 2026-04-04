# ADR 0002: Output Abstraction and v3.0.0 Breaking Changes

## Status

Accepted

## Context

FindeRS v2.x had hardcoded output formatting with `println!` calls scattered throughout the codebase. As the tool matured, users requested:
1. Colored output for better readability
2. Multiple output modes (files-only, count, JSON) for different use cases
3. Machine-readable output for scripting

Additionally, the custom output format (`   4: /path/to/file.rs          content`) differed from industry standards like grep/ripgrep, making it harder for users familiar with those tools.

## Decision

We decided to:

1. **Create an Output trait abstraction** to decouple output formatting from search logic
2. **Standardize output format** to `path:line: content` (matching grep/ripgrep)
3. **Add coloured output** with community-standard environment variable support
4. **Implement multiple output modes** via trait implementations
5. **Version as 3.0.0 (MAJOR)** due to breaking output format change

### Output Trait Design

```rust
pub trait Output {
    fn write_match(&mut self, match_result: &SearchMatch);
    fn write_file(&mut self, path: &Path);
    fn finalize(&mut self) {}
}
```

**Rationale:**
- Simple, focused interface
- `write_match`: handles search results
- `write_file`: handles file-only mode
- `finalize`: allows buffered outputs (JSON, count) to flush

### Output Implementations

1. **StandardOutput**: Default `path:line: content` format with colours
2. **FilesOnlyOutput**: Only paths (like `grep -l`)
3. **CountOutput**: Match counts per file (like `grep -c`)
4. **JsonOutput**: Structured JSON for machine processing

**Rationale:**
- Trait objects (`Box<dyn Output>`) enable runtime mode selection
- Each implementation is self-contained and testable
- Easy to add new output modes in future

### Color Library Choice: termcolor

**Alternatives considered:**
- `colored` crate: Simple API but less flexible
- `owo-colors` crate: Modern, but termcolor is industry standard
- `ansi_term` crate: Deprecated

**Chosen: termcolor**
- Cross-platform (Windows, Unix)
- Built-in TTY detection via `ColorChoice::Auto`
- Used by ripgrep and other mature Rust CLI tools
- Integrates with `StandardStream` for proper buffering

### Environment Variable Standards

Implemented full compliance with:
- **NO_COLOR** (https://no-color.org/): Universal opt-out
- **CLICOLOR** + **CLICOLOR_FORCE** (https://bixense.com/clicolors/): BSD/macOS standard

**Priority order:**
1. CLI flags (`--colour`/`--no-colour`)
2. `NO_COLOR`
3. `CLICOLOR_FORCE`
4. `CLICOLOR`
5. Auto-detect TTY

**Rationale:** Respecting community standards improves compatibility with user workflows and other tools.

### Color Scheme

- **Paths**: Green (visible, not harsh)
- **Line numbers**: Cyan (distinct from paths)
- **Matches**: White on Magenta (high contrast, different from grep's red)

**Rationale:**
- Works on both dark and light terminals
- Green/cyan combination is common in CLI tools
- Magenta background chosen over red for distinctiveness

### Output Format Breaking Change

**Old (v2.x):**
```
   4: /path/to/file.rs                                      line content
```

**New (v3.0.0):**
```
/path/to/file.rs:4: line content
```

**Justification for MAJOR version bump:**
- CLI output is the primary API for a CLI tool
- Scripts parsing output WILL break
- Better to be conservative and clearly signal change
- Matches ecosystem norms (ripgrep treats output changes as major)

**Benefits of new format:**
- **Standard structure**: Matches grep/ripgrep (easier to learn)
- **Easier parsing**: Consistent `path:line: content` pattern
- **Better UX**: No fixed-width padding (handles long paths, narrow terminals)
- **Future-proof**: Simpler to maintain and extend

### JSON Output Design

**Format:**
```json
[
  {
    "path": "src/lib.rs",
    "matches": [
      {"line": 42, "content": "..."}
    ]
  }
]
```

**Rationale:**
- Array of objects (one per file) is standard JSON structure
- Sorted by path for deterministic output
- Simple schema easy to parse with jq or other tools
- Provides stable API for scripts (unlike text format which may change)

### Mutual Exclusivity via Clap

Output modes enforced declaratively via clap's `conflicts_with` attributes rather than runtime validation.

**Rationale:**
- Clearer error messages from clap
- No code needed - declarative
- Faster failure (at parse time, not runtime)

## Consequences

### Positive

1. **Extensibility**: Adding new output modes requires only implementing the trait
2. **Testability**: Each output mode can be tested independently
3. **User choice**: Multiple output modes serve different use cases
4. **Standards compliance**: Respecting NO_COLOR/CLICOLOR improves compatibility
5. **Familiar format**: Standard `path:line: content` reduces learning curve
6. **Machine-readable**: JSON output enables robust scripting

### Negative

1. **Breaking change**: Existing scripts parsing output will break
2. **Migration burden**: Users must update their scripts for v3.0.0
3. **Increased complexity**: Trait objects add slight runtime overhead
4. **Dependency increase**: Added termcolor, serde, serde_json

### Mitigation

1. **Clear communication**: BREAKING CHANGES section in CHANGELOG with migration guide
2. **JSON for stability**: Recommend `--json` for scripts needing stable API
3. **Version signal**: MAJOR bump (3.0.0) clearly indicates breaking change
4. **Documentation**: Examples showing new format and migration patterns

## Alternatives Considered

### Alternative 1: Keep old format, add new modes separately

**Pros:** No breaking change, gradual migration
**Cons:** Two output formats to maintain, confusion about which is "standard"
**Rejected:** Technical debt would accumulate; better to standardize once

### Alternative 2: Make format configurable

**Pros:** Users can choose format
**Cons:** More complexity, testing burden, no clear default
**Rejected:** CLI tools should have one good way to do things; configuration adds complexity

### Alternative 3: Use string formatting instead of trait

**Pros:** Simpler, no trait objects
**Cons:** Output logic still scattered, harder to test, less extensible
**Rejected:** Trait abstraction provides better separation of concerns

## Related

- GitHub Issue #17: Colored output
- GitHub Issue #22: Multiple output modes
- Interface Design: `docs/interface-v3.0.0.md`
- CHANGELOG: Breaking changes section

## References

- NO_COLOR standard: https://no-color.org/
- CLICOLORS specification: https://bixense.com/clicolors/
- Semantic Versioning: https://semver.org/
- termcolor crate: https://docs.rs/termcolor/
