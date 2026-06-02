use crate::error::PrimeGenError;
use crate::{MAX_N, MAX_WORKERS};

/// Integer square root using Newton's method.
/// Pure integer implementation — accurate for all usize values without f64 precision issues.
#[inline]
pub const fn isqrt(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    let mut x = n;
    let mut y = x.div_ceil(2);
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

/// Returns the odd number at the given index relative to `base_odd`.
///
/// `base_odd` must itself be odd. Index 0 yields `base_odd`; index 1 yields `base_odd + 2`;
/// and so on. Inverse of [`value_to_odd_index`].
///
/// [`value_to_odd_index`]: crate::utils::value_to_odd_index
#[inline]
pub const fn odd_at(idx: usize, base_odd: usize) -> usize {
    base_odd + 2 * idx
}

/// Returns the odd-only index of `value` relative to `base_odd`.
///
/// `value` must be odd and `value >= base_odd`. The result is the inverse of [`odd_at`]:
/// `value_to_odd_index(odd_at(i, b), b) == i`.
///
/// # Panics
///
/// Debug-asserts that `value >= base_odd`.
///
/// [`odd_at`]: crate::utils::odd_at
#[inline]
pub fn value_to_odd_index(value: usize, base_odd: usize) -> usize {
    debug_assert!(
        value >= base_odd,
        "value_to_odd_index: value={} < base_odd={}",
        value,
        base_odd
    );
    (value - base_odd) / 2
}

/// Estimate the number of primes up to n using the Prime Number Theorem.
/// Returns a safe capacity for Vec::with_capacity (at least 1).
///
/// Uses the upper bound n / (ln(n) - 1.1) for ln(n) > 1.1 (i.e., n >= 4)
/// to avoid reallocations. The constant 1.1 comes from the PNT refinement:
/// π(n) ≈ n / (ln(n) - 1) provides a tight upper bound. Using 1.1 instead
/// of 1.0 adds a small safety margin to ensure the estimate never
/// underestimates.
///
/// For smaller n (ln(n) <= 1.1), returns `n` as a safe upper bound.
///
/// Note: `n as f64` loses precision above 2^53 (~9e15), but MAX_N is 1e15,
/// so this is safe for all valid inputs.
#[must_use]
pub fn estimate_prime_count(n: usize) -> usize {
    if n <= 2 {
        return 1;
    }
    let ln_n = (n as f64).ln();
    // Threshold: ln(n) > 1.1 means n > e^1.1 ≈ 3.0, so formula applies for n >= 4
    const LN_THRESHOLD: f64 = 1.1;
    let estimated = if ln_n > LN_THRESHOLD {
        (n as f64 / (ln_n - LN_THRESHOLD)) as usize
    } else {
        n
    };
    estimated.max(1)
}

/// Validate that segment_size is non-zero.
pub fn validate_segment_size(segment_size: usize) -> Result<(), PrimeGenError> {
    if segment_size == 0 {
        return Err(PrimeGenError::InvalidInput(
            "segment_size cannot be zero".to_string(),
        ));
    }
    Ok(())
}

/// Validate workers parameter.
pub fn validate_workers(workers: usize) -> Result<(), PrimeGenError> {
    if workers == 0 {
        return Err(PrimeGenError::InvalidInput(format!(
            "workers={} is invalid: must be >= 1",
            workers
        )));
    }
    if workers > MAX_WORKERS {
        return Err(PrimeGenError::InvalidInput(format!(
            "workers={} exceeds maximum allowed value of {}",
            workers, MAX_WORKERS
        )));
    }
    Ok(())
}

/// Format a number with comma separators (e.g., 1234567 -> "1,234,567")
pub fn format_number(n: usize) -> String {
    let s = n.to_string();
    let len = s.len();
    if len <= 3 {
        return s;
    }

    let mut result = String::with_capacity(len + len / 3);
    for (i, ch) in s.chars().enumerate() {
        if i > 0 && (len - i).is_multiple_of(3) {
            result.push(',');
        }
        result.push(ch);
    }
    result
}

/// Validate that n does not exceed the maximum supported input size.
pub fn validate_n(n: usize) -> Result<(), PrimeGenError> {
    if n > MAX_N {
        return Err(PrimeGenError::InvalidInput(format!(
            "n={} exceeds maximum supported value {} (1 quadrillion). \
             Generating primes above this limit would require impractical computation time.",
            n, MAX_N
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate_prime_count_edge_cases() {
        assert_eq!(estimate_prime_count(0), 1);
        assert_eq!(estimate_prime_count(1), 1);
        assert_eq!(estimate_prime_count(2), 1);
    }

    #[test]
    fn test_estimate_prime_count_small_values() {
        assert_eq!(estimate_prime_count(3), 3);
        assert!(estimate_prime_count(4) >= 4);
        assert!(estimate_prime_count(10) >= 4);
    }

    #[test]
    fn test_estimate_prime_count_upper_bound() {
        let test_cases: [(usize, usize); 12] = [
            (10, 4),
            (100, 25),
            (1_000, 168),
            (10_000, 1_229),
            (100_000, 9_592),
            (1_000_000, 78_498),
            (10_000_000, 664_579),
            (100_000_000, 5_761_455),
            (1_000_000_000, 50_847_534),
            (10_000_000_000, 455_052_511),
            (100_000_000_000, 4_118_054_813),
            (1_000_000_000_000, 37_607_912_018),
        ];
        for (n, actual_pi) in test_cases {
            let estimated = estimate_prime_count(n);
            assert!(
                estimated >= actual_pi,
                "estimate_prime_count({}) = {} must be >= actual π(n) = {}",
                n,
                estimated,
                actual_pi
            );
        }
    }

    #[test]
    fn test_estimate_prime_count_reasonable_overestimate() {
        let n = 1_000_000;
        let estimated = estimate_prime_count(n);
        let actual = 78498;
        assert!(
            estimated < actual * 2,
            "estimate {} is too far from actual {}",
            estimated,
            actual
        );
    }

    #[test]
    fn test_estimate_prime_count_always_sufficient() {
        let known_pi: [(usize, usize); 8] = [
            (10, 4),
            (100, 25),
            (1_000, 168),
            (10_000, 1_229),
            (100_000, 9_592),
            (1_000_000, 78_498),
            (10_000_000, 664_579),
            (100_000_000, 5_761_455),
        ];
        for (n, actual) in known_pi {
            let est = estimate_prime_count(n);
            assert!(
                est >= actual,
                "estimate_prime_count({n}) = {est} < actual π(n) = {actual}",
            );
        }
    }

    #[test]
    fn test_validate_n_exceeds_max() {
        let result = validate_n(MAX_N + 1);
        assert!(
            matches!(result, Err(PrimeGenError::InvalidInput(msg)) if msg.contains("exceeds maximum"))
        );
    }

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(0), "0");
        assert_eq!(format_number(1), "1");
        assert_eq!(format_number(999), "999");
        assert_eq!(format_number(1_000), "1,000");
        assert_eq!(format_number(12_345), "12,345");
        assert_eq!(format_number(123_456), "123,456");
        assert_eq!(format_number(1_234_567), "1,234,567");
        assert_eq!(format_number(1_000_000_000), "1,000,000,000");
    }

    #[test]
    fn test_odd_at() {
        assert_eq!(odd_at(0, 3), 3);
        assert_eq!(odd_at(1, 3), 5);
        assert_eq!(odd_at(2, 3), 7);
        assert_eq!(odd_at(0, 11), 11);
        assert_eq!(odd_at(3, 11), 17);
    }

    #[test]
    fn test_value_to_odd_index() {
        assert_eq!(value_to_odd_index(3, 3), 0);
        assert_eq!(value_to_odd_index(5, 3), 1);
        assert_eq!(value_to_odd_index(7, 3), 2);
        assert_eq!(value_to_odd_index(11, 11), 0);
        assert_eq!(value_to_odd_index(17, 11), 3);
    }

    #[test]
    fn test_odd_helpers_inverse() {
        for base_odd in [3_usize, 11, 99, 1_001, 999_999] {
            for idx in [0_usize, 1, 5, 100, 10_000] {
                assert_eq!(value_to_odd_index(odd_at(idx, base_odd), base_odd), idx);
            }
        }
    }
}
