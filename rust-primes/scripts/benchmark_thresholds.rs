//! Script to benchmark different threshold values and find optimal cutoff points.
//!
//! Usage: cargo run --release --bin bench_thresholds

use std::env;

/// Test range of n values and collect timing data for each algorithm
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        print_help();
        return;
    }

    println!("=== Threshold Benchmark Script ===\n");
    println!("Testing algorithm performance across different input sizes...\n");

    // Test range: from small (10K) to very large (500M)
    let test_values = [
        10_000,
        100_000,
        500_000,
        1_000_000,
        2_000_000,
        5_000_000,
        10_000_000,
        20_000_000,
        50_000_000,
        100_000_000,
        200_000_000,
        500_000_000,
    ];

    let segment_size = 1_000_000;
    let workers = 4;

    println!(
        "{:<15} | {:<12} | {:<12} | {:<12} | Best",
        "n", "Classic", "Segmented", "Parallel"
    );
    println!("{}", "-".repeat(60));

    for n in test_values.iter() {
        let classic_time = bench_classic(*n);
        let segmented_time = bench_segmented(*n, segment_size);
        let parallel_time = bench_parallel(*n, workers, segment_size);

        let best = if classic_time <= segmented_time && classic_time <= parallel_time {
            "Classic"
        } else if segmented_time <= parallel_time {
            "Segmented"
        } else {
            "Parallel"
        };

        println!(
            "{:>15} | {:>10.2?} | {:>10.2?} | {:>10.2?} | {}",
            n,
            format_duration(classic_time),
            format_duration(segmented_time),
            format_duration(parallel_time),
            best
        );
    }

    println!("\n=== Analysis ===\n");

    // Find crossover points
    let mut classic_to_segmented: Option<usize> = None;
    let mut segmented_to_parallel: Option<usize> = None;

    for n in test_values.iter() {
        let classic_time = bench_classic(*n);
        let segmented_time = bench_segmented(*n, segment_size);

        if classic_to_segmented.is_none() && segmented_time < classic_time {
            classic_to_segmented = Some(*n);
        }

        let parallel_time = bench_parallel(*n, workers, segment_size);

        if segmented_to_parallel.is_none() && parallel_time < segmented_time {
            segmented_to_parallel = Some(*n);
        }
    }

    println!("Optimal thresholds (approximate):");
    println!(
        "  Classic → Segmented: n ≈ {}",
        classic_to_segmented.unwrap_or(test_values[test_values.len() - 1])
    );
    println!(
        "  Segmented → Parallel: n ≈ {}",
        segmented_to_parallel.unwrap_or(test_values[test_values.len() - 1])
    );

    // Round thresholds to clean powers of 10
    fn round_threshold(n: usize) -> usize {
        if n <= 100_000 {
            100_000
        } else if n <= 1_000_000 {
            1_000_000
        } else if n <= 10_000_000 {
            10_000_000
        } else if n <= 100_000_000 {
            100_000_000
        } else {
            n
        }
    }

    let classic_threshold = classic_to_segmented
        .map(round_threshold)
        .unwrap_or(1_000_000);

    // Ensure parallel threshold is >= segmented threshold
    let mut parallel_threshold = segmented_to_parallel
        .map(round_threshold)
        .unwrap_or(100_000_000);

    if parallel_threshold < classic_threshold {
        parallel_threshold = classic_threshold;
    }

    println!("\nRecommendations (rounded to clean thresholds):");
    println!("  Classic sieve: n < {}", classic_threshold);
    println!(
        "  Segmented sieve: {} <= n < {}",
        classic_threshold, parallel_threshold
    );
    println!(
        "  Parallel segmented sieve: n >= {} (with --parallel flag)",
        parallel_threshold
    );

    if let Some(cross) = segmented_to_parallel {
        println!(
            "  - Use parallel sieve for n >= {} with --parallel flag",
            cross
        );
    }

    println!("\nNote: Thresholds depend on hardware. Re-run this benchmark");
    println!("      on your target machine to get optimal values.");
}

fn bench_classic(n: usize) -> f64 {
    let start = std::time::Instant::now();

    // Warm up
    let _ = primes::sieve_of_eratosthenes(1000, None);

    let result = primes::sieve_of_eratosthenes(n, None);
    let elapsed = start.elapsed().as_secs_f64();

    if result.is_ok() {
        elapsed
    } else {
        eprintln!("Warning: Classic sieve failed for n={}: {:?}", n, result);
        f64::MAX
    }
}

fn bench_segmented(n: usize, segment_size: usize) -> f64 {
    let start = std::time::Instant::now();

    // Warm up
    let _ = primes::segmented_sieve(1000, 100, None);

    let result = primes::segmented_sieve(n, segment_size, None);
    let elapsed = start.elapsed().as_secs_f64();

    if result.is_ok() {
        elapsed
    } else {
        eprintln!("Warning: Segmented sieve failed for n={}: {:?}", n, result);
        f64::MAX
    }
}

fn bench_parallel(n: usize, workers: usize, segment_size: usize) -> f64 {
    let start = std::time::Instant::now();

    // Warm up
    let _ = primes::parallel_segmented_sieve(1000, 2, 100, None);

    let result = primes::parallel_segmented_sieve(n, workers, segment_size, None);
    let elapsed = start.elapsed().as_secs_f64();

    if result.is_ok() {
        elapsed
    } else {
        eprintln!("Warning: Parallel sieve failed for n={}: {:?}", n, result);
        f64::MAX
    }
}

fn format_duration(secs: f64) -> String {
    if secs < 0.001 {
        format!("{:.2}μs", secs * 1_000_000.0)
    } else if secs < 1.0 {
        format!("{:.2}ms", secs * 1_000.0)
    } else {
        format!("{:.2}s", secs)
    }
}

fn print_help() {
    println!("Threshold Benchmark - Find optimal algorithm cutoffs");
    println!("\nUsage: cargo run --release --bin bench_thresholds [OPTIONS]");
    println!("\nOptions:");
    println!("  -h, --help     Show this help message");
    println!("  --custom=N     Test custom values (comma-separated)");
    println!("  --workers=N    Number of parallel workers (default: uses available CPU count)");
    println!("  --segment=S    Segment size in bytes (default: 1_000_000)");
    println!("\nDescription:");
    println!("  This script benchmarks the three prime generation algorithms");
    println!("  across a range of input sizes to find optimal threshold values.");
}
