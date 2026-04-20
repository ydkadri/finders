# FindeRS Roadmap

This file organizes planned features and improvements into logical groups. Pick features from these groups based on priority and effort.

**Note:** Detailed implementation plans are created when starting work on a feature (following the 5-phase workflow in CLAUDE.md).

---

## 🚀 CI/CD & Release Improvements

**Goal:** Enhance development workflow, releases, and quality gates

### High Priority
- [ ] **#34 - Cross-platform Release Binaries** (medium-priority, ci/cd)
  - Build Linux, macOS, Windows binaries in release workflow
  - Attach to GitHub releases
  - Makes installation easier (no cargo needed)
  - **Related:** #35 (checksums/archives)

- [ ] **#35 - Checksums and Compressed Archives** (medium-priority, ci/cd)
  - Generate SHA256 checksums for release binaries
  - Create .tar.gz/.zip archives
  - Enhances security and distribution
  - **Pair with:** #34

### Medium Priority
- [ ] **#33 - Benchmark Regression Detection** (medium-priority, performance, ci/cd)
  - Automatically detect performance regressions in PRs
  - Integrate with existing benchmark infrastructure
  - Fail CI if significant slowdown detected

- [ ] **#29 - cargo-deny Integration** (medium-priority, ci/cd)
  - License compliance checking
  - Dependency policy enforcement
  - Security advisories (complement to cargo-audit)

### Low Priority
- [ ] **#36 - Automate Changelog Generation** (low-priority, ci/cd)
  - Generate CHANGELOG.md from commit messages
  - Integrate with release workflow

- [ ] **#32 - Multi-version Rust Testing** (low-priority, ci/cd)
  - Test against stable, beta, nightly
  - Ensures compatibility across Rust versions

---

## ⚡ Performance Optimizations

**Goal:** Make finder faster and more efficient

### High Priority
- [ ] **#18 - Optimize Case-Insensitive Search** (high-priority, performance)
  - Currently slow compared to case-sensitive
  - Profile and optimize hot path

- [ ] **#16 - Parallel File Processing** (high-priority, performance)
  - Use rayon for parallel search
  - Significant speedup on multi-core systems
  - Major architectural change

### Low Priority
- [ ] **#27 - Optimize Metadata Calls** (low-priority, performance)
  - Reduce filesystem metadata overhead
  - Profile and eliminate unnecessary calls

- [ ] **#26 - Remove Allocations from Hot Path** (low-priority, performance)
  - Eliminate allocations in search loop
  - Use stack buffers where possible

---

## 🔍 Search Features & UX

**Goal:** Add features that make finder more powerful and user-friendly

### High Priority
- [ ] **#17 - Colored Output** (high-priority, enhancement)
  - Highlight matches in output
  - Color file paths, line numbers
  - Major UX improvement

### Medium Priority
- [ ] **#22 - Multiple Output Modes** (medium-priority, enhancement)
  - JSON output for scripting
  - Count-only mode
  - Files-only mode (like grep -l)

- [ ] **#21 - Context Lines Support** (medium-priority, enhancement)
  - Add -A (after), -B (before), -C (context) flags
  - Like grep context lines
  - Helps understand match context

- [ ] **#20 - Exclude Flag** (medium-priority, enhancement)
  - --exclude flag for filtering out paths
  - Like grep --exclude
  - Common use case: exclude test files, vendor dirs

- [ ] **#19 - Glob Pattern Support** (medium-priority, enhancement)
  - Advanced file pattern matching
  - More flexible than simple wildcards

### Low Priority
- [ ] **#24 - Progress Indicator** (low-priority, enhancement)
  - Show progress for large directory searches
  - Prevents "is it frozen?" confusion

- [ ] **#23 - Respect .gitignore** (low-priority, enhancement)
  - Skip files ignored by git
  - Common use case in git repositories

- [ ] **#25 - Binary File Detection** (low-priority, enhancement)
  - Detect and skip binary files
  - Avoid garbled output

---

## 🛠️ Code Quality & Reliability

**Goal:** Improve code robustness, test coverage, and error handling

### High Priority
- [ ] **#15 - Graceful Regex Error Handling** (high-priority, bug)
  - Invalid regex patterns currently panic or error poorly
  - Provide helpful error messages

- [ ] **#14 - Replace unwrap() with Error Handling** (high-priority, bug)
  - Audit codebase for unwrap() calls
  - Replace with proper error handling
  - Improves stability

### Medium Priority
- [ ] **#42 - Improve Test Coverage to 80%+** (medium-priority, enhancement)
  - Current: ~75%
  - Target: 80%+
  - Focus on error paths, edge cases
  - Update CI threshold

- [ ] **#52 - Migrate to anyhow for Better Error Context** (medium-priority, enhancement)
  - Replace std::io::Error with anyhow::Result
  - Add .context() calls for better error messages
  - Improves user experience when errors occur

### Low Priority
- [ ] **#51 - Remove Underscore Prefixes from Private Methods** (low-priority, enhancement)
  - Clean up _sensitive_search, _insensitive_search, _find
  - Rust visibility system is sufficient
  - Minor style cleanup

- [ ] **#54 - Fix Function Ordering in Impl Blocks** (low-priority, enhancement)
  - Move constructors before public methods
  - Follow style guide conventions
  - Minor organizational improvement

- [ ] **#55 - API Design Review and Alignment** (medium-priority, enhancement)
  - Review flag design (verbose, skip-warnings consistency)
  - Error handling strategy (Result vs skip/warn)
  - Finder and search_files API consistency
  - Ensure public API is minimal and clear

---

## 📚 Documentation

**Goal:** Improve documentation and make it more accessible

### Medium Priority
- [ ] **#63 - GitHub Pages Documentation** (medium-priority, documentation)
  - Host user-facing documentation on GitHub Pages
  - Better formatting than README alone
  - Include: installation guide, usage examples, CLI reference, color examples
  - Searchable documentation with syntax highlighting
  - Makes onboarding easier for new users

### Low Priority
- [ ] **#53 - Add Documentation for All Public APIs** (medium-priority, documentation)
  - Document all public traits, structs, functions
  - Include usage examples
  - Improves crates.io documentation quality
  - (Moved from Code Quality section)

---

## 📊 Feature Selection Guide

### Quick Wins (High Impact, Low Effort)
1. **#14 + #15 - Error Handling Bugs** - Critical stability fixes, clear scope
2. **#17 - Colored Output** - Major UX improvement, straightforward implementation
3. **#35 - Checksums/Archives** - Completes release workflow, low effort
4. **#51 - Method Naming Cleanup** - Quick style fix to include with #14/#15

### Complete Stories (Grouped Features)
- **Release Workflow:** #34 + #35 (cross-platform binaries + checksums)
- **Advanced Search:** #19 + #20 + #21 (glob + exclude + context)
- **Performance Suite:** #16 + #18 + #27 + #26 (parallel + optimizations)

### Foundation Work (Enables Future Features)
- **#16 - Parallel Processing** - Architectural foundation for performance
- **#14 - Error Handling** - Enables better user experience
- **#42 - Test Coverage** - Enables confident refactoring

---

## 🎯 Recommended Next Steps

Based on impact, effort, and current momentum:

### Option A: Complete Release Workflow
**Features:** #34 (binaries) + #35 (checksums)  
**Impact:** High - Makes installation easier for users  
**Effort:** Low-Medium - Builds on existing release workflow  
**Timeline:** 1-2 sessions

### Option B: Performance Improvements
**Features:** #18 (case-insensitive) + #27 (metadata)  
**Impact:** Medium - Measurable speed improvements  
**Effort:** Medium - Requires profiling and optimization  
**Timeline:** 2-3 sessions

### Option C: UX Enhancements
**Features:** #17 (colors) + #22 (output modes)  
**Impact:** High - Visible user experience improvements  
**Effort:** Low-Medium - Straightforward implementations  
**Timeline:** 1-2 sessions

### Option D: Code Quality Foundation
**Features:** #14 (unwrap removal) + #15 (regex errors) + #51 (method naming)  
**Impact:** High - Fixes critical bugs, improves stability  
**Effort:** Low-Medium - Well-defined fixes  
**Timeline:** 1 session  
**Next:** Follow with #42 (coverage), #52 (anyhow), #53 (docs) in subsequent releases

---

## 📋 v3.0.2 Error Handling Implementation Plan

**Release Type:** PATCH (bug fixes + minor code cleanup)  
**Status:** Ready to implement after project structure review and cross-platform binaries  
**PR Strategy:** Single PR with clean commit structure

### Scope
1. **#14 - Replace unwrap() with Error Handling**
   - 5 locations identified: searcher.rs:77, file_finder/mod.rs:27,43,44, lib.rs:60
   - Use `?` operator with proper Result types
   - Provide clear error messages

2. **#15 - Graceful Regex Error Handling**
   - Change `ReSearcher::new()` to return `Result<ReSearcher, regex::Error>`
   - Update CLI to display user-friendly errors for invalid patterns
   - No panic on bad regex input

3. **#51 - Remove Underscore Prefixes from Private Methods**
   - searcher.rs: `_sensitive_search` → `sensitive_search`
   - searcher.rs: `_insensitive_search` → `insensitive_search`
   - file_finder/mod.rs: `_find` → `find_internal`

### Implementation Approach

**Error Handling Strategy:**
- Keep `std::io::Error` for now (anyhow migration deferred to #52)
- Return `Result` types at error boundaries
- Propagate errors with `?` operator
- Clear error messages for user-facing errors

**Specific Solutions:**
1. **file_finder unwrap() calls (lines 27, 43, 44)**:
   - Use `filter_map` to gracefully skip problematic files
   - Always report skipped files to stderr (no verbose flag needed)
   - Skip files with: metadata errors (permissions), non-UTF8 names
   
2. **lib.rs path display (line 60)**:
   - Use `to_string_lossy()` instead of `unwrap()`
   - Displays � for invalid UTF-8 (matches grep/rg behavior)
   
3. **searcher.rs regex unwrap (line 77)**:
   - Change `ReSearcher::new()` to return `Result<ReSearcher, regex::Error>`
   - Update CLI to catch and display user-friendly error message
   - Example: "Invalid regex pattern: unclosed group"

4. **Private method naming (3 locations)**:
   - `_sensitive_search` → `sensitive_search`
   - `_insensitive_search` → `insensitive_search`
   - `_find` → `find_internal`

**Warning Message Format:**
```
Warning: Cannot read metadata: /path/to/file (Permission denied)
Warning: Skipping file with non-UTF8 name
```

**Commit Structure:**
1. Fix unwrap() calls in file_finder (3 locations, use filter_map with warnings)
2. Fix unwrap() call in lib.rs (use to_string_lossy)
3. Fix regex error handling in searcher.rs + update CLI error display
4. Remove underscore prefixes from private methods
5. Add tests for error cases (invalid regex, file access errors)
6. Bump version: 3.0.1 → 3.0.2

**Testing:**
- All existing tests must pass
- Add tests for error cases (invalid regex, file access errors)
- Run full test suite + benchmarks before release

### Post-Release Work
After v3.0.2 ships, continue with:
- #42 - Improve test coverage to 80%+ (stays on v3.0.2, no version bump)
- #52 - Migrate to anyhow (future minor enhancement)
- #53 - Add API documentation (#7 task - future enhancement)
- #54 - Fix function ordering (future enhancement)
- Performance optimizations: #18, #27 (PATCH bumps), #16 (MINOR bump)

---

## 📋 Cross-Platform Release Workflow - Implementation Plan

**Scope:** #34 (cross-platform binaries) + #35 (checksums/archives)  
**Status:** Ready to implement  
**Version Bump:** None (workflow-only changes)  
**PR Strategy:** Single PR with workflow modifications

### Scope

1. **#34 - Cross-platform Release Binaries**
   - Build for 4 platforms: Linux x64, macOS Intel, macOS Apple Silicon, Windows x64
   - Use native GitHub runners (no cross-compilation)
   - Test on each platform before building release binary

2. **#35 - Checksums and Compressed Archives**
   - Create tar.gz archives (Linux/macOS) and zip archives (Windows)
   - Generate SHA256 checksums for each archive
   - Naming: `finder-v{version}-{arch}-{os}.{tar.gz|zip}`

### Technical Approach

**Build Matrix Strategy:**
```yaml
build-binaries:
  strategy:
    matrix:
      include:
        - os: ubuntu-latest, target: x86_64-unknown-linux-gnu, archive: tar.gz
        - os: macos-13, target: x86_64-apple-darwin, archive: tar.gz
        - os: macos-14, target: aarch64-apple-darwin, archive: tar.gz
        - os: windows-latest, target: x86_64-pc-windows-msvc, archive: zip
```

**Workflow Structure:**
1. `build-binaries` job (matrix):
   - Run tests on platform
   - Build release binary
   - Upload binary as artifact
   
2. `publish-release` job (single runner):
   - Download all platform binaries
   - Create archives (tar.gz/zip) with proper naming
   - Generate SHA256 checksums
   - Create GitHub release with all archives + checksums
   
3. `publish-crate` job (unchanged):
   - Publish to crates.io after release created

**Artifact Naming:**
- `finder-v2.1.2-x86_64-linux.tar.gz` + `.sha256`
- `finder-v2.1.2-x86_64-macos.tar.gz` + `.sha256`
- `finder-v2.1.2-aarch64-macos.tar.gz` + `.sha256`
- `finder-v2.1.2-x86_64-windows.zip` + `.sha256`

**Archive Contents:**
- Binary only (finder or finder.exe)
- Preserve executable permissions on Unix platforms

### Commit Structure

Single commit (workflow-only change):
1. Add cross-platform builds and checksums to release workflow

### Review Milestones

**After commit:** Validate workflow structure before testing
- Review matrix configuration
- Check archive/checksum generation logic
- Verify integration with existing workflow

**After merge:** Test with next release
- Will be tested naturally with v2.2.0 (Option C features)
- Can also test manually with workflow_dispatch if needed

### Testing Strategy

**Pre-merge:**
- Review workflow YAML syntax
- Verify matrix configuration
- Check artifact paths and naming

**Post-merge:**
- Trigger workflow manually with existing tag (workflow_dispatch)
- Verify all 4 platform binaries build successfully
- Check archive/checksum generation
- Test downloading and extracting archives

---

## 📋 v3.0.0 UX Enhancements - Implementation Plan

**Scope:** #17 (Colored Output) + #22 (Multiple Output Modes)  
**Status:** ✅ COMPLETED AND SHIPPED  
**Version Bump:** MAJOR (2.1.2 → 3.0.0) - Breaking Changes  
**PR Strategy:** Single PR with multiple logical commits

### Scope

1. **#17 - Colored Output**
   - Add `--colour`/`--no-colour` flags (UK spelling)
   - Auto-detect TTY for default behavior
   - Respect NO_COLOR, CLICOLOR, and CLICOLOR_FORCE environment variables
   - Color scheme: Green paths, Cyan line numbers, White-on-Magenta matches

2. **#22 - Multiple Output Modes**
   - `-l, --files-with-matches` - Output only file paths with matches
   - `-c, --count` - Output match count per file
   - `--json` - Structured JSON output for tooling
   - Mutual exclusivity validation

3. **Output Format Standardization**
   - Change from custom aligned format to standard `path:line: content`
   - Remove quotes from file-only output
   - Matches grep/ripgrep conventions

### Technical Approach

**Dependencies:**
- `termcolor` - Cross-platform colored terminal output
- `atty` - TTY detection for auto-color mode  
- `serde` + `serde_json` - JSON serialization

**Architecture Changes:**
1. **Output abstraction layer**
   - Create `output` module with trait-based design
   - Implement: `StandardOutput`, `FilesOnlyOutput`, `CountOutput`, `JsonOutput`
   - Centralize color application logic

2. **CLI flag handling**
   - Add color and output mode flags to clap
   - Add validation for mutual exclusivity
   - Environment variable checking (NO_COLOR, CLICOLOR, CLICOLOR_FORCE)

3. **Color application**
   - Use termcolor's `StandardStream` for cross-platform support
   - Wrap match highlighting in color codes
   - Auto-detect TTY vs pipe/redirect

**Output Format Changes:**
```rust
// OLD: Custom aligned format
println!("{:>4}: {:<56} {}", rownum, path, line);

// NEW: Standard format with colors
println!("{}:{}:{}", green(path), cyan(rownum), highlight_matches(line));
```

### Commit Structure

Logical commits for incremental review:

1. **Add output abstraction and refactor existing output**
   - Create `src/output.rs` module
   - Define `Output` trait
   - Implement `StandardOutput` (replaces current println!)
   - Update lib.rs to use trait
   - Tests pass, behavior unchanged

2. **Add color support to standard output**
   - Add termcolor and atty dependencies
   - Implement color application in StandardOutput
   - Add `--colour`/`--no-colour` flags
   - Add TTY detection and environment variable support
   - Color scheme: green path, cyan line number, white-on-magenta matches

3. **Standardize output format (path:line: content)**
   - Change from aligned format to standard format
   - Remove quotes from file-only paths
   - Update tests for new format
   - Breaking change, but more standard

4. **Add files-with-matches mode (-l)**
   - Implement `FilesOnlyOutput`
   - Add `-l, --files-with-matches` flag
   - Wire up to output system
   - Add tests

5. **Add count mode (-c)**
   - Implement `CountOutput`  
   - Add `-c, --count` flag
   - Track and output match counts per file
   - Add tests

6. **Add JSON output mode (--json)**
   - Add serde dependencies
   - Implement `JsonOutput`
   - Add `--json` flag
   - Serialize results to JSON array
   - Add tests

7. **Add output mode mutual exclusivity validation**
   - Add clap validation for conflicting flags
   - Error messages for invalid combinations
   - Add tests for error cases

8. **Update documentation and examples**
   - Create ADR for architectural decisions (output abstraction, color library choice, breaking changes)
   - Update CHANGELOG with prominent BREAKING CHANGES section:
     * Output format change documented with before/after examples
     * Migration guide for users with scripts
     * Justification for v3.0.0 MAJOR bump
     * Note that --json provides stable output format
   - Update README with color examples and v3.0.0 changes
   - Add usage examples for new modes

9. **Bump version: 2.1.2 → 3.0.0**

### Review Milestones

**After Commit 1:** Foundation review
- Validate output abstraction design
- Check that current behavior unchanged
- Ensure tests pass

**After Commit 2:** Color implementation review
- Test color output visually
- Verify TTY detection works
- Check environment variable support

**After Commit 3:** Format change review  
- Validate new format matches grep/ripgrep
- Ensure breaking change is acceptable
- Update any affected tests

**After Commits 4-6:** Feature completeness review
- Test all output modes
- Verify mutual exclusivity
- Check JSON format validity

**Final:** Ready for merge
- All tests passing
- Documentation updated
- Version bumped
- Clean commit history

### Testing Strategy

**Unit tests:**
- Output trait implementations
- Color code generation
- TTY detection logic
- JSON serialization

**Integration tests:**
- Each output mode with real files
- Color flag combinations
- Environment variable behavior
- Mutual exclusivity validation

**Manual testing:**
- Visual inspection of colors on different terminals
- Pipe to file (no colors)
- Pipe to less -R (with --colour)
- JSON parsing with jq

### Breaking Changes (Why MAJOR 3.0.0)

⚠️ **Output format change will break scripts that parse finder output**

**Changes:**
- OLD: `   4: /path/to/file.rs                                      line content`
- NEW: `/path/to/file.rs:4: line content`
- File-only mode: Remove quotes from paths

**Impact:**
- Scripts parsing line numbers will break
- Fixed-width padding assumptions will fail
- Path extraction logic needs updating

**Justification:**
- New format is industry standard (grep/ripgrep)
- Easier to parse programmatically (consistent structure)
- Better terminal width handling (no fixed padding)
- More maintainable long-term

**Communication (CRITICAL):**
- Prominently document in CHANGELOG under "Breaking Changes" section
- Highlight in GitHub release notes with migration guide
- Update PR description with breaking change warning
- Consider release announcement noting v3.0.0 requires script updates

---

## 🎯 Current Priority (Active Work)

### Phase 1: Project Structure Review (#75)
**Status:** Next up  
**Version Bump:** None (documentation/structure only)  
**Effort:** Low  
**Impact:** Foundation for future work

Review and document:
- Current project structure and organization
- Module boundaries and responsibilities
- Opportunities for improvement
- Align with Rust best practices

### Phase 2: Cross-Platform Release Workflow (#34 + #35)
**Status:** Plan complete, ready to implement  
**Version Bump:** None (workflow-only changes)  
**Effort:** Low-Medium  
**Impact:** High - Makes installation easier for users

Build binaries for all platforms, generate checksums, create archives.

### Phase 3: Error Handling Improvements (v3.0.2)
**Status:** Plan complete, ready after Phase 1-2  
**Version Bump:** PATCH (3.0.1 → 3.0.2)  
**Effort:** Low-Medium  
**Impact:** High - Stability and reliability improvements

Includes: #14 (unwrap removal), #15 (regex errors), #51 (method naming)

### Future: Performance Optimizations
**Version Bump:** MINOR or PATCH depending on scope
- #18 (case-insensitive optimization) - PATCH
- #27 (metadata optimization) - PATCH
- #16 (parallel processing) - MINOR (new capability)

---

**Last Updated:** 2026-04-21  
**Current Version:** 3.0.1
