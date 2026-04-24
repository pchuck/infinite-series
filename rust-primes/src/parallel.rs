use std::cmp::min;

use crate::classic::sieve_of_eratosthenes;
use crate::error::PrimeGenError;
use crate::ProgressCallback;
use crate::segmented::sieve_segment_odd_only;
use crate::utils::{estimate_prime_count, isqrt, validate_segment_size, validate_workers};

/// Parallel Segmented Sieve (odd-only)
/// Best for n >= 5,000,000
/// Uses multiple threads for concurrent segment processing
///
/// Memory: O(sqrt(n) + segment_size) per worker (see segmented_sieve for details).
///
/// # Arguments
/// * `n` - Upper bound (exclusive) for prime generation
/// * `workers` - Number of worker threads
/// * `segment_size` - Size of each segment in elements
/// * `progress` - Optional callback receiving segment count updates
///
/// # Examples
///
/// ```
/// use primes::parallel_segmented_sieve;
///
/// let primes = parallel_segmented_sieve(1000, 2, 100, None).unwrap();
/// assert_eq!(primes.len(), 168); // 168 primes below 1000
///
/// // Multi-threaded progress tracking
/// use std::sync::Arc;
/// let progress = Arc::new(|delta: usize| {
///     // Thread-safe progress updates
/// }) as primes::ProgressCallback;
/// let result = parallel_segmented_sieve(1_000_000, 4, 1_000_000, Some(progress));
/// assert!(result.is_ok());
/// ```
pub fn parallel_segmented_sieve(
    n: usize,
    workers: usize,
    segment_size: usize,
    progress: Option<ProgressCallback>,
) -> Result<Vec<usize>, PrimeGenError> {
    if n <= 2 {
        return Ok(Vec::new());
    }

    validate_workers(workers)?;
    validate_segment_size(segment_size)?;

    let base_limit = isqrt(n);
    let all_base_primes = sieve_of_eratosthenes(base_limit + 1, None)?;
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

            let base_primes_ref = &base_primes_odd;
            let progress_ref = &progress;

            handles.push(s.spawn(move || {
                let mut worker_primes = Vec::new();
                let mut is_prime = vec![true; segment_size / 2 + 1];

                for seg_idx in start_seg..end_seg {
                    let Some(low) = seg_idx.checked_mul(segment_size) else {
                        break;
                    };
                    let high = min(low.saturating_add(segment_size), n);

                    if high <= 2 {
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

        let mut all_primes = Vec::with_capacity(estimate_prime_count(n));
        for handle in handles {
            match handle.join() {
                Ok(worker_primes) => all_primes.extend(worker_primes),
                Err(e) => {
                    let msg = e
                        .downcast::<String>()
                        .map(|s| *s)
                        .or_else(|e| e.downcast::<&str>().map(|s| s.to_string()))
                        .unwrap_or_else(|_| "Unknown panic".to_string());
                    return Err(PrimeGenError::WorkerThreadPanic(msg));
                }
            }
        }

        Ok(all_primes)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::segmented::segmented_sieve;

    #[test]
    fn test_validation_parallel_segmented_sieve_workers_zero() {
        let result = parallel_segmented_sieve(100, 0, 100, None);
        assert!(
            matches!(result, Err(PrimeGenError::InvalidInput(msg)) if msg.contains("workers=0"))
        );
    }

    #[test]
    fn test_parallel_matches_segmented() {
        for &n in &[100, 500, 1000, 5000] {
            let segmented = segmented_sieve(n, 100, None).unwrap();
            let parallel = parallel_segmented_sieve(n, 2, 100, None).unwrap();
            assert_eq!(segmented, parallel, "Failed for n={}", n);
        }
    }

    #[test]
    fn test_parallel_various_workers() {
        let expected = segmented_sieve(10000, 100, None).unwrap();
        for workers in 1..=4 {
            let result = parallel_segmented_sieve(10000, workers, 100, None).unwrap();
            assert_eq!(result, expected, "Failed for workers={}", workers);
        }
    }
}
