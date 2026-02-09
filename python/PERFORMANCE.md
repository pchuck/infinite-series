# Performance Optimization Summary

## Optimizations

### 1. Prime Generator (`prime_generator.py`)

#### Trial Division:
- **Time Complexity**: O(n sqrt n)
- **Performance**: Slow for large numbers

#### Sieve of Eratosthenes:
- **Time Complexity**: O(n log log n)
- **Performance**: 2.7x - 8.3x faster depending on input size

#### Memory:
- Optimized memory usage with slice assignment

### 2. Test Suite (`test_generators.py`)

#### Features
- Performance benchmarks
- Tests for very large inputs (100,000 primes)
- Verifies < 1 second for 100,000 primes

### 3. Performance Comparison Tool (`performance_comparison.py`)

#### Features:
- Benchmarks Trial Division vs Sieve
- Shows speed ratios
- Demonstrates performance gains at different scales

## Relative Performance

| Input Size | Unoptimized | Optimized      | Speedup |
|------------|-------------|----------------|---------|
| 100        | 0.0001s     | 0.0000s        | 2.53x   |
| 1,000      | 0.0005s     | 0.0002s        | 2.70x   |
| 10,000     | 0.0052s     | 0.0008s        | 6.23x   |
| 100,000    | 0.0895s     | 0.0108s        | 8.31x   |

## Benchmarks
- ** m3 ultra **: `[PERF] n=5000000000 | primes=234954223 | time=44.543s | primes/s=5,274,722`
- ** 7900x **: [PERF] n=500000000 | primes=26355867 | time=14.489s | primes/s=1,819,013
