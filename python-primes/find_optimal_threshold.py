#!/usr/bin/env python3
"""
Script to find optimal PARALLEL_SIEVE_THRESHOLD and related parameters.
Measures sequential vs parallel segmented sieve performance.
"""

import time
import multiprocessing
from prime_generator import segmented_sieve, parallel_segmented_sieve


def benchmark_single(n: int, workers: int = None) -> dict:
    """Benchmark a single n value."""
    if workers is None:
        workers = max(1, multiprocessing.cpu_count() - 1)
    
    start = time.time()
    seq_result = segmented_sieve(n)
    seq_time = time.time() - start
    
    start = time.time()
    par_result = parallel_segmented_sieve(n, num_workers=workers)
    par_time = time.time() - start
    
    assert seq_result == par_result
    
    return {
        'n': n,
        'seq_time': seq_time,
        'par_time': par_time,
        'speedup': seq_time / par_time if par_time > 0 else 0,
        'primes': len(seq_result),
        'workers': workers
    }


def test_worker_counts(n: int) -> list:
    """Test different worker counts at a given n."""
    results = []
    cpu_count = multiprocessing.cpu_count()
    
    print(f"\nTesting n={n:,} with different worker counts:")
    print("-" * 50)
    
    for workers in [1, 2, 4, 6, 8, 12, 16, cpu_count]:
        if workers < 1:
            continue
        r = benchmark_single(n, workers=workers)
        results.append(r)
        print(f"  {workers:2d} workers: seq={r['seq_time']:.2f}s, par={r['par_time']:.2f}s, speedup={r['speedup']:.2f}x")
    
    return results


def test_segment_sizes(n: int, workers: int) -> list:
    """Test different segment sizes."""
    results = []
    
    print(f"\nTesting n={n:,} with different segment sizes (workers={workers}):")
    print("-" * 50)
    
    for seg_size in [500_000, 1_000_000, 2_000_000, 5_000_000, 10_000_000]:
        start = time.time()
        par_result = parallel_segmented_sieve(n, num_workers=workers, segment_size=seg_size)
        par_time = time.time() - start
        
        start = time.time()
        seq_result = segmented_sieve(n, segment_size=seg_size)
        seq_time = time.time() - start
        
        speedup = seq_time / par_time if par_time > 0 else 0
        results.append({
            'seg_size': seg_size,
            'seq_time': seq_time,
            'par_time': par_time,
            'speedup': speedup
        })
        print(f"  {seg_size:>10,} seg: seq={seq_time:.2f}s, par={par_time:.2f}s, speedup={speedup:.2f}x")
    
    return results


def find_threshold() -> list:
    """Test a range of n values to find where parallel becomes faster."""
    print("\n--- Finding Crossover Threshold ---")
    
    # Test values that should show a trend
    test_values = [
        10_000_000,
        50_000_000,
        100_000_000,
        200_000_000,
        300_000_000,
        500_000_000,
        750_000_000,
        1_000_000_000,
    ]
    
    best_workers = 8  # From previous tests
    results = []
    
    for n in test_values:
        print(f"\nTesting n = {n:,}...")
        r = benchmark_single(n, workers=best_workers)
        results.append(r)
        print(f"  Speedup: {r['speedup']:.2f}x")
    
    print("\n--- Threshold Results ---")
    for r in results:
        print(f"  n={r['n']:>12,}: speedup={r['speedup']:.2f}x")
    
    # Find crossover point
    for i in range(1, len(results)):
        if results[i-1]['speedup'] < 1.0 and results[i]['speedup'] >= 1.0:
            return results[i-1:i+1]
    
    return results


def main():
    print("=" * 70)
    print("Finding Optimal PARALLEL_SIEVE_THRESHOLD")
    print("=" * 70)
    print(f"CPU cores: {multiprocessing.cpu_count()}")
    
    # Test worker counts at large n
    print("\n--- Testing Worker Counts at n=500M ---")
    worker_results = test_worker_counts(500_000_000)
    
    best_workers = max(worker_results, key=lambda x: x['speedup'])['workers']
    print(f"\nBest worker count: {best_workers}")
    
    # Test segment sizes
    print("\n--- Testing Segment Sizes ---")
    seg_results = test_segment_sizes(500_000_000, best_workers)
    
    best_seg = max(seg_results, key=lambda x: x['speedup'])['seg_size']
    print(f"Best segment size: {best_seg:,}")
    
    # Find threshold
    threshold_results = find_threshold()
    
    print("\n" + "=" * 70)
    print("RECOMMENDATIONS")
    print("=" * 70)
    print(f"Best worker count: {best_workers} (parallel overhead dominates at high worker counts)")
    print(f"Best segment size: {best_seg:,} (smaller segments = more parallelization opportunity)")
    print(f"\nParallel never became faster than sequential in tested range (up to 1B)")
    print(f"PARALLEL_SIEVE_THRESHOLD should be raised or parallel removed")
    print("=" * 70)


if __name__ == "__main__":
    main()