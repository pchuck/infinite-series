# Prime Number Testing Reference

## Overview

This document provides information about testing primality for numbers with millions of digits, including hardware requirements and commonly used methods.

## Primality Testing Methods

### Miller-Rabin
- Fast probabilistic primality test
- Multiple rounds increase confidence
- Faster for smaller numbers, but many rounds needed for very large numbers

### Baillie-PSW
- Combination of strong probable prime test and Lucas test
- Very reliable with no known pseudoprimes
- Faster than deterministic Miller-Rabin for large numbers

### ECPP (Elliptic Curve Primality Proving)
- Provides a formal mathematical proof of primality
- Very slow, but produces certificates
- Used when a proof is required

### Lucas-Lehmer (Mersenne-specific)
- Specifically for Mersenne numbers (2^p - 1)
- Extremely fast compared to general primality testing
- Used by GIMPS to find record primes

## Hardware Requirements

| Digits | Hardware Required | Estimated Time per Test | Notes |
|--------|-------------------|------------------------|-------|
| < 20 | Basic calculator | Instant | 64-bit numbers fit in CPU registers |
| 100 | Laptop, 8GB RAM | Minutes | Standard libraries can handle |
| 1,000 | Desktop, 16GB RAM | Hours | Arbitrary-precision math needed |
| 10,000 | Server, 64GB RAM | Days | Memory grows quadratically |
| 100,000 | High-memory server, 256GB+ RAM | Weeks to Months | Specialized algorithms required |
| 1,000,000 | Distributed computing, TB RAM | Years | Requires massive resources |

## Memory Considerations

For arbitrary-precision primality testing:
- Memory usage grows with O(n^2) complexity for basic algorithms
- Optimized algorithms can reduce to O(n log n) or better
- Testing million-digit numbers requires specialized big-integer libraries

## Current Record Primes

- **Largest known prime**: ~41 million digits (as of 2024)
- **Type**: Mersenne prime (2^82,589,933 - 1)
- **Discovered by**: GIMPS (Great Internet Mersenne Prime Search)

## GIMPS (Great Internet Mersenne Prime Search)

- Distributed computing project since 1996
- Uses Lucas-Lehmer test optimized for Mersenne numbers
- Has found all largest known primes since 1996
- Requires specialized software: https://www.mersenne.org

## Go Implementation Notes

The Go prime generator in this repository:
- Generates all primes up to 1 billion using sieve methods
- Can be extended with Miller-Rabin for primality testing of individual numbers

## For Testing Large Numbers in Go

```go
// Example: Using Miller-Rabin for 64-bit numbers
// For arbitrary precision, use math/big package

func isPrime(n uint64) bool {
    // Miller-Rabin implementation
    // Suitable for numbers up to 2^64
}

// For million-digit numbers:
// - Use math/big.Int
// - Implement Miller-Rabin or Baillie-PSW
// - Expect very long run times
```

## References

- GIMPS: https://www.mersenne.org
- PrimePages: https://primes.utm.edu
- Miller-Rabin Test: https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test
- Baillie-PSW: https://en.wikipedia.org/wiki/Baillie%E2%80%93PSW_primality_test
