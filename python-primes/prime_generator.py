#!/usr/bin/env python3
"""
Prime Number Generator
Generates all prime numbers less than a given number n
Optimized with Sieve of Eratosthenes algorithm

Optimizations:
- Odd-only sieve: skips even numbers (2x memory + work reduction)
- bytearray sieve with slice assignment for fast composite marking
- memoryview for zero-copy segment operations
- heapq.merge for O(n) parallel result merging
- Shared multiprocessing.Value instead of Manager for progress counter
"""

import heapq
import multiprocessing
import math
import sys
import threading
import time as time_module
from typing import List, Optional, Callable, Any

# Type alias for multiprocessing shared counter
SharedCounter = Any

SEGMENTED_SIEVE_THRESHOLD = 10_000_000
PARALLEL_SIEVE_THRESHOLD = 500_000_000
DEFAULT_SEGMENT_SIZE = 1_000_000


def _sieve_segment_odd_only(
    low: int,
    high: int,
    base_primes: List[int],
    is_prime: bytearray,
) -> List[int]:
    """Process a single segment using odd-only sieve.

    Shared helper used by both sequential and parallel segmented sieves.

    Args:
        low: Segment low bound (inclusive)
        high: Segment high bound (exclusive), must be > 2
        base_primes: Precomputed odd primes up to sqrt(n) (excludes 2)
        is_prime: Reusable bytearray buffer (at least (high - low) // 2 bytes)

    Returns:
        List of primes found in [max(low, 2), high)
    """
    primes: List[int] = []

    # Handle the prime 2 if it falls in this segment
    if low <= 2 < high:
        primes.append(2)

    # Odd-only sieve: index i represents number odd_low + 2*i
    # odd_low is the first odd number >= max(low, 3)
    odd_low = max(low, 3)
    if odd_low % 2 == 0:
        odd_low += 1
    if odd_low >= high:
        return primes

    seg_len = (high - odd_low + 1) // 2  # count of odd numbers in [odd_low, high)
    if seg_len <= 0:
        return primes

    # Reset buffer
    is_prime[:seg_len] = b'\x01' * seg_len
    segment_view = memoryview(is_prime)[:seg_len]

    for p in base_primes:
        # Find first odd multiple of p in [odd_low, high)
        start = ((odd_low + p - 1) // p) * p
        if start < p * p:
            start = p * p
        if start % 2 == 0:
            start += p

        if start >= high:
            continue

        # Map to index in odd-only array
        adjusted_start = (start - odd_low) // 2
        step = p  # step in index space = p (since each index step = 2 numbers)
        count = (seg_len - adjusted_start + step - 1) // step
        segment_view[adjusted_start::step] = b'\x00' * count

    # Extract primes using bytes.find()
    data = bytes(segment_view)
    _append = primes.append
    idx = -1
    while True:
        idx = data.find(1, idx + 1)
        if idx == -1:
            break
        _append(odd_low + 2 * idx)

    return primes


def _worker_process_segment_chunk(
    start_seg: int,
    end_seg: int,
    n: int,
    segment_size: int,
    base_primes: List[int],
    progress_counter: Optional[SharedCounter]
) -> List[int]:
    """Worker function to process a chunk of segments in parallel

    Args:
        start_seg: Starting segment index (inclusive)
        end_seg: Ending segment index (exclusive)
        n: Upper bound for primes
        segment_size: Size of each segment
        base_primes: Precomputed odd primes up to sqrt(n) (excludes 2)
        progress_counter: Shared multiprocessing.Value counter

    Returns:
        List of primes found in this worker's chunk (already sorted)
    """
    primes: List[int] = []

    # Reusable buffer for segments
    is_prime = bytearray(segment_size)

    for seg_idx in range(start_seg, end_seg):
        low = seg_idx * segment_size
        high = min(low + segment_size, n)

        if high <= 2:
            if progress_counter is not None:
                with progress_counter.get_lock():
                    progress_counter.value += 1
            continue

        seg_primes = _sieve_segment_odd_only(low, high, base_primes, is_prime)
        primes.extend(seg_primes)

        if progress_counter is not None:
            with progress_counter.get_lock():
                progress_counter.value += 1

    return primes


def sieve_of_eratosthenes(
    n: int, progress_callback: Optional[Callable[[int], None]] = None
) -> List[int]:
    """Generate all prime numbers less than n using Sieve of Eratosthenes

    Uses odd-only sieve for 2x memory and work reduction.

    Args:
        n: Upper bound (exclusive)
        progress_callback: Optional function to call with current iteration count

    Returns:
        List of all primes less than n

    Raises:
        ValueError: If n is negative
    """
    if n < 0:
        raise ValueError(f"n must be non-negative, got {n}")

    if n <= 2:
        return []

    if n <= 3:
        return [2]

    # Odd-only sieve: index i represents number 2*i + 3
    # So index 0 = 3, index 1 = 5, index 2 = 7, ...
    sieve_size = (n - 3 + 1) // 2  # count of odd numbers in [3, n)
    sieve = bytearray(b'\x01') * sieve_size

    max_check = math.isqrt(n)
    _n = sieve_size

    for current in range(3, max_check + 1, 2):
        idx = (current - 3) // 2
        if sieve[idx]:
            # Mark multiples of current starting at current*current
            # current*current is odd (odd*odd), so (current*current - 3) // 2 is the index
            start_idx = (current * current - 3) // 2
            step = current  # step in index space = current (since each index += 2 numbers)
            count = (_n - start_idx + step - 1) // step
            sieve[start_idx:_n:step] = b'\x00' * count

        if progress_callback:
            progress_callback((current - 3) // 2)

    # Extract primes
    primes = [2]
    _append = primes.append
    data = bytes(sieve)
    idx = -1
    while True:
        idx = data.find(1, idx + 1)
        if idx == -1:
            break
        _append(2 * idx + 3)

    return primes


def segmented_sieve(
    n: int,
    segment_size: int = DEFAULT_SEGMENT_SIZE,
    progress_callback: Optional[Callable[[int], None]] = None
) -> List[int]:
    """Generate all prime numbers less than n using Segmented Sieve of Eratosthenes

    Uses odd-only sieve for 2x memory and work reduction.
    Memory: O(sqrt(n) + segment_size) instead of O(n)

    Args:
        n: Upper bound (exclusive)
        segment_size: Size of each segment (default 1_000_000)
        progress_callback: Optional function called with segment index for progress

    Returns:
        List of all primes less than n

    Raises:
        ValueError: If n is negative
    """
    if n < 0:
        raise ValueError(f"n must be non-negative, got {n}")

    if n <= 2:
        return []

    base_limit = math.isqrt(n)
    all_base_primes = sieve_of_eratosthenes(base_limit + 1)
    # Odd base primes only (exclude 2) for segment sieving
    base_primes_odd = [p for p in all_base_primes if p > 2]

    segments = (n + segment_size - 1) // segment_size
    primes: List[int] = []

    # Reusable buffer for segments
    is_prime = bytearray(segment_size)

    for seg_idx in range(segments):
        low = seg_idx * segment_size
        high = min(low + segment_size, n)

        if high <= 2:
            if progress_callback:
                progress_callback(seg_idx + 1)
            continue

        seg_primes = _sieve_segment_odd_only(low, high, base_primes_odd, is_prime)
        primes.extend(seg_primes)

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

    Uses odd-only sieve and heapq.merge for O(n) result merging.
    Memory: O(sqrt(n) + segment_size) instead of O(n)
    Parallelism: Processes segments in parallel across workers

    Args:
        n: Upper bound (exclusive)
        num_workers: Number of worker processes (default cpu_count - 1)
        segment_size: Size of each segment (default 1_000_000)
        progress_callback: Optional function called with segment count delta for progress

    Returns:
        List of all primes less than n

    Raises:
        ValueError: If n is negative
    """
    if n < 0:
        raise ValueError(f"n must be non-negative, got {n}")

    if n <= 2:
        return []

    base_limit = math.isqrt(n)
    all_base_primes = sieve_of_eratosthenes(base_limit + 1)
    # Odd base primes only (exclude 2) for segment sieving
    base_primes_odd = [p for p in all_base_primes if p > 2]

    segments = (n + segment_size - 1) // segment_size

    if num_workers is None:
        num_workers = max(1, multiprocessing.cpu_count() - 1)

    num_workers = min(num_workers, segments)

    progress_counter: Optional[SharedCounter] = None
    monitor_thread: Optional[threading.Thread] = None
    stop_monitoring = threading.Event()

    if progress_callback:
        try:
            # Manager().Value is required here because pool.starmap pickles
            # arguments -- plain multiprocessing.Value cannot be pickled.
            manager = multiprocessing.Manager()
            progress_counter = manager.Value('i', 0)

            def monitor_progress() -> None:
                last_seen = 0
                while not stop_monitoring.is_set():
                    current = progress_counter.value  # type: ignore[union-attr]
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

        worker_args.append((start_seg, end_seg, n, segment_size, base_primes_odd, progress_counter))  # type: ignore[arg-type]

    all_primes: List[int] = []

    try:
        with multiprocessing.Pool(processes=num_workers) as pool:
            results = pool.starmap(_worker_process_segment_chunk, worker_args)

        # Each worker's results are already sorted (contiguous segments).
        # Use heapq.merge for O(n) merge of pre-sorted lists.
        all_primes = list(heapq.merge(*results))
    except (OSError, multiprocessing.ProcessError, AttributeError):
        # Fallback to sequential on multiprocessing errors
        return segmented_sieve(n, segment_size, progress_callback)
    finally:
        # Signal monitor thread to stop and wait for it
        if monitor_thread is not None and monitor_thread.is_alive():
            stop_monitoring.set()
            monitor_thread.join(timeout=1.0)

    return all_primes


def _get_tqdm():
    try:
        from tqdm import tqdm
        return tqdm, True
    except ImportError:
        return None, False


class SimpleProgressBar:
    def __init__(self, total: int, description: str = "Generating primes"):
        self.total = total
        self.completed = 0
        self.description = description
        self.width = 40
        self.start_time = time_module.time()
        self.last_update = 0.0

    def update(self, delta: int = 1) -> None:
        self.completed += delta
        now = time_module.time()
        if now - self.last_update < 0.05 and self.completed < self.total:
            return
        self.last_update = now
        self.render()

    def render(self) -> None:
        if self.total == 0:
            return
        percent = min(1.0, self.completed / self.total)
        filled = int(percent * self.width)
        bar = "=" * filled + " " * (self.width - filled)

        elapsed = time_module.time() - self.start_time
        rate = self.completed / elapsed if elapsed > 0 else 0

        if rate >= 1_000_000:
            rate_str = f"{rate/1_000_000:.1f}M/s"
        elif rate >= 1_000:
            rate_str = f"{rate/1_000:.1f}K/s"
        else:
            rate_str = f"{rate:.0f}/s"

        remaining = self.total - self.completed
        if rate > 0 and remaining > 0:
            eta_secs = remaining / rate
            if eta_secs >= 3600:
                eta_str = f"{int(eta_secs/3600)}h{int((eta_secs%3600)/60)}m"
            elif eta_secs >= 60:
                eta_str = f"{int(eta_secs/60)}m{int(eta_secs%60)}s"
            else:
                eta_str = f"{int(eta_secs)}s"
        else:
            eta_str = "0s"

        print(
            f"\r{self.description}: [{bar}] {percent*100:3.0f}% | {self.completed}/{self.total} | {rate_str} | eta {eta_str}    ",
            end="", flush=True, file=sys.stderr
        )

    def finish(self) -> None:
        self.completed = self.total
        self.render()
        print(flush=True, file=sys.stderr)


def generate_primes(
    n: int,
    show_progress: bool = False,
    parallel: bool = False,
    force_algorithm: Optional[str] = None
) -> List[int]:
    """Generate all prime numbers less than n.

    Auto-selects algorithm based on n for optimal performance.

    Args:
        n: Upper bound (exclusive)
        show_progress: Display progress indicator
        parallel: Use parallel processing CPU workers (for large n)
        force_algorithm: Override auto-selection ('classic', 'segmented', or 'parallel')

    Returns:
        List of primes as integers

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

    # Build progress callback if requested
    progress_cb: Optional[Callable[[int], None]] = None
    progress_bar_obj: Optional[SimpleProgressBar] = None
    tqdm_ctx = None

    if show_progress:
        tqdm_cls, tqdm_available = _get_tqdm()
        if use_segmented:
            segments = (n + DEFAULT_SEGMENT_SIZE - 1) // DEFAULT_SEGMENT_SIZE
            if tqdm_available and tqdm_cls is not None:
                tqdm_ctx = tqdm_cls(total=segments, desc="Generating primes", unit="segments")
                def progress_cb_tqdm(seg_idx: int) -> None:
                    tqdm_ctx.update(1)  # type: ignore[union-attr]
                progress_cb = progress_cb_tqdm
            else:
                progress_bar_obj = SimpleProgressBar(segments, "Generating primes")
                def progress_cb_simple(seg_idx: int) -> None:
                    progress_bar_obj.update(1)  # type: ignore[union-attr]
                progress_cb = progress_cb_simple
        else:
            total = math.isqrt(n)
            if tqdm_available and tqdm_cls is not None:
                tqdm_ctx = tqdm_cls(total=total, desc="Generating primes", unit="iterations")
                def progress_cb_tqdm_classic(current: int) -> None:
                    tqdm_ctx.update(1)  # type: ignore[union-attr]
                progress_cb = progress_cb_tqdm_classic
            else:
                progress_bar_obj = SimpleProgressBar(total, "Generating primes")
                def progress_cb_simple_classic(current: int) -> None:
                    progress_bar_obj.update(1)  # type: ignore[union-attr]
                progress_cb = progress_cb_simple_classic

    # Select and run algorithm
    primes: List[int]
    if use_segmented:
        if use_parallel:
            primes = parallel_segmented_sieve(n, segment_size=DEFAULT_SEGMENT_SIZE,
                                              progress_callback=progress_cb)
        else:
            primes = segmented_sieve(n, progress_callback=progress_cb)
    else:
        primes = sieve_of_eratosthenes(n, progress_callback=progress_cb)

    # Clean up progress
    if tqdm_ctx is not None:
        tqdm_ctx.close()
    if progress_bar_obj is not None:
        progress_bar_obj.finish()

    return primes


def main() -> None:
    """Main function to handle user input and output

    Raises:
        TypeError: If n is not a valid integer
    """
    import argparse
    import time

    parser = argparse.ArgumentParser(description='Generate prime numbers less than n')
    parser.add_argument('n', type=int, nargs='?', help='Upper bound (exclusive)')
    parser.add_argument('--progress', '-p', action='store_true',
                       help='Show progress indicator')
    parser.add_argument(
        '--parallel', action='store_true',
        help='Use parallel CPU processing (for large n >= 500M)'
    )
    parser.add_argument(
        '--quiet', '-q', action='store_true',
        help='Only print count (no prime list)'
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
            if not args.quiet:
                print("Primes less than", n, ":", ", ".join(str(p) for p in primes))
                print(f"Total primes: {len(primes)}")
            else:
                print(len(primes))
        else:
            print("No primes less than", n)

        # Output performance summary to stderr
        primes_per_sec = len(primes) / elapsed if elapsed > 0 else 0
        largest_prime = primes[-1] if primes else 0
        print(
            f"Done! Largest prime < {n} is {largest_prime}. "
            f"Generated {len(primes)} primes in {elapsed:.3f}s ({primes_per_sec:,.0f} primes/s).",
            file=sys.stderr,
        )


if __name__ == "__main__":
    main()
