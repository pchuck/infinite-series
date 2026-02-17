//! High-performance prime number generator
//!
//! Provides three algorithms:
//! - Classic Sieve of Eratosthenes (best for n < 1M)
//! - Segmented Sieve (best for n >= 1M)
//! - Parallel Segmented Sieve (best for n >= 100M)
//!
//! All algorithms use odd-only sieves for 2x memory and work reduction.

use std::cmp::min;
use std::sync::Arc;

pub const DEFAULT_SEGMENT_SIZE: usize = 1_000_000;
pub const PARALLEL_THRESHOLD: usize = 100_000_000;

/// Process a single segment using odd-only sieve.
/// Shared helper used by both sequential and parallel segmented sieves.
///
/// `low`/`high` define the segment range [low, high).
/// `base_primes_odd` are odd primes up to sqrt(n) (excludes 2).
/// `is_prime` is a reusable buffer (at least (high - low) / 2 elements).
///
/// Returns primes found in [max(low, 2), high).
fn sieve_segment_odd_only(
    low: usize,
    high: usize,
    base_primes_odd: &[usize],
    is_prime: &mut [bool],
) -> Vec<usize> {
    let mut primes = Vec::new();

    // Handle the prime 2 if it falls in this segment
    if low <= 2 && high > 2 {
        primes.push(2);
    }

    // Odd-only sieve: index i represents number odd_low + 2*i
    let mut odd_low = if low < 3 { 3 } else { low };
    if odd_low % 2 == 0 {
        odd_low += 1;
    }
    if odd_low >= high {
        return primes;
    }

    let seg_len = (high - odd_low).div_ceil(2); // count of odd numbers in [odd_low, high)
    if seg_len == 0 {
        return primes;
    }

    // Reset buffer
    is_prime[..seg_len].fill(true);

    for &p in base_primes_odd {
        // Find first odd multiple of p in [odd_low, high)
        let mut start = low.div_ceil(p) * p;
        if start < p * p {
            start = p * p;
        }
        if start % 2 == 0 {
            start += p;
        }

        if start >= high {
            continue;
        }

        // Map to index in odd-only array
        let adjusted_start = (start - odd_low) / 2;
        let step = p; // step in index space = p (each index step = 2 numbers)
        let mut j = adjusted_start;
        while j < seg_len {
            is_prime[j] = false;
            j += step;
        }
    }

    // Extract primes
    for (i, &is_p) in is_prime[..seg_len].iter().enumerate() {
        if is_p {
            primes.push(odd_low + 2 * i);
        }
    }

    primes
}

/// Classic Sieve of Eratosthenes (odd-only)
/// Best for n < 1,000,000
pub fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    if n <= 2 {
        return Vec::new();
    }

    if n <= 3 {
        return vec![2];
    }

    // Odd-only sieve: index i represents number 2*i + 3
    let sieve_size = (n - 3).div_ceil(2); // count of odd numbers in [3, n)
    let mut sieve = vec![true; sieve_size];

    let limit = (n as f64).sqrt() as usize;
    let mut current = 3;
    while current <= limit {
        let idx = (current - 3) / 2;
        if sieve[idx] {
            // Mark multiples starting at current*current
            let start_idx = (current * current - 3) / 2;
            let step = current;
            let mut j = start_idx;
            while j < sieve_size {
                sieve[j] = false;
                j += step;
            }
        }
        current += 2;
    }

    // Extract primes
    let mut primes = Vec::with_capacity(n / ((n as f64).ln() as usize).max(1));
    primes.push(2);
    for (i, &is_p) in sieve.iter().enumerate() {
        if is_p {
            primes.push(2 * i + 3);
        }
    }

    primes
}

/// Segmented Sieve of Eratosthenes (odd-only)
/// Best for n >= 1,000,000
/// Uses O(sqrt(n) + segment_size) memory
pub fn segmented_sieve(
    n: usize,
    segment_size: usize,
    progress: Option<Arc<dyn Fn(usize) + Send + Sync>>,
) -> Vec<usize> {
    if n <= 2 {
        return Vec::new();
    }

    let base_limit = (n as f64).sqrt() as usize;
    let all_base_primes = sieve_of_eratosthenes(base_limit + 1);
    // Odd base primes only (exclude 2) for segment sieving
    let base_primes_odd: Vec<usize> = all_base_primes.into_iter().filter(|&p| p > 2).collect();

    let segments = n.div_ceil(segment_size);
    let mut primes = Vec::with_capacity(n / ((n as f64).ln() as usize).max(1));

    // Reusable buffer for segments
    let mut is_prime = vec![true; segment_size];

    for seg_idx in 0..segments {
        let low = seg_idx * segment_size;
        let high = min(low + segment_size, n);

        if high <= 2 {
            if let Some(ref callback) = progress {
                callback(1);
            }
            continue;
        }

        let seg_primes = sieve_segment_odd_only(low, high, &base_primes_odd, &mut is_prime);
        primes.extend(seg_primes);

        if let Some(ref callback) = progress {
            callback(1);
        }
    }

    primes
}

/// Parallel Segmented Sieve (odd-only)
/// Best for n >= 100,000,000
/// Uses multiple threads for concurrent segment processing
pub fn parallel_segmented_sieve(
    n: usize,
    workers: usize,
    segment_size: usize,
    progress: Option<Arc<dyn Fn(usize) + Send + Sync>>,
) -> Vec<usize> {
    if n <= 2 {
        return Vec::new();
    }

    let base_limit = (n as f64).sqrt() as usize;
    let all_base_primes = sieve_of_eratosthenes(base_limit + 1);
    // Odd base primes only (exclude 2) for segment sieving
    let base_primes_odd: Vec<usize> = all_base_primes.into_iter().filter(|&p| p > 2).collect();

    let segments = n.div_ceil(segment_size);
    let num_workers = min(workers, segments);

    let chunk_size = segments.div_ceil(num_workers);

    std::thread::scope(|s| {
        let mut handles = Vec::new();

        for worker_idx in 0..num_workers {
            let start_seg = worker_idx * chunk_size;
            let end_seg = min(start_seg + chunk_size, segments);

            if start_seg >= segments {
                break;
            }

            // Share base_primes by reference instead of cloning --
            // thread::scope guarantees they outlive the spawned threads
            let base_primes_ref = &base_primes_odd;
            let progress_ref = &progress;

            handles.push(s.spawn(move || {
                // Each worker accumulates all primes from its contiguous range
                // into a single Vec (already sorted since segments are contiguous)
                let mut worker_primes = Vec::new();
                let mut is_prime = vec![true; segment_size];

                for seg_idx in start_seg..end_seg {
                    let low = seg_idx * segment_size;
                    let high = min(low + segment_size, n);

                    if high <= 2 {
                        if let Some(ref callback) = progress_ref {
                            callback(1);
                        }
                        continue;
                    }

                    let seg_primes =
                        sieve_segment_odd_only(low, high, base_primes_ref, &mut is_prime);
                    worker_primes.extend(seg_primes);

                    if let Some(ref callback) = progress_ref {
                        callback(1);
                    }
                }

                worker_primes
            }));
        }

        // Workers process contiguous segment ranges, so results are already
        // in order. Just concatenate worker vectors in order.
        let mut all_primes = Vec::with_capacity(n / ((n as f64).ln() as usize).max(1));
        for handle in handles {
            all_primes.extend(handle.join().unwrap());
        }

        all_primes
    })
}

/// Auto-select algorithm based on n
pub fn generate_primes(
    n: usize,
    parallel: bool,
    workers: Option<usize>,
    segment_size: Option<usize>,
    progress: Option<Arc<dyn Fn(usize) + Send + Sync>>,
) -> Vec<usize> {
    if n <= 2 {
        return Vec::new();
    }

    let workers = workers.unwrap_or_else(|| {
        std::thread::available_parallelism()
            .map(|p| p.get())
            .unwrap_or(4)
    });
    let segment_size = segment_size.unwrap_or(DEFAULT_SEGMENT_SIZE);

    if parallel && n >= PARALLEL_THRESHOLD {
        parallel_segmented_sieve(n, workers, segment_size, progress)
    } else if n >= DEFAULT_SEGMENT_SIZE {
        segmented_sieve(n, segment_size, progress)
    } else {
        sieve_of_eratosthenes(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sieve_small() {
        assert_eq!(sieve_of_eratosthenes(10), vec![2, 3, 5, 7]);
        assert_eq!(
            sieve_of_eratosthenes(30),
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
        );
    }

    #[test]
    fn test_sieve_empty() {
        assert_eq!(sieve_of_eratosthenes(0), Vec::<usize>::new());
        assert_eq!(sieve_of_eratosthenes(1), Vec::<usize>::new());
        assert_eq!(sieve_of_eratosthenes(2), Vec::<usize>::new());
    }

    #[test]
    fn test_sieve_boundary() {
        assert_eq!(sieve_of_eratosthenes(3), vec![2]);
        assert_eq!(sieve_of_eratosthenes(4), vec![2, 3]);
        assert_eq!(sieve_of_eratosthenes(5), vec![2, 3]);
        assert_eq!(sieve_of_eratosthenes(6), vec![2, 3, 5]);
    }

    #[test]
    fn test_segmented_matches_classic() {
        for &n in &[100, 500, 1000, 5000] {
            let classic = sieve_of_eratosthenes(n);
            let segmented = segmented_sieve(n, 100, None);
            assert_eq!(classic, segmented, "Failed for n={}", n);
        }
    }

    #[test]
    fn test_parallel_matches_segmented() {
        for &n in &[100, 500, 1000, 5000] {
            let segmented = segmented_sieve(n, 100, None);
            let parallel = parallel_segmented_sieve(n, 2, 100, None);
            assert_eq!(segmented, parallel, "Failed for n={}", n);
        }
    }

    #[test]
    fn test_large_input() {
        let primes = segmented_sieve(1_000_000, DEFAULT_SEGMENT_SIZE, None);
        assert_eq!(primes.len(), 78498);
        assert_eq!(primes[0], 2);
        assert_eq!(primes.last().unwrap(), &999983);
    }

    #[test]
    fn test_no_composites() {
        let primes = sieve_of_eratosthenes(200);
        for &p in &primes {
            assert!(p >= 2, "Found value < 2: {}", p);
            if p > 2 {
                assert!(p % 2 != 0, "Found even composite: {}", p);
                let mut d = 3;
                while d * d <= p {
                    assert!(p % d != 0, "Found composite: {} (divisible by {})", p, d);
                    d += 2;
                }
            }
        }
    }

    #[test]
    fn test_segmented_various_segment_sizes() {
        let expected = sieve_of_eratosthenes(1000);
        for &seg_size in &[1, 7, 10, 50, 100, 999, 1000, 2000] {
            let result = segmented_sieve(1000, seg_size, None);
            assert_eq!(result, expected, "Failed for segment_size={}", seg_size);
        }
    }

    #[test]
    fn test_parallel_various_workers() {
        let expected = segmented_sieve(10000, 100, None);
        for workers in 1..=4 {
            let result = parallel_segmented_sieve(10000, workers, 100, None);
            assert_eq!(result, expected, "Failed for workers={}", workers);
        }
    }
}
