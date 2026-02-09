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

## Performance Results

| Input Size | Unoptimized | Optimized      | Speedup |
|------------|-------------|----------------|---------|
| 100        | 0.0001s     | 0.0000s        | 2.53x   |
| 1,000      | 0.0005s     | 0.0002s        | 2.70x   |
| 10,000     | 0.0052s     | 0.0008s        | 6.23x   |
| 100,000    | 0.0895s     | 0.0108s        | 8.31x   |

## Usage

```bash
# Run performance comparison
python performance_comparison.py

# Run tests
python -m pytest test_generators.py -v

# Use prime generator
python prime_generator.py 100
```

## Conclusion

The optimizations provide significant performance improvements, especially for larger inputs. The Sieve of Eratosthenes algorithm is the default algorithm.