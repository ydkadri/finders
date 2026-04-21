# 0003. GitHub Pages Deployment Strategy

Date: 2026-04-20

## Status

Accepted

## Context

FindeRS maintains two types of generated content that need to be hosted on GitHub Pages:

1. **Documentation** (rustdoc) - Generated from Rust source code
2. **Benchmarks** - Comparison benchmarks run on each release

Initially, we attempted to use GitHub Actions artifacts for documentation deployment while the comparison benchmarks workflow pushed directly to the `gh-pages` branch at `/benchmarks/`. This created a conflict: GitHub Pages was configured for "workflow" deployment (Actions artifacts), but the benchmarks were being pushed to a branch that wasn't being served, resulting in a 404 at https://ydkadri.github.io/finders/benchmarks/.

**The core problem:** GitHub Pages only supports one deployment source - either Actions artifacts OR a branch, but not both simultaneously.

## Decision

We will use **branch-based deployment** with the `gh-pages` branch as the single source for all GitHub Pages content. Both documentation and benchmarks will be pushed to this branch by their respective workflows.

### Implementation

1. **Documentation workflow** (`.github/workflows/docs.yml`):
   - Switched from `actions/upload-pages-artifact` + `actions/deploy-pages` to `peaceiris/actions-gh-pages@v3`
   - Deploys rustdoc output to root of `gh-pages` branch
   - Uses `keep_files: true` to preserve existing benchmark files

2. **Comparison workflow** (`.github/workflows/comparison.yml`):
   - Already pushes to `gh-pages` branch at `/benchmarks/` subdirectory
   - Changed from `--force-with-lease` to `--force` for push (see consequences)

3. **GitHub Pages settings** (manual):
   - Changed from "GitHub Actions" source to "Deploy from a branch"
   - Selected `gh-pages` branch, `/ (root)` folder

### Directory Structure

```
gh-pages branch:
├── .nojekyll              # Disable Jekyll processing
├── index.html             # Documentation root
├── [rustdoc files]        # All documentation
└── benchmarks/            # Benchmark reports
    ├── index.html
    ├── history.json
    └── latest/
        └── report/        # Criterion reports
```

## Consequences

### Positive

- **Single source of truth**: All content served from one branch eliminates deployment conflicts
- **Consistent deployment**: Both workflows use similar patterns (push to gh-pages)
- **File preservation**: Documentation and benchmarks coexist without overwriting each other
- **Standard GitHub Pages**: Uses well-established branch-based deployment pattern

### Negative

- **Manual configuration required**: GitHub Pages source setting must be changed via web UI after merge
- **Force push required**: Using `--force` instead of `--force-with-lease` loses safety check
  - Acceptable because `gh-pages` is automation-only (no human edits)
  - Necessary because branch may be updated between workflows cloning and pushing
- **Workflow coordination**: Two workflows must coordinate to avoid conflicts
  - Mitigated by `keep_files: true` preserving non-overlapping content

### Neutral

- **Branch vs artifact deployment**: Trade-off between safety (artifacts) and flexibility (branch)
- **Third-party action dependency**: Now depend on `peaceiris/actions-gh-pages` for docs deployment

## Alternatives Considered

### Alternative 1: Actions Artifacts for Both

**Description**: Convert benchmarks workflow to also use Actions artifacts deployment

**Pros**:
- More "modern" GitHub Pages pattern
- Better safety (no force pushes)
- Official GitHub action support

**Cons**:
- Would require significant restructuring of comparison workflow
- Harder to maintain historical benchmark data (artifacts are ephemeral by nature)
- More complex: need to download previous history, update, re-upload
- Workflow runs would need special Pages deployment permissions

**Why not chosen**: Branch-based deployment better fits our benchmark history requirements. The comparison workflow already pushes to gh-pages successfully.

### Alternative 2: Separate GitHub Pages Sites

**Description**: Host documentation and benchmarks on separate URLs (e.g., separate repos or subdomains)

**Pros**:
- Complete isolation between content types
- No coordination needed

**Cons**:
- More complex for users to discover both resources
- Would require additional repository or configuration
- Unnecessarily complex for our needs

**Why not chosen**: Overengineering for a personal project. Single site with subdirectories is simpler.

### Alternative 3: Documentation-Only on Pages

**Description**: Remove benchmarks from GitHub Pages entirely, serve elsewhere or not at all

**Pros**:
- Eliminates deployment coordination issues
- Could use simpler docs.yml workflow

**Cons**:
- Loses public benchmark visibility
- Benchmarks are valuable for transparency and tracking performance
- Would need alternative hosting solution

**Why not chosen**: Public benchmarks are a feature, not a problem to eliminate.

## References

- [PR #85: Fix GitHub Pages deployment for benchmarks](https://github.com/ydkadri/finders/pull/85)
- [PR #82: Fix comparison workflow: use --force instead of --force-with-lease](https://github.com/ydkadri/finders/pull/82)
- [Issue: Benchmarks 404](https://github.com/ydkadri/finders/issues/79) (if exists)
- [GitHub Pages documentation](https://docs.github.com/en/pages/getting-started-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site)
- [peaceiris/actions-gh-pages documentation](https://github.com/peaceiris/actions-gh-pages)
