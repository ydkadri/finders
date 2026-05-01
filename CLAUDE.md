# Claude Code Instructions for FindeRS

This file contains project-specific instructions for Claude Code when working with this repository. General workflow preferences are in Claude's MEMORY.md and apply unless overridden here.

## Table of Contents
- [Philosophy](#philosophy) ⭐
- [Project Context](#project-context)
- [CRITICAL Rules](#critical-rules) 🚨
- [Development Workflow](#development-workflow)
- [Git Workflow](#git-workflow)
- [Versioning](#versioning)
- [Quick Reference](#quick-reference)
- [Code Standards Reference](#code-standards-reference) 📚

---

## Philosophy

These principles guide how we work together:

**Experiment and iterate**: Try things, see what works, throw away what doesn't quickly. "If it doesn't agree with experiment, it's wrong." - Feynman

**Question everything**: Always question me. Push back if something seems wrong. A few more questions leading to a better solution is preferred over rushing to the wrong implementation.

**Context is king**: Provide enough context to understand and debug. Error messages, logs, documentation - always include relevant context.

**Simple is better than complex**: Choose simple solutions over complex ones. Don't over-engineer.

**User interface and user outcomes are paramount**: Everything else can be changed later, but getting the user experience right is critical.

---

## Project Context

- **Type**: Personal Project - CLI Tool
- **Purpose**: Simplified alternative to find+grep for searching files by name and content
- **Language**: Rust
- **Package Manager**: cargo
- **Task Runner**: justfile (to be added)
- **Testing**: cargo test, criterion (benchmarks)
- **Code Quality**: rustfmt, clippy
- **CI/CD**: GitHub Actions (personal project)
- **Distribution**: crates.io, GitHub releases with binaries
- **Spelling**: British English (colour, behaviour, optimise, etc.) for all documentation, comments, and user-facing text. Preserve US spelling only in external crate names, environment variable standards, and code identifiers where required by convention.

---

## CRITICAL Rules 🚨

**These rules override any defaults and must be followed exactly:**

### Question-Asking Protocol (MOST IMPORTANT)

Before implementing features, ask questions **one at a time**:

- **Ask ONE question** - Allow user to focus and give detailed answers
- **Provide context** - Explain why the question matters
- **Offer suggestions** - Include your recommendation
- **Number questions** - Track progress through decision-making

Examples of what to ask about:
- User journey: What is the user trying to accomplish?
- User experience: How should the interface look/feel?
- Design decisions: Data structures, APIs, algorithms
- Error handling strategies
- Testing approach
- Performance trade-offs

**Why this is critical**: Asking good questions one at a time leads to better design decisions and saves significant rework time.

### Validate Changes Against Rust Style Guide (BLOCKING)

**All code changes must comply with Rust best practices:**

Reference: `~/Documents/ydkadri/claude/languages/rust.md`

Key requirements:
- Naming: structs/enums `PascalCase`, functions/vars `snake_case`, traits verb-based
- Error handling: use `Result` and `Option`, thiserror for custom errors, anyhow for apps
- Testing: unit tests in modules, integration tests in `tests/`, benchmarks in `benches/`
- Documentation: all public APIs documented with `///` comments
- Code quality: `cargo fmt`, `cargo clippy -- -D warnings`, `cargo test` all pass

**Before push:** Run quality checks to catch issues automatically.

### Architecture Decision Records (Encouraged)

**For significant architectural decisions, consider creating an ADR:**

**When to write an ADR (encouraged but not required):**
- Major dependency additions (new crates, frameworks)
- Breaking API changes or CLI interface changes
- Error handling strategy changes
- Performance architecture decisions (parallelism, caching, algorithms)
- File formats, protocols, or data structures
- Tool selection (benchmarking, CI platforms, etc.)

**Process:**
1. Copy `docs/adr/template.md` to `docs/adr/NNNN-title.md`
2. Fill in context, decision, consequences, alternatives
3. Include ADR in the same PR as implementation
4. Update `docs/adr/README.md` with link to new ADR

**Location**: `docs/adr/`  
**Template**: `docs/adr/template.md`  
**Guide**: `docs/adr/README.md`

ADRs help future maintainers (including you!) understand why decisions were made.

---

## Development Workflow

### Feature Implementation Workflow (CRITICAL)

**Goal: Small increments, fast feedback, fewer review rounds.**

Work happens in phases with explicit checkpoints for early alignment.

---

#### Phase 1: Align on Approach

1. Discuss user journey with questions (one at a time)
2. Write user journey document if complex (`docs/user-journeys/NN-feature-name.md`)
   - For simple features, discussion may be sufficient
   - For complex features, document user goals, workflow, and outcomes

**CHECKPOINT 1**: Push draft PR with user journey doc (if created)
- **Request review**: "User journey complete - validating we're solving the right problem"
- User validates: Is this the right problem to solve?

---

#### Phase 2: Design Interface

3. Document interface changes if adding public APIs or CLI commands
   - For CLI changes: document new flags, commands, output formats
   - For library changes: document public function signatures and usage

**CHECKPOINT 2**: Push interface docs to same PR (if significant changes)
- **Request review**: "Interface design complete - validating API ergonomics before implementation"
- User validates: Is the interface clear and well-designed?

---

#### Phase 3: Plan Implementation

4. Create implementation plan in ROADMAP.md (for significant features):
   ```markdown
   ## [Feature Name] - Implementation Plan
   
   **PR Strategy**: Single PR | Multiple smaller PRs
   
   **Commit Structure**:
   1. [Self-contained unit 1] - what and why
   2. [Self-contained unit 2] - what and why
   ...
   
   **Review Milestones**:
   - After Commit X: Why review here? (e.g., "Validate foundation")
   - After Commit Y: Why review here? (e.g., "Before building on this")
   - Final: Ready for merge after version bump
   
   **Technical Approach**:
   - Key architectural decisions
   - Design patterns used
   - Integration points
   - Performance considerations
   ```

**CHECKPOINT 3**: Push plan to ROADMAP.md
- **Request review**: "Implementation plan complete - agreeing on commit structure and milestones"
- User validates: Agree on granularity, PR strategy, and review points?

---

#### Phase 4: Implement Incrementally

5. Implement according to plan:
   - Write tests first for each unit
   - Implement the functionality
   - Keep commits matching the plan structure
   - **Keep fixup commits during draft phase** - makes incremental review easier

**Push at planned milestones**:
- After completing each milestone from plan
- **Always include context**: "Milestone X complete: [what] - ready for review to [why]"
- Example: "Core search optimization complete - ready for review to validate before adding features on top"

**When to push for milestone review:**

✅ Completed a planned commit/unit
✅ Foundation work that later work builds on
✅ Complete feature slice working end-to-end
✅ Before a major direction change needs validation
✅ After significant refactor affecting many files

❌ Not after every single commit (too granular)
❌ Not when stuck on implementation detail (try to solve first)

**PR stays in DRAFT** - Allows fixup commits without breaking review flow

---

#### Phase 5: Finalize

6. Self-validate before asking for final review:
   - Run `cargo fmt` and ensure formatting is correct
   - Run `cargo clippy -- -D warnings` and fix all issues
   - Run `cargo test` and verify all tests pass
   - Run `cargo bench` to ensure no performance regressions
   - Check all changes against Rust style guide
   
7. Update documentation:
   - CHANGELOG.md with user-facing changes
   - README.md if features or commands changed
   - Review existing docs for accuracy
   - Update inline documentation (/// comments)

8. Version bump:
   - Propose version type (patch/minor/major) and get confirmation
   - Manually update version in `Cargo.toml`
   - Update "Current Version" in this file
   - **Note:** Do NOT create git tags yet - tags are created manually after merge (see Versioning section)

9. **Rebase to clean commit history**:
   - Squash fixup commits into their parent commits
   - Ensure each commit is self-contained and logical
   - Verify all tests pass after rebase: `cargo test`

10. **Mark PR ready for final review**
    - **Request review**: "Ready for final review - all feedback addressed, tests passing, docs updated"
    - Wait for CI to pass (use `gh pr view <number> --json statusCheckRollup`)

---

**Key Principles:**

- **3 upfront checkpoints** catch issues when they're cheap to fix
- **Milestone reviews during implementation** prevent building on wrong foundation  
- **Draft PR + fixup commits** make incremental review easier
- **Clean history at the end** via rebase before marking ready
- **Explicit review requests** with context help reviewer understand what and why

---

## Git Workflow

**This section guides how we work together on commits and PRs.**

### Branch Strategy

Branches should be descriptive and categorical:

- **`feature/description`** - New features or enhancements
- **`fix/description`** - Bug fixes
- **`patch/description`** - Small patches, typos, minor corrections
- **`docs/description`** - Documentation-only changes
- **`perf/description`** - Performance optimizations

Use descriptive names that explain what the branch does, not just ticket numbers.

**NEVER push directly to main** - ALL changes go through PRs.

### Commit History

**Goal: Clean, logical commits that tell a story.**

**During draft PR phase:**
- Fixup commits are ENCOURAGED - makes incremental review easier
- "Fix linting", "Address feedback", "Fix typo" commits are fine
- Reviewer can see what changed since last review without re-reading everything

**Before marking PR ready (Phase 5):**
- Rebase to squash fixups into logical feature units
- Each commit = complete, cohesive piece of functionality
- Related changes grouped together (feature + tests + docs)
- Commits tell a clear story
- Reviewers can understand each commit in isolation

**Final commit structure example:**
```
✅ GOOD - After rebase, logical units:
1. Add parallel file processing with rayon
2. Add colored output support
3. Add glob pattern support with tests
4. Update CLI and documentation
5. Bump version: 2.1.1 → 2.2.0

✅ ALSO GOOD - During draft, incremental changes visible:
1. Add parallel file processing
2. Fix clippy warnings
3. Address feedback: simplify thread pool
4. Add colored output
5. Fix benchmarks
...
[Then rebase before marking ready]
```

### Handling Review Feedback

**At milestones during draft PR:**
- Add fixup commits addressing feedback
- Push with context: "Addressed feedback on milestone X: [what changed]"
- Keeps incremental changes visible for next review

**Before marking ready (Phase 5):**
- Rebase to incorporate all feedback into logical commits
- Use `git rebase -i` to squash fixups
- Verify tests pass after rebase
- Push cleaned history

### Commit Messages

Keep commit messages clear and informative:

- First line: Brief summary (under 70 characters)
- Blank line
- Detailed explanation if needed (why this change, not what changed)
- Reference issues/PRs if relevant

```
Add parallel file processing with rayon

Replace sequential file processing with parallel implementation
using rayon's thread pool. Provides 3-5x speedup on multi-core
systems with no change to CLI interface.

Benchmarks show improvement from 2.1s to 450ms on test corpus
of 10k files.

Fixes #42
```

### Git Safety Protocol

- NEVER update the git config
- NEVER run destructive git commands (push --force, reset --hard, checkout ., restore ., clean -f, branch -D) unless the user explicitly requests these actions
  - `git push --force-with-lease` is acceptable and preferred over `--force`
- NEVER skip hooks (--no-verify, --no-gpg-sign, etc) unless the user explicitly requests it
- NEVER run force push to main/master, warn the user if they request it
- CRITICAL: Always create NEW commits rather than amending, unless the user explicitly requests a git amend
- When staging files, prefer adding specific files by name rather than using "git add -A" or "git add ."

---

## Versioning

### Semantic Versioning

- **Patch** (2.1.0 → 2.1.1): Bug fixes, documentation, minor improvements
- **Minor** (2.1.0 → 2.2.0): New features, enhancements, new capabilities
- **Major** (2.0.0 → 3.0.0): Breaking changes, incompatible API changes

### Version Management

- Manually update `version` in `Cargo.toml`
- Update CHANGELOG.md ([Unreleased] → versioned section with date)
- Update "Current Version" field at bottom of this file
- Create git commit: "Release vX.Y.Z - [description]"

**IMPORTANT: Tags are created manually AFTER merge to main**
- Do NOT create tags locally before merge (prevents conflicts during PR rebases)
- Do NOT push tags in version bump commits
- Tags trigger the release workflow automatically

### Release Process

**Step 1: Merge version bump PR to main**
- CI runs tests and coverage on main branch
- No auto-tagging (we create tags manually)

**Step 2: Create and push tag manually**
```bash
git checkout main && git pull
git tag -a vX.Y.Z -m "Release vX.Y.Z"
git push origin vX.Y.Z
```

**Step 3: Automated release workflow (triggered by tag push)**
1. Tag push triggers `release.yml` workflow
2. Build release binaries (Linux, macOS, Windows)
3. Publish to crates.io
4. Create GitHub release with binaries and notes
5. Benchmark workflow runs and creates PR with updated results

### Hotfixes for Old Releases

If you need to patch a previously released version (e.g., critical bug in production while main has moved ahead):

**Step 1: Create hotfix branch from old tag**
```bash
git fetch --tags
git checkout v3.0.0              # Checkout the old release tag
git checkout -b release/v3.0.1   # Create hotfix branch
```

**Step 2: Make fixes and commit**
```bash
# Make your fixes
git add .
git commit -m "Fix critical security issue in v3.0.x"

# Update version in Cargo.toml and CHANGELOG
# Commit: "Release v3.0.1 - Security hotfix"
```

**Step 3: Push branch and tag**
```bash
git push -u origin release/v3.0.1
git tag -a v3.0.1 -m "Release v3.0.1"
git push origin v3.0.1            # Triggers release workflow
```

**Step 4: Consider backporting to main**
```bash
git checkout main
git cherry-pick <commit-hash>     # Cherry-pick the fix if applicable
```

**Notes:**
- Tags point to commits, not branches - release workflow works regardless of branch
- Hotfix branches are kept for historical reference
- Always tag the release branch commit, not main
- Consider if the fix should also be applied to main

---

## Quick Reference

### Common Commands

```bash
# Development
cargo build              # Build in debug mode
cargo build --release    # Build optimized release
cargo run -- [args]      # Run with arguments

# Testing & Quality
cargo test               # Run all tests
cargo test --lib         # Run library tests only
cargo bench              # Run internal benchmarks
cargo bench --bench comparison_benchmarks  # Run comparison benchmarks (requires ripgrep)
cargo fmt                # Format code
cargo clippy -- -D warnings  # Lint with warnings as errors

# Documentation
cargo doc --open         # Generate and open docs

# Publishing
cargo publish --dry-run  # Test publishing
cargo publish            # Publish to crates.io
```

### Pre-Push Checklist

Before pushing code or marking PR ready:

- [ ] Code formatted: `cargo fmt`
- [ ] No clippy warnings: `cargo clippy -- -D warnings`
- [ ] All tests pass: `cargo test`
- [ ] Benchmarks run: `cargo bench` (for performance changes)
- [ ] Documentation updated (README, CHANGELOG, inline docs)
- [ ] Commit history clean (if marking ready)

### Feature Implementation Quick Reference

**Phases with checkpoints:**
1. **Align** → User journey doc (if complex) → Request review
2. **Design** → Interface docs (if significant) → Request review  
3. **Plan** → Implementation plan in ROADMAP.md → Request review
4. **Implement** → Code according to plan → Request review at milestones
5. **Finalize** → Self-validate, update docs, bump version, rebase → Mark ready

**Before each push:**
- Run quality checks: `cargo fmt && cargo clippy && cargo test`
- Include context: what milestone and why reviewing

**Before marking ready:**
- All tests passing (`cargo test`)
- All benchmarks stable (`cargo bench`)
- All docs updated (CHANGELOG, README)
- Clean commit history (rebase/squash fixups)
- Version bumped

---

## Code Standards Reference 📚

**For detailed Rust standards, refer to:**

`~/Documents/ydkadri/claude/languages/rust.md`

### Key Standards

**Naming:**
- Structs/Enums: `PascalCase` nouns (e.g., `DataParser`, `ConnectionState`)
- Traits: `PascalCase` 3rd person present tense verbs (e.g., `Parses`, `Searches`)
- Functions/variables: `snake_case` describing their outcome
- Constants: `SCREAMING_SNAKE_CASE`

**Function Ordering:**
- Define functions before they are called (read top-to-bottom)
- In impl blocks: private methods first, then constructors, then public methods

**Error Handling:**
- Use `Result` and `Option` appropriately
- Use `thiserror` for library custom errors
- Use `anyhow` for application error handling with context
- Avoid `unwrap()` and `expect()` except in tests

**Testing:**
- Unit tests in `#[cfg(test)]` modules
- Integration tests in `tests/` directory
- Benchmarks in `benches/` directory with criterion
- Use table-driven tests for multiple similar cases
- Test happy paths, edge cases, and error conditions

**Documentation:**
- All public APIs documented with `///` comments
- Include examples in documentation
- Document errors with `# Errors` section
- Document safety with `# Safety` section for unsafe code

**Logging:**
- Use `tracing` for structured logging
- Log levels: TRACE (detailed), DEBUG (flow), INFO (milestones), WARN (unexpected), ERROR (failures)
- Never log sensitive data (passwords, tokens, PII)

**Performance:**
- Provide benchmarks for performance-critical code
- Use `criterion` for benchmarking
- Avoid unnecessary allocations
- Use appropriate data structures

### Important Notes

- The Rust style guide provides detailed explanations and examples
- This CLAUDE.md file contains project-specific workflow instructions
- **When uncertain**: Check the style guide first, then ask user for validation if still ambiguous
- CRITICAL rules in this file (Question-Asking Protocol, Git workflow) always take precedence

---

**Last Updated**: 2026-05-01  
**Current Version**: 3.2.0
