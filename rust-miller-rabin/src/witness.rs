//! Miller-Rabin witness testing and modular arithmetic
//!
//! This module implements the core Miller-Rabin primality test algorithm,
//! including modular exponentiation and witness detection.

use num_bigint::BigUint;
use num_traits::{One, Zero};

/// Decomposes n-1 into d * 2^s where d is odd.
///
/// This is the first step in the Miller-Rabin test, writing n-1 in the form
/// required by the algorithm.
///
/// # Examples
/// ```
/// use miller_rabin_tester::decompose_into_d_and_s;
/// use num_bigint::BigUint;
///
/// let n = BigUint::from(561u32); // 561 - 1 = 560 = 35 * 2^4
/// let (d, s) = decompose_into_d_and_s(&n);
/// assert_eq!(d, BigUint::from(35u32));
/// assert_eq!(s, 4);
/// ```
pub fn decompose_into_d_and_s(n: &BigUint) -> (BigUint, usize) {
    let one = BigUint::one();
    let mut d = n.clone() - &one;
    let mut s: usize = 0;

    while (&d & &one).is_zero() {
        d >>= 1usize;
        s += 1;
    }

    (d, s)
}

/// Modular exponentiation: computes (base^exp) % modulus efficiently
/// using the square-and-multiply algorithm.
///
/// This is a critical performance function used extensively in Miller-Rabin testing.
///
/// # Examples
/// ```
/// use miller_rabin_tester::mod_pow;
/// use num_bigint::BigUint;
///
/// let base = BigUint::from(3u32);
/// let exp = BigUint::from(10u32);
/// let modulus = BigUint::from(7u32);
/// let result = mod_pow(base, exp, &modulus);
/// assert_eq!(result, BigUint::from(4u32)); // 3^10 mod 7 = 59049 mod 7 = 4
/// ```
pub fn mod_pow(mut base: BigUint, mut exp: BigUint, modulus: &BigUint) -> BigUint {
    if modulus.is_one() {
        return BigUint::zero();
    }

    let mut result = BigUint::one();
    base %= modulus;

    while !exp.is_zero() {
        if (&exp & &one()).is_one() {
            result = (&result * &base) % modulus;
        }
        base = (&base * &base) % modulus;
        exp >>= 1usize;
    }

    result
}

/// Tests if 'a' is a Miller-Rabin witness for the compositeness of n.
///
/// Returns true if 'a' is NOT a witness (n may be prime),
/// returns false if 'a' IS a witness (n is definitely composite).
///
/// # Arguments
/// * `a` - The base to test (must be 2 <= a < n)
/// * `d` - The odd component from n-1 = d * 2^s
/// * `s` - The power of 2 from n-1 = d * 2^s
/// * `n` - The number being tested for primality
///
/// # Examples
/// ```
/// use miller_rabin_tester::{miller_rabin_witness, decompose_into_d_and_s};
/// use num_bigint::BigUint;
///
/// // Test if 2 is a witness for 561 (it is - 561 is composite)
/// let n = BigUint::from(561u32);
/// let a = BigUint::from(2u32);
/// let (d, s) = decompose_into_d_and_s(&n);
/// let is_not_witness = miller_rabin_witness(&a, &d, s, &n);
/// assert!(!is_not_witness); // 2 IS a witness that 561 is composite
/// ```
pub fn miller_rabin_witness(a: &BigUint, d: &BigUint, s: usize, n: &BigUint) -> bool {
    let x = mod_pow(a.clone(), d.clone(), n);
    let one = BigUint::one();
    let n_minus_1 = n - &one;

    if x == one || x == n_minus_1 {
        return true;
    }

    let mut current = x;
    for _ in 0..s {
        let y = (&current * &current) % n;
        if y == n_minus_1 {
            return true;
        }
        if y == one {
            return false;
        }
        current = y;
    }

    false
}

/// Convenience function that performs a single Miller-Rabin test with base 'a'.
///
/// Returns true if 'a' is NOT a witness (n may be prime relative to base a),
/// returns false if 'a' IS a witness (n is definitely composite).
///
/// # Examples
/// ```
/// use miller_rabin_tester::miller_rabin_test;
/// use num_bigint::BigUint;
///
/// let n = BigUint::from(104729u32); // Known prime
/// let a = BigUint::from(2u32);
/// assert!(miller_rabin_test(&a, &n)); // 2 is not a witness for this prime
/// ```
pub fn miller_rabin_test(a: &BigUint, n: &BigUint) -> bool {
    if n <= a || a.is_zero() {
        return *n == BigUint::from(2u32);
    }
    let (d, s) = decompose_into_d_and_s(n);
    miller_rabin_witness(a, &d, s, n)
}

/// Helper function to create BigUint::one()
#[inline]
fn one() -> BigUint {
    BigUint::one()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompose() {
        // 561 - 1 = 560 = 35 * 2^4
        let n = BigUint::from(561u32);
        let (d, s) = decompose_into_d_and_s(&n);
        assert_eq!(d, BigUint::from(35u32));
        assert_eq!(s, 4);
    }

    #[test]
    fn test_decompose_prime() {
        // 17 - 1 = 16 = 1 * 2^4
        let n = BigUint::from(17u32);
        let (d, s) = decompose_into_d_and_s(&n);
        assert_eq!(d, BigUint::one());
        assert_eq!(s, 4);
    }

    #[test]
    fn test_mod_pow() {
        let base = BigUint::from(3u32);
        let exp = BigUint::from(10u32);
        let modulus = BigUint::from(7u32);
        assert_eq!(mod_pow(base, exp, &modulus), BigUint::from(4u32));

        // Edge cases
        assert_eq!(
            mod_pow(BigUint::from(2u32), BigUint::zero(), &BigUint::from(7u32)),
            BigUint::one()
        );
        assert_eq!(
            mod_pow(BigUint::from(5u32), BigUint::one(), &BigUint::from(7u32)),
            BigUint::from(5u32)
        );
    }

    #[test]
    fn test_witness_for_composite() {
        // 561 is a Carmichael number, but 2 is a witness
        let n = BigUint::from(561u32);
        let a = BigUint::from(2u32);
        let (d, s) = decompose_into_d_and_s(&n);
        assert!(!miller_rabin_witness(&a, &d, s, &n)); // 2 IS a witness
    }

    #[test]
    fn test_witness_for_prime() {
        // 17 is prime, no base should be a witness
        let n = BigUint::from(17u32);
        let a = BigUint::from(2u32);
        let (d, s) = decompose_into_d_and_s(&n);
        assert!(miller_rabin_witness(&a, &d, s, &n)); // 2 is NOT a witness
    }
}
