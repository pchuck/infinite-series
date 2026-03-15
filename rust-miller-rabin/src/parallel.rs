//! Parallel execution support for Miller-Rabin primality testing
//!
//! This module provides thread-based parallelism for testing multiple bases
//! concurrently, with support for early termination when a witness is found.

use crate::bases::{filter_bases_for_n, get_test_bases_for_size};
use crate::error::Result;
use crate::progress::ProgressCallback;
use crate::witness::{decompose_into_d_and_s, miller_rabin_witness, mod_pow, witness_check};

use num_bigint::BigUint;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

/// Parallel Miller-Rabin witness testing with early termination
///
/// Tests multiple bases in parallel, stopping early if any base proves
/// the number is composite.
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
        let chunk_size = bases.len().div_ceil(threads);

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
                        return true; // Another thread found a witness
                    }

                    let a_big = BigUint::from(bases_ref[j]);
                    if !miller_rabin_witness(&a_big, d_ref, s, n_ref, None) {
                        stop_ref.store(true, Ordering::Relaxed);
                        return false;
                    }
                }
                true
            }));
        }

        collect_results(handles)
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
        if !miller_rabin_witness(&a_big, d, s, n, None) {
            stop_flag.store(true, Ordering::Relaxed);
            return false;
        }
    }
    true
}

/// Parallel Miller-Rabin with progress reporting
///
/// Worker threads increment a shared atomic counter during modular
/// exponentiation. A polling thread reads the counter and forwards
/// updates to the progress callback.
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

    let completed_poll = Arc::clone(&completed_bits);
    let done_poll = Arc::clone(&done_flag);

    std::thread::scope(|scope| {
        // Progress polling thread
        let poll_handle = scope.spawn(move || {
            let poll_interval = std::time::Duration::from_millis(50);
            loop {
                let bits = completed_poll.load(Ordering::Relaxed);
                progress_callback(bits.min(total_bits_all_bases), total_bits_all_bases);

                if done_poll.load(Ordering::Relaxed) {
                    let bits = completed_poll.load(Ordering::Relaxed);
                    progress_callback(bits.min(total_bits_all_bases), total_bits_all_bases);
                    break;
                }
                std::thread::sleep(poll_interval);
            }
        });

        // Worker threads -- use the shared mod_pow + witness_check
        let mut handles = Vec::with_capacity(threads);
        let chunk_size = bases.len().div_ceil(threads);

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

                    // Use the single shared mod_pow with progress tracking
                    let x = mod_pow(a_big, d_ref, n_ref, Some(&completed));

                    // Use the shared witness_check
                    if !witness_check(x, s, n_ref) {
                        stop_ref.store(true, Ordering::Relaxed);
                        return false;
                    }
                }
                true
            }));
        }

        let result = collect_results(handles);

        done_flag.store(true, Ordering::Relaxed);
        let _ = poll_handle.join();

        result
    })
}

/// Sequential witness testing with direct progress callback
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
    let update_interval = (total_bits_all_bases / 100).max(10);

    for a in bases {
        if stop_flag.load(Ordering::Relaxed) {
            return true;
        }

        let a_big = BigUint::from(*a);

        // Use the shared mod_pow with progress tracking
        let x = mod_pow(a_big, d, n, Some(&completed_bits));

        // Report progress after each mod_pow completes
        let bits = completed_bits.load(Ordering::Relaxed);
        if bits % update_interval == 0 || bits >= total_bits_all_bases {
            progress_callback(bits.min(total_bits_all_bases), total_bits_all_bases);
        }

        // Use the shared witness_check
        if !witness_check(x, s, n) {
            stop_flag.store(true, Ordering::Relaxed);
            progress_callback(total_bits_all_bases, total_bits_all_bases);
            return false;
        }
    }

    progress_callback(total_bits_all_bases, total_bits_all_bases);
    true
}

/// Wait for all scoped threads and return true only if all passed
fn collect_results(handles: Vec<std::thread::ScopedJoinHandle<'_, bool>>) -> bool {
    let mut all_passed = true;
    for handle in handles {
        if let Ok(passed) = handle.join() {
            if !passed {
                all_passed = false;
            }
        }
    }
    all_passed
}

/// High-level interface for parallel primality testing
pub fn is_probable_prime_parallel(
    n: &BigUint,
    threads: usize,
    custom_bases: &[u64],
) -> Result<bool> {
    // Delegate to lib.rs check_small_primes via the public API
    if n < &BigUint::from(2u32) {
        return Ok(false);
    }

    if let Some(result) = crate::check_small_primes(n) {
        return Ok(result);
    }

    let (d, s) = decompose_into_d_and_s(n);
    let bases: Vec<u64> = if custom_bases.is_empty() {
        filter_bases_for_n(get_test_bases_for_size(n), n)
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
