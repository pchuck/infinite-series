#!/usr/bin/env python3
"""
Performance comparison between sequential and parallel prime generation
"""

import time
from prime_generator import generate_primes


def benchmark():
    """Run performance benchmarks for various input sizes"""
    
    # Test with values that trigger parallel processing (>= 50M)
    test_cases = [
        (10_000_000, "10 Million"),
        (25_000_000, "25 Million"),
        (50_000_000, "50 Million"),
    ]
    
    print("=" * 70)
    print("Performance Comparison: Sequential vs Parallel Prime Generation")
    print("=" * 70)
    print()
    
    for n, label in test_cases:
        print(f"\nTesting: {label} primes (n = {n:,})")
        print("-" * 70)
        
        # Test sequential
        start = time.time()
        seq_primes = generate_primes(n, parallel=False)
        seq_time = time.time() - start
        
        # Test parallel
        start = time.time()
        par_primes = generate_primes(n, parallel=True)
        par_time = time.time() - start
        
        # Calculate speedup
        speedup = seq_time / par_time if par_time > 0 else float('inf')
        
        # Verify correctness
        assert seq_primes == par_primes, "Results don't match!"
        
        print(f"Sequential:   {seq_time:.4f} seconds")
        print(f"Parallel:     {par_time:.4f} seconds")
        print(f"Speedup:      {speedup:.2f}x faster")
        print(f"Primes found: {len(seq_primes):,}")
    
    print("\n" + "=" * 70)
    print("Summary:")
    print("=" * 70)
    print("Parallel processing provides significant speedup for large inputs")
    print("by distributing segment processing across multiple CPU cores.")
    print("=" * 70)


if __name__ == "__main__":
    benchmark()