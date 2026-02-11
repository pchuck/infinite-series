use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::cmp::Ordering;
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};
use std::sync::Arc;

const M_R_TEST_BASES_64: &[u64] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

const M_R_TEST_BASES_128: &[u64] = &[
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67,
];

pub fn get_test_bases_for_size(n: &BigUint) -> Vec<u64> {
    let n_128_max = BigUint::from(3_474_749_660_399u64);

    if n < &n_128_max {
        M_R_TEST_BASES_64
            .iter()
            .filter(|a| BigUint::from(**a).cmp(n) == Ordering::Less)
            .map(|x| *x)
            .collect()
    } else {
        M_R_TEST_BASES_128
            .iter()
            .filter(|a| BigUint::from(**a).cmp(n) == Ordering::Less)
            .map(|x| *x)
            .collect()
    }
}

pub fn filter_bases_for_n(bases: &[u64], n: &BigUint) -> Vec<u64> {
    bases
        .iter()
        .filter(|a| BigUint::from(**a).cmp(n) == Ordering::Less)
        .map(|x| *x)
        .collect()
}

fn check_small_primes(n: &BigUint) -> Option<bool> {
    if n < &BigUint::from(2usize) {
        return Some(false);
    }

    for p in [2u64, 3u64, 5u64] {
        let small_p = BigUint::from(p);
        if n == &small_p {
            return Some(true);
        }
        if n % small_p == Zero::zero() {
            return Some(false);
        }
    }

    None
}

pub fn is_probable_prime(n: &BigUint) -> bool {
    if let Some(result) = check_small_primes(n) {
        return result;
    }

    let (d, s) = decompose_into_d_and_s(n);

    for a in get_test_bases_for_size(n) {
        let a_big = BigUint::from(a);
        if !miller_rabin_witness(&a_big, &d, s, n) {
            return false;
        }
    }

    true
}

fn test_bases_sequential(n: &BigUint, d: &BigUint, s: usize, bases: &[u64]) -> bool {
    for a in bases {
        let a_big = BigUint::from(*a);
        if !miller_rabin_witness(&a_big, d, s, n) {
            return false;
        }
    }
    true
}

fn test_bases_parallel(n: &BigUint, d: &BigUint, s: usize, bases: &[u64], threads: usize) -> bool {
    if threads <= 1 || bases.len() < 2 {
        return test_bases_sequential(n, d, s, bases);
    }

    std::thread::scope(|scope| -> bool {
        let mut handles: Vec<_> = Vec::with_capacity(threads);
        let chunk_size = (bases.len() + threads - 1) / threads;

        for i in 0..threads {
            let start_idx = i * chunk_size;
            if start_idx >= bases.len() {
                break;
            }
            let end_idx = std::cmp::min(start_idx + chunk_size, bases.len());
            let d_copy = d.clone();
            let n_copy = n.clone();
            let bases_ref = &bases;

            handles.push(scope.spawn(move || -> bool {
                for j in start_idx..end_idx {
                    if j >= bases_ref.len() {
                        break;
                    }
                    let a_big = BigUint::from(bases_ref[j]);
                    if !miller_rabin_witness(&a_big, &d_copy, s, &n_copy) {
                        return false;
                    }
                }
                true
            }));
        }

        let mut all_passed = true;
        while let Some(handle) = handles.pop() {
            if !handle.join().unwrap() {
                all_passed = false;
            }
        }
        all_passed
    })
}

pub fn is_probable_prime_parallel(n: &BigUint, threads: usize) -> bool {
    if let Some(result) = check_small_primes(n) {
        return result;
    }

    let (d, s) = decompose_into_d_and_s(n);
    let bases = get_test_bases_for_size(n);

    test_bases_parallel(n, &d, s, &bases, threads)
}

fn decompose_into_d_and_s(n: &BigUint) -> (BigUint, usize) {
    let one = BigUint::from(1usize);
    let mut d = n.clone() - one;
    let mut s: usize = 0usize;
    while (d.clone() % BigUint::from(2usize)) == Zero::zero() {
        d >>= 1usize;
        s += 1usize;
    }
    (d, s)
}

fn miller_rabin_witness(a: &BigUint, d: &BigUint, s: usize, n: &BigUint) -> bool {
    let x = mod_pow(a.clone(), d.clone(), n);
    let one = BigUint::from(1usize);
    let n_minus_1 = n.clone() - one.clone();

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

pub fn miller_rabin_test(a: &BigUint, n: &BigUint) -> bool {
    if n <= a || *a == Zero::zero() {
        return *n == BigUint::from(2usize);
    }
    let (d, s) = decompose_into_d_and_s(n);
    miller_rabin_witness(a, &d, s, n)
}

pub fn mod_pow(mut base: BigUint, mut exp: BigUint, modulus: &BigUint) -> BigUint {
    if *modulus == One::one() {
        return Zero::zero();
    }

    let mut result = One::one();
    base = base % modulus;

    let two = BigUint::from(2usize);
    while !exp.is_zero() {
        if (exp.clone() % two.clone()) == One::one() {
            result = (&result * &base) % modulus;
        }
        base = (&base * &base) % modulus;
        exp >>= 1usize;
    }

    result
}

pub fn is_probable_prime_with_bases(n: &BigUint, custom_bases: &[u64]) -> bool {
    if let Some(result) = check_small_primes(n) {
        return result;
    }

    let (d, s) = decompose_into_d_and_s(n);

    let bases: Vec<u64> = if custom_bases.is_empty() {
        get_test_bases_for_size(n)
    } else {
        filter_bases_for_n(custom_bases, n)
    };

    test_bases_sequential(n, &d, s, &bases)
}

pub fn is_probable_prime_parallel_with_bases(
    n: &BigUint,
    threads: usize,
    custom_bases: &[u64],
) -> bool {
    if let Some(result) = check_small_primes(n) {
        return result;
    }

    let (d, s) = decompose_into_d_and_s(n);

    let bases: Vec<u64> = if custom_bases.is_empty() {
        get_test_bases_for_size(n)
    } else {
        filter_bases_for_n(custom_bases, n)
    };

    test_bases_parallel(n, &d, s, &bases, threads)
}

pub type ProgressCallback = dyn Fn(usize, usize) + Send + Sync;

fn mod_pow_with_progress(
    mut base: BigUint,
    mut exp: BigUint,
    modulus: &BigUint,
    progress_callback: Option<&ProgressCallback>,
    total_steps: usize,
    current_step: &mut usize,
) -> BigUint {
    if *modulus == One::one() {
        return Zero::zero();
    }

    let mut result = One::one();
    base = base % modulus;

    let two = BigUint::from(2usize);
    let total_bits = exp.bits() as usize;
    let mut bits_processed = 0usize;

    while !exp.is_zero() {
        if (exp.clone() % two.clone()) == One::one() {
            result = (&result * &base) % modulus;
        }
        base = (&base * &base) % modulus;
        exp >>= 1usize;

        bits_processed += 1;
        if let Some(cb) = progress_callback {
            if total_bits > 0 && bits_processed % 10 == 0 {
                let sub_progress = (bits_processed * 100) / total_bits;
                let overall_progress = *current_step * 100 + sub_progress.min(100);
                cb(overall_progress, total_steps * 100);
            }
        }
    }

    *current_step += 1;
    result
}

fn miller_rabin_witness_with_progress(
    a: &BigUint,
    d: &BigUint,
    s: usize,
    n: &BigUint,
    progress_callback: Option<&ProgressCallback>,
    total_bases: usize,
    current_base: &mut usize,
) -> bool {
    let x = mod_pow_with_progress(
        a.clone(),
        d.clone(),
        n,
        progress_callback,
        total_bases,
        current_base,
    );
    let one = BigUint::from(1usize);
    let n_minus_1 = n.clone() - one.clone();

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

pub fn is_probable_prime_with_progress(n: &BigUint, progress_callback: &ProgressCallback) -> bool {
    if let Some(result) = check_small_primes(n) {
        return result;
    }

    let (d, s) = decompose_into_d_and_s(n);
    let bases = get_test_bases_for_size(n);
    let total_bases = bases.len();
    let mut current_base = 0usize;

    for a in &bases {
        let a_big = BigUint::from(*a);
        if !miller_rabin_witness_with_progress(
            &a_big,
            &d,
            s,
            n,
            Some(progress_callback),
            total_bases,
            &mut current_base,
        ) {
            return false;
        }
    }
    progress_callback(total_bases * 100, total_bases * 100);

    true
}

fn mod_pow_parallel(
    mut base: BigUint,
    mut exp: BigUint,
    modulus: &BigUint,
    completed_bits: &Arc<AtomicUsize>,
    total_bits_all_bases: usize,
    progress_callback: &ProgressCallback,
) -> BigUint {
    if *modulus == One::one() {
        return Zero::zero();
    }

    let mut result = One::one();
    base = base % modulus;

    let two = BigUint::from(2usize);

    while !exp.is_zero() {
        if (exp.clone() % two.clone()) == One::one() {
            result = (&result * &base) % modulus;
        }
        base = (&base * &base) % modulus;
        exp >>= 1usize;

        if total_bits_all_bases > 0 {
            let bits = completed_bits.fetch_add(1, AtomicOrdering::Relaxed);
            // Update every 1% of total work or every 10 operations, whichever is larger
            let update_interval = (total_bits_all_bases / 100).max(10);
            if bits % update_interval == 0 || bits == total_bits_all_bases - 1 {
                let progress = (bits * 100) / total_bits_all_bases;
                progress_callback(progress.min(100), 100);
            }
        }
    }

    result
}

fn miller_rabin_witness_parallel(
    a: &BigUint,
    d: &BigUint,
    s: usize,
    n: &BigUint,
    completed_bits: &Arc<AtomicUsize>,
    total_bits_all_bases: usize,
    progress_callback: &ProgressCallback,
) -> bool {
    let x = mod_pow_parallel(
        a.clone(),
        d.clone(),
        n,
        completed_bits,
        total_bits_all_bases,
        progress_callback,
    );
    let one = BigUint::from(1usize);
    let n_minus_1 = n.clone() - one.clone();

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

pub fn is_probable_prime_parallel_with_progress(
    n: &BigUint,
    threads: usize,
    progress_callback: &ProgressCallback,
) -> bool {
    if let Some(result) = check_small_primes(n) {
        return result;
    }

    let (d, s) = decompose_into_d_and_s(n);
    let bases = get_test_bases_for_size(n);

    if threads <= 1 || bases.len() < 2 {
        return is_probable_prime_with_progress(n, progress_callback);
    }

    let total_bases = bases.len();
    let d_bits = d.bits() as usize;
    let total_bits_all_bases = total_bases * d_bits;
    let completed_bits = Arc::new(AtomicUsize::new(0));

    std::thread::scope(|scope| -> bool {
        let mut handles: Vec<_> = Vec::with_capacity(threads);
        let chunk_size = (bases.len() + threads - 1) / threads;

        for i in 0..threads {
            let start_idx = i * chunk_size;
            if start_idx >= bases.len() {
                break;
            }
            let end_idx = std::cmp::min(start_idx + chunk_size, bases.len());
            let d_copy = d.clone();
            let n_copy = n.clone();
            let bases_ref = &bases;
            let completed = Arc::clone(&completed_bits);

            handles.push(scope.spawn(move || -> bool {
                for j in start_idx..end_idx {
                    if j >= bases_ref.len() {
                        break;
                    }
                    let a_big = BigUint::from(bases_ref[j]);
                    if !miller_rabin_witness_parallel(
                        &a_big,
                        &d_copy,
                        s,
                        &n_copy,
                        &completed,
                        total_bits_all_bases,
                        progress_callback,
                    ) {
                        return false;
                    }
                }
                true
            }));
        }

        let mut all_passed = true;
        while let Some(handle) = handles.pop() {
            if !handle.join().unwrap() {
                all_passed = false;
            }
        }
        all_passed
    })
}
