# Performance

Understanding FindeRS performance characteristics and optimization strategies.

## Performance Goals

FindeRS is designed to be **fast enough** for daily development tasks, not the absolute fastest tool. The goal is:

> "Fast enough that you never notice, simple enough that you always remember"

## Benchmark Results

### Comparison with Other Tools

**Latest benchmark results:** [View detailed benchmarks →](https://ydkadri.github.io/finders/benchmarks/)

The benchmark suite compares FindeRS against `find+grep` and `ripgrep` across different repository sizes (small, medium, large) and search patterns (common, rare). Results are updated automatically on every release.

**Key observations:**
- FindeRS is significantly faster than find+grep
- FindeRS is comparable to ripgrep for small to medium projects
- For very large codebases (100k+ files), ripgrep pulls ahead
- For typical development work (< 10k files), the difference is negligible

### Real-world Performance

**Typical project sizes:**
- Small project (React app): ~500 files → **5ms**
- Medium project (Rust service): ~2,000 files → **12ms**
- Large project (monorepo): ~20,000 files → **95ms**

All well within "feels instant" territory.

## Performance Characteristics

### What Makes FindeRS Fast

1. **Streaming Architecture**
   - Files processed as found, not loaded into memory
   - Memory usage stays constant regardless of result count
   - Starts outputting matches immediately

2. **Efficient File Walking**
   - Uses platform-optimized directory traversal
   - Minimal allocations during directory scanning
   - Skips unreadable files quickly

3. **Smart Pattern Matching**
   - Literal string search uses optimized Boyer-Moore variant
   - Regex compilation happens once, not per file
   - UTF-8 validation only when needed

### What Limits Performance

1. **Single-threaded by design**
   - Simpler implementation, easier to reason about
   - Sufficient for typical use cases
   - Parallelization planned for future versions

2. **No .gitignore handling**
   - Searches all files, including build artifacts
   - Can be slower on projects with large `node_modules/` or `target/`
   - Solution: Use shell patterns to limit search scope

3. **No directory caching**
   - Each search walks the directory tree fresh
   - Good: Always up-to-date results
   - Bad: Repeated searches don't get faster

## Optimization Strategies

### Limit Search Scope

Instead of searching the entire project:

```bash
# ❌ Slow: searches everything including node_modules
finder -s "pattern"

# ✅ Fast: search only source directory
finder src/ -s "pattern"

# ✅ Fast: target specific file types in directory
finder src/ -f ".rs" -s "pattern"
```

### Use Specific File Patterns

```bash
# ❌ Slower: search all files then filter
finder -s "TODO" | grep ".rs:"

# ✅ Faster: filter files during search
finder -f ".rs" -s "TODO"
```

### Choose the Right Output Mode

```bash
# If you only need file paths:
finder -s "pattern" -l  # Faster, stops after first match per file

# If you need match counts:
finder -s "pattern" -c  # Faster, no need to format output

# If you need full context:
finder -s "pattern"     # Slower, formats each match
```

### Optimize Regex Patterns

```bash
# ❌ Slow: complex regex
finder -r ".*TODO.*|.*FIXME.*"

# ✅ Fast: simpler alternative
finder -r "TODO|FIXME"

# ✅ Faster: literal search if no regex needed
finder -s "TODO"
```

### Exclude Large Directories

```bash
# Manually exclude directories
finder src/ tests/ -s "pattern"

# Or use find to pre-filter
find . -type f -not -path "*/node_modules/*" -not -path "*/target/*" | \
  xargs finder -s "pattern"
```

## Memory Usage

FindeRS has minimal memory footprint:

- **Base memory:** ~2-3 MB (Rust binary overhead)
- **Per-file overhead:** negligible (streaming processing)
- **Large results:** constant memory (prints as it finds)

**Example:** Searching 100k files with 10k matches uses ~3MB RAM.

## Disk I/O Patterns

FindeRS is I/O bound, not CPU bound:

- Directory traversal is sequential (OS-optimized)
- File reads are buffered (8KB chunks)
- No unnecessary seeks or multiple passes

**Tip:** Performance on SSD vs HDD:
- SSD: ~10x faster due to random access patterns
- HDD: limited by seek time when walking large directory trees

## Scaling Considerations

### When FindeRS is Fast Enough

- Projects under 50k files
- Local development (SSD)
- Ad-hoc searches (not in tight loops)
- Interactive use

### When to Consider Alternatives

- Monorepos with 100k+ files → use ripgrep
- Repeated searches (CI/CD) → cache file lists
- Network filesystems → use local checkouts
- Need .gitignore filtering → use ripgrep

## Future Performance Improvements

Planned optimizations:

1. **Parallel file processing**
   - Process multiple files concurrently
   - Target: 3-5x speedup on multi-core systems
   - Status: Planned for v4.0.0

2. **Directory ignore patterns**
   - Skip common build directories automatically
   - Target: 2x speedup on typical projects
   - Status: Under consideration

3. **Incremental search results**
   - Streaming JSON output
   - Target: Better experience with large result sets
   - Status: Planned for v4.1.0

## Benchmarking Your Use Case

To benchmark on your own projects:

```bash
# Simple timing
time finder -s "pattern" > /dev/null

# Compare with ripgrep
time rg "pattern" > /dev/null

# Test different approaches
time finder src/ -s "pattern" > /dev/null  # Limited scope
time finder -f ".rs" -s "pattern" > /dev/null  # File filtering
```

## Performance Profiling

For detailed performance analysis:

```bash
# Build with profiling
cargo build --release --features profiling

# Run with profiling (requires Instruments on macOS or perf on Linux)
cargo instruments -t time --release -- -s "pattern"
```

## Next Steps

- Review [Comparison](./comparison.md) with other tools
- Check [Common Use Cases](../examples/common-use-cases.md) for efficient patterns
- See [Advanced Patterns](../examples/advanced-patterns.md) for optimization techniques
