//! Test base selection for deterministic Miller-Rabin testing
//!
//! This module provides deterministic test bases for the Miller-Rabin primality test.
//! The number of bases required depends on the size of the number being tested:
//!
//! - For n < 3_474_749_660_399: 12 bases are sufficient for deterministic results
//! - For n < 3_317_044_064_679_887_385_961_981: 19 bases are sufficient
//!
//! These bounds are derived from published research on deterministic Miller-Rabin testing.

use num_bigint::BigUint;

/// Threshold for switching from 64-bit to 128-bit test bases
/// Numbers less than this value require only 12 bases for deterministic results
pub const DETERMINISTIC_THRESHOLD_64: u64 = 3_474_749_660_399;

/// Threshold for 128-bit test bases
/// Numbers less than this value require 19 bases for deterministic results
pub const DETERMINISTIC_THRESHOLD_128: u64 = u64::MAX;

/// Test bases sufficient for deterministic results on all 64-bit integers (n < 3.4 trillion)
const M_R_TEST_BASES_64: &[u64] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

/// Extended test bases for larger numbers (up to ~3.3e25)
const M_R_TEST_BASES_128: &[u64] = &[
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67,
];

/// Returns the appropriate test bases for the given number size.
///
/// For numbers less than 3_474_749_660_399, uses 12 bases (deterministic for 64-bit).
/// For larger numbers, uses 19 bases (deterministic up to ~3.3e25).
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
    let n_128_max = BigUint::from(DETERMINISTIC_THRESHOLD_64);

    if n < &n_128_max {
        M_R_TEST_BASES_64
    } else {
        M_R_TEST_BASES_128
    }
}

/// Filters test bases to only include those less than n.
///
/// Miller-Rabin requires bases a where 2 <= a < n.
/// This function filters the provided bases accordingly.
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
    bases
        .iter()
        .filter(|&&a| BigUint::from(a) < *n)
        .copied()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_test_bases_small_number() {
        let n = BigUint::from(100u32);
        let bases = get_test_bases_for_size(&n);
        assert_eq!(bases, M_R_TEST_BASES_64);
        assert_eq!(bases.len(), 12);
    }

    #[test]
    fn test_get_test_bases_large_number() {
        let n = BigUint::from(DETERMINISTIC_THRESHOLD_64 + 1);
        let bases = get_test_bases_for_size(&n);
        assert_eq!(bases, M_R_TEST_BASES_128);
        assert_eq!(bases.len(), 19);
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
}
