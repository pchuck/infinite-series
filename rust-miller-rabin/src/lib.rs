use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::cmp::Ordering;

const M_R_TEST_BASES_64: &[u64] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

pub fn is_probable_prime(n: &BigUint) -> bool {
    let two: BigUint = BigUint::from(2usize);
    if n < &two {
        return false;
    }

    for p in [2u64, 3u64, 5u64] {
        let small_p = BigUint::from(p);
        if n == &small_p {
            return true;
        }
        if n % small_p == Zero::zero() {
            return false;
        }
    }

    let (d, s) = decompose_into_d_and_s(n);

    for a in get_test_bases(n) {
        let a_big = BigUint::from(a);
        if !miller_rabin_witness(&a_big, &d, s, n) {
            return false;
        }
    }

    true
}

pub fn is_probable_prime_parallel(n: &BigUint, threads: usize) -> bool {
    let two: BigUint = BigUint::from(2usize);
    if n < &two {
        return false;
    }

    for p in [2u64, 3u64, 5u64] {
        let small_p = BigUint::from(p);
        if n == &small_p {
            return true;
        }
        if n % small_p == Zero::zero() {
            return false;
        }
    }

    let (d, s) = decompose_into_d_and_s(n);
    let bases = get_test_bases(n);

    if threads <= 1 || bases.len() < 2 {
        for a in &bases {
            let a_big = BigUint::from(*a);
            if !miller_rabin_witness(&a_big, &d, s, n) {
                return false;
            }
        }
        return true;
    }

    let chunk_size = (bases.len() + threads - 1) / threads;
    let mut all_passed = true;

    std::thread::scope(|scope| {
        let mut handles = Vec::new();
        for i in 0..threads {
            let start_idx = i * chunk_size;
            if start_idx >= bases.len() {
                break;
            }
            let end_idx = std::cmp::min(start_idx + chunk_size, bases.len());
            let d_copy = d.clone();
            let s_val = s;
            let n_copy = n.clone();
            let bases_copy = bases.clone();

            handles.push(scope.spawn(move || -> bool {
                for j in start_idx..end_idx {
                    if j >= bases_copy.len() {
                        break;
                    }
                    let a_big = BigUint::from(bases_copy[j]);
                    if !miller_rabin_witness(&a_big, &d_copy, s_val, &n_copy) {
                        return false;
                    }
                }
                true
            }));
        }

        for handle in handles {
            all_passed = all_passed && handle.join().unwrap();
        }
    });

    all_passed
}

fn decompose_into_d_and_s(n: &BigUint) -> (BigUint, usize) {
    let one: BigUint = One::one();
    let mut d: BigUint = n.clone() - one;
    let mut s: usize = 0usize;
    while (d.clone() % BigUint::from(2usize)) == Zero::zero() {
        d >>= 1usize;
        s += 1usize;
    }
    (d, s)
}

fn get_test_bases(n: &BigUint) -> Vec<u64> {
    M_R_TEST_BASES_64
        .iter()
        .filter(|a| BigUint::from(**a).cmp(n) == Ordering::Less)
        .map(|x| *x)
        .collect()
}

fn miller_rabin_witness(a: &BigUint, d: &BigUint, s: usize, n: &BigUint) -> bool {
    let x = mod_pow(a.clone(), d.clone(), n);
    let one: BigUint = One::one();
    let n_minus_1 = n - one;
    if x == One::one() || x == n_minus_1 {
        return true;
    }

    let mut current = x;
    for _ in 0..s {
        let y = (&current * &current) % n;
        if y == n_minus_1 {
            return true;
        }
        if y == One::one() {
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
