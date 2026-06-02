use std::cmp::min;

use crate::classic::sieve_of_eratosthenes;
use crate::error::PrimeGenError;
use crate::utils::{
    estimate_prime_count, isqrt, odd_at, validate_segment_size, value_to_odd_index,
};
use crate::ProgressCallback;

/// Process a single segment using odd-only sieve.
/// Shared helper used by both sequential and parallel segmented sieves.
///
/// `low`/`high` define the segment range [low, high).
/// `base_primes_odd` are odd primes up to sqrt(n) (excludes 2).
/// `is_prime` is a reusable buffer (at least (high - low) / 2 + 1 elements).
///
/// Returns primes found in [max(low, 2), high).
pub fn sieve_segment_odd_only(
    low: usize,
    high: usize,
    base_primes_odd: &[usize],
    is_prime: &mut [bool],
) -> Vec<usize> {
    let mut primes = Vec::new();

    if low <= 2 && high > 2 {
        primes.push(2);
    }

    let mut odd_low = if low < 3 { 3 } else { low };
    if odd_low.is_multiple_of(2) {
        odd_low += 1;
    }
    if odd_low >= high {
        return primes;
    }

    let seg_len = (high - odd_low).div_ceil(2);
    if seg_len == 0 {
        return primes;
    }

    is_prime[..seg_len].fill(true);

    for &p in base_primes_odd {
        let mut start = low.div_ceil(p) * p;
        if start < p * p {
            start = p * p;
        }
        if start.is_multiple_of(2) {
            start += p;
        }

        if start >= high {
            continue;
        }

        debug_assert!(
            start >= odd_low,
            "sieve start underflow: start={start} < odd_low={odd_low} \
             (p={p}, low={low}, high={high})"
        );
        let adjusted_start = value_to_odd_index(start, odd_low);
        let step = p;
        for j in (adjusted_start..seg_len).step_by(step) {
            is_prime[j] = false;
        }
    }

    for (i, &is_p) in is_prime[..seg_len].iter().enumerate() {
        if is_p {
            primes.push(odd_at(i, odd_low));
        }
    }

    primes
}

/// Segmented Sieve of Eratosthenes (odd-only)
/// Best for n >= 1,000,000
///
/// Memory: O(sqrt(n) + segment_size). Uses odd-only indexing to store only odd numbers,
/// allocating `segment_size` bools but using ~`segment_size/2` for actual data.
/// This simplifies implementation by avoiding dynamic per-segment allocation.
///
/// # Arguments
/// * `n` - Upper bound (exclusive) for prime generation
/// * `segment_size` - Size of each segment in elements
/// * `progress` - Optional callback receiving segment count updates
///
/// # Examples
///
/// ```
/// use primes::segmented_sieve;
///
/// let primes = segmented_sieve(100, 10, None).unwrap();
/// assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]);
///
/// // Progress callback example
/// let progress = Some(std::sync::Arc::new(|delta: usize| {
///     let _ = delta; // Use the parameter
/// }) as primes::ProgressCallback);
/// let result = segmented_sieve(1000, 100, progress);
/// assert!(result.is_ok());
/// ```
pub fn segmented_sieve(
    n: usize,
    segment_size: usize,
    progress: Option<ProgressCallback>,
) -> Result<Vec<usize>, PrimeGenError> {
    if n <= 2 {
        return Ok(Vec::new());
    }

    validate_segment_size(segment_size)?;

    let base_limit = isqrt(n);
    let all_base_primes = sieve_of_eratosthenes(base_limit + 1, None)?;
    let base_primes_odd: Vec<usize> = all_base_primes.into_iter().filter(|&p| p > 2).collect();

    let segments = n.div_ceil(segment_size);
    let mut primes = Vec::with_capacity(estimate_prime_count(n));

    let mut is_prime = vec![true; segment_size / 2 + 1];

    for seg_idx in 0..segments {
        let Some(low) = seg_idx.checked_mul(segment_size) else {
            break;
        };
        let high = min(low.saturating_add(segment_size), n);

        if high <= 2 {
            continue;
        }

        let seg_primes = sieve_segment_odd_only(low, high, &base_primes_odd, &mut is_prime);
        primes.extend(seg_primes);

        if let Some(ref callback) = progress {
            callback(1);
        }
    }

    Ok(primes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DEFAULT_SEGMENT_SIZE;

    #[test]
    fn test_validation_segmented_sieve_zero_segment() {
        let result = segmented_sieve(100, 0, None);
        assert!(
            matches!(result, Err(PrimeGenError::InvalidInput(msg)) if msg.contains("segment_size cannot be zero"))
        );
    }

    #[test]
    fn test_segmented_matches_classic() {
        for &n in &[100, 500, 1000, 5000] {
            let classic = sieve_of_eratosthenes(n, None).unwrap();
            let segmented = segmented_sieve(n, 100, None).unwrap();
            assert_eq!(classic, segmented, "Failed for n={}", n);
        }
    }

    #[test]
    fn test_large_input() {
        let primes = segmented_sieve(1_000_000, DEFAULT_SEGMENT_SIZE, None).unwrap();
        assert_eq!(primes.len(), 78498);
        assert_eq!(primes[0], 2);
        assert_eq!(primes.last().unwrap(), &999_983);
    }

    #[test]
    fn test_segmented_various_segment_sizes() {
        let expected = sieve_of_eratosthenes(1000, None).unwrap();
        for &seg_size in &[1, 7, 10, 50, 100, 999, 1000, 2000] {
            let result = segmented_sieve(1000, seg_size, None).unwrap();
            assert_eq!(result, expected, "Failed for segment_size={}", seg_size);
        }
    }
}
