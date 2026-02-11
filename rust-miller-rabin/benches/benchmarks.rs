use criterion::{black_box, criterion_group, criterion_main, BenchmarkId};
use miller_rabin_tester::{get_test_bases_for_size, is_probable_prime, is_probable_prime_parallel};
use num_bigint::BigUint;
use std::str::FromStr;

fn setup_medium_number() -> BigUint {
    BigUint::from_str("123456789012345678901234567890").unwrap()
}

fn setup_large_prime() -> BigUint {
    BigUint::from_str("1299709").unwrap()
}

fn setup_composite() -> BigUint {
    BigUint::from_str("104730").unwrap()
}

fn bench_is_probable_prime(c: &mut criterion::Bencher) {
    let n = setup_large_prime();
    c.iter(|| is_probable_prime(black_box(&n)));
}

fn bench_parallel(c: &mut criterion::Bencher) {
    let n = setup_large_prime();
    c.iter(|| is_probable_prime_parallel(black_box(&n), 4));
}

fn bench_get_bases(c: &mut criterion::Bencher) {
    let n = setup_medium_number();
    c.iter(|| get_test_bases_for_size(black_box(&n)));
}

criterion_group!(
    benches,
    bench_is_probable_prime,
    bench_parallel,
    bench_get_bases
);
criterion_main!(benches);
