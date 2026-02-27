//! Benchmarks for prime number generators
//!
//! Run with: cargo bench

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use primes::{generate_primes, segmented_sieve, sieve_of_eratosthenes, DEFAULT_SEGMENT_SIZE};

fn bench_classic_sieve(c: &mut Criterion) {
    let mut group = c.benchmark_group("classic_sieve");

    for &n in &[1_000, 10_000, 100_000, 1_000_000] {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_function(format!("n_{}", n), |b| {
            b.iter(|| sieve_of_eratosthenes(black_box(n)))
        });
    }

    group.finish();
}

fn bench_segmented_sieve(c: &mut Criterion) {
    let mut group = c.benchmark_group("segmented_sieve");

    for &n in &[100_000, 1_000_000, 10_000_000] {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_function(format!("n_{}", n), |b| {
            b.iter(|| segmented_sieve(black_box(n), DEFAULT_SEGMENT_SIZE, None))
        });
    }

    group.finish();
}

fn bench_auto_selection(c: &mut Criterion) {
    let mut group = c.benchmark_group("auto_selection");

    // Tests auto-selection logic:
    // n < 1M: classic sieve
    // n >= 1M: segmented sieve
    // n >= 100M: parallel segmented sieve (if parallel=true)
    for &n in &[100_000, 1_000_000, 10_000_000] {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_function(format!("n_{}_sequential", n), |b| {
            b.iter(|| generate_primes(black_box(n), false, None, None, None))
        });
    }

    // Parallel version for large inputs
    for &n in &[10_000_000, 50_000_000] {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_function(format!("n_{}_parallel", n), |b| {
            b.iter(|| generate_primes(black_box(n), true, None, None, None))
        });
    }

    group.finish();
}

fn bench_segment_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("segment_sizes");
    let n = 10_000_000;

    for &seg_size in &[100_000, 500_000, 1_000_000, 5_000_000] {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_function(format!("seg_{}", seg_size), |b| {
            b.iter(|| segmented_sieve(black_box(n), seg_size, None))
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_classic_sieve,
    bench_segmented_sieve,
    bench_auto_selection,
    bench_segment_sizes,
    bench_parallel_large,
    bench_segmented_large
);
criterion_main!(benches);
