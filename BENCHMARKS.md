# Performance Benchmarks

Testing environment: AMD Ryzen 9 7900X 12-Core Processor

## Sequential (Single-threaded)

| Input | Python | Go | Rust | Rust Speedup |
|-------|--------|-----|------|--------------|
| 1M | ~100ms | ~6ms | ~5ms | **20x** |
| 10M | ~1.2s | ~63ms | ~52ms | **23x** |

## Parallel (Multi-core, with progress)

| Input | Python | Go | Rust | Winner |
|-------|--------|-----|------|--------|
| 10M | ~0.35s | ~7ms | ~6ms | Rust (1.2x Go) |
| 50M | ~2s | ~40ms | ~35ms | Rust (1.1x Go) |
| 100M | ~5s | ~85ms | ~70ms | Rust (1.2x Go) |

## Rate (Primes per Second, quiet mode)

| Input | Python | Go | Rust | Rust vs Python |
|-------|--------|-----|------|----------------|
| 10M | ~1.7M/s | ~10.6M/s | ~12.8M/s | **7.5x** |
| 100M | ~900K/s | ~6M/s | ~7.5M/s | **8x** |

## System Benchmarks

Run `make benchmark` to generate benchmark data for the current system.

| System | Python (primes/s) | Go (primes/s) | Rust (primes/s) |
|--------|-------------------|---------------|-----------------|
| Ryzen 9 7900X (Linux) | 2,004,809 | 61,862,555 | 88,468,221 |

## Key Observations

### Why Rust/Go are Faster than Python

1. **Compilation**: Rust/Go compile to native machine code; Python is interpreted
2. **Memory Management**: Lower overhead, efficient allocation, no GC pauses
3. **Type System**: Native integer types vs. Python objects (28 bytes per int)
4. **No GIL**: True parallelism in compiled languages (Python uses multiprocessing to work around GIL)

### Rust vs Go

- **Rust**: ~20% faster, zero-cost abstractions, no runtime overhead, `thread::scope` for safe borrowing
- **Go**: Simpler concurrency model, faster compilation, easier debugging, goroutines are lightweight
- **Both**: 7-8x faster than Python on large inputs
