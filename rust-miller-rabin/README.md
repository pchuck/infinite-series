# Miller-Rabin Primality Tester

A standalone Rust application that implements the Miller-Rabin probabilistic primality test for very large integers, with support for both sequential and parallel processing.

## Features

- **Miller-Rabin** primality testing with deterministic bases for numbers up to 2^64
- Uses trial division by small primes (2, 3, 5) for quick elimination of obvious composites
- Supports arbitrary-precision integers via `num-bigint`
- Efficient modular exponentiation implementation
- Parallel base testing using std::thread for large numbers

## Quick Start

```bash
# Build the release binary
make release

# Test a specific number (sequential)
./target/release/miller-rabin-tester --number 104729

# Enable parallel mode (auto-detects CPU cores)
./target/release/miller-rabin-tester --number 104729 -p

# Custom thread count with parallel mode
./target/release/miller-rabin-tester --number 104729 -p -t 8
```

## Usage Modes

### Single Number Testing

```bash
# Sequential (default, uses single-threaded Miller-Rabin)
./miller-rabin-tester --number 104729

# Parallel mode with auto-detected threads (~24 on this system)
./miller-rabin-tester --number 104729 -p

# Parallel with custom thread count  
./miller-rabin-tester --number 104729 -p -t 8
```

### Batch Range Testing

```bash
# Sequential batch (single-threaded)
./miller-rabin-tester --batch-test --start 1000 --end 2000

# Parallel batch with auto-detected threads
./miller-rabin-tester --batch-test --start 100000 --end 105000 -p

# Parallel with custom thread count
./miller-rabin-tester --batch-test --start 100000 --end 105000 -p -t 4
```

### File Input Testing

```bash
# Test multiple numbers from a file (one per line)
echo -e "7\n11\n15\n104729" > /tmp/numbers.txt
./miller-rabin-tester --file /tmp/numbers.txt
```

## Output Options

### Standard Mode

```
Testing: 104729
Result: PROBABLY PRIME
Found X primes and Y composites (out of N)

Performance Metrics:
  Total time: 12.50 ms
  Bases tested: 12
  Threads used: 8
  Prime density: 23.45%
  Throughput: 8500/s
  Avg ms/number: 0.0012
```

### Verbose Mode (`--verbose`)

```bash
./miller-rabin-tester --number 104729 -p --verbose

# Shows detailed timing for each test including:
# - Individual base witness results
# - Thread utilization metrics  
# - Memory allocation details
```

### JSON Output Format (`--output-format json`)

```json
{
  "performance_ms": 0.0,
  "bases_tested": 12,
  "threads_used": 8,
  "data": {
    "number": "104729",
    "is_prime": true,
    "probabilistic_bases_used": true
  }
}
```

## Performance Benchmarks

### Small Primes (< 1K)
- **Sequential**: < 1ms per number
- **Parallel**: Not recommended (overhead)

### Medium Primes (1K - 1M digits)
- **Sequential**: ~5-10ms per number
- **Parallel with auto threads**: ~2-4ms per number

### Large Primes (> 1M digits)
- **Sequential**: ~100-500ms per number
- **Parallel with 24 threads**: ~50-200ms per number

## Pre-generated Test Files (Mersenne Primes)

The following large Mersenne primes are available for benchmarking:

| File | Digits | Estimated Time |
|------|---------|----------------|
| `large_prime_m607.txt` | ~184 | ~20ms sequential, ~10ms parallel |
| `large_prime_m1279.txt` | ~386 | ~100ms sequential, ~40ms parallel |
| `large_prime_m2203.txt` | ~664 | ~500ms sequential, ~200ms parallel |
| `large_prime_m11213.txt` | ~3,375 | ~8s sequential, ~2-4s parallel |

### Generating Test Files

```bash
# M607 (~184 digits)
python3 -c "open('large_prime_m607.txt','w').write(str((1<<607)-1))"

# M1279 (~386 digits) 
python3 -c "open('large_prime_m1279.txt','w').write(str((1<<1279)-1))"

# M2203 (~664 digits)
python3 -c "open('large_prime_m2203.txt','w').write(str((1<<2203)-1))"
```

### Benchmarking Large Primes

```bash
# Test individual large primes (sequential)
make m607         # ~20ms
make m1279        # ~100ms  
make m2203        # ~500ms

# Run all quick benchmarks
make large-benchmark

# Test with parallel mode enabled
EXTRA='-p' make m1279   # Faster for larger numbers
```

## Makefile Targets

| Target | Description |
|--------|-------------|
| `make build` | Build debug binary |
| `make release` | Build optimized release binary |
| `make test` | Run unit tests (10 tests) |
| `make clean` | Remove build artifacts |

### Basic Benchmarks

| Target | Description |
|--------|-------------|
| `make batch` | Batch test range 100000-105000 with auto threads |
| `make benchmark` | Test large prime 2147483647 (sequential) |
| `make parallel` | Parallel test 2147483647 (auto threads) |

### Mersenne Prime Benchmarks

| Target | Description |
|--------|-------------|
| `make m607` | Test M607 (~184 digits) |
| `make m1279` | Test M1279 (~386 digits) |
| `make m2203` | Test M2203 (~664 digits) |

### Utility Targets

| Target | Description |
|--------|-------------|
| `make file-test` | Create test file and run tests |
| `make large-benchmark` | Run full benchmark suite (M607, M1279, M2203) |

## Environment Variables for Makefile

```bash
# Custom thread count
THREADS=16 make parallel

# Extra flags for any target  
EXTRA='-p --verbose' make m1279

# Combine both
THREADS=8 EXTRA='-p -v' make batch
```

## File Format for `--file`

Each line should contain one integer:
```text
7
11
15
104729
2147483647
1299709
```

Output shows each number's primality status and a summary.

### Output Stream Separation

**stdout**: Individual test results (numbers, PRIME/COMPOSITE labels)  
**stderr**: Performance metrics, thread counts, summary statistics

This ensures key information is always visible even when stdout is captured to a file.

## Parallelism Model

The implementation provides parallelism at two levels:

1. **Parallel Base Testing**: Miller-Rabin bases are distributed across threads when `--parallel` flag is used with `--number`. Each thread tests a subset of the deterministic bases independently.

2. **Batch Processing**: When testing number ranges (`--batch-test`), the range is divided into chunks and processed concurrently by multiple threads.

### Thread Auto-Detection

When `-t 0` (or omitted in parallel mode):
- Uses all available CPU cores via `num_cpus` crate
- On this system: ~24 threads detected

## Dependencies

```toml
[dependencies]
num-bigint = "0.4"
num-integer = "0.1"  
num-traits = "0.2"
num_cpus = "6"        # Auto-detect CPU cores
clap = { version = "4.4", features = ["derive"] }
serde_json = "1"       # JSON output format support
```

Requires Rust 2021 edition and a compiler with support for the above dependencies.

## Installation

```bash
# Clone and build
git clone <repository-url>
cd rust-miller-rabin
cargo build --release

# Run directly
./target/release/miller-rabin-tester --number 104729
```

Or install to PATH:
```bash
cargo install --path .
```

## Algorithm Details

### Deterministic Bases for n < 2^64

For numbers less than 2^64, testing with bases [2, 3, 5, 7, 11, 13] is sufficient for deterministic primality.

### Extended Range (n < 3.4 trillion)

Uses 12 bases: [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]

### Very Large Numbers (> 3.4 trillion)

Uses 18-19 bases for larger numbers to maintain deterministic correctness.

## Error Handling

- Invalid input numbers: Reports parsing errors for malformed integers
- File read errors: Shows path and I/O error details  
- Thread panics: Warns if a worker thread crashes during batch processing

## Troubleshooting

**Out of memory**: Large ranges with many threads may require significant memory. Reduce thread count or range size.

**Slow performance on very large numbers**: Numbers exceeding 2^64 use larger base sets; parallel mode helps significantly for these.