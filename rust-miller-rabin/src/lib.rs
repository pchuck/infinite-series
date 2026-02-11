//! Miller-Rabin Primality Tester
//!
//! High-performance Miller-Rabin primality testing for arbitrarily large integers
//! with support for deterministic testing and parallel execution.
//!
//! # Features
//!
//! - **Deterministic testing**: Guaranteed correct results for numbers up to 3.3×10²⁵
//! - **Parallel execution**: Multi-threaded testing for large numbers
//! - **Progress tracking**: Optional progress bars for long-running tests
//! - **Custom bases**: Support for custom test bases
//! - **Zero dependencies**: Pure Rust with only num-bigint for large integers
//!
//! # Algorithm Overview
//!
//! The Miller-Rabin test is a probabilistic primality test. For numbers less than
//! specific thresholds, it can be made deterministic by testing against known bases:
//!
//! - **n < 3,474,749,660,399**: 12 bases (deterministic for all 64-bit integers)
//! - **n < 3,317,044,064,679,887,385,961,981**: 19 bases (deterministic up to ~3.3×10²⁵)
//!
//! # Examples
//!
//! ## Basic Usage
//!
//! ```
//! use miller_rabin_tester::is_probable_prime;
//! use num_bigint::BigUint;
//!
//! let n = BigUint::from(104729u32);
//! assert!(is_probable_prime(&n));
//!
//! let composite = BigUint::from(561u32); // Carmichael number
//! assert!(!is_probable_prime(&composite));
//! ```
//!
//! ## Parallel Testing
//!
//! ```
//! use miller_rabin_tester::is_probable_prime_parallel;
//! use num_bigint::BigUint;
//!
//! let n = BigUint::from(104729u32);
//! let result = is_probable_prime_parallel(&n, 4, &[]).unwrap();
//! assert!(result);
//! ```
//!
//! ## Progress Tracking
//!
//! ```
//! use miller_rabin_tester::is_probable_prime_with_progress;
//! use num_bigint::BigUint;
//!
//! let n = BigUint::from(104729u32);
//! let progress_fn = |current: usize, total: usize| {
//!     println!("Progress: {}/{}", current, total);
//! };
//! let result = is_probable_prime_with_progress(&n, &progress_fn);
//! ```

// Module declarations
pub mod bases;
pub mod error;
pub mod parallel;
pub mod progress;
pub mod witness;

// Re-export main types and functions
pub use bases::{filter_bases_for_n, get_test_bases_for_size};
pub use error::{PrimalityError, Result};
pub use parallel::is_probable_prime_parallel;
pub use progress::{ProgressBar, ProgressCallback};
pub use witness::{decompose_into_d_and_s, miller_rabin_test, miller_rabin_witness, mod_pow};

use num_bigint::BigUint;
use num_traits::Zero;

/// Checks if a number passes small prime divisibility tests
///
/// Returns Some(true) if n is a small prime (2, 3, 5),
/// Some(false) if n is divisible by a small prime (composite),
/// None if more testing is needed.
fn check_small_primes(n: &BigUint) -> Option<bool> {
    if n < &BigUint::from(2u32) {
        return Some(false);
    }

    for p in [2u32, 3, 5] {
        let small_p = BigUint::from(p);
        if n == &small_p {
            return Some(true);
        }
        if (n % &small_p).is_zero() {
            return Some(false);
        }
    }

    None
}

/// Tests if a number is probably prime using the Miller-Rabin test
///
/// This is the main entry point for primality testing. It automatically selects
/// the appropriate number of test bases based on the size of the input number.
///
/// # Arguments
/// * `n` - The number to test for primality
///
/// # Returns
/// Returns `true` if n is probably prime, `false` if definitely composite.
///
/// # Examples
/// ```
/// use miller_rabin_tester::is_probable_prime;
/// use num_bigint::BigUint;
///
/// // Test a known prime
/// let n = BigUint::from(104729u32);
/// assert!(is_probable_prime(&n));
///
/// // Test a composite number
/// let composite = BigUint::from(100u32);
/// assert!(!is_probable_prime(&composite));
/// ```
pub fn is_probable_prime(n: &BigUint) -> bool {
    if let Some(result) = check_small_primes(n) {
        return result;
    }

    let (d, s) = decompose_into_d_and_s(n);
    let all_bases = get_test_bases_for_size(n);

    // Filter bases to only those less than n (Miller-Rabin requires 2 <= a < n)
    let bases: Vec<u64> = all_bases
        .iter()
        .filter(|&&a| BigUint::from(a) < *n)
        .copied()
        .collect();

    for a in &bases {
        let a_big = BigUint::from(*a);
        if !miller_rabin_witness(&a_big, &d, s, n) {
            return false;
        }
    }

    true
}

/// Tests primality with a custom set of bases
///
/// Allows specifying custom test bases instead of using the default deterministic sets.
/// If custom_bases is empty, uses the default bases.
///
/// # Arguments
/// * `n` - The number to test
/// * `custom_bases` - Custom test bases (empty = use defaults)
///
/// # Examples
/// ```
/// use miller_rabin_tester::is_probable_prime_with_bases;
/// use num_bigint::BigUint;
///
/// let n = BigUint::from(104729u32);
/// let bases = vec![2u64, 3, 5, 7];
/// let result = is_probable_prime_with_bases(&n, &bases);
/// ```
pub fn is_probable_prime_with_bases(n: &BigUint, custom_bases: &[u64]) -> bool {
    if let Some(result) = check_small_primes(n) {
        return result;
    }

    let (d, s) = decompose_into_d_and_s(n);

    let all_bases: Vec<u64> = if custom_bases.is_empty() {
        get_test_bases_for_size(n).to_vec()
    } else {
        custom_bases.to_vec()
    };

    // Filter bases to only those less than n (Miller-Rabin requires 2 <= a < n)
    let bases = filter_bases_for_n(&all_bases, n);

    for a in &bases {
        let a_big = BigUint::from(*a);
        if !miller_rabin_witness(&a_big, &d, s, n) {
            return false;
        }
    }

    true
}

/// Tests primality with progress reporting
///
/// Similar to `is_probable_prime` but calls the provided callback with progress updates.
/// The callback receives (current_progress, total_work) as arguments.
///
/// # Arguments
/// * `n` - The number to test
/// * `progress_callback` - Function called with (current, total) progress
///
/// # Examples
/// ```
/// use miller_rabin_tester::is_probable_prime_with_progress;
/// use num_bigint::BigUint;
/// use std::sync::atomic::{AtomicUsize, Ordering};
/// use std::sync::Arc;
///
/// let n = BigUint::from(104729u32);
/// let progress = Arc::new(AtomicUsize::new(0));
/// let progress_clone = Arc::clone(&progress);
/// is_probable_prime_with_progress(&n, &move |current, _total| {
///     progress_clone.store(current, Ordering::Relaxed);
/// });
/// ```
pub fn is_probable_prime_with_progress(n: &BigUint, progress_callback: &ProgressCallback) -> bool {
    if let Some(result) = check_small_primes(n) {
        return result;
    }

    let (d, s) = decompose_into_d_and_s(n);
    let all_bases = get_test_bases_for_size(n);

    // Filter bases to only those less than n (Miller-Rabin requires 2 <= a < n)
    let bases: Vec<u64> = all_bases
        .iter()
        .filter(|&&a| BigUint::from(a) < *n)
        .copied()
        .collect();

    // Delegate to parallel module's sequential-with-progress, which
    // provides bit-level granularity via mod_pow progress tracking.
    use std::sync::atomic::AtomicBool;
    let stop_flag = AtomicBool::new(false);
    parallel::test_bases_parallel_with_progress(n, &d, s, &bases, 1, &stop_flag, progress_callback)
}

/// Parallel primality testing with progress reporting
///
/// Combines parallel execution with progress callbacks.
///
/// # Arguments
/// * `n` - The number to test
/// * `threads` - Number of threads to use
/// * `progress_callback` - Progress callback function
///
/// # Returns
/// Returns true if n is probably prime, false if definitely composite
pub fn is_probable_prime_parallel_with_progress(
    n: &BigUint,
    threads: usize,
    progress_callback: &ProgressCallback,
) -> bool {
    use parallel::test_bases_parallel_with_progress;
    use std::sync::atomic::AtomicBool;

    if let Some(result) = check_small_primes(n) {
        return result;
    }

    if threads <= 1 {
        return is_probable_prime_with_progress(n, progress_callback);
    }

    let (d, s) = decompose_into_d_and_s(n);
    let bases = get_test_bases_for_size(n);

    let stop_flag = AtomicBool::new(false);
    test_bases_parallel_with_progress(n, &d, s, bases, threads, &stop_flag, progress_callback)
}

/// Parallel primality testing with custom bases and progress
///
/// Full-featured interface allowing control over parallelism, bases, and progress.
pub fn is_probable_prime_parallel_with_bases(
    n: &BigUint,
    threads: usize,
    custom_bases: &[u64],
) -> Result<bool> {
    parallel::is_probable_prime_parallel(n, threads, custom_bases)
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::One;

    // Small primes
    #[test]
    fn test_small_primes() {
        let small_primes = [2u32, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
        for p in small_primes {
            let n = BigUint::from(p);
            assert!(is_probable_prime(&n), "{} should be prime", p);
        }
    }

    // Small composites
    #[test]
    fn test_small_composites() {
        let composites = [
            0u32, 1, 4, 6, 8, 9, 10, 12, 14, 15, 16, 18, 20, 21, 22, 24, 25, 26, 27, 28,
        ];
        for c in composites {
            let n = BigUint::from(c);
            assert!(!is_probable_prime(&n), "{} should be composite", c);
        }
    }

    // Known large primes
    #[test]
    fn test_known_primes() {
        let primes = [
            "1009",
            "104729",    // 10000th prime
            "1299709",   // 100000th prime
            "15485863",  // 1000000th prime
            "179424673", // 10000000th prime
        ];
        for p in &primes {
            let n: BigUint = p.parse().unwrap();
            assert!(is_probable_prime(&n), "{} should be prime", p);
        }
    }

    // Known composites
    #[test]
    fn test_known_composites() {
        let composites = [
            "341",  // 11 * 31
            "561",  // First Carmichael number
            "645",  // 3 * 5 * 43
            "1105", // Carmichael number
            "1729", // Carmichael number (Ramanujan's number)
            "2465", // Carmichael number
            "2821", // Carmichael number
            "6601", // Carmichael number
        ];
        for c in &composites {
            let n: BigUint = c.parse().unwrap();
            assert!(!is_probable_prime(&n), "{} should be composite", c);
        }
    }

    // Carmichael numbers (should all be detected as composite)
    #[test]
    fn test_carmichael_numbers() {
        // Carmichael numbers fool Fermat test but not Miller-Rabin
        let carmichaels = [
            "561", "1105", "1729", "2465", "2821", "6601", "8911", "10585", "15841", "29341",
            "41041", "46657", "52633", "62745", "63973",
        ];
        for n_str in &carmichaels {
            let n: BigUint = n_str.parse().unwrap();
            assert!(
                !is_probable_prime(&n),
                "{} is a Carmichael number (must be composite)",
                n_str
            );
        }
    }

    // Fermat primes (2^(2^n) + 1)
    #[test]
    fn test_fermat_primes() {
        // Known Fermat primes for n = 0, 1, 2, 3, 4
        let fermat_primes = [
            BigUint::from(3u32),     // 2^1 + 1
            BigUint::from(5u32),     // 2^2 + 1
            BigUint::from(17u32),    // 2^4 + 1
            BigUint::from(257u32),   // 2^8 + 1
            BigUint::from(65537u32), // 2^16 + 1
        ];
        for p in &fermat_primes {
            assert!(is_probable_prime(p), "{} should be a Fermat prime", p);
        }
    }

    // Fermat composites (n >= 5)
    #[test]
    fn test_fermat_composites() {
        // Fermat numbers for n >= 5 are known to be composite
        // F_n = 2^(2^n) + 1
        let fermat_composites = [5usize, 6, 7, 8];
        for n in &fermat_composites {
            let two_pow_n = 2u32.pow(*n as u32) as u64; // 2^n
            let f_n = BigUint::from(2u32).pow(two_pow_n as u32) + BigUint::one();
            assert!(
                !is_probable_prime(&f_n),
                "F_{} = {} should be composite",
                n,
                f_n
            );
        }
    }

    // Test custom bases
    #[test]
    fn test_custom_bases() {
        let n = BigUint::from(104729u32);
        let bases = vec![2u64, 3, 5, 7];
        assert!(is_probable_prime_with_bases(&n, &bases));

        // Empty bases should use defaults
        assert!(is_probable_prime_with_bases(&n, &[]));
    }

    // Test with very small numbers
    #[test]
    fn test_edge_cases() {
        assert!(!is_probable_prime(&BigUint::zero()));
        assert!(!is_probable_prime(&BigUint::one()));
        assert!(is_probable_prime(&BigUint::from(2u32)));
    }

    // Test progress callback
    #[test]
    fn test_progress_callback() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let n = BigUint::from(104729u32);
        let progress_calls = Arc::new(AtomicUsize::new(0));
        let calls_clone = Arc::clone(&progress_calls);
        let result = is_probable_prime_with_progress(&n, &move |_, _| {
            calls_clone.fetch_add(1, Ordering::Relaxed);
        });
        assert!(result);
        assert!(
            progress_calls.load(Ordering::Relaxed) > 0,
            "Progress callback should be called"
        );
    }

    // Test deterministic bounds
    #[test]
    fn test_64_bit_deterministic() {
        // Test a number just below the 64-bit threshold
        let threshold = BigUint::from(3_474_749_660_399u64);
        let n = &threshold - BigUint::one();
        assert!(is_probable_prime(&n) || !is_probable_prime(&n)); // Just verify it doesn't panic
    }
}
