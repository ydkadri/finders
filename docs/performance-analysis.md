# Performance Analysis & Optimization Opportunities

## Current Performance Baseline

From comparison benchmarks (2026-04-21):

| Scenario | finder | ripgrep | Gap |
|----------|--------|---------|-----|
| Small (~100 files) | 2ms | 5ms | ✅ 2.5x faster |
| Medium (~1K files) | 13ms | 8ms | ⚠️ 1.6x slower |
| Large (~5K files) | 61ms | 25ms | ⚠️ 2.4x slower |

**Key observation:** We're faster on small repos but slower as size grows. This suggests scaling issues, not fundamental algorithm problems.

## Architecture Overview

###Current Design (Sequential)

```
For each file:
  1. Open file
  2. Create BufReader (8KB buffer)
  3. Read line by line
  4. Search each line
  5. Write matches to output
```

This is simple and memory-efficient, but **processes files one at a time**.

## Where Time Goes (Hypothesis)

Without profiling data, we reason about likely bottlenecks:

### 1. **File I/O** (likely biggest bottleneck)
- Opening files: system call overhead
- Reading from disk: waiting for disk I/O
- **Why it matters:** In Python, I/O is often hidden because it's slow anyway. In Rust, CPU operations are so fast that I/O becomes the bottleneck

### 2. **Sequential Processing** (architectural bottleneck)
- We process one file at a time
- While reading file N, CPU is mostly idle waiting for disk
- Modern CPUs have multiple cores - we're using only one

### 3. **String Operations** (moderate impact)
- Allocating strings for each line: `String::from(line)`
- Case-insensitive search: `.to_lowercase()` allocates new strings
- Match position calculation: iterating through line multiple times

### 4. **Output Writing** (small impact)
- Writing to stdout: probably fast for terminal, might block on pipes
- Colored output: some overhead for escape codes

## Learning: CPU vs I/O Bound

**In Python:**
- Most code is CPU-bound (Python interpreter is slow)
- I/O doesn't stand out as much because everything is slow

**In Rust:**
- CPU operations are 10-100x faster than Python
- This makes I/O the bottleneck - we spend most time waiting for disk

**Example:**
```rust
// This is FAST in Rust (nanoseconds)
if line.contains("TODO") { }

// This is SLOW (microseconds to milliseconds)
let file = File::open(path)?;
```

## Optimization Opportunities

### Low-Hanging Fruit (10-30% improvement)

1. **Reduce allocations in hot path**
   - Avoid `to_lowercase()` when not needed
   - Reuse buffers where possible

2. **Optimize string matching**
   - Use `memchr` crate for byte-level scanning (what ripgrep uses)
   - Avoid unnecessary UTF-8 validation

3. **Better buffering**
   - Experiment with buffer sizes (current: 8KB)
   - Read larger chunks when searching rare patterns

### Medium Effort (30-50% improvement)

4. **Parallel file processing**
   - Use `rayon` to process multiple files concurrently
   - Keep sequential per-file processing (simpler)
   - **Tradeoff:** More complex, output ordering changes

5. **Memory-mapped I/O**
   - Use `mmap` for files instead of `read()`
   - Let OS handle paging
   - **Tradeoff:** More memory usage, complexity

### Bigger Changes (50%+ improvement, major rework)

6. **Parallel line processing**
   - Split files into chunks, search in parallel
   - Requires careful line boundary handling

7. **SIMD string matching**
   - Use CPU vector instructions for pattern matching
   - Requires unsafe code and platform-specific logic

## Next Steps

We'll tackle these in order:

**Phase 1:** Measure current performance properly
- Add detailed timing to understand bottlenecks
- Profile if we can set up tools

**Phase 2:** Low-hanging fruit
- String allocation optimization
- Buffer size experimentation

**Phase 3:** Parallelization
- Use rayon for parallel file processing
- Write ADR for this decision

## Learning Resources

**For understanding performance in Rust:**
- [The Rust Performance Book](https://nnethercote.github.io/perf-book/)
- Ripgrep's blog posts on performance
- "Programming Rust" chapter on performance

**Key concepts:**
- **Allocation is expensive:** Every `String::new()` or `.to_lowercase()` hits the allocator
- **I/O is expensive:** Opening files, reading from disk
- **Parallelism helps I/O:** While one thread waits for disk, another can use CPU
