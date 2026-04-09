# Python Prime Generators

A Python project containing optimized prime generation utilities.

## Features

### Prime Generator
- **Auto-Algorithm Selection**: Automatically chooses between classic sieve, segmented sieve, and parallel segmented sieve based on input size for optimal performance
- **Memory-Efficient Segmented Sieve**: Processes primes in segments to reduce memory from O(n) to O(sqrt n), enabling generation of primes up to billions
- **Parallel Processing**: Multi-core CPU utilization for very large inputs (≥1B) with configurable worker processes
- **Performance Metrics**: Built-in timing and throughput statistics (primes/second) output to stderr
- **Smart Fallbacks**: Gracefully degrades to sequential processing if parallel processing fails
- Supports both command-line arguments and interactive input

## Installation

No external dependencies required for basic usage. Uses only Python standard library.

Optional dependency:
- `tqdm>=4.65.0` - For progress bars (install with `pip install tqdm`)


## Quick Start

```bash
make help
make run-progress     # With progress bar
make run-progress-parallel  # Parallel + progress
make test             # Run tests
make lint             # Run ruff linter
```


## Usage

### Prime Generator

```bash
# Using command-line argument
python prime_generator.py 100

# With progress bar
python prime_generator.py 1000000 --progress

# With CPU parallel processing (for large inputs >= 500M)
python prime_generator.py 500000000 --parallel --progress

# Interactive mode
python prime_generator.py
# Then enter a number when prompted
```

## Performance

The prime generator uses the Sieve of Eratosthenes algorithm with several optimizations:
- Odd-only sieve (2x memory/work reduction)
- Bytearray with slice assignment
- Memory-efficient segmented sieve for large inputs
- Optional parallel processing for inputs ≥1B

Real-world performance:
- ~5.3M primes/s on an M3 Ultra (n=5B)
- Memory usage: O(sqrt(n)) for segmented sieve

See [Performance](PERFORMANCE.md) for detailed benchmarks.

### Memory Efficiency

The segmented sieve reduces memory usage from O(n) to O(sqrt n):
- Generates primes up to 1B with only ~32KB memory for base primes
- Processes in segments of configurable size (default: 1M)

### CPU Parallel Processing

Available for very large inputs (≥ 1B):
- Uses `cpu_count() // 4` worker processes by default
- Progress tracking via shared counter
- Note: Parallel processing has significant overhead from process spawning,
  inter-process communication, and result merging. On systems with fewer cores
  or for inputs below ~1B, sequential processing may be faster or equivalent.

## Testing

All tests verify:
- Correctness: Parallel produces identical results to sequential
- Edge cases: n <= 2, worker counts, segment boundaries
- Progress tracking: Shared counter updates correctly

```bash
# Run all tests
python -m pytest test_generators.py -v

# Run with coverage
python -m pytest test_generators.py --cov=. --cov-report=html

# Compare algorithms:
python performance_comparison.py
```

## Project Structure

```
.
├── prime_generator.py           # Prime number generator (optimized)
├── test_generators.py           # Comprehensive test suite
├── performance_comparison.py    # Sieve vs trial division benchmark
├── parallel_comparison.py       # Sequential vs parallel benchmark
├── find_optimal_threshold.py    # Threshold tuning utility
├── PERFORMANCE.md               # Performance benchmarks
├── PARALLEL.md                  # Parallel processing documentation
└── README.md                    # This file
```

## Code Style

- Type hints for all functions
- Follows existing code conventions
- Passes ruff linting


