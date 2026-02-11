//! Test base selection for deterministic Miller-Rabin testing
//!
//! This module provides deterministic test bases for the Miller-Rabin primality test.
//! The number of bases required depends on the size of the number being tested:
//!
//! - For n < 3,474,749,660,399: 12 bases are sufficient for deterministic results
//! - For larger n: 19 bases provide deterministic coverage up to ~3.3x10^25
//!
//! These bounds are derived from published research on deterministic Miller-Rabin testing.

use num_bigint::BigUint;

/// Numbers below this threshold require only 12 bases for deterministic results.
/// Above this threshold, 19 bases are used.
pub const SMALL_NUMBER_THRESHOLD: u64 = 3_474_749_660_399;

/// Test bases sufficient for deterministic results when n < SMALL_NUMBER_THRESHOLD
const BASES_SMALL: &[u64] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

/// Extended test bases for larger numbers
const BASES_LARGE: &[u64] = &[
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67,
];

/// The largest base value in any base set (67).
/// Used for fast-path optimization in `filter_bases_for_n`.
const MAX_BASE: u64 = 67;

/// Returns the appropriate test bases for the given number size.
///
/// For numbers less than [`SMALL_NUMBER_THRESHOLD`], uses 12 bases.
/// For larger numbers, uses 19 bases.
///
/// Note: The returned slice is NOT filtered by n. Bases >= n must be
/// removed before use. See [`filter_bases_for_n`].
///
/// # Examples
/// ```
/// use miller_rabin_tester::get_test_bases_for_size;
/// use num_bigint::BigUint;
///
/// let n = BigUint::from(1000u32);
/// let bases = get_test_bases_for_size(&n);
/// assert_eq!(bases.len(), 12);
/// ```
pub fn get_test_bases_for_size(n: &BigUint) -> &'static [u64] {
    // Compare using bit count first (fast path): SMALL_NUMBER_THRESHOLD is ~42 bits.
    // Any n with > 42 bits is definitely above the threshold.
    let threshold_bits = 42; // ceil(log2(3_474_749_660_399)) = 42
    let n_bits = n.bits();

    if n_bits > threshold_bits {
        BASES_LARGE
    } else if n_bits < threshold_bits {
        BASES_SMALL
    } else {
        // Exact comparison only when bit counts match
        let threshold = BigUint::from(SMALL_NUMBER_THRESHOLD);
        if n < &threshold {
            BASES_SMALL
        } else {
            BASES_LARGE
        }
    }
}

/// Filters test bases to only include those less than n.
///
/// Miller-Rabin requires bases a where 2 <= a < n.
///
/// This uses a fast path: since the largest possible base is 67,
/// any n > 67 means all bases pass, avoiding per-element conversion.
///
/// # Examples
/// ```
/// use miller_rabin_tester::filter_bases_for_n;
/// use num_bigint::BigUint;
///
/// let n = BigUint::from(10u32);
/// let bases = &[2u64, 3, 5, 7, 11, 13];
/// let filtered = filter_bases_for_n(bases, &n);
/// assert_eq!(filtered, vec![2, 3, 5, 7]);
/// ```
pub fn filter_bases_for_n(bases: &[u64], n: &BigUint) -> Vec<u64> {
    // Fast path: if n > MAX_BASE, all bases pass (no conversion needed)
    if n > &BigUint::from(MAX_BASE) {
        return bases.to_vec();
    }

    // For small n, extract as u64 and compare natively (no BigUint per element)
    let n_u64: u64 = n.try_into().unwrap_or(u64::MAX);
    bases.iter().filter(|&&a| a < n_u64).copied().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_test_bases_small_number() {
        let n = BigUint::from(100u32);
        let bases = get_test_bases_for_size(&n);
        assert_eq!(bases, BASES_SMALL);
        assert_eq!(bases.len(), 12);
    }

    #[test]
    fn test_get_test_bases_large_number() {
        let n = BigUint::from(SMALL_NUMBER_THRESHOLD + 1);
        let bases = get_test_bases_for_size(&n);
        assert_eq!(bases, BASES_LARGE);
        assert_eq!(bases.len(), 19);
    }

    #[test]
    fn test_get_test_bases_at_threshold() {
        let n = BigUint::from(SMALL_NUMBER_THRESHOLD);
        let bases = get_test_bases_for_size(&n);
        assert_eq!(bases, BASES_LARGE);

        let n = BigUint::from(SMALL_NUMBER_THRESHOLD - 1);
        let bases = get_test_bases_for_size(&n);
        assert_eq!(bases, BASES_SMALL);
    }

    #[test]
    fn test_filter_bases() {
        let bases = &[2u64, 3, 5, 7, 11, 13];
        let n = BigUint::from(10u32);
        let filtered = filter_bases_for_n(bases, &n);
        assert_eq!(filtered, vec![2, 3, 5, 7]);
    }

    #[test]
    fn test_filter_bases_empty() {
        let bases = &[2u64, 3, 5];
        let n = BigUint::from(2u32);
        let filtered = filter_bases_for_n(bases, &n);
        assert!(filtered.is_empty());
    }

    #[test]
    fn test_filter_bases_all_pass() {
        // n > 67 means all bases pass the filter
        let n = BigUint::from(1000u32);
        let filtered = filter_bases_for_n(BASES_LARGE, &n);
        assert_eq!(filtered, BASES_LARGE.to_vec());
    }

    #[test]
    fn test_filter_bases_boundary() {
        // n = 68: MAX_BASE is 67, so all bases should pass
        let n = BigUint::from(68u32);
        let filtered = filter_bases_for_n(BASES_LARGE, &n);
        assert_eq!(filtered, BASES_LARGE.to_vec());

        // n = 67: base 67 should be excluded
        let n = BigUint::from(67u32);
        let filtered = filter_bases_for_n(BASES_LARGE, &n);
        assert!(!filtered.contains(&67));
        assert!(filtered.contains(&61));
    }
}
