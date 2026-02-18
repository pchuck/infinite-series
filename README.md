# Prime Number Generator: Python vs Go vs Rust

## Overview

High-performance prime number generators in Python, Go, and Rust with consistent APIs and comparable algorithms. All three implementations use odd-only segmented sieves for optimal memory and cache utilization.

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

### Rust GUI (Prime Visualizer)
```bash
cd rust
cargo run --bin primes_gui    # Run GUI
```

The GUI provides interactive visualizations of prime number distributions:

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

**Parameters:**
- **Prime Size** / **Non-Prime Size** - Controls dot sizes (Ulam Spiral, Sacks Spiral, Grid, Row, Prime Wheel, Hexagonal Lattice, Triangular Lattice, Fermat's Spiral, Sacks Mobius Spiral, Ulam Mobius Spiral)
- **Show Numbers** - Display numbers on the visualization (when zoomed in)
- **Modulo** - Ring modulus (Prime Wheel: try 6, 30, 210 to see different prime patterns)
- **Zeros** - Number of zeros to display (Riemann Zeta: 1-20)

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

## CLI Usage Examples

### Generate primes < 1000
```bash
./primes 1000                          # Go
python prime_generator.py 1000         # Python
./target/release/primes -n 1000        # Rust
```

### Count primes < 10M (quiet mode)
```bash
./primes --quiet 10000000              # Go
python prime_generator.py 10000000 --quiet  # Python
./target/release/primes -n 10000000 --quiet # Rust
```

### With progress bar (10M)
```bash
./primes --progress 10000000           # Go
python prime_generator.py 10000000 --progress  # Python
./target/release/primes -n 10000000 -P # Rust
```

### Parallel processing (100M)
```bash
./primes --parallel --progress 100000000           # Go
python prime_generator.py 100000000 --parallel --progress  # Python
./target/release/primes -n 100000000 -p -P         # Rust
```

## Algorithm Selection

Each implementation auto-selects the best algorithm based on input size:

| n Range | Algorithm | Memory |
|---------|-----------|--------|
| n < 1M | Classic Sieve (odd-only) | O(n/2) |
| 1M <= n < 100M | Segmented Sieve (odd-only) | O(sqrt(n) + segment/2) |
| n >= 100M | Parallel Segmented Sieve (odd-only) | O(sqrt(n) + segment/2) per worker |

All algorithms skip even numbers, halving both memory usage and composite-marking work compared to a full sieve.

## Performance Optimizations

### Implemented Optimizations

All three implementations share the same core optimization strategy:

| Optimization | Python | Go | Rust | Impact |
|--------------|--------|-----|------|--------|
| Odd-only sieve | yes | yes | yes | 2x memory reduction, ~2x faster marking |
| Shared segment helper (DRY) | yes | yes | yes | Single sieving implementation, no duplication |
| SIMD-optimized extraction | `bytes.find()` | `bytes.IndexByte` | iterator | Hardware-accelerated prime extraction |
| Pre-allocated result vectors | yes | yes | yes | Reduces reallocations |
| Efficient buffer reset | slice assign | for loop | `.fill()` | Zero-allocation per segment |
| Delta-based progress callbacks | yes | yes | yes | Correct progress tracking |
| O(n) parallel result merging | `heapq.merge` | indexed collect | ordered concat | Avoids O(n log n) sort |
| Shared base primes (no copy) | N/A (pickle) | shared slice | `&[usize]` ref | Zero-copy in parallel workers |
| Bounded channel buffers | N/A | `numWorkers*2` | N/A | Limits peak memory in parallel mode |
| Streamed output | generator join | `strings.Builder` | `BufWriter` | Avoids huge in-memory string |

### Language-Specific Details

**Python:**
- `bytearray` with slice assignment for fast composite marking
- `memoryview` for zero-copy segment operations
- `math.isqrt()` for exact integer square root (no float imprecision)
- `heapq.merge()` for O(n) merging of pre-sorted parallel results
- Progress bar writes to stderr (not stdout) to avoid mixing with data
- Returns `List[int]` (not `List[str]`) to avoid millions of string allocations

**Go:**
- `bytes.IndexByte` for SIMD-optimized prime extraction (replaces hand-rolled linear scan)
- Simple `for` loop buffer reset (replaces `bytes.Repeat` which allocated per segment)
- Atomic counter with monitor goroutine for parallel progress tracking
- Bounded work/result channels to limit memory pressure
- Proper error handling on CLI input parsing

**Rust:**
- `thread::scope` allows sharing `base_primes` by reference (zero-clone)
- Each parallel worker returns a single `Vec<usize>` (no intermediate `SegmentResult` structs)
- `BufWriter` for streaming output directly to stdout
- Correct `format_number` with comma separators for all magnitudes
- `div_ceil()` for idiomatic ceiling division

### Future Optimization Candidates

1. **Bit-packed storage** -- 8x further memory reduction on top of odd-only (16x total vs original)
2. **Wheel factorization (mod 30)** -- skip multiples of 2, 3, 5 to eliminate 73% of candidates
3. **SIMD composite marking** -- pre-computed masks for small primes applied with vectorized AND
4. **Cache-optimal segment sizing** -- auto-tune segment size to fit L1/L2 cache
5. **Maintained starting offsets** -- avoid per-segment division by tracking offsets across segments

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

## Comparison

| Feature | Python | Go | Rust |
|---------|--------|-----|-----|
| Sieve type | Odd-only bytearray | Odd-only `[]byte` | Odd-only `Vec<bool>` |
| Memory (100M primes) | ~50MB sieve | ~50MB sieve | ~50MB sieve |
| Progress bar | tqdm / fallback | Custom ANSI | Custom ANSI |
| Parallelism | multiprocessing | Goroutines + channels | Thread scope |
| Compilation | Interpreted | Compiled | Compiled |
| Concurrency | Process-based | True parallel | True parallel |
| Type safety | Optional (mypy) | Native | Native |

## Project Structure

```
infinite-series/
├── Makefile              # Top-level orchestration
├── AGENTS.md             # Guidelines for AI agents
├── README.md             # This file
├── python/
│   ├── Makefile
│   ├── prime_generator.py      # Core implementation
│   ├── test_generators.py      # Test suite (37 tests)
│   ├── performance_comparison.py
│   └── parallel_comparison.py
├── golang/
│   ├── Makefile
│   ├── cmd/primes/main.go      # CLI entry point
│   ├── prime/
│   │   ├── primes.go           # Core implementation
│   │   ├── primes_test.go      # Test suite
│   │   └── primes_benchmark_test.go
│   └── internal/progress/
│       └── progress.go         # Progress bar
└── rust/
    ├── Makefile
    ├── Cargo.toml
    └── src/
        ├── lib.rs              # Core implementation (9 tests)
        ├── primes_cli.rs       # CLI entry point (1 test)
        ├── primes_gui.rs       # GUI visualization (eframe/egui)
        └── progress.rs         # Progress bar
```

## Language-Specific Notes

### Python
- Requires: Python 3.12+, pytest, ruff (optional), mypy (optional), tqdm (optional)
- Uses `bytearray` with odd-only indexing for memory-efficient sieving
- Optional tqdm progress bar with custom ANSI fallback to stderr
- `--quiet` flag for count-only output

### Go
- Requires: Go 1.21+
- Custom ANSI progress bar (no external dependencies)
- Uses goroutines with bounded work/result channels
- Atomic progress counter with monitor goroutine for parallel mode

### Rust
- Requires: Rust 1.75+
- Two binaries: `primes_cli` (CLI) and `primes_gui` (interactive visualization)
- CLI: only external dependency is `clap` for argument parsing
- GUI: uses `eframe`/`egui` for cross-platform graphics
- Custom ANSI progress bar with rate display
- Uses `thread::scope` for safe scoped parallelism with zero-copy data sharing
