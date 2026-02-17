# Rust Prime Number Generator

High-performance prime number generator with CLI and interactive GUI visualization.

## Quick Start

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

## GUI Visualization

Run `cargo run --bin primes_gui` to launch the interactive visualization.

### Visualization Types

| Visualization | Description |
|--------------|-------------|
| **Ulam Spiral** | Classic diagonal prime pattern - primes form distinctive diagonal lines (Stanislaw Ulam, 1963) |
| **Sacks Spiral** | Archimedean spiral (radius = sqrt(n)) - reveals curved patterns in prime distribution (Robert Sacks, 1994, numberspiral.com) |
| **Grid** | Square grid layout starting from top-left - simple Cartesian view |
| **Row** | Single horizontal number line - shows distribution along a line |
| **Prime Wheel** | Concentric rings by modulo - primes cluster on spokes coprime to the modulus |

### GUI Parameters

- **Cell Spacing** - Controls dot size/spacing (Ulam Spiral, Sacks Spiral, Grid)
- **Modulo** - Ring modulus (Prime Wheel: try 6, 30, 210 to see different prime patterns)

## Algorithm Selection

Auto-selects the best algorithm based on input size:

| n Range | Algorithm | Memory |
|---------|-----------|--------|
| n < 1M | Classic Sieve (odd-only) | O(n/2) |
| 1M <= n < 100M | Segmented Sieve (odd-only) | O(sqrt(n) + segment/2) |
| n >= 100M | Parallel Segmented Sieve (odd-only) | O(sqrt(n) + segment/2) per worker |

## Performance

| Input | Time | Rate |
|-------|------|------|
| 1M | ~5ms | ~200M/s |
| 10M | ~52ms | ~192M/s |
| 100M | ~700ms | ~143M/s |

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
