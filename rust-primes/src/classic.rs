use crate::error::PrimeGenError;
use crate::utils::{estimate_prime_count, isqrt};
use crate::ProgressCallback;

/// Classic Sieve of Eratosthenes (odd-only)
/// Best for n < 1,000,000
///
/// # Arguments
/// * `n` - Upper bound (exclusive) for prime generation
/// * `progress` - Optional callback invoked once when sieving completes
///
/// # Examples
///
/// ```
/// use primes::sieve_of_eratosthenes;
///
/// let primes = sieve_of_eratosthenes(10, None).unwrap();
/// assert_eq!(primes, vec![2, 3, 5, 7]);
///
/// // Empty results for small inputs
/// assert_eq!(sieve_of_eratosthenes(0, None).unwrap(), Vec::<usize>::new());
/// assert_eq!(sieve_of_eratosthenes(1, None).unwrap(), Vec::<usize>::new());
/// assert_eq!(sieve_of_eratosthenes(2, None).unwrap(), Vec::<usize>::new());
/// ```
pub fn sieve_of_eratosthenes(
    n: usize,
    progress: Option<ProgressCallback>,
) -> Result<Vec<usize>, PrimeGenError> {
    if n <= 2 {
        return Ok(Vec::new());
    }

    if n <= 3 {
        return Ok(vec![2]);
    }

    let sieve_size = (n - 3).div_ceil(2);
    let sieve_size = sieve_size.max(1);
    let mut sieve = vec![true; sieve_size];

    let limit = isqrt(n);
    let mut current = 3;
    while current <= limit {
        let idx = (current - 3) / 2;
        if idx < sieve_size && sieve[idx] {
            let Some(squared) = current.checked_mul(current) else {
                break;
            };
            let start_idx = squared.saturating_sub(3) / 2;
            let step = current;
            for j in (start_idx..sieve_size).step_by(step) {
                sieve[j] = false;
            }
        }
        current += 2;
    }

    let mut primes = Vec::with_capacity(estimate_prime_count(n));
    primes.push(2);
    for (i, &is_p) in sieve.iter().enumerate() {
        if is_p {
            let prime = 2 * i + 3;
            if prime < n {
                primes.push(prime);
            }
        }
    }

    if let Some(ref callback) = progress {
        callback(1);
    }

    Ok(primes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sieve_small() {
        assert_eq!(sieve_of_eratosthenes(10, None).unwrap(), vec![2, 3, 5, 7]);
        assert_eq!(
            sieve_of_eratosthenes(30, None).unwrap(),
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
        );
    }

    #[test]
    fn test_sieve_empty() {
        assert_eq!(sieve_of_eratosthenes(0, None).unwrap(), Vec::<usize>::new());
        assert_eq!(sieve_of_eratosthenes(1, None).unwrap(), Vec::<usize>::new());
        assert_eq!(sieve_of_eratosthenes(2, None).unwrap(), Vec::<usize>::new());
    }

    #[test]
    fn test_sieve_boundary() {
        assert_eq!(sieve_of_eratosthenes(3, None).unwrap(), vec![2]);
        assert_eq!(sieve_of_eratosthenes(4, None).unwrap(), vec![2, 3]);
        assert_eq!(sieve_of_eratosthenes(5, None).unwrap(), vec![2, 3]);
        assert_eq!(sieve_of_eratosthenes(6, None).unwrap(), vec![2, 3, 5]);
    }

    #[test]
    fn test_no_composites() {
        let primes = sieve_of_eratosthenes(200, None).unwrap();
        for &p in &primes {
            assert!(p >= 2, "Found value < 2: {}", p);
            if p > 2 {
                assert!(!p.is_multiple_of(2), "Found even composite: {}", p);
                let mut d = 3;
                while d * d <= p {
                    assert!(p % d != 0, "Found composite: {} (divisible by {})", p, d);
                    d += 2;
                }
            }
        }
    }
}
