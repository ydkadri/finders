---
name: Comparison Benchmark Architecture
description: Design decisions for benchmarking finder against find+grep and ripgrep
type: architecture
status: Accepted
date: 2026-04-04
---

# ADR-0001: Comparison Benchmark Architecture

## Status

**Accepted** — Implemented in PR #45

## Context

FindeRS provides functionality similar to `find` + `grep` pipelines and `ripgrep`. To understand competitive positioning and track performance improvements over time, we need systematic comparison benchmarks.

### Requirements

1. **Fair comparison**: Equivalent commands across all tools
2. **Representative workloads**: Realistic file structures and search patterns
3. **Historical tracking**: Performance trends across releases
4. **Automation**: Minimal manual intervention
5. **Visibility**: Easy access to results
6. **Low overhead**: Reasonable CI resource usage

### Alternatives Considered

#### Timing: When to Run Benchmarks?

**Option A: Weekly scheduled runs**
- Pro: Catches regressions between releases
- Pro: Regular performance visibility
- Con: CI overhead adds up (52 runs/year)
- Con: Performance tied to arbitrary dates, not versions
- Con: May detect regressions in unreleased code

**Option B: On every PR**
- Pro: Immediate feedback on performance impact
- Con: Very high CI overhead (every PR)
- Con: Noisy signal (may vary by runner load)
- Con: Slows PR feedback loop significantly

**Option C: Release-based + manual dispatch** ✅ **Selected**
- Pro: Performance metrics tied to specific versions
- Pro: Minimal CI overhead (releases are infrequent)
- Pro: Still allows ad-hoc testing via manual trigger
- Pro: Users care about "v2.1.1 performance" not "March 15 performance"
- Con: Won't catch regressions until release

#### Results Publishing: Where to Store Results?

**Option A: Commit JSON to main branch**
- Pro: Results versioned with code
- Con: Clutters git history with generated data
- Con: Binary/large files bloat repository
- Con: Results not easily browsable

**Option B: GitHub Pages from /docs in main** 
- Pro: Results versioned with code
- Pro: GitHub Pages serves automatically
- Con: Still clutters main branch with generated HTML
- Con: Mixing code and deployment artifacts

**Option C: GitHub Pages from separate gh-pages branch** ✅ **Selected**
- Pro: Clean separation (main = code, gh-pages = results)
- Pro: Standard practice for documentation sites
- Pro: Easy to regenerate/nuke if needed
- Pro: No code in deployment branch
- Con: Separate branch to manage

**Option D: External service (like Codecov for benchmarks)**
- Pro: No repository maintenance
- Con: External dependency
- Con: May have costs
- Con: Less control over presentation

#### Result Updates: How to Update README?

**Option A: Auto-create PR with README changes**
- Pro: Benchmark results visible in README
- Con: Creates noise (PR for every benchmark run)
- Con: Merge conflicts if multiple releases
- Con: README contains stale data between runs

**Option B: README links to GitHub Pages only** ✅ **Selected**
- Pro: README stays stable (no auto-updates)
- Pro: GitHub Pages is always current
- Pro: No PR overhead
- Pro: Detailed results live on proper documentation site
- Con: Users must click through to see numbers

## Decision

### Benchmark Triggers

Run comparison benchmarks on:
1. **Release tags** (`v*` pattern): Automatic benchmark when new version released
2. **Manual dispatch**: Allow ad-hoc testing via GitHub Actions UI

### Test Scenarios

6 benchmark scenarios covering realistic usage:
- **Repository sizes**: small (~100 files), medium (~1K files), large (~10K files)
- **Search patterns**: 
  - Common: "function" keyword (appears in ~50% of files)
  - Rare: unique marker (appears in 1 file)

Rationale:
- Small repos: Common in focused tools/libraries
- Medium repos: Typical application codebases
- Large repos: Monorepos and large frameworks
- Common pattern: Most searches match many files
- Rare pattern: Tests worst-case (full scan, minimal matches)

### Tools Compared

1. **find + grep**: Traditional Unix pipeline baseline
2. **ripgrep**: Modern fast alternative (industry standard)
3. **finder**: This project

Equivalent commands:
```bash
# find + grep
find <dir> -type f -name "*.rs" -exec grep -l <pattern> {} \;

# ripgrep
rg -l --glob "*.rs" <pattern> <dir>

# finder
finder <dir> --file-pattern "*.rs" --search-pattern <pattern>
```

### GitHub Pages Structure

Deploy to `gh-pages` branch (orphan, no code):

```
gh-pages/
├── index.html              # Landing page with version history
├── history.json            # [{version, date, timestamp, url}, ...]
└── latest/                 # Full Criterion HTML (overwritten each run)
    └── (Criterion output)
```

**Why this structure:**
- `index.html`: Single entry point for users
- `history.json`: Machine-readable history for programmatic access
- `latest/`: Detailed reports without bloating history (keep only most recent)
- Force-push: No git history (deployment artifacts, not source)

### README Integration

README contains:
- Link to GitHub Pages site
- Brief description of benchmarks
- No auto-generated tables (kept manually if desired)

Rationale: Keep README stable, let Pages be dynamic source of truth.

## Consequences

### Positive

1. **Performance visibility tied to versions**: Users can see "v2.1.1 is 20% faster than v2.1.0"
2. **Low CI overhead**: Only runs on releases (infrequent) or manual trigger
3. **Clean repository**: No benchmark artifacts in main branch history
4. **Standard tooling**: GitHub Pages is well-supported, no external dependencies
5. **Historical tracking**: Can see performance trends across releases
6. **Fair comparisons**: All tools tested on identical fixtures with equivalent commands

### Negative

1. **Late regression detection**: Won't catch performance issues until release
   - Mitigation: Manual dispatch available for pre-release testing
2. **Requires ripgrep installation**: CI must install external tool
   - Mitigation: Well-documented, stable tool; easy to install
3. **Manual README updates**: Benchmark summary tables must be updated by hand
   - Mitigation: GitHub Pages provides authoritative, always-current data
4. **gh-pages branch management**: Separate branch to maintain
   - Mitigation: Automated; force-push means no history to manage

### Validation

After implementation, verify:
- [ ] Workflow triggers on tag push
- [ ] Workflow can be manually dispatched
- [ ] All 6 benchmark scenarios run successfully
- [ ] GitHub Pages site accessible at https://ydkadri.github.io/finders/
- [ ] History table updates correctly
- [ ] Latest Criterion reports display properly
- [ ] Graceful handling if ripgrep unavailable locally

## References

- Issue #37: Original proposal for comparison benchmarks
- Criterion documentation: https://github.com/bheisler/criterion.rs
- GitHub Pages documentation: https://docs.github.com/en/pages
- Ripgrep: https://github.com/BurntSushi/ripgrep
- Related: Issue #33 (benchmark regression detection - internal benchmarks only)
