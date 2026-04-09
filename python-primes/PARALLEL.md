# CPU Parallelization Implementation

## Overview

CPU parallel processing has been implemented for prime generation using Python's `multiprocessing` module. The segmented sieve algorithm processes independent segments in parallel across worker processes.

## Architecture

### Worker Strategy
- Workers are assigned consecutive chunks of segments to process
- Each worker receives `(start_segment_idx, end_segment_idx)` range
- Workers return list of primes found in their chunk
- Main process merges and sorts results from all workers

### Worker Count
- Default: `cpu_count() // 4` (conservative to avoid overhead)
- Can be overridden via parameter

### Progress Tracking
- Shared counter via `multiprocessing.Value('i', 0)`
- Workers atomically increment shared counter as they complete segments
- Background thread polls shared counter to update tqdm progress bar

## Implementation Details

### Worker Function: `_worker_process_segment_chunk()`

Processes a range of segments in parallel workers:

```python
def _worker_process_segment_chunk(
    start_seg: int,
    end_seg: int,
    n: int,
    segment_size: int,
    base_primes: List[int],
    progress_counter: Any
) -> List[int]:
```

**Key features:**
- Top-level function for picklability with multiprocessing
- Processes segments from `start_seg` to `end_seg - 1`
- Uses same sieve logic as sequential version
- Atomically increments shared counter after each segment
- Returns primes found in worker's chunk

### Parallel Function: `parallel_segmented_sieve()`

Main parallel implementation:

```python
def parallel_segmented_sieve(
    n: int,
    num_workers: Optional[int] = None,
    segment_size: int = DEFAULT_SEGMENT_SIZE,
    progress_callback: Optional[Callable[[int], None]] = None
) -> List[int]:
```

**Implementation steps:**
1. Compute base primes (sqrt(n)) - done in main process before spawning workers
2. Calculate total segments and chunk size per worker
3. Create shared `multiprocessing.Value('i', 0)` for progress tracking
4. Spawn workers with ProcessPoolExecutor or multiprocessing.Pool
5. Launch thread to poll shared counter for tqdm updates
6. Collect results from all workers
7. Merge and return sorted primes

### Integration with generate_primes()

Added parameter: `parallel: bool = False`

**Auto-selection logic:**
- Only activates when n >= PARALLEL_SIEVE_THRESHOLD (default 1B)
- Can be forced via `force_algorithm="parallel"`
- Falls back to sequential if multiprocessing fails

## Usage Examples

### Command-line
```bash
# Auto-select parallel for very large inputs (>= 1B)
python prime_generator.py 1000000000 --progress

# Force parallel mode (use caution - may be slower for small n)
python prime_generator.py 1000000000 --parallel --progress
```

### Python API
```python
from prime_generator import generate_primes, parallel_segmented_sieve

# Auto-select based on n (parallel only for n >= 1B)
primes = generate_primes(1_000_000_000, show_progress=True, parallel=True)

# Direct parallel call with custom workers
primes = parallel_segmented_sieve(
    1_000_000_000,
    num_workers=4,
    segment_size=1_000_000
)

# For most cases, segmented sieve (sequential) is recommended:
primes = generate_primes(100_000_000, show_progress=True)  # No parallel flag
```

## Performance Characteristics

### When Parallel Helps

Parallel processing provides speedup when:
- Very large input size (>= PARALLEL_SIEVE_THRESHOLD, typically 1B+)
- Many CPU cores available (8+)
- Low multiprocessing overhead relative to work time
- Good cache locality for workers

### When Sequential is Faster

Sequential is often faster when:
- Input size < 1B (overhead dominates computation)
- Limited CPU cores (< 8)
- High process spawning/serialization cost
- Memory-bound operations (sieve is memory-intensive)
- Test/sandbox environments with virtualized CPUs

### Realistic Speedups

Speedup depends on:
1. Number of CPUs: More cores = more potential speedup
2. Input size: Larger inputs amortize overhead better
3. Hardware characteristics: CPU cache, memory bandwidth
4. Python multiprocessing implementation overhead

**Note:** Parallel processing has significant overhead from process spawning,
inter-process communication, and result merging. Benchmarks show that parallel
processing typically only becomes beneficial for inputs >= 1B on systems with
8+ physical cores. For most use cases, the segmented sieve (sequential) provides
excellent performance with zero overhead. 

## Edge Cases Handled

1. **Small n**: Falls back to sequential for n < PARALLEL_SIEVE_THRESHOLD
2. **Workers > segments**: Caps workers at number of segments
3. **Uneven chunks**: Handles remainder division correctly
4. **Windows vs Linux multiprocessing**: Compatible with both systems
5. **Multiprocessing failures**: Gracefully falls back to sequential
6. **Low-core environments**: Conservative worker count (cpu_count // 4) avoids overhead

## Testing

```python
class TestParallelSieve(unittest.TestCase):
    def test_parallel_matches_sequential():
        """Parallel and sequential should produce identical results"""
        
    def test_parallel_with_various_worker_counts():
        """Test correctness with different worker counts"""
        
    def test_generate_primes_with_parallel():
        """Test generate_primes with parallel=True"""
        
    def test_progress_with_parallel():
        """Test that progress parameter works with parallel execution"""
```

## Summary

Segmented sieve algorithm's independent segments make it well-suited for parallel processing. 
