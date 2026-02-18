# Python Prime Generators

A Python project containing optimized prime generation utilities.

## Features

### Prime Generator
- **Auto-Algorithm Selection**: Automatically chooses between classic sieve, segmented sieve, and parallel segmented sieve based on input size for optimal performance
- **Memory-Efficient Segmented Sieve**: Processes primes in segments to reduce memory from O(n) to O(sqrt n), enabling generation of primes up to billions
- **Parallel Processing**: Multi-core CPU utilization for very large inputs (≥500M) with configurable worker processes
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

The prime generator uses the Sieve of Eratosthenes algorithm, which is significantly faster than trial division:
- **Small numbers (100)**: 2.5x faster
- **Medium numbers (1,000)**: 2.7x faster
- **Large numbers (10,000)**: 6.2x faster
- **Very large numbers (100,000)**: 8.3x faster

Real-world performance
- 5.3M primes/s on an M3 Ultra

See [Performance](PERFORMANCE.md)

### Memory Efficiency

The segmented sieve reduces memory usage from O(n) to O(sqrt n):
- Generates primes up to 1B with only ~32KB memory for base primes
- Processes in segments of configurable size (default: 1M)

### CPU Parallel Processing

Available for very large inputs (≥ 500M):
- Uses `cpu_count() - 1` worker processes by default
- Progress tracking via shared counter
- Note: Speedup depends on CPU cores, cache locality, and input size
- In environments with limited CPUs or high multiprocessing overhead, sequential may be faster

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
├── prime_generator.py      # Prime number generator (optimized)
├── test_generators.py      # Comprehensive test suite
├── performance_comparison.py  # Performance benchmark tool (old vs optimized)
├── parallel_comparison.py     # Parallel vs sequential comparison
├── OPTIMIZATION_SUMMARY.md  # Detailed optimization notes
└── README.md               # This file
```

## Code Style

- Type hints for all functions
- Follows existing code conventions
- Passes ruff linting


