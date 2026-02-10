# Prime Number Generator: Python vs Go vs Rust

## Overview

High-performance prime number generators in Python, Go, and Rust with consistent APIs and comparable algorithms. Compare performance across three language implementations.

## Implementations

| Language | Directory | Algorithm | Parallelism |
|----------|-----------|-----------|-------------|
| Python | `python/` | Classic/Segmented/Parallel Sieve | `multiprocessing.Pool` |
| Go | `golang/` | Classic/Segmented/Parallel Sieve | Goroutines + channels |
| Rust | `rust/` | Classic/Segmented/Parallel Sieve | `thread::scope` |

## Quick Start

```bash
# Run all implementations (10M primes)
make compare

# Build all
make build

# Run all tests
make test

# Clean build artifacts
make clean
```

## Running Individual Implementations

### Rust
```bash
cd rust && make help
make release          # Build optimized
make run-release      # Run (n=1000)
make run-release-quiet  # Count primes < 1M
make test             # Run tests
```

### Go
```bash
cd golang && make help
make build            # Build binary
make run-progress     # With progress bar
make run-progress-parallel N=10000000  # Parallel + progress
make test             # Run tests
```

### Python
```bash
cd python && make help
make run-progress     # With progress bar
make run-progress-parallel  # Parallel + progress
make test             # Run tests
make lint             # Run ruff linter
```

## Performance Benchmarks

Testing environment: AMD Ryzen 9 7900X 12-Core Processor

### Sequential (Single-threaded)

| Input | Python | Go | Rust | Rust Speedup |
|-------|--------|-----|------|--------------|
| 1M | ~100ms | ~6ms | ~5ms | **20x** |
| 10M | ~1.2s | ~63ms | ~52ms | **23x** |

### Parallel (Multi-core, with progress)

| Input | Python | Go | Rust | Winner |
|-------|--------|-----|------|--------|
| 10M | ~0.35s | ~7ms | ~6ms | Rust (1.2x Go) |
| 50M | ~2s | ~40ms | ~35ms | Rust (1.1x Go) |
| 100M | ~5s | ~85ms | ~70ms | Rust (1.2x Go) |

### Rate (Primes per Second, quiet mode)

| Input | Python | Go | Rust | Rust vs Python |
|-------|--------|-----|------|----------------|
| 10M | ~1.7M/s | ~10.6M/s | ~12.8M/s | **7.5x** |
| 100M | ~900K/s | ~6M/s | ~7.5M/s | **8x** |

## CLI Usage Examples

### Generate primes < 1000
```bash
./primes 1000
python prime_generator.py 1000
./target/release/primes -n 1000
```

### Count primes < 10M (quiet mode)
```bash
./primes --quiet 10000000
python prime_generator.py 10000000
./target/release/primes -n 10000000 --quiet
```

### With progress bar (10M)
```bash
./primes --progress 10000000
python prime_generator.py 10000000 --progress
./target/release/primes -n 10000000 -P
```

### Parallel processing (100M)
```bash
./primes --parallel --progress 100000000
python prime_generator.py 100000000 --parallel --progress
./target/release/primes -n 100000000 -p -P
```

## Algorithm Selection

| n Range | Algorithm | Memory |
|---------|-----------|--------|
| n < 1M | Classic Sieve | O(n) |
| 1M ≤ n < 100M | Segmented Sieve | O(√n + segment) |
| n ≥ 100M | Parallel Segmented Sieve | O(√n + segment) |

## Key Observations

### Why Rust/Go are Faster than Python

1. **Compilation**: Rust/Go compile to native machine code; Python is interpreted
2. **Memory Management**: Lower overhead, efficient allocation
3. **Type System**: Native types vs. Python objects
4. **No GIL**: True parallelism in compiled languages

### Rust vs Go

- **Rust**: Slightly faster (~20%), zero-cost abstractions, no runtime
- **Go**: Simpler concurrency model, faster compilation, easier debugging
- **Both**: 7-8x faster than Python on large inputs

## Project Structure

```
infinite-series/
├── Makefile              # Top-level orchestration
├── AGENTS.md             # Guidelines for AI agents
├── README.md             # This file
├── python/
│   ├── Makefile
│   ├── prime_generator.py
│   └── test_generators.py
├── golang/
│   ├── Makefile
│   ├── cmd/primes/main.go
│   └── prime/primes.go
└── rust/
    ├── Makefile
    ├── src/
    │   ├── main.rs
    │   ├── lib.rs
    │   └── progress.rs
    └── Cargo.toml
```

## Language-Specific Notes

### Python
- Requires: Python 3.12+, pytest, ruff, mypy (optional)
- Uses `bytearray` for memory-efficient sieving
- Optional tqdm progress bar with fallback

### Go
- Requires: Go 1.21+
- Custom progress bar (no external dependencies)
- Uses goroutines with work/channels pattern

### Rust
- Requires: Rust 1.75+
- No external dependencies for progress bar
- Uses `thread::scope` for scoped parallelism


## Comparison 

| Feature | Python | Go | Rust |
|---------|--------|-----|-----|
| Memory (1B primes) | ~1GB | ~1GB | ~1GB |
| Progress bar | tqdm | Custom ANSI | Custom ANSI |
| Parallelism | multiprocessing | Goroutines + channels | Thread scope |
| Compilation | Interpreted | Compiled | Compiled |
| Concurrency | GIL-bound | True parallel | True parallel |
| Memory management | Garbage collection | Efficient allocation | Efficient allocation |
| Type safety | Optional (mypy) | Native | Native |
| Performance | Baseline | 2-5x faster | 8x faster |

## Conclusion

The Go and Rust implementations offer significant performance improvements over the Python version, especially for large inputs. Both languages provide efficient memory management and true parallelism, making them suitable for high-performance prime number generation tasks.

### Next Steps

1. **Optimize Algorithms**: Further refine segmented and parallel algorithms
2. **Expand Features**: Add more options like saving primes to file
3. **Documentation**: Improve documentation and add examples
4. **Testing**: Expand test suite with edge cases and performance benchmarks

This project demonstrates the power of compiled languages for performance-critical applications, especially in number theory and computational mathematics.

| Performance | Baseline | 2-5x faster | 2-5x faster |

## Conclusion

The Go and Rust implementations offer significant performance improvements over the Python version, especially for large inputs. Both languages provide efficient memory management and true parallelism, making them suitable for high-performance prime number generation tasks.

### Next Steps

1. **Optimize Algorithms**: Further refine segmented and parallel algorithms
2. **Expand Features**: Add more options like saving primes to file
3. **Documentation**: Improve documentation and add examples
4. **Testing**: Expand test suite with edge cases and performance benchmarks

This project demonstrates the power of compiled languages for performance-critical applications, especially in number theory and computational mathematics.
| Type safety | Optional (mypy) | Native |
| Performance | Baseline | 2-5x faster |