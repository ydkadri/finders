# 6. Parallel File Processing with Rayon

Date: 2026-05-01

## Status

Accepted

## Context

FindeRS processes files sequentially, one at a time. This creates a performance bottleneck on medium and large repositories:

- **Small repos** (~100 files): 2ms (faster than ripgrep 5ms)
- **Medium repos** (~1K files): 13ms (slower than ripgrep 8ms)
- **Large repos** (~5K files): 61ms (slower than ripgrep 25ms - 2.4x gap)

Analysis showed the bottleneck is **scaling**, not fundamental algorithms. File I/O is the primary bottleneck - CPU sits idle waiting for disk while processing files sequentially.

**Performance characteristics:**
- Opening files: 10μs per file (system call overhead)
- Reading files: 100μs per file (disk I/O wait)
- Searching content: 50μs per file (CPU work, fast)
- Writing output: 5μs per file (stdout, fast)

With sequential processing, the CPU is mostly idle during I/O waits. On an 8-core machine, we're using only 12.5% of available processing power.

## Decision

Implement parallel file processing using Rayon's data parallelism library.

**Implementation approach:**
1. Convert sequential iteration to parallel: `.iter()` → `.par_iter()`
2. Wrap output writer in `Mutex` for thread-safe access
3. Mark `Outputs` trait as `Send` to enable cross-thread sharing
4. Use Rayon's default thread pool (scales to CPU core count)

**Code changes:**
```rust
// Before
for path in paths {
    search_file(&searcher, &path, verbose, output)?;
}

// After
let output_mutex = Mutex::new(output);
paths.par_iter().try_for_each(|path| {
    search_file_parallel(&searcher, path, verbose, &output_mutex)
})?;
```

## Consequences

### Positive

- **30-60% performance improvement** on medium/large repositories
  - Medium repos: Expected 13ms → ~9ms
  - Large repos: Expected 61ms → ~35ms
- **Better CPU utilisation**: Verified 593% CPU usage (using ~6 of 8 cores)
- **Scales with hardware**: More cores = better performance
- **Simple implementation**: ~50 lines of code change
- **Minimal code complexity**: Rayon handles thread pool management
- **Maintained code correctness**: All existing tests pass

### Negative

- **Non-deterministic output order**: Files processed in parallel, output order varies
  - **Mitigation**: Acceptable for a search tool (users care about finding matches, not order)
  - **Alternative**: Sort results before output (adds latency)
- **Mutex contention**: Threads wait to write output
  - **Impact**: Minimal (writing is fast, ~5μs per match)
  - **Measured**: Still achieving 593% CPU despite contention
- **Increased memory usage**: Each thread has its own stack (~2MB per thread)
  - **Impact**: 8 threads × 2MB = 16MB overhead (negligible on modern systems)
- **Minimal improvement on small repos**: Thread creation overhead dominates
  - **Measured**: Small datasets show 44% CPU (overhead > benefit)

### Neutral

- **New dependency**: Added `rayon = "1.10"` (well-maintained, widely used)
- **Thread safety requirements**: Outputs trait now requires `Send` bound
- **Binary size**: Minimal increase (~50KB with rayon)

## Alternatives Considered

### Alternative 1: Async I/O with Tokio

**Description**: Use async/await with Tokio runtime for concurrent file processing

**Pros**:
- Lightweight tasks (futures ~100 bytes vs threads ~2MB)
- Can handle thousands of concurrent operations
- Industry standard for I/O-heavy applications

**Cons**:
- File I/O on macOS/Linux lacks true async support
  - `tokio::fs::read()` uses blocking thread pool internally
  - No performance advantage over explicit threading
- Requires rewriting all I/O code with `async/await`
- Async trait complexity (requires `async-trait` crate)
- More complex error handling (`Result` + futures)
- Larger dependency footprint (tokio runtime + ecosystem)

**Why not chosen**: 
- Async file I/O is implemented with thread pools anyway (no real benefit)
- Multi-threading is simpler and more direct for our use case
- We have hundreds of large tasks, not thousands of tiny tasks

### Alternative 2: Manual Thread Pool

**Description**: Create thread pool manually with `std::thread` and channel-based work distribution

**Pros**:
- No external dependencies
- Full control over thread count and work distribution
- Could tune for specific workload patterns

**Cons**:
- Significant implementation complexity (~200-300 lines)
- Need to implement work-stealing or load balancing manually
- Error handling across threads is complex
- Would reinvent what Rayon already provides
- More opportunities for bugs (data races, deadlocks)

**Why not chosen**:
- Rayon provides work-stealing out of the box
- Tested, maintained, widely used in Rust ecosystem
- Time to implement would far exceed adding dependency
- "Don't reinvent the wheel" principle

### Alternative 3: Process-Based Parallelism

**Description**: Fork multiple processes and distribute files across them

**Pros**:
- Complete isolation between processes
- No shared memory concerns

**Cons**:
- High overhead (process creation ~1-10ms each)
- Need IPC (pipes/sockets) for results
- Complex coordination and error handling
- Poor performance for small/medium repos
- Platform-specific considerations (Windows process model different)

**Why not chosen**:
- Much higher overhead than threads
- Excessive complexity for the benefit
- Thread-based approach is more efficient

### Alternative 4: Stay Sequential

**Description**: Keep current sequential implementation, optimize other areas

**Pros**:
- No code changes needed
- Deterministic output order
- No thread safety concerns

**Cons**:
- Cannot close performance gap with ripgrep
- Underutilises modern multi-core CPUs
- Sequential I/O waits leave CPU idle

**Why not chosen**:
- Leaves significant performance on the table
- Modern systems have 4-16 cores (wasted)
- Parallel processing is the biggest single optimization opportunity

## Performance Impact

**Measured results:**
- 593% CPU usage on 2000-file dataset (using ~6 of 8 cores)
- Expected 30-40% improvement on medium repos (~1K files)
- Expected 40-60% improvement on large repos (~5K+ files)
- Minimal improvement on small repos due to overhead

**Why not 800% (8 cores fully used)?**
1. Mutex contention when writing output (acceptable)
2. Disk I/O bandwidth limits (can only read so fast)
3. Thread coordination overhead (Rayon work-stealing)
4. File opening serialisation (kernel limits)

**Remaining gap with ripgrep:**
- Before: 2.4x slower on large repos
- After: ~1.6x slower (estimated)
- Ripgrep uses additional optimisations (mmap, SIMD, custom thread pool tuning)

## Future Optimisation Opportunities

1. **Batched output writes**: Reduce mutex contention by buffering writes per thread
2. **Memory-mapped I/O**: Use `mmap` instead of `read()` for large files
3. **Buffer size tuning**: Experiment with CHUNK_SIZE (current 8KB)
4. **SIMD string matching**: Use vector instructions for pattern matching (marginal gains)

## References

- [Rayon documentation](https://docs.rs/rayon/)
- [The Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [docs/parallel-implementation-explained.md](../parallel-implementation-explained.md) - Deep dive explanation
- [docs/parallel-performance-results.md](../parallel-performance-results.md) - Verification results
- [docs/performance-analysis.md](../performance-analysis.md) - Initial analysis
