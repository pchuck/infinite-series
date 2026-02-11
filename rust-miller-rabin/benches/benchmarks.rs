use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use miller_rabin_tester::{get_test_bases_for_size, is_probable_prime, is_probable_prime_parallel};
use num_bigint::BigUint;
use std::str::FromStr;

/// Setup a medium-sized composite number (~30 digits)
fn setup_medium_number() -> BigUint {
    BigUint::from_str("123456789012345678901234567890").unwrap()
}

/// Setup a 1000-digit prime-like number
fn setup_large_number() -> BigUint {
    BigUint::from_str(
        "9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
         9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
         9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
         9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
         9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
         9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
         9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
         9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
         9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
         9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999"
    ).unwrap()
}

/// Setup a known prime (10000th prime)
fn setup_known_prime() -> BigUint {
    BigUint::from_str("104729").unwrap()
}

/// Setup a Carmichael number (composite that passes Fermat test)
fn setup_carmichael() -> BigUint {
    BigUint::from_str("561").unwrap()
}

/// Setup a 2048-bit RSA-like modulus (product of two large primes)
fn setup_rsa_like() -> BigUint {
    // This is a 100-digit composite (similar size to RSA-332)
    BigUint::from_str(
        "13246262207629420740603607822142663672706853207725274630351642067930095383327073407130580626380015663351\
         63836019235347919642964852530383220502248528621761927933777230241243"
    ).unwrap()
}

fn bench_is_probable_prime(c: &mut Criterion) {
    let mut group = c.benchmark_group("is_probable_prime");

    // Small prime
    group.bench_with_input(
        BenchmarkId::new("small_prime", "104729"),
        &setup_known_prime(),
        |b, n| {
            b.iter(|| is_probable_prime(black_box(n)));
        },
    );

    // Medium composite
    group.bench_with_input(
        BenchmarkId::new("medium_composite", "30_digits"),
        &setup_medium_number(),
        |b, n| {
            b.iter(|| is_probable_prime(black_box(n)));
        },
    );

    // Carmichael number (requires all bases)
    group.bench_with_input(
        BenchmarkId::new("carmichael", "561"),
        &setup_carmichael(),
        |b, n| {
            b.iter(|| is_probable_prime(black_box(n)));
        },
    );

    // RSA-like large composite
    group.bench_with_input(
        BenchmarkId::new("rsa_like", "100_digits"),
        &setup_rsa_like(),
        |b, n| {
            b.iter(|| is_probable_prime(black_box(n)));
        },
    );

    // Very large number (1000 digits)
    group.bench_with_input(
        BenchmarkId::new("very_large", "1000_digits"),
        &setup_large_number(),
        |b, n| {
            b.iter(|| is_probable_prime(black_box(n)));
        },
    );

    group.finish();
}

fn bench_parallel(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel");

    // Compare sequential vs parallel for large numbers
    group.bench_with_input(
        BenchmarkId::new("sequential", "100_digits"),
        &setup_rsa_like(),
        |b, n| {
            b.iter(|| is_probable_prime(black_box(n)));
        },
    );

    group.bench_with_input(
        BenchmarkId::new("parallel_2_threads", "100_digits"),
        &setup_rsa_like(),
        |b, n| {
            b.iter(|| is_probable_prime_parallel(black_box(n), 2, &[]));
        },
    );

    group.bench_with_input(
        BenchmarkId::new("parallel_4_threads", "100_digits"),
        &setup_rsa_like(),
        |b, n| {
            b.iter(|| is_probable_prime_parallel(black_box(n), 4, &[]));
        },
    );

    // Large number parallel scaling
    group.bench_with_input(
        BenchmarkId::new("parallel_8_threads", "1000_digits"),
        &setup_large_number(),
        |b, n| {
            b.iter(|| is_probable_prime_parallel(black_box(n), 8, &[]));
        },
    );

    group.finish();
}

fn bench_get_bases(c: &mut Criterion) {
    let mut group = c.benchmark_group("get_test_bases");

    group.bench_with_input(
        BenchmarkId::new("bases_for_size", "medium"),
        &setup_medium_number(),
        |b, n| {
            b.iter(|| get_test_bases_for_size(black_box(n)));
        },
    );

    group.bench_with_input(
        BenchmarkId::new("bases_for_size", "large"),
        &setup_large_number(),
        |b, n| {
            b.iter(|| get_test_bases_for_size(black_box(n)));
        },
    );

    group.finish();
}

criterion_group!(
    benches,
    bench_is_probable_prime,
    bench_parallel,
    bench_get_bases
);
criterion_main!(benches);
