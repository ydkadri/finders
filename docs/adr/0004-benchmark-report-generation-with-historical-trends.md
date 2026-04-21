# 0004. Benchmark Report Generation with Historical Trends

Date: 2026-04-20

## Status

Accepted

## Context

The comparison benchmarks workflow originally generated a basic HTML page showing only the latest benchmark results. The page had several limitations:

1. **No historical context**: Only showed current run, no performance trends over time
2. **Static HTML in workflow**: 150+ line heredoc embedded in YAML workflow file
3. **Limited metadata**: `history.json` stored only version/date/URL, not actual results
4. **Poor maintainability**: HTML/CSS mixed with workflow logic, hard to modify
5. **No visual indicators**: Results displayed as plain numbers without change indicators

Users (and future maintainers) needed:
- **Trend visibility**: Is performance improving or regressing over releases?
- **Change detection**: Quick visual feedback on performance deltas
- **Maintainable reports**: Easier to modify templates without touching workflow YAML
- **Historical data**: Ability to analyze performance patterns across releases

## Decision

We will generate custom HTML benchmark reports using **Python with Jinja2 templating** and store **full benchmark results** in the history file to enable historical trend analysis.

### Architecture

```
Workflow (comparison.yml)
    ↓
1. Run benchmarks (criterion)
    ↓
2. Parse finders-benchmarks.json → results array
    ↓
3. Update history.json (store full results + metadata)
    ↓
4. Python script: generate_benchmark_report.py
    ↓
5. Render Jinja2 template → index.html
    ↓
6. Deploy to gh-pages/benchmarks/
```

### Components

1. **History storage** (`.github/workflows/comparison.yml`):
   ```json
   {
     "version": "v3.0.1",
     "date": "2026-04-21",
     "timestamp": "2026-04-21T00:00:00Z",
     "results": [
       {"scenario": "small_common", "pattern": "common", "size": "Small (~100 files)", 
        "finder": 2, "find_grep": 86, "ripgrep": 5},
       ...
     ]
   }
   ```
   Previously stored only `{version, date, url}` metadata.

2. **Report generator** (`.github/scripts/generate_benchmark_report.py`):
   - Reads `benchmarks/history.json`
   - Calculates performance changes (±5% thresholds for better/worse/neutral)
   - Prepares data for summary table and trend visualizations
   - Renders Jinja2 templates with data

3. **Templates** (`.github/templates/`):
   - `benchmark_report.html` - Jinja2 template for HTML structure
   - `benchmark_report.css` - Separate stylesheet
   - Clean separation of presentation from logic

### Features

**Summary Table**:
- All 6 scenarios (3 sizes × 2 patterns)
- Comparison: finder vs find+grep vs ripgrep
- Color-coded change indicators:
  - 🟢 Green ↓: >5% faster (better)
  - ⚪ Gray →: Within ±5% (similar)
  - 🔴 Red ↑: >5% slower (worse)

**Historical Trends**:
- Tracks last 5 benchmark runs
- Mini sparkline bar charts for visual trends
- Percentage change from oldest to latest in window
- Identifies performance regressions early

**Graceful Degradation**:
- Handles mixed old/new history format
- Skips entries without `results` field
- First run shows single entry (no trends yet)
- Builds up history over subsequent runs

## Consequences

### Positive

- **Better visibility**: Performance trends visible at a glance with color coding
- **Maintainability**: HTML templates separate from workflow logic
- **Extensibility**: Easy to add new visualizations or metrics
- **Historical analysis**: Full result storage enables future data analysis
- **Professional presentation**: Modern, clean UI improves project credibility
- **Early detection**: Regressions spotted immediately with color indicators
- **Template reusability**: Jinja2 templates can be modified without workflow changes

### Negative

- **Additional dependency**: Requires Python with Jinja2 in CI environment
  - Mitigation: GitHub Actions runners include Python by default
  - Only `pip install jinja2` needed
- **Storage overhead**: History file grows with each benchmark run
  - ~500 bytes per entry × max ~20-30 relevant entries = ~15KB total (negligible)
- **Migration burden**: Old history entries lack `results` field
  - Mitigation: Code handles mixed format gracefully
  - Old entries don't show trends but don't break reports
- **Maintenance surface**: Additional files to maintain (Python script, templates)
  - Trade-off: Separation improves maintainability despite file count

### Neutral

- **Python in CI**: Workflow now runs Python script vs pure bash/jq
  - Python + Jinja2 more maintainable than heredoc approach
- **Report generation time**: Adds ~1-2 seconds to workflow
  - Negligible compared to benchmark execution time (~2-3 minutes)

## Alternatives Considered

### Alternative 1: Continue with Static HTML Heredoc

**Description**: Keep the existing 150+ line heredoc in workflow YAML

**Pros**:
- No new dependencies
- Self-contained in workflow
- Simple to understand

**Cons**:
- Hard to maintain (HTML/CSS/data mixed in YAML)
- No historical trends possible
- Poor separation of concerns
- Difficult to test changes locally

**Why not chosen**: Maintainability and feature limitations. The heredoc approach was already becoming unwieldy at 150 lines, and adding trends would make it worse.

### Alternative 2: Static Site Generator (Jekyll, Hugo, etc.)

**Description**: Use a full static site generator for benchmark pages

**Pros**:
- Rich ecosystem of themes and plugins
- Professional-looking output
- Built-in templating

**Cons**:
- Significant overhead for simple benchmark page
- Complex configuration
- Harder to customize for specific needs
- Adds substantial dependencies

**Why not chosen**: Overengineering. Our needs are simple: one page with a table and some charts. A full SSG is excessive.

### Alternative 3: JavaScript-Based Rendering

**Description**: Store data as JSON, use client-side JS to render charts/tables

**Pros**:
- Interactive visualizations possible
- Lightweight server-side (just serve JSON)
- Modern web approach

**Cons**:
- Requires JavaScript enabled (accessibility concern)
- More complex for simple tabular data
- Harder to maintain
- No benefit for our static content

**Why not chosen**: Server-side rendering is simpler and more accessible. We don't need interactivity for basic performance tables.

### Alternative 4: Store Only Aggregates, Not Full Results

**Description**: Store summary statistics rather than complete result arrays

**Pros**:
- Smaller history file
- Faster processing

**Cons**:
- Limits future analysis options
- Can't recalculate aggregates if methodology changes
- Loss of granularity

**Why not chosen**: Storage is cheap, flexibility is valuable. Full results enable richer future analysis without re-running old benchmarks.

## References

- [PR #86: Add custom benchmark reports with historical trends](https://github.com/ydkadri/finders/pull/86)
- [PR #88: Fix benchmark report to handle old history format](https://github.com/ydkadri/finders/pull/88)
- [Jinja2 documentation](https://jinja.palletsprojects.com/)
- [Criterion.rs](https://github.com/bheisler/criterion.rs) - Rust benchmarking framework used
- [benchmark history.json](https://gist.github.com/ydkadri/5616380737bada94e84764d02b816b38) - Live history data
- [Live benchmark page](https://ydkadri.github.io/finders/benchmarks/)
