# FindeRS Project Structure Review

**Date:** 2026-04-21  
**Current Version:** 3.0.1  
**Status:** Complete

## Executive Summary

FindeRS has a clean, well-organized structure that follows Rust best practices. The codebase is small (< 1000 LOC), maintainable, and exhibits clear separation of concerns. Minor improvements are recommended but no major restructuring is needed.

**Key Strengths:**
- Clear module boundaries with single responsibilities
- Minimal public API surface
- Good separation of CLI and library concerns
- Consistent naming conventions
- Well-organized output abstraction

**Recommended Improvements:**
- Extract error types to dedicated module (future work)
- Consider splitting `file_finder` into smaller submodules if complexity grows
- Add more granular re-exports in lib.rs for better API discoverability

## Current Structure

```
finders/
├── src/
│   ├── bin/
│   │   └── finder.rs              # CLI binary (118 lines)
│   ├── file_finder/
│   │   ├── mod.rs                 # File discovery (137 lines)
│   │   └── path_parser.rs         # Path validation (43 lines)
│   ├── output/
│   │   ├── mod.rs                 # Output trait + re-exports (34 lines)
│   │   ├── colour.rs              # Colour configuration (192 lines)
│   │   ├── standard.rs            # Standard output formatter (125 lines)
│   │   ├── json.rs                # JSON output formatter (151 lines)
│   │   ├── count.rs               # Count output formatter (66 lines)
│   │   └── files_only.rs          # Files-only output formatter (55 lines)
│   ├── lib.rs                     # Library entry + search_files() (162 lines)
│   └── searcher.rs                # Search implementations (297 lines)
├── tests/
│   ├── cli_integration_tests.rs   # CLI integration tests
│   └── error_handling_tests.rs    # Error handling tests
└── benches/
    ├── search_benchmarks.rs       # Internal benchmarks
    ├── comparison_benchmarks.rs   # Comparison benchmarks
    └── fixtures/                  # Benchmark test data
```

**Total Source LOC:** ~1,380 lines (excluding tests/benches)

## Module Analysis

### 1. `lib.rs` (Library Entry Point)

**Purpose:** Public library entry point, core search orchestration

**Public API:**
```rust
pub mod file_finder;
pub mod output;
pub mod searcher;

pub fn search_files(
    searcher: impl searcher::Searches,
    paths: Vec<PathBuf>,
    verbose: bool,
    output: &mut dyn output::Outputs,
) -> Result<(), Error>
```

**Strengths:**
- ✅ Clear, minimal public API
- ✅ Core orchestration logic in one place
- ✅ Streaming architecture (processes files one at a time)
- ✅ Good error handling with verbose mode

**Considerations:**
- Re-exports are implicit (users must do `finders::searcher::Searcher`)
- Could add convenience re-exports for common types:
  ```rust
  pub use file_finder::Finder;
  pub use searcher::{Searcher, ReSearcher, Searches};
  pub use output::{Outputs, SearchMatch, ColourMode};
  ```

**Recommendation:** ✅ Structure is good, consider adding convenience re-exports for better ergonomics

---

### 2. `bin/finder.rs` (CLI Binary)

**Purpose:** Command-line interface, argument parsing, user-facing entry point

**Strengths:**
- ✅ Clean separation from library code
- ✅ Well-organized with clap declarative arg parsing
- ✅ Minimal business logic (delegates to library)
- ✅ Good error messaging for invalid regex

**Code Organization:**
```rust
// Lines 1-18:   Imports + ASCII art
// Lines 19-65:  Clap CLI structure (args)
// Lines 67-117: main() function (orchestration)
```

**Strengths:**
- ✅ ASCII art separate from logic
- ✅ Single main() function, no helper functions needed
- ✅ Clear flow: parse args → create finder → create output → search

**Recommendation:** ✅ Structure is excellent, no changes needed

---

### 3. `file_finder/` Module

**Purpose:** File discovery, directory traversal, path validation

**Structure:**
```
file_finder/
├── mod.rs          # Finder struct + find logic
└── path_parser.rs  # Path validation helper
```

**Public API:**
```rust
pub struct Finder<'a> { pub path: &'a Path }

impl Finder<'_> {
    pub fn new(root: Option<&str>) -> Result<Finder<'_>, Error>
    pub fn find(&self, query: Option<&str>) -> Vec<PathBuf>
}
```

**Strengths:**
- ✅ Single responsibility: file discovery
- ✅ Good separation: mod.rs (logic) vs path_parser.rs (validation)
- ✅ Proper error handling with warnings for unreadable files
- ✅ follows_links(true) documented via code

**Code Organization in `mod.rs`:**
```rust
// Lines 1-8:   Imports + struct definition
// Lines 10-15: Constructor (new)
// Lines 17-19: Private helper (find_internal)
// Lines 21-44: unfiltered_find (private)
// Lines 46-86: filtered_find (private)
// Lines 88-93: Public API (find)
// Lines 96-136: Tests
```

**Issues:**
- ⚠️ Function ordering: private methods before constructor
  - Rust style guide: constructors before other methods
  - Current: find_internal → unfiltered_find → filtered_find → new → find
  - Better: new → find → find_internal → unfiltered_find → filtered_find

- ⚠️ Code duplication between `unfiltered_find` and `filtered_find`
  - Both have identical metadata handling logic (lines 24-37 vs 54-66)
  - Could extract common iterator chain

**Recommendation:**
- Fix function ordering (follow style guide)
- Consider extracting shared logic to reduce duplication
- Structure is otherwise solid

---

### 4. `searcher.rs` Module

**Purpose:** Search pattern matching (literal and regex)

**Public API:**
```rust
pub struct SearchResult {
    pub rownum: usize,
    pub line: String,
    pub match_positions: Vec<(usize, usize)>,
}

pub trait Searches {
    fn search<'a>(&'a self, contents: &'a str) -> Vec<SearchResult>;
    fn search_line(&self, line: &str, rownum: usize) -> Option<SearchResult>;
}

pub struct Searcher<'a> { ... }
impl Searcher<'_> {
    pub fn new(query: &str, case_insensitive: bool) -> Searcher<'_>
}

pub struct ReSearcher { ... }
impl ReSearcher {
    pub fn new(pattern: &str) -> Result<ReSearcher, regex::Error>
}
```

**Strengths:**
- ✅ Trait-based design allows polymorphism
- ✅ Both legacy (search) and streaming (search_line) methods
- ✅ Good separation: literal search vs regex search
- ✅ Comprehensive test coverage

**Code Organization:**
```rust
// Lines 1-9:    Imports + SearchResult struct
// Lines 11-27:  Searcher + ReSearcher struct definitions + trait
// Lines 29-37:  SearchResult impl (constructor)
// Lines 39-98:  Searcher implementation (new, sensitive, insensitive, helpers)
// Lines 101-114: ReSearcher implementation (new, regex helpers)
// Lines 116-164: Trait implementations (Searches for Searcher and ReSearcher)
// Lines 166-296: Tests
```

**Strengths:**
- ✅ Logical grouping: struct → impl → trait impl
- ✅ Private methods (sensitive_search, insensitive_search) correctly private
- ✅ Good naming: descriptive and consistent

**Issues:**
- ⚠️ Function ordering within impls: helpers before constructors
  - Current: new → sensitive_search → find_match_positions → insensitive_search
  - Better: new → public methods → private helpers

**Recommendation:**
- Fix function ordering (constructors first, then methods, then helpers)
- Structure is otherwise excellent

---

### 5. `output/` Module

**Purpose:** Output formatting abstraction (standard, JSON, count, files-only)

**Structure:**
```
output/
├── mod.rs         # Trait definition + re-exports
├── colour.rs      # Colour mode configuration
├── standard.rs    # Standard "path:line: content" output
├── json.rs        # JSON output
├── count.rs       # Count output (path:count)
├── files_only.rs  # Files-only output (just paths)
```

**Public API:**
```rust
// Trait
pub trait Outputs {
    fn write_match(&mut self, match_result: &SearchMatch);
    fn write_file(&mut self, path: &Path);
    fn finalize(&mut self) {}
}

// Implementations
pub struct StandardOutput { ... }
pub struct JsonOutput { ... }
pub struct CountOutput { ... }
pub struct FilesOnlyOutput { ... }
pub enum ColourMode { Auto, Always, Never }
```

**Strengths:**
- ✅ Excellent abstraction: trait-based polymorphism
- ✅ Single responsibility per formatter
- ✅ Colour logic isolated in dedicated module
- ✅ Clean separation: colour config vs formatting logic
- ✅ Each formatter is small and focused (55-192 lines)

**Code Organization in `mod.rs`:**
```rust
// Lines 1-7:  Submodule declarations
// Lines 9-13: Re-exports (public API)
// Lines 15-21: SearchMatch struct (shared data)
// Lines 23-33: Outputs trait (abstraction)
```

**Strengths:**
- ✅ Perfect separation: mod.rs only contains trait + re-exports
- ✅ Each formatter in its own file
- ✅ Shared types (SearchMatch) defined centrally

**Recommendation:** ✅ This is exemplary module organization, no changes needed

---

### 6. Testing Structure

**Current Structure:**
```
tests/
├── cli_integration_tests.rs    # End-to-end CLI tests
└── error_handling_tests.rs     # Error handling tests

src/
└── <modules>/                  # Unit tests in #[cfg(test)] modules
```

**Strengths:**
- ✅ Good separation: unit tests inline, integration tests in `tests/`
- ✅ Integration tests exercise real CLI binary
- ✅ Error handling tests in dedicated file

**Test Coverage by Module:**
- `lib.rs`: 3 tests (streaming, search_line, chunking)
- `file_finder/mod.rs`: 3 tests (initialisation, errors)
- `file_finder/path_parser.rs`: 3 tests (valid, empty, invalid paths)
- `searcher.rs`: 8 tests (case sensitive/insensitive, regex, invalid patterns)
- `output/colour.rs`: 6 tests (environment variables, TTY detection)
- `output/json.rs`: 4 tests (serialisation, accumulation)
- `output/count.rs`: 2 tests (aggregation, multiple files)
- `output/files_only.rs`: 2 tests (deduplication, multiple files)

**Current Coverage:** ~75% (from CI badge)

**Recommendation:**
- ✅ Structure is solid
- Consider adding more edge case tests for 80%+ coverage (#42)
- Benchmark structure is also good (internal + comparison)

---

## Detailed Review: Questions from Issue #75

### 1. Module Organization

**Q: Is the split between lib.rs and modules clear?**

✅ **Yes.** Each module has a single, well-defined responsibility:
- `lib.rs`: Orchestration and library entry point
- `file_finder`: File discovery
- `searcher`: Pattern matching
- `output`: Result formatting

**Q: Should output formatters be under `output/` or `formatters/`?**

✅ **Current `output/` is better.** Rationale:
- Trait is named `Outputs` (noun), not `Formats` (verb)
- Module contains output configuration (ColourMode), not just formatters
- "Output" is broader and more accurate than "formatter"

**Q: Any modules that should be split or merged?**

⚠️ **Minor considerations:**
- `file_finder` could be split if it grows:
  - `file_finder/discovery.rs` (Finder logic)
  - `file_finder/filtering.rs` (pattern matching)
  - Currently fine at 137 lines, but watch for growth

- Consider future `errors/` module:
  - Currently using `std::io::Error` everywhere
  - Future #52 (anyhow migration) or custom error types would warrant dedicated module

**Recommendation:** Current structure is good, no immediate changes needed

---

### 2. Public API Surface

**Q: What should be public vs private?**

Current public API:
```rust
// lib.rs
pub fn search_files(...)

// Modules
pub mod file_finder;  // Finder struct
pub mod searcher;     // Searcher, ReSearcher, Searches trait
pub mod output;       // Outputs trait, all formatters, ColourMode
```

✅ **API surface is appropriately minimal:**
- Only one public function (`search_files`)
- All modules public for library users
- Internal helpers correctly private

**Q: Is the API intuitive for library users?**

⚠️ **Slightly verbose due to no re-exports:**
```rust
// Current: Users must write
use finders::file_finder::Finder;
use finders::searcher::Searcher;
use finders::output::{StandardOutput, ColourMode};

// Better: With re-exports in lib.rs
use finders::{Finder, Searcher, StandardOutput, ColourMode};
```

**Recommendation:** Add convenience re-exports to lib.rs (see "Improvements" section below)

---

### 3. Separation of Concerns

**Q: CLI logic vs library logic clearly separated?**

✅ **Yes, excellent separation:**
- `lib.rs`: Pure library logic, no CLI dependencies
- `bin/finder.rs`: CLI-only (clap, arg parsing, exit codes)
- Library is usable independently of CLI

**Q: Is `main.rs` minimal or does it contain business logic?**

✅ **Minimal.** The 118-line `finder.rs` only contains:
- Argument parsing (clap struct)
- Output mode selection
- Delegation to library functions

All business logic is in the library.

**Q: Should configuration be a separate module?**

⚠️ **Not yet, but watch for:**
- Currently only `ColourMode` configuration (in `output/colour.rs`)
- If search configuration grows (ignore patterns, limits, etc.), extract to `config/` module
- Current: 1 config type, fine where it is
- Future: 3+ config types → dedicated module

**Recommendation:** Current separation is excellent, no changes needed

---

### 4. File/Module Naming

**Q: Consistent naming conventions?**

✅ **Yes:**
- Modules: `snake_case` (file_finder, searcher)
- Structs: `PascalCase` (Finder, Searcher, ReSearcher)
- Traits: `PascalCase` 3rd-person verbs (Searches, Outputs)
- Functions: `snake_case` verbs (search_files, write_match)

**Q: Names reflect their purpose clearly?**

✅ **Yes:**
- `file_finder` → finds files
- `searcher` → searches content
- `output` → formats output
- `Searches` → things that can search
- `Outputs` → things that produce output

**Q: Follow Rust naming standards?**

✅ **Yes, follows Rust API Guidelines:**
- Traits are verbs (Searches, Outputs)
- Constructors named `new()`
- Methods named for what they do (find, search, write_match)

**Recommendation:** Naming is excellent, no changes needed

---

### 5. Code Organization Within Files

**Q: Function ordering: public before private, or called before caller?**

⚠️ **Inconsistent:**
- `file_finder/mod.rs`: ✅ Constructor first, but private helpers mixed before public methods
- `searcher.rs`: ✅ Constructor first, private helpers after
- `output/` modules: ✅ Consistent ordering

**Rust Style Guide Recommendation:**
1. Constructor (`new`)
2. Public methods
3. Private methods

**Current vs Recommended:**

```rust
// file_finder/mod.rs - CURRENT (inconsistent)
fn find_internal()      // Private helper
fn unfiltered_find()    // Private
fn filtered_find()      // Private
pub fn new()            // Constructor
pub fn find()           // Public method

// RECOMMENDED
pub fn new()            // Constructor first
pub fn find()           // Public method
fn find_internal()      // Private helpers
fn unfiltered_find()
fn filtered_find()
```

**Recommendation:** Standardise function ordering across all modules (#54)

---

### 6. Testing Structure

**Q: Unit tests in modules or separate test files?**

✅ **Current approach is correct:**
- Unit tests in `#[cfg(test)]` modules (close to code they test)
- Integration tests in `tests/` directory (test public API)
- Benchmark tests in `benches/` directory

This follows Rust best practices.

**Q: Integration tests well organised?**

✅ **Yes:**
- `cli_integration_tests.rs`: CLI-specific tests (regex validation)
- `error_handling_tests.rs`: Error scenarios (nonexistent dirs, binary files)

Clear separation by test type.

**Q: Test utilities/helpers properly structured?**

✅ **Yes:**
- Benchmark fixtures in `benches/fixtures/` module
- Test helpers inline in test modules (appropriate for current scale)

**Recommendation:** Testing structure is excellent, no changes needed

---

## Recommendations Summary

### High Priority (Style Guide Compliance)

1. **Fix Function Ordering** (#54)
   - Move constructors before public methods in all impl blocks
   - Affects: `file_finder/mod.rs`, `searcher.rs`
   - Effort: Low (mechanical refactor)
   - Impact: Consistency, readability

### Medium Priority (API Ergonomics)

2. **Add Convenience Re-exports to `lib.rs`**
   - Re-export commonly-used types at crate root
   - Makes library API more ergonomic
   - Effort: Low (5 minutes)
   - Impact: Better user experience for library consumers

   ```rust
   // Add to lib.rs
   pub use file_finder::Finder;
   pub use searcher::{Searcher, ReSearcher, Searches, SearchResult};
   pub use output::{Outputs, SearchMatch, ColourMode, StandardOutput, JsonOutput, CountOutput, FilesOnlyOutput};
   ```

### Low Priority (Code Quality)

3. **Reduce Code Duplication in `file_finder`**
   - Extract shared metadata-handling logic from `unfiltered_find` and `filtered_find`
   - Effort: Low-Medium
   - Impact: Maintainability

4. **Consider Future `errors/` Module**
   - When migrating to anyhow (#52) or adding custom errors
   - Not needed now with simple `std::io::Error`
   - Effort: N/A (future work)
   - Impact: Better error handling

### Deferred (No Immediate Action)

5. **Monitor `file_finder` Module Growth**
   - Currently 137 lines, no action needed
   - If grows to 300+ lines, consider splitting
   - Watch for: additional filtering logic, caching, gitignore support

6. **Extract Configuration Module**
   - Only if configuration types grow beyond ColourMode
   - Future features might add: ignore patterns, limits, caching config
   - Not needed for current scope

---

## Conclusion

**Overall Assessment:** ✅ **Excellent structure for project size and complexity**

FindeRS demonstrates:
- Clear module boundaries
- Good separation of concerns
- Minimal public API surface
- Appropriate use of traits for abstraction
- Well-organized tests

**No major restructuring needed.** The recommended improvements are minor style and ergonomics fixes, not architectural changes.

**Next Steps:**
1. Fix function ordering in impl blocks (#54) - can be done with v3.0.2
2. Add convenience re-exports - can be done with v3.0.2
3. Reduce code duplication in file_finder - future enhancement
4. Continue monitoring as codebase grows

---

## Appendix: Comparison with Rust Project Standards

| Criterion | FindeRS | Rust Standard | Status |
|-----------|---------|---------------|--------|
| Module organization | Small, focused modules | ✅ Single responsibility | ✅ Excellent |
| Public API | Minimal, well-defined | ✅ Hide internals | ✅ Excellent |
| CLI vs Library separation | Clean separation | ✅ src/bin/ for CLI | ✅ Excellent |
| Naming conventions | Consistent snake/PascalCase | ✅ Follow guidelines | ✅ Excellent |
| Function ordering | Constructor first (mostly) | ✅ Constructor → public → private | ⚠️ Minor inconsistency |
| Test structure | Unit + integration | ✅ #[cfg(test)] + tests/ | ✅ Excellent |
| Documentation | Public APIs documented | ✅ /// comments | ⚠️ Some missing (#53) |

**Overall Grade: A-** (excellent structure, minor improvements needed)

---

**Author:** Claude Code  
**Review Date:** 2026-04-21  
**Next Review:** After major feature additions or when LOC exceeds 2000
