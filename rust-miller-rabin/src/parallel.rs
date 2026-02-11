//! Parallel execution support for Miller-Rabin primality testing
//!
//! This module provides thread-based parallelism for testing multiple bases
//! concurrently, with support for early termination when a witness is found.

use crate::bases::{filter_bases_for_n, get_test_bases_for_size};
use crate::error::Result;
use crate::progress::ProgressCallback;
use crate::witness::{decompose_into_d_and_s, miller_rabin_witness};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

/// Parallel Miller-Rabin witness testing with early termination
///
/// Tests multiple bases in parallel, stopping early if any base proves
/// the number is composite. Uses atomic operations for coordination.
///
/// # Arguments
/// * `n` - The number to test
/// * `d` - Odd component from n-1 = d * 2^s
/// * `s` - Power of 2 component
/// * `bases` - Test bases to check
/// * `threads` - Number of threads to use
/// * `stop_flag` - Atomic flag for early termination
///
/// # Returns
/// Returns true if n is probably prime (no witnesses found),
/// returns false if n is definitely composite (witness found).
pub fn test_bases_parallel(
    n: &BigUint,
    d: &BigUint,
    s: usize,
    bases: &[u64],
    threads: usize,
    stop_flag: &AtomicBool,
) -> bool {
    if threads <= 1 || bases.len() < 2 {
        return test_bases_sequential(n, d, s, bases, stop_flag);
    }

    std::thread::scope(|scope| {
        let mut handles = Vec::with_capacity(threads);
        let chunk_size = (bases.len() + threads - 1) / threads;

        for i in 0..threads {
            let start_idx = i * chunk_size;
            if start_idx >= bases.len() {
                break;
            }
            let end_idx = std::cmp::min(start_idx + chunk_size, bases.len());

            let d_ref = d;
            let n_ref = n;
            let bases_ref = bases;
            let stop_ref = stop_flag;

            handles.push(scope.spawn(move || -> bool {
                for j in start_idx..end_idx {
                    if stop_ref.load(Ordering::Relaxed) {
                        return true;
                    }

                    let a_big = BigUint::from(bases_ref[j]);
                    if !miller_rabin_witness(&a_big, d_ref, s, n_ref) {
                        stop_ref.store(true, Ordering::Relaxed);
                        return false;
                    }
                }
                true
            }));
        }

        let mut all_passed = true;
        for handle in handles {
            if let Ok(passed) = handle.join() {
                if !passed {
                    all_passed = false;
                }
            }
        }
        all_passed
    })
}

/// Sequential witness testing with early termination support
fn test_bases_sequential(
    n: &BigUint,
    d: &BigUint,
    s: usize,
    bases: &[u64],
    stop_flag: &AtomicBool,
) -> bool {
    for a in bases {
        if stop_flag.load(Ordering::Relaxed) {
            return true;
        }

        let a_big = BigUint::from(*a);
        if !miller_rabin_witness(&a_big, d, s, n) {
            stop_flag.store(true, Ordering::Relaxed);
            return false;
        }
    }
    true
}

/// Parallel Miller-Rabin with progress reporting
///
/// Worker threads increment a shared atomic counter during modular
/// exponentiation. The calling thread polls that counter and forwards
/// updates to `progress_callback` so the progress bar can render.
pub fn test_bases_parallel_with_progress(
    n: &BigUint,
    d: &BigUint,
    s: usize,
    bases: &[u64],
    threads: usize,
    stop_flag: &AtomicBool,
    progress_callback: &ProgressCallback,
) -> bool {
    if bases.is_empty() {
        return true;
    }

    // For 1 thread or 1 base, run sequentially with progress
    if threads <= 1 || bases.len() < 2 {
        return test_bases_sequential_with_progress(n, d, s, bases, stop_flag, progress_callback);
    }

    let d_bits = d.bits() as usize;
    let total_bits_all_bases = bases.len() * d_bits;
    if total_bits_all_bases == 0 {
        return true;
    }

    let completed_bits = Arc::new(AtomicUsize::new(0));
    let done_flag = Arc::new(AtomicBool::new(false));

    // Spawn a polling thread that reads the shared counter and calls
    // the progress callback from a single thread (avoids Send/Sync
    // issues with the callback reference).
    let completed_poll = Arc::clone(&completed_bits);
    let done_poll = Arc::clone(&done_flag);
    let total_for_poll = total_bits_all_bases;

    std::thread::scope(|scope| {
        // Progress polling thread
        let poll_handle = scope.spawn(move || {
            let poll_interval = std::time::Duration::from_millis(50);
            loop {
                let bits = completed_poll.load(Ordering::Relaxed);
                progress_callback(bits.min(total_for_poll), total_for_poll);

                if done_poll.load(Ordering::Relaxed) {
                    // Final update
                    let bits = completed_poll.load(Ordering::Relaxed);
                    progress_callback(bits.min(total_for_poll), total_for_poll);
                    break;
                }
                std::thread::sleep(poll_interval);
            }
        });

        // Worker threads
        let mut handles = Vec::with_capacity(threads);
        let chunk_size = (bases.len() + threads - 1) / threads;

        for i in 0..threads {
            let start_idx = i * chunk_size;
            if start_idx >= bases.len() {
                break;
            }
            let end_idx = std::cmp::min(start_idx + chunk_size, bases.len());

            let d_ref = d;
            let n_ref = n;
            let bases_ref = bases;
            let stop_ref = stop_flag;
            let completed = Arc::clone(&completed_bits);

            handles.push(scope.spawn(move || -> bool {
                for j in start_idx..end_idx {
                    if stop_ref.load(Ordering::Relaxed) {
                        return true;
                    }

                    let a_big = BigUint::from(bases_ref[j]);

                    let x =
                        mod_pow_with_progress(a_big, d_ref.clone(), n_ref, &completed, stop_ref);

                    // Witness check (same logic as miller_rabin_witness)
                    let one = BigUint::one();
                    let n_minus_1 = n_ref - &one;

                    if x == one || x == n_minus_1 {
                        continue;
                    }

                    let mut current = x;
                    let mut is_composite = true;
                    for _ in 0..s {
                        let y = (&current * &current) % n_ref;
                        if y == n_minus_1 {
                            is_composite = false;
                            break;
                        }
                        if y == one {
                            break;
                        }
                        current = y;
                    }

                    if is_composite {
                        stop_ref.store(true, Ordering::Relaxed);
                        return false;
                    }
                }
                true
            }));
        }

        // Wait for all workers
        let mut all_passed = true;
        for handle in handles {
            if let Ok(passed) = handle.join() {
                if !passed {
                    all_passed = false;
                }
            }
        }

        // Signal the polling thread to stop, then wait for it
        done_flag.store(true, Ordering::Relaxed);
        let _ = poll_handle.join();

        all_passed
    })
}

/// Sequential witness testing with progress callback
fn test_bases_sequential_with_progress(
    n: &BigUint,
    d: &BigUint,
    s: usize,
    bases: &[u64],
    stop_flag: &AtomicBool,
    progress_callback: &ProgressCallback,
) -> bool {
    let d_bits = d.bits() as usize;
    let total_bits_all_bases = bases.len() * d_bits;
    if total_bits_all_bases == 0 {
        return true;
    }

    let completed_bits = AtomicUsize::new(0);

    for a in bases {
        if stop_flag.load(Ordering::Relaxed) {
            return true;
        }

        let a_big = BigUint::from(*a);

        // mod_pow with inline progress reporting
        let x = mod_pow_with_sequential_progress(
            a_big,
            d.clone(),
            n,
            &completed_bits,
            total_bits_all_bases,
            progress_callback,
        );

        let one = BigUint::one();
        let n_minus_1 = n - &one;

        if x == one || x == n_minus_1 {
            continue;
        }

        let mut current = x;
        let mut is_composite = true;
        for _ in 0..s {
            let y = (&current * &current) % n;
            if y == n_minus_1 {
                is_composite = false;
                break;
            }
            if y == one {
                break;
            }
            current = y;
        }

        if is_composite {
            stop_flag.store(true, Ordering::Relaxed);
            progress_callback(total_bits_all_bases, total_bits_all_bases);
            return false;
        }
    }

    progress_callback(total_bits_all_bases, total_bits_all_bases);
    true
}

/// Modular exponentiation with shared atomic progress counter
///
/// Used by parallel workers: increments `completed_bits` per loop
/// iteration. A separate polling thread reads the counter and calls
/// the user-visible callback.
fn mod_pow_with_progress(
    mut base: BigUint,
    mut exp: BigUint,
    modulus: &BigUint,
    completed_bits: &Arc<AtomicUsize>,
    stop_flag: &AtomicBool,
) -> BigUint {
    if modulus.is_one() {
        return BigUint::zero();
    }

    let mut result = BigUint::one();
    base %= modulus;

    while !exp.is_zero() {
        if stop_flag.load(Ordering::Relaxed) {
            return result; // Early exit, result doesn't matter
        }

        if (&exp & &BigUint::one()).is_one() {
            result = (&result * &base) % modulus;
        }
        base = (&base * &base) % modulus;
        exp >>= 1usize;

        completed_bits.fetch_add(1, Ordering::Relaxed);
    }

    result
}

/// Modular exponentiation with direct progress callback
///
/// Used by the sequential path: calls `progress_callback` directly
/// at throttled intervals.
fn mod_pow_with_sequential_progress(
    mut base: BigUint,
    mut exp: BigUint,
    modulus: &BigUint,
    completed_bits: &AtomicUsize,
    total_bits_all_bases: usize,
    progress_callback: &ProgressCallback,
) -> BigUint {
    if modulus.is_one() {
        return BigUint::zero();
    }

    let mut result = BigUint::one();
    base %= modulus;

    let update_interval = (total_bits_all_bases / 100).max(10);

    while !exp.is_zero() {
        if (&exp & &BigUint::one()).is_one() {
            result = (&result * &base) % modulus;
        }
        base = (&base * &base) % modulus;
        exp >>= 1usize;

        let bits = completed_bits.fetch_add(1, Ordering::Relaxed) + 1;
        if bits % update_interval == 0 || bits >= total_bits_all_bases {
            progress_callback(bits.min(total_bits_all_bases), total_bits_all_bases);
        }
    }

    result
}

/// High-level interface for parallel primality testing
///
/// # Arguments
/// * `n` - Number to test
/// * `threads` - Number of threads (0 = auto-detect)
/// * `custom_bases` - Optional custom bases (empty = use defaults)
///
/// # Returns
/// Returns true if n is probably prime, false if definitely composite
pub fn is_probable_prime_parallel(
    n: &BigUint,
    threads: usize,
    custom_bases: &[u64],
) -> Result<bool> {
    if n < &BigUint::from(2u32) {
        return Ok(false);
    }

    // Check small primes first
    for p in [2u32, 3, 5] {
        let p_big = BigUint::from(p);
        if n == &p_big {
            return Ok(true);
        }
        if (n % &p_big).is_zero() {
            return Ok(false);
        }
    }

    let (d, s) = decompose_into_d_and_s(n);
    let bases: Vec<u64> = if custom_bases.is_empty() {
        get_test_bases_for_size(n).to_vec()
    } else {
        filter_bases_for_n(custom_bases, n)
    };

    let stop_flag = AtomicBool::new(false);
    let result = test_bases_parallel(n, &d, s, &bases, threads, &stop_flag);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_prime() {
        let n = BigUint::from(104729u32);
        let result = is_probable_prime_parallel(&n, 4, &[]).unwrap();
        assert!(result);
    }

    #[test]
    fn test_parallel_composite() {
        let n = BigUint::from(561u32);
        let result = is_probable_prime_parallel(&n, 4, &[]).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_early_termination() {
        let n = BigUint::from(561u32);
        let (d, s) = decompose_into_d_and_s(&n);
        let bases = vec![2u64, 3, 5, 7, 11, 13];
        let stop_flag = AtomicBool::new(false);

        let result = test_bases_parallel(&n, &d, s, &bases, 2, &stop_flag);
        assert!(!result);
        assert!(stop_flag.load(Ordering::Relaxed));
    }

    #[test]
    fn test_parallel_with_progress_prime() {
        use std::sync::atomic::AtomicUsize;

        let n = BigUint::from(104729u32);
        let (d, s) = decompose_into_d_and_s(&n);
        let bases = vec![2u64, 3, 5, 7, 11, 13];
        let stop_flag = AtomicBool::new(false);
        let calls = Arc::new(AtomicUsize::new(0));
        let calls_clone = Arc::clone(&calls);

        let result = test_bases_parallel_with_progress(
            &n,
            &d,
            s,
            &bases,
            2,
            &stop_flag,
            &move |_current, _total| {
                calls_clone.fetch_add(1, Ordering::Relaxed);
            },
        );
        assert!(result);
        assert!(
            calls.load(Ordering::Relaxed) > 0,
            "progress callback was never called"
        );
    }

    #[test]
    fn test_sequential_with_progress_composite() {
        use std::sync::atomic::AtomicUsize;

        let n = BigUint::from(561u32);
        let (d, s) = decompose_into_d_and_s(&n);
        let bases = vec![2u64, 3, 5, 7];
        let stop_flag = AtomicBool::new(false);
        let calls = Arc::new(AtomicUsize::new(0));
        let calls_clone = Arc::clone(&calls);

        let result = test_bases_sequential_with_progress(
            &n,
            &d,
            s,
            &bases,
            &stop_flag,
            &move |_current, _total| {
                calls_clone.fetch_add(1, Ordering::Relaxed);
            },
        );
        assert!(!result);
        assert!(
            calls.load(Ordering::Relaxed) > 0,
            "progress callback was never called"
        );
    }
}
