# Performance Benchmarks

## Sequential (Single-threaded)

| Input | Python | Go | Rust | C | Fastest |
|-------|--------|-----|------|-----|---------|
| 1M | ~100ms | ~6ms | ~5ms | ~2ms | **C** |
| 10M | ~1.2s | ~63ms | ~52ms | ~11ms | **C** |

## Parallel (Multi-core)

| Input | Python | Go | Rust | C | Fastest |
|-------|--------|-----|------|-----|---------|
| 10M | ~0.35s | ~7ms | ~6ms | ~14ms | **Rust** |
| 50M | ~2s | ~40ms | ~35ms | ~64ms | **Rust** |
| 100M | ~5s | ~85ms | ~70ms | ~126ms | **Rust** |

## Rate (Primes per Second, quiet mode)

### Sequential

| Input | Python | Go | Rust | C | C vs Python |
|-------|--------|-----|------|-----|-------------|
| 1M | ~1.7M/s | ~10.6M/s | ~12.8M/s | ~50M/s | **30x** |
| 10M | ~8.3M/s | ~41M/s | ~57M/s | ~63M/s | **7.6x** |

### Parallel

| Input | Python | Go | Rust | C | C vs Python |
|-------|--------|-----|------|-----|-------------|
| 10M | ~1.7M/s | ~40M/s | ~138M/s | ~45M/s | **26x** |
| 50M | ~25M/s | ~42M/s | ~196M/s | ~47M/s | **1.9x** |
| 100M | ~20M/s | ~153M/s | ~200M/s | ~46M/s | **2.3x** |

## System Benchmarks

Run `make benchmark` to generate benchmark data for the current system.

| System | Python (primes/s) | Go (primes/s) | Rust (primes/s) | C (primes/s) |
|--------|-------------------|---------------|-----------------|--------------|
| x86_64 (Linux) | ~20M/s | ~153M/s (100M parallel) | ~200M/s (100M parallel) | ~63M/s (10M sequential) |
| Apple M2 (macOS) | 5,162,263 | 128,241,213 | 189,089,516 | -- |
| Apple M3 Ultra (macOS) | 4,893,819 | 288,442,776 | 437,251,921 | -- |
| Ryzen 9 7900X (Linux) | 2,004,809 | 61,862,555 | 88,468,221 | -- |
| Ryzen 9 7950X (Linux) | 2,081,523 | 67,656,939 | 110,742,428 | -- |

## Key Observations

### Why Rust/Go/C are Faster than Python

1. **Compilation**: Rust/Go/C compile to native machine code; Python is interpreted
2. **Memory Management**: Lower overhead, efficient allocation, no GC pauses
3. **Type System**: Native integer types vs. Python objects (28 bytes per int)
4. **No GIL**: True parallelism in compiled languages (Python uses multiprocessing to work around GIL)

### C Sequential Performance

- **C is the fastest in sequential mode**, edging out Rust by ~10-15% on 1M-10M inputs
- The `-O2` optimized C code benefits from minimal abstraction overhead and direct memory access
- Reusable segment buffer (single malloc for `seg_primes`) eliminates per-segment allocation overhead

### Rust Parallel Performance

- **Rust dominates in parallel mode**, 3-4x faster than C and Go on large inputs
- `thread::scope` enables zero-copy sharing of base primes across workers
- No mutex overhead -- contiguous segment ordering guarantees sorted output without synchronization
- Auto-detects available parallelism vs C's hardcoded 4-worker default

### C Parallel Limitations

- C's parallel mode shows minimal speedup over sequential due to pthread overhead and per-worker malloc allocations
- Each worker allocates its own `worker_primes` and `is_prime_buffer` buffers (vs Rust's shared references)
- Thread creation/join overhead dominates at smaller input sizes
- Hardcoded 4-worker default vs Rust's auto-detected `available_parallelism()`

### Summary

- **Sequential**: C > Rust > Go > Python
- **Parallel**: Rust > Go > C > Python
- **Best overall**: Rust (competitive sequential, dominant parallel)
