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

Note: CLI performance includes I/O overhead. Micro-benchmarks (using Criterion) show higher throughput:
- Classic sieve: 333 Melem/s - 1.34 Gelem/s
- Segmented sieve: 333 Melem/s - 613 Melem/s

| Input | Hardware | Parallel | Time | Rate |
|-------|----------|----------|------|------|
| 1M | AMD 7900X | ✗ | ~5ms | ~16M/s |
| 10M | AMD 7900X | ✗ | ~24ms | ~27M/s |
| 100M | AMD 7900X | ✗ | ~199ms | ~29M/s |
| 100M | AMD 7900X | ✓ (24t) | ~106ms | ~55M/s |
| 100M | AMD 7950X | ✓ (24t) | ~86ms | ~67M/s |
| 100M | Apple M3 Ultra | ✓ (24t) | ~17ms | ~346M/s |


## Quick Start

### Make
```bash
make help
make release            # Build optimized
make run-release        # Run (n=1000)
make run-release-quiet  # Count primes < 1M
make test               # Test
```

Note: Timing claims above reflect measured performance on specific hardware. Actual times vary by CPU architecture.

### Cargo

```bash
# Build
cargo build

# Run CLI
cargo run -- -n 1000000 --quiet

# Run tests
cargo test
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
| `-p, --parallel` | Use parallel processing |
| `-w, --workers` | Number of worker threads |
| `--segment` | Segment size for segmented sieve |
| `-q, --quiet` | Only print count (no prime list) |

## GUI Visualization

For interactive visualization, see [../rust-gui](../rust-gui/README.md).

## Algorithm Selection

Auto-selects the best algorithm based on input size:

| n Range | Algorithm | Memory |
|---------|-----------|--------|
| n < 1M | Classic Sieve (odd-only) | O(n/2) |
| 1M <= n < 100M | Segmented Sieve (odd-only) | O(sqrt(n) + segment/2) |
| n >= 100M | Parallel Segmented Sieve (odd-only) | O(sqrt(n) + segment/2) per worker |

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
- Include hardware specifications
- Report both sequential and parallel results
- Note whether I/O is included in timing
