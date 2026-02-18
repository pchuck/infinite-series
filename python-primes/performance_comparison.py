#!/usr/bin/env python3
"""
Performance comparison between old and optimized prime generation algorithms
"""

import time
from prime_generator import generate_primes


def old_is_prime(num):
    """Old inefficient prime checking algorithm"""
    if num < 2:
        return False
    if num == 2:
        return True
    if num % 2 == 0:
        return False
    
    # Check odd divisors up to sqrt(num)
    for i in range(3, int(num**0.5) + 1, 2):
        if num % i == 0:
            return False
    return True


def old_generate_primes(n):
    """Old inefficient prime generation"""
    primes = []
    for num in range(2, n):
        if old_is_prime(num):
            primes.append(str(num))
    return primes


def benchmark():
    """Run performance benchmarks"""
    test_cases = [
        (100, "Small (100)"),
        (1000, "Medium (1,000)"),
        (10000, "Large (10,000)"),
        (100000, "Very Large (100,000)"),
    ]
    
    print("=" * 70)
    print("Performance Comparison: Old vs Optimized Prime Generation")
    print("=" * 70)
    print()
    
    for n, label in test_cases:
        print(f"\nTesting: {label} (n = {n:,})")
        print("-" * 70)
        
        # Test old algorithm
        start = time.time()
        old_generate_primes(n)
        old_time = time.time() - start
        
        # Test optimized algorithm
        start = time.time()
        optimized_result = generate_primes(n)
        optimized_time = time.time() - start
        
        # Calculate speedup
        speedup = old_time / optimized_time if optimized_time > 0 else float('inf')
        
        print(f"Old algorithm: {old_time:.4f} seconds")
        print(f"Optimized:     {optimized_time:.4f} seconds")
        print(f"Speedup:       {speedup:.2f}x faster")
        print(f"Primes found:  {len(optimized_result):,}")
    
    print("\n" + "=" * 70)
    print("Summary:")
    print("=" * 70)
    print("The Sieve of Eratosthenes algorithm provides significant performance")
    print("improvements, especially for larger numbers, by eliminating the need")
    print("to check each number individually for primality.")
    print("=" * 70)


if __name__ == "__main__":
    benchmark()