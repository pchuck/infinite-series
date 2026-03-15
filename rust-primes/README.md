# Rust Prime Number Generator

## Overview

A high-performance prime number generator with CLI.

## Key Features

### CLI

High performance prime generation using a parallel segmented sieve and highly optimized code.

Performance varies by hardware:
- Apple M3 Ultra: ~346M primes/s (parallel, 100M)
- AMD Ryzen 9 7950X: ~67M primes/s (parallel, 100M)  
- AMD Ryzen 9 7900X: ~55M primes/s (parallel, 100M), ~29M/s (sequential)

Note: CLI performance includes I/O overhead. Micro-benchmarks (using Criterion) show higher throughput (7900X):
- Classic sieve: 542 Melem/s - 1.34 Gelem/s
- Segmented sieve: 520 Melem/s - 613 Melem/s

| Input | Hardware | Parallel | Time | Rate |
|-------|----------|----------|------|------|
| 1M | AMD 7900X | ✗ | ~5ms | ~16M/s |
| 10M | AMD 7900X | ✗ | ~24ms | ~27M/s |
| 100M | AMD 7900X | ✗ | ~199ms | ~29M/s |
| 100M | AMD 7900X | ✓ (24t) | ~106ms | ~55M/s |
| 100M | AMD 7950X | ✓ (24t) | ~86ms | ~67M/s |
| 100M | Apple M3 Ultra | ✓ (24t) | ~17ms | ~346M/s |

**Notes:**
- Performance varies by hardware and CPU architecture
- Parallel processing uses all available threads by default
- I/O overhead affects CLI timing; micro-benchmarks show pure computation performance
- For n < 100M, parallel flag is ignored (warning shown)


## Quick Start

### Make
```bash
make help
make release            # Build optimized
make run-release        # Run (n=10000)
make test               # Test
```

Note: Timing claims above reflect measured performance on specific hardware. Actual times vary by CPU architecture.

### Cargo

```bash
# Build
cargo build

# Run CLI (generate primes < 1M, quiet mode)
cargo run -- -n 1000000 --quiet

# Run tests
cargo test

# Run with progress bar
cargo run -- -n 10000000 -P

# Run parallel (for n >= 100M)
cargo run -- -n 100000000 -p
```

## CLI Usage

```bash
# Generate primes < 1000
cargo run -- 1000

# Count primes < 1M (quiet mode)
cargo run -- -n 1000000 --quiet

# With progress bar
cargo run -- -n 10000000 -P

# Parallel processing (for n >= 100M)
cargo run -- -n 100000000 -p -P
```

### CLI Options

| Option | Description |
|--------|-------------|
| `-n, --n` | Upper bound (exclusive) for prime generation |
| `-P, --progress` | Show progress bar |
| `-p, --parallel` | Use parallel processing (for n >= 100M) |
| `-w, --workers` | Number of worker threads (default: all available) |
| `--segment` | Segment size for segmented sieve (default: 1M) |
| `-q, --quiet` | Only print count (no prime list) |

**Notes:**
- Parallel processing automatically enabled for n >= 100M when `-p` flag is used
- Progress bar shows segments processed, not percentage of primes found
- Quiet mode is useful for scripting and benchmarking without I/O overhead

## GUI Visualization

For interactive visualization, see [../rust-gui](../rust-gui/README.md).

## Algorithm Selection

Auto-selects the best algorithm based on input size:

| n Range | Algorithm | Memory |
|---------|-----------|--------|
| n < 1M | Classic Sieve (odd-only) | O(n/2) |
| 1M <= n < 100M | Segmented Sieve (odd-only) | O(sqrt(n) + segment/2) |
| n >= 100M | Parallel Segmented Sieve (odd-only) | O(sqrt(n) + segment/2) per worker |

**Notes:**
- Parallel processing automatically selects segment size based on input

## Project Structure

```
rust-primes/
├── Cargo.toml
├── Makefile
└── src/
    ├── lib.rs              # Core implementation (Sieve algorithms)
    ├── primes_cli.rs       # CLI entry point
    └── progress.rs         # Progress bar
```

## Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

## Performance Benchmarks

For detailed performance benchmarks, run:

```bash
cargo bench
```

See `benches/prime_benchmarks.rs` for benchmark definitions.

### Benchmark Results

Recent results (AMD 7900X, 24 threads):
- **parallel_large/n_100000000**: ~64ms (1.58 Gelem/s)
- **parallel_large/n_200000000**: ~115ms (1.74 Gelem/s)
- **segmented_large/n_100000000**: ~184ms (543 Melem/s)
- **segmented_large/n_200000000**: ~383ms (524 Melem/s)

Parallel processing is ~3x faster than sequential for large inputs.

## Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_sieve_small

# Run with output
cargo test -- --nocapture

# Run clippy linter
cargo clippy

# Format code
cargo fmt
```

## Contributing

Performance contributions are welcome! When submitting benchmarks:
- Include hardware specifications (CPU, threads)
- Report both sequential and parallel results
- Note whether I/O is included in timing

### Memory Optimization

The current implementation uses an "odd-only" sieve (stores only odd numbers)
for 2x memory savings. Future improvements could use bit-packed arrays for
additional memory efficiency at the cost of some computation overhead.
