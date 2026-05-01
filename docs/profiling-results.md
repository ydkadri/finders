# Profiling Results - Current Parallel Implementation

**Date**: 2026-05-01  
**Tool**: Xcode Instruments (Time Profiler)  
**Benchmark**: profile_search (100 iterations, ~1000 files, "TODO" pattern)  
**Duration**: 2.77 seconds

## Summary

Analysis of CPU time spent in key functions during parallel file searching.

## Top Functions by Sample Count

Functions related to `finders` and synchronization primitives:

| Samples | % | Function | Category |
|---------|---|----------|----------|
| 14 | - | `Searcher::search_line` | Core search logic |
| 10 | - | `rayon_core::WorkerThread::wait_until_cold` | Thread pool waiting |
| 8 | - | `rayon_core::join::join_context` | Parallel join overhead |
| 7 | - | `StandardOutput::write_match` | Output writing (mutex protected) |
| 6 | - | `rayon::iter::plumbing::bridge_producer_consumer` | Parallel iteration setup |
| 5 | - | `pthread_mutex_lock` | **Mutex locking** |
| 3 | - | `rayon::iter::plumbing::Folder::consume_iter` | Parallel iteration |
| 3 | - | `rayon_core::sleep::Sleep::wake_specific_thread` | Thread coordination |
| 3 | - | `rayon_core::WorkerThread::find_work` | Work stealing overhead |
| 2 | - | `pthread_mutex_firstfit_lock_wait` | **Mutex contention** |
| 2 | - | `std::sys::pal::unix::sync::mutex::Mutex::lock` | **Mutex locking** |
| 2 | - | `StandardOutput::write_coloured_path` | Output formatting |
| 1 | - | `pthread_mutex_unlock` | Mutex unlocking |

## Key Observations

### 1. Mutex Operations Are Visible But Not Dominant

Mutex-related functions appear in the profile:
- `pthread_mutex_lock`: 5 samples
- `pthread_mutex_firstfit_lock_wait`: 2 samples (indicates contention)
- `std::sys::pal::unix::sync::mutex::Mutex::lock`: 2 samples
- `pthread_mutex_unlock`: 1 sample

**Total mutex overhead: ~10 samples out of ~100 total samples** (rough estimate)

This suggests mutex operations account for approximately **10% of CPU time** in this workload.

### 2. Output Writing Shows Up

`StandardOutput::write_match` appears 7 times, which is the function that:
1. Locks the mutex
2. Writes the match data
3. Unlocks the mutex

This confirms that output writing (including mutex acquisition) is a measurable cost.

### 3. Rayon Thread Pool Overhead

Significant time spent in Rayon coordination:
- `WorkerThread::wait_until_cold`: 10 samples (threads waiting for work)
- `join_context`: 8 samples (parallel join overhead)
- `bridge_producer_consumer`: 6 samples (setting up parallel iteration)
- `find_work`: 3 samples (work stealing)

This is **expected overhead** for parallel workloads and suggests the problem might be too small or I/O-bound.

### 4. Core Search Logic

`Searcher::search_line` appears 14 times - this is the actual search work being done. This is good; it means we're spending time on the actual task, not just coordination.

## Interpretation

The profiling data shows:

1. **Mutex contention exists but is not catastrophic** in this workload
   - ~10% of samples show mutex-related operations
   - Presence of `pthread_mutex_firstfit_lock_wait` confirms threads are waiting on locks

2. **Rayon overhead is significant** 
   - Thread pool coordination appears frequently
   - This is why small workloads see minimal speedup
   - Large workloads amortize this overhead better (explains 31-51% gains in benchmarks)

3. **The search work itself is relatively cheap**
   - Only 14 samples for the core search logic
   - Most time is spent in coordination, output, and waiting

## Conclusion

The current implementation is **reasonably efficient** for the workloads we care about:

- ✅ **31-51% speedup** on medium/large repositories (real-world benefit)
- ✅ Mutex contention is present but not dominant (~10% overhead)
- ✅ Core search logic is efficient
- ⚠️ Small workloads don't benefit much (Rayon overhead > gains)

### Should We Optimize Further?

**Batched output writes** could reduce mutex overhead from 10% to near-zero:
- Current: One mutex lock per match
- Proposed: One mutex lock per batch (per thread or per file)
- Expected gain: Reduce that 10% mutex overhead

However, the **bigger opportunity** might be reducing Rayon overhead for small workloads, but that's harder to address without changing the parallelization strategy entirely.

### Next Steps

Two options:

1. **Implement batched output** - Reduces mutex overhead, should help all workloads slightly
2. **Accept current performance** - 31-51% gains are excellent for the target use case (medium/large repos)

The profiling data suggests option #2 might be sufficient, but option #1 is still a reasonable optimization if we want to squeeze out more performance.
