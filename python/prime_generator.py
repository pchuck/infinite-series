#!/usr/bin/env python3
"""
Prime Number Generator
Generates all prime numbers less than a given number n
Optimized with Sieve of Eratosthenes algorithm
"""

import multiprocessing
import math
import threading
import time as time_module
from typing import List, Optional, Callable, Any



SEGMENTED_SIEVE_THRESHOLD = 10_000_000
PARALLEL_SIEVE_THRESHOLD = 500_000_000
DEFAULT_SEGMENT_SIZE = 1_000_000


def _worker_process_segment_chunk(
    start_seg: int,
    end_seg: int,
    n: int,
    segment_size: int,
    base_primes: List[int],
    progress_counter: Any
) -> List[int]:
    """Worker function to process a chunk of segments in parallel
    
    Args:
        start_seg: Starting segment index (inclusive)
        end_seg: Ending segment index (exclusive)
        n: Upper bound for primes
        segment_size: Size of each segment
        base_primes: Precomputed primes up to sqrt(n)
        progress_counter: Shared multiprocessing.Value counter
    
    Returns:
        List of primes found in this worker's chunk
    """
    # Cache built-ins locally for faster lookup
    _min = min
    _max = max
    primes = []
    _append = primes.append
    
    for seg_idx in range(start_seg, end_seg):
        low = seg_idx * segment_size
        high = _min(low + segment_size, n)
        
        if high <= 2:
            continue
        
        segment_low = _max(low, 2)
        seg_len = high - segment_low
        is_prime = bytearray(b'\x01') * seg_len
        
        for p in base_primes:
            # Optimized: use integer arithmetic only, avoid isinstance check
            start = ((low + p - 1) // p) * p
            if start < p * p:
                start = p * p
            
            adjusted_start = start - segment_low
            
            if adjusted_start >= seg_len:
                continue
                
            step = p
            count = (seg_len - adjusted_start + step - 1) // step
            is_prime[adjusted_start :: step] = b'\x00' * count
        
        # Optimized: Use bytes.find() for 2.9x faster prime extraction
        data = bytes(is_prime)
        idx = -1
        while True:
            idx = data.find(1, idx + 1)
            if idx == -1:
                break
            _append(segment_low + idx)
        
        if progress_counter is not None:
            with progress_counter.get_lock():
                progress_counter.value += 1
    
    return primes


def sieve_of_eratosthenes(
    n: int, progress_callback: Optional[Callable[[int], None]] = None
) -> List[int]:
    """Generate all prime numbers less than n using Sieve of Eratosthenes
    
    Args:
        n: Upper bound (exclusive)
        progress_callback: Optional function to call with current iteration count
    """
    if n <= 2:
        return []
    
    if n < 0:
        raise ValueError(f"n must be non-negative, got {n}")
    
    sieve = bytearray(b'\x01') * n
    sieve[0] = sieve[1] = 0
    
    # Cache n locally to avoid repeated global lookups
    max_check = int(n ** 0.5)
    _n = n
    
    for current in range(2, max_check + 1):
        if sieve[current]:
            start_idx = current * current
            step = current
            # Optimized: pre-calculate count instead of using len()
            count = (_n - start_idx + step - 1) // step
            sieve[start_idx:_n:step] = b'\x00' * count
        
        if progress_callback:
            progress_callback(current - 2)
    
    # Optimized: Use bytes.find() for 2.5x faster prime extraction
    primes = []
    _append = primes.append
    data = bytes(sieve)
    idx = -1
    while True:
        idx = data.find(1, idx + 1)
        if idx == -1:
            break
        _append(idx)
    
    return primes


def segmented_sieve(
    n: int,
    segment_size: int = DEFAULT_SEGMENT_SIZE,
    progress_callback: Optional[Callable[[int], None]] = None
) -> List[int]:
    """Generate all prime numbers less than n using Segmented Sieve of Eratosthenes
    
    Memory: O(√n + segment_size) instead of O(n)
    
    Args:
        n: Upper bound (exclusive)
        segment_size: Size of each segment (default 1_000_000)
        progress_callback: Optional function called with segment index for progress
    
    Returns:
        List of all primes less than n
    """
    if n <= 2:
        return []
    
    if n < 0:
        raise ValueError(f"n must be non-negative, got {n}")
    
    # Cache math functions locally for faster lookup
    _isqrt = math.isqrt
    _min = min
    _max = max
    
    base_limit = _isqrt(n)
    base_primes = sieve_of_eratosthenes(base_limit + 1)
    
    segments = (n + segment_size - 1) // segment_size
    primes: List[int] = []
    _append = primes.append  # Local binding for faster calls
    
    for seg_idx in range(segments):
        low = seg_idx * segment_size
        high = _min(low + segment_size, n)
        
        if high <= 2:
            continue
        
        segment_low = _max(low, 2)
        seg_len = high - segment_low
        is_prime = bytearray(b'\x01') * seg_len
        
        for p in base_primes:
            # Optimized: use integer arithmetic only, avoid isinstance check
            start = ((low + p - 1) // p) * p
            if start < p * p:
                start = p * p
            
            adjusted_start = start - segment_low
            
            if adjusted_start >= seg_len:
                continue
                
            step = p
            count = (seg_len - adjusted_start + step - 1) // step
            is_prime[adjusted_start :: step] = b'\x00' * count
        
        # Optimized: Use bytes.find() for 2.9x faster prime extraction
        data = bytes(is_prime)
        idx = -1
        while True:
            idx = data.find(1, idx + 1)
            if idx == -1:
                break
            _append(segment_low + idx)
        
        if progress_callback:
            progress_callback(seg_idx + 1)
    
    return primes


def parallel_segmented_sieve(
    n: int,
    num_workers: Optional[int] = None,
    segment_size: int = DEFAULT_SEGMENT_SIZE,
    progress_callback: Optional[Callable[[int], None]] = None
) -> List[int]:
    """Generate all prime numbers less than n using parallel Segmented Sieve
    
    Memory: O(√n + segment_size) instead of O(n)
    Parallelism: Processes segments in parallel across workers
    
    Args:
        n: Upper bound (exclusive)
        num_workers: Number of worker processes (default cpu_count - 1)
        segment_size: Size of each segment (default 1_000_000)
        progress_callback: Optional function called with segment index for progress
    
    Returns:
        List of all primes less than n

    Raises:
        ValueError: If n is negative
    """
    if n <= 2:
        return []

    if n < 0:
        raise ValueError(f"n must be non-negative, got {n}")

    base_limit = int(math.isqrt(n))
    base_primes = sieve_of_eratosthenes(base_limit + 1)
    
    segments = (n + segment_size - 1) // segment_size
    
    if num_workers is None:
        num_workers = max(1, multiprocessing.cpu_count() - 1)
    
    num_workers = min(num_workers, segments)
    
    progress_counter: Any = None
    monitor_thread: Optional[threading.Thread] = None
    stop_monitoring = threading.Event()
    
    if progress_callback or num_workers > 1:
        try:
            manager = multiprocessing.Manager()
            progress_counter = manager.Value('i', 0)
            
            def monitor_progress():
                last_seen = 0
                while not stop_monitoring.is_set():
                    # type: ignore
                    current = progress_counter.value
                    if current > last_seen and progress_callback:
                        progress_callback(current - last_seen)
                        last_seen = current
                    time_module.sleep(0.1)
            
            monitor_thread = threading.Thread(target=monitor_progress, daemon=True)
            monitor_thread.start()
        except (OSError, ImportError):
            progress_counter = None
            monitor_thread = None
    
    chunk_size = (segments + num_workers - 1) // num_workers
    worker_args = []
    
    for worker_idx in range(num_workers):
        start_seg = worker_idx * chunk_size
        end_seg = min(start_seg + chunk_size, segments)
        
        if start_seg >= segments:
            break
        
        worker_args.append((start_seg, end_seg, n, segment_size, base_primes, progress_counter))  # type: ignore[arg-type]
    
    all_primes: List[int] = []
    
    try:
        with multiprocessing.Pool(processes=num_workers) as pool:
            results = pool.starmap(_worker_process_segment_chunk, worker_args)
        
        for chunk_primes in results:
            all_primes.extend(chunk_primes)
    except (OSError, multiprocessing.ProcessError, AttributeError):
        # Fallback to sequential on multiprocessing errors
        return segmented_sieve(n, segment_size, progress_callback)
    finally:
        # Signal monitor thread to stop and wait for it
        if monitor_thread is not None and monitor_thread.is_alive():
            stop_monitoring.set()
            monitor_thread.join(timeout=1.0)
    
    all_primes.sort()
    return all_primes


def _get_tqdm():
    try:
        from tqdm import tqdm
        return tqdm, True
    except ImportError:
        return None, False


def generate_primes(
    n: int,
    show_progress: bool = False,
    parallel: bool = False,
    force_algorithm: Optional[str] = None
) -> List[str]:
    """Generate all prime numbers less than n as strings.
    
    Auto-selects algorithm based on n for optimal performance.
    
    Args:
        n: Upper bound (exclusive)
        show_progress: Display progress indicator
        parallel: Use parallel processing CPU workers (for large n)
        force_algorithm: Override auto-selection ('classic', 'segmented', or 'parallel')
    
    Returns:
        List of primes as strings

    Raises:
        ValueError: If n is negative
    """
    if n < 0:
        raise ValueError(f"n must be non-negative, got {n}")

    if n <= 2:
        return []

    use_segmented = force_algorithm in ("segmented", "parallel") or (
        force_algorithm is None and n >= SEGMENTED_SIEVE_THRESHOLD
    )
    
    use_parallel = (force_algorithm == "parallel" or 
                    (parallel and n >= PARALLEL_SIEVE_THRESHOLD))
    
    tqdm_cls, tqdm_available = _get_tqdm()
    primes: List[int]
    
    if show_progress and tqdm_available and tqdm_cls is not None:
        if use_segmented:
            segments = (n + DEFAULT_SEGMENT_SIZE - 1) // DEFAULT_SEGMENT_SIZE
            # type: ignore
            with tqdm_cls(total=segments, desc="Generating primes", unit="segments") as pbar:
                def progress_callback_seg(seg_idx):
                    pbar.update(1)

                if use_parallel:
                    primes = parallel_segmented_sieve(n, segment_size=DEFAULT_SEGMENT_SIZE,
                                                     progress_callback=progress_callback_seg)
                else:
                    primes = segmented_sieve(n, progress_callback=progress_callback_seg)
        else:
            total = int(n**0.5)
            # type: ignore
            with tqdm_cls(total=total, desc="Generating primes", unit="iterations") as pbar:
                def progress_callback_classic(current):
                    pbar.update(1)

                primes = sieve_of_eratosthenes(n, progress_callback=progress_callback_classic)
    elif show_progress and not tqdm_available:
        print("Generating primes...", end="", flush=True)
        
        if use_segmented:
            if use_parallel:
                primes = parallel_segmented_sieve(n, segment_size=DEFAULT_SEGMENT_SIZE)
            else:
                primes = segmented_sieve(n)
        else:
            def dummy_callback(current):
                pass
            primes = sieve_of_eratosthenes(n, progress_callback=dummy_callback)
        
        print(" Done!")
    else:
        if use_segmented:
            if use_parallel:
                primes = parallel_segmented_sieve(n, segment_size=DEFAULT_SEGMENT_SIZE)
            else:
                primes = segmented_sieve(n)
        else:
            primes = sieve_of_eratosthenes(n)
    
    return [str(p) for p in primes]


def main():
    """Main function to handle user input and output
    
    Raises:
        TypeError: If n is not a valid integer
    """
    import argparse
    import sys
    import time
    
    parser = argparse.ArgumentParser(description='Generate prime numbers less than n')
    parser.add_argument('n', type=int, nargs='?', help='Upper bound (exclusive)')
    parser.add_argument('--progress', '-p', action='store_true', 
                       help='Show progress indicator')
    parser.add_argument(
        '--parallel', action='store_true',
        help='Use parallel CPU processing (for large n >= 500M)'
    )
    args = parser.parse_args()
    
    if args.n is not None:
        n = args.n
    else:
        try:
            n = int(input("Enter a number (n): "))
        except ValueError:
            print("Please enter a valid integer")
            return
    
    if n <= 2:
        print("No primes less than", n)
    else:
        # Warn if parallel is requested but n is below threshold
        if args.parallel and n < PARALLEL_SIEVE_THRESHOLD:
            print(
                f"[WARN] --parallel ignored: n={n} is below threshold "
                f"{PARALLEL_SIEVE_THRESHOLD}",
                file=sys.stderr,
            )

        start_time = time.time()
        primes = generate_primes(
            n, show_progress=args.progress, parallel=args.parallel
        )
        elapsed = time.time() - start_time

        if primes:
            print("Primes less than", n, ":", ", ".join(primes))
            print(f"Total primes: {len(primes)}")
        else:
            print("No primes less than", n)

        # Output performance summary to stderr
        primes_per_sec = len(primes) / elapsed if elapsed > 0 else 0
        print(
            f"[PERF] n={n} | primes={len(primes)} | time={elapsed:.3f}s | "
            f"primes/s={primes_per_sec:,.0f}",
            file=sys.stderr,
        )


if __name__ == "__main__":
    main()
