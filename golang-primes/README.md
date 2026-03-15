# Go Prime Number Generator

A high-performance prime number generator implemented in Go, ported from the Python implementation.

## Features

- **Algorithms**: Classic Sieve, Segmented Sieve, and Parallel Segmented Sieve
- **Auto-algorithm selection**: Automatically chooses optimal algorithm based on input size
- **Memory-efficient**: Segmented sieve reduces memory from O(n) to O(sqrt(n))
- **Parallel processing**: Multi-core CPU utilization for very large inputs
- **Progress bar**: Lightweight custom progress indicator
- **No external dependencies**: Uses only Go standard library

## Quick Start
```bash
make help
make build            # Build binary
make run-progress     # With progress bar
make run-progress-parallel N=10000000  # Parallel + progress
make test             # Run tests
```

## Installation

```bash
go build -o primes ./cmd/primes
```

## Usage

### Command Line

```bash
# Generate primes less than 100
./primes 100

# With progress bar
./primes 1000000 --progress

# With parallel processing (for large n >= 100M)
./primes 100000000 --parallel

# Specify workers and segment size
./primes 100000000 --parallel --workers 8 --segment 1000000

# Interactive mode
./primes
```

### Flags

| Flag | Description | Default |
|------|-------------|---------|
| `-n` | Upper bound (exclusive) | Interactive input |
| `--progress` | Show progress bar | false |
| `--parallel` | Use parallel processing | false |
| `--workers` | Number of worker processes | NumCPU |
| `--segment` | Segment size for segmented sieve | 1,000,000 |

### Programmatic Usage

```go
package main

import (
    "fmt"
    "github.com/pchuck/infinite-series/golang-prime"
)

func main() {
    // Auto-select algorithm based on n
    primes := prime.GeneratePrimes(1000000, false, nil)
    fmt.Printf("Found %d primes\n", len(primes))

    // Use specific algorithm
    primes = prime.SieveOfEratosthenes(10000)
    primes = prime.SegmentedSieve(10000000, 1000000, nil)
    primes = prime.ParallelSegmentedSieve(100000000, 8, 1000000, nil)
}
```

## Algorithms

### Classic Sieve of Eratosthenes
- **Best for**: n < 1,000,000
- **Memory**: O(n)
- **Time**: O(n log log n)

### Segmented Sieve
- **Best for**: n >= 1,000,000
- **Memory**: O(sqrt(n) + segment_size)
- **Time**: O(n log log n)

### Parallel Segmented Sieve
- **Best for**: n >= 100,000,000
- **Uses**: Multiple CPU cores via goroutines
- **Workers**: Default = NumCPU

## Performance

Benchmarks on typical hardware:

| n | Algorithm | Time | Memory |
|---|-----------|------|--------|
| 1,000 | Classic | ~0.1ms | ~1KB |
| 100,000 | Classic | ~10ms | ~100KB |
| 1,000,000 | Segmented | ~50ms | ~1MB |
| 10,000,000 | Segmented | ~500ms | ~10MB |
| 100,000,000 | Parallel | ~2s | ~100MB |
| 1,000,000,000 | Parallel | ~20s | ~1GB |

## Testing

```bash
# Run all tests
go test ./prime/... -v

# Run with benchmarks
go test ./prime/... -bench=.

# Run specific benchmark
go test ./prime/... -bench=BenchmarkGeneratePrimes

# Run all tests with memory profiling
go test ./prime/... -bench=. -benchmem -memprofile=mem.out
```

## Project Structure

```
golang/
├── go.mod
├── README.md
├── prime/
│   ├── primes.go                    # Core algorithms
│   ├── primes_test.go                # Unit tests
│   └── primes_benchmark_test.go      # Performance benchmarks
├── internal/
│   └── progress/
│       └── progress.go               # Lightweight progress bar
└── cmd/
    └── primes/
        └── main.go                   # CLI entrypoint
```

