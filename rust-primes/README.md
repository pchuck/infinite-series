# Rust Prime Number Generator

## Overview

A high-performance prime number generator with CLI.

## Key Features

### CLI

High performance prime generation using a parallel segmented sieve and highly optimized code. Output approaching 200M primes/s (on an AMD 7950x and Apple M3 Ultra).

| Input | Time | Rate |
|-------|------|------|
| 1M | ~5ms | ~200M/s |
| 10M | ~52ms | ~192M/s |
| 100M | ~700ms | ~143M/s |


## Quick Start

### Make
```bash
make help
make release            # Build optimized
make run-release        # Run (n=1000)
make run-release-quiet  # Count primes < 1M
make test               # Test
```

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
