# Rust Prime Number Generator and Visualizer

## Overview

High-performance prime number generator with CLI and interactive visualizater GUI

![Rust Prime Number Visualizer Screenshot](resources/rust_prime_visualizer_sacks_spiral_screenshot.png)

## Key Features

### CLI

High performance prime generation using a parallel segmented sieve and highly optimized code. Output approaching 200M primes/s (on an AMD 7950x and Apple M3 Ultra).

| Input | Time | Rate |
|-------|------|------|
| 1M | ~5ms | ~200M/s |
| 10M | ~52ms | ~192M/s |
| 100M | ~700ms | ~143M/s |


### GUI
Prime distribution visualizations including:

| Visualization | Description |
|--------------|-------------|
| **Ulam Spiral** | Classic diagonal prime pattern - primes form distinctive diagonal lines (Stanislaw Ulam, 1963) |
| **Sacks Spiral** | Archimedean spiral (radius = sqrt(n)) - reveals curved patterns in prime distribution (Robert Sacks, 1994, numberspiral.com) |
| **Grid** | Square grid layout starting from top-left - simple Cartesian view |
| **Row** | Single horizontal number line - shows distribution along a line |
| **Prime Wheel** | Concentric rings by modulo - primes cluster on spokes coprime to the modulus |
| **Prime Density** | Graph of π(x) vs x/ln(x) - visualizes the Prime Number Theorem (prime counting function vs approximation) |
| **Riemann Zeta** | Critical strip plot showing non-trivial zeros on the critical line σ=0.5 - visualizes the connection between prime distribution and the Riemann Hypothesis |
| **Hexagonal Lattice** | Hexagonal lattice spiral - symmetric 6-direction spiral on hexagonal grid (60° intervals) |
| **Triangular Lattice** | Triangular lattice spiral - symmetric 3-direction spiral on triangular grid (120° intervals) |
| **Fermat's Spiral** | Phyllotaxis spiral - golden angle placement (r = sqrt(n), theta = n * 137.5°), same pattern as sunflower seed arrangements |
| **Sacks Mobius Spiral** | Archimedean spiral using prime index with gap-colored lines (white=close, gray=far) |
| **Ulam Mobius Spiral** | Square-grid spiral using prime index with gap-colored lines (white=close, gray=far) |
| **Prime Density Gradient** | Heatmap grid showing local prime density across the number space |


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
cargo run --bin primes_cli -- -n 1000000 --quiet

# Run GUI
cargo run --bin primes_gui

# Run tests
cargo test
```

## CLI Usage

```bash
# Generate primes < 1000
cargo run --bin primes_cli -- 1000

# Count primes < 1M (quiet mode)
cargo run --bin primes_cli -- -n 1000000 --quiet

# With progress bar
cargo run --bin primes_cli -- -n 10000000 -P

# Parallel processing (for n >= 100M)
cargo run --bin primes_cli -- -n 100000000 -p -P
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

## GUI Usage

Run `cargo run --bin primes_gui` to launch the interactive visualization.

## Algorithm Selection

Auto-selects the best algorithm based on input size:

| n Range | Algorithm | Memory |
|---------|-----------|--------|
| n < 1M | Classic Sieve (odd-only) | O(n/2) |
| 1M <= n < 100M | Segmented Sieve (odd-only) | O(sqrt(n) + segment/2) |
| n >= 100M | Parallel Segmented Sieve (odd-only) | O(sqrt(n) + segment/2) per worker |

## Project Structure

```
rust/
├── Cargo.toml
├── Makefile
└── src/
    ├── lib.rs              # Core implementation (Sieve algorithms)
    ├── primes_cli.rs       # CLI entry point
    ├── primes_gui.rs       # GUI visualization
    └── progress.rs         # Progress bar
```

## Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run specific binary
cargo run --bin primes_cli
cargo run --bin primes_gui
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
