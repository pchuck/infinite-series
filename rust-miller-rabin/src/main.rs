use clap::Parser;
use miller_rabin_tester::{
    get_test_bases_for_size, is_probable_prime, is_probable_prime_parallel,
    is_probable_prime_parallel_with_bases, is_probable_prime_with_bases,
};
use num_bigint::{BigUint, ToBigUint};
use serde_json::json;
use std::fs;
use std::io::{self, Write};
use std::str::FromStr;
use std::thread;

fn get_available_threads() -> usize {
    let cores = num_cpus::get();
    if cores < 1 {
        return 4;
    }
    cores
}

#[derive(Parser, Debug)]
#[command(name = "miller-rabin")]
struct Args {
    #[arg(short, long)]
    number: Option<String>,

    #[arg(short = 'f', long)]
    file: Option<String>,

    #[arg(
        short = 'p',
        long,
        help = "Enable parallel processing (auto-detects CPU cores)"
    )]
    parallel: bool,

    #[arg(short, long)]
    batch_test: bool,

    #[arg(long)]
    start: Option<usize>,

    #[arg(long)]
    end: Option<usize>,

    #[arg(
        short = 't',
        long,
        default_value_t = 0,
        help = "Number of threads (0=auto-detect)"
    )]
    threads: usize,

    #[arg(short = 'b', long)]
    bases: Option<String>,

    #[arg(long, help = "Show detailed performance metrics")]
    verbose: bool,

    #[arg(long, default_value = "text", help = "Output format: text or json")]
    output_format: String,
}

fn parse_big_uint(s: &str) -> Result<BigUint, String> {
    BigUint::from_str(s).map_err(|e| e.to_string())
}

fn parse_bases(s: &str) -> Vec<u64> {
    s.split(',').filter_map(|x| x.trim().parse().ok()).collect()
}

fn read_numbers_from_file(path: &str) -> io::Result<Vec<String>> {
    let content = fs::read_to_string(path)?;
    Ok(content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().to_string())
        .collect())
}

struct PerfMetrics {
    start_time: std::time::Instant,
    bases_tested: usize,
    threads_used: usize,
}

impl PerfMetrics {
    fn new() -> Self {
        Self {
            start_time: std::time::Instant::now(),
            bases_tested: 0,
            threads_used: 1,
        }
    }

    fn elapsed_ms(&self) -> f64 {
        self.start_time.elapsed().as_millis() as f64
    }

    fn throughput(&self, count: usize) -> f64 {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        if elapsed > 0.0 {
            count as f64 / elapsed
        } else {
            0.0
        }
    }
}

fn format_duration(ms: f64) -> String {
    if ms < 1.0 {
        format!("{:.3} ms", ms)
    } else if ms < 1000.0 {
        format!("{:.2} ms", ms)
    } else {
        format!("{:.2} s", ms / 1000.0)
    }
}

fn output_json(metrics: &PerfMetrics, data: serde_json::Value) {
    let json_output = json!({
        "performance_ms": metrics.elapsed_ms(),
        "bases_tested": metrics.bases_tested,
        "threads_used": metrics.threads_used,
        "data": data
    });
    if let Ok(s) = serde_json::to_string_pretty(&json_output) {
        println!("{}", s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::ToBigUint;

    #[test]
    fn test_small_primes() {
        let small_primes = [2usize, 3, 5, 7, 11, 13, 17, 19];
        for p in small_primes {
            if let Some(n) = p.to_biguint() {
                assert!(is_probable_prime(&n), "{} should be prime", p);
            }
        }
    }

    #[test]
    fn test_small_composites() {
        let composites = [4usize, 6, 8, 9, 10, 12, 14, 15];
        for c in composites {
            if let Some(n) = c.to_biguint() {
                assert!(!is_probable_prime(&n), "{} should be composite", c);
            }
        }
    }

    #[test]
    fn test_known_primes() {
        let primes = ["1009", "104729"];
        for p in &primes {
            if let Ok(n) = parse_big_uint(p) {
                assert!(is_probable_prime(&n), "{} should be prime", p);
            }
        }
    }

    #[test]
    fn test_known_composites() {
        let composites = ["341", "561", "645"];
        for c in &composites {
            if let Ok(n) = parse_big_uint(c) {
                assert!(!is_probable_prime(&n), "{} should be composite", c);
            }
        }
    }

    #[test]
    fn test_large_primes() {
        let large_primes = ["1299709"];
        for p in &large_primes {
            if let Ok(n) = parse_big_uint(p) {
                assert!(is_probable_prime(&n), "{} should be prime", p);
            }
        }
    }

    #[test]
    fn test_large_composites() {
        let large_composites = ["104730"];
        for c in &large_composites {
            if let Ok(n) = parse_big_uint(c) {
                assert!(!is_probable_prime(&n), "{} should be composite", c);
            }
        }
    }

    #[test]
    fn test_carmichael_numbers() {
        let carmichaels = ["561", "1105", "1729", "2465"];
        for n in &carmichaels {
            if let Ok(num) = parse_big_uint(n) {
                assert!(!is_probable_prime(&num), "{} is Carmichael (composite)", n);
            }
        }
    }

    #[test]
    fn test_fermat_primes() {
        for p in [0usize, 1, 2] {
            let pow_2_p = BigUint::from(2usize).pow(p as u32);
            let n = pow_2_p.pow(2) + BigUint::from(1usize);
            assert!(is_probable_prime(&n), "{} should be prime", n);
        }
    }

    #[test]
    fn test_fermat_composites() {
        for p in [5usize, 6] {
            let pow_2_p = BigUint::from(2usize).pow(p as u32);
            let n = pow_2_p.pow(2) + BigUint::from(1usize);
            assert!(!is_probable_prime(&n), "{} should be composite", n);
        }
    }

    #[test]
    fn test_mr_testing() {}
}

fn main() {
    let args = Args::parse();
    let mut metrics = PerfMetrics::new();

    let threads: usize = if args.threads > 0 {
        args.threads
    } else {
        get_available_threads()
    };
    metrics.threads_used = if args.parallel { threads } else { 1 };

    if args.file.is_some() && (args.number.is_some() || args.batch_test) {
        eprintln!("Error: Cannot combine --file with --number or --batch-test");
        std::process::exit(1);
    }

    if let Some(file_path) = &args.file {
        match read_numbers_from_file(file_path) {
            Ok(numbers) => {
                let total = numbers.len();
                println!("Testing {} numbers from file: {}", total, file_path);
                if args.bases.is_some() {
                    eprintln!(
                        "Warning: Custom bases ignored in file mode (uses deterministic bases)"
                    );
                }
                let mut primes_found = 0;
                let mut composites_found = 0;

                for n_str in &numbers {
                    if let Ok(n) = parse_big_uint(n_str) {
                        print!("  {}: ", n_str);
                        let _ = io::stdout().flush();
                        metrics.bases_tested += get_test_bases_for_size(&n).len();
                        let is_prime = is_probable_prime(&n);

                        if args.verbose {
                            print!(" [bases tested]");
                        }
                        println!("{}", if is_prime { "PRIME" } else { "COMPOSITE" });
                        if is_prime {
                            primes_found += 1;
                        } else {
                            composites_found += 1;
                        }
                    }
                }

                let elapsed = metrics.elapsed_ms();
                eprintln!(
                    "Found {} primes and {} composites (out of {})",
                    primes_found, composites_found, total
                );

                if args.verbose {
                    eprintln!(
                        "Performance: {:.2} ms total, {:.3} ms/number, throughput: {:.0}/s",
                        elapsed,
                        elapsed / total as f64,
                        metrics.throughput(total)
                    );
                }

                if args.output_format == "json" {
                    output_json(
                        &metrics,
                        json!({
                            "primes_found": primes_found,
                            "composites_found": composites_found
                        }),
                    );
                }
            }
            Err(e) => eprintln!("Error reading file '{}': {}", file_path, e),
        }
    } else if let Some(n_str) = &args.number {
        match parse_big_uint(n_str) {
            Ok(n) => {
                println!("Testing: {}", n);
                let custom_bases: Vec<u64> =
                    args.bases.as_ref().map_or(Vec::new(), |s| parse_bases(s));
                let result = if !custom_bases.is_empty() && args.parallel {
                    is_probable_prime_parallel_with_bases(&n, threads, &custom_bases)
                } else if !custom_bases.is_empty() {
                    let bases = miller_rabin_tester::filter_bases_for_n(&custom_bases, &n);
                    metrics.bases_tested = bases.len();
                    is_probable_prime_with_bases(&n, &custom_bases)
                } else if args.parallel {
                    let bases = get_test_bases_for_size(&n);
                    metrics.bases_tested = bases.len();
                    is_probable_prime_parallel(&n, threads)
                } else {
                    let bases = get_test_bases_for_size(&n);
                    metrics.bases_tested = bases.len();
                    is_probable_prime(&n)
                };

                if result {
                    println!("Result: PROBABLY PRIME");
                } else {
                    println!("Result: COMPOSITE");
                }

                if args.verbose || args.output_format == "json" {
                    let elapsed = metrics.elapsed_ms();
                    eprintln!(
                        "Performance: {} total, bases tested: {}, threads used: {}",
                        format_duration(elapsed),
                        metrics.bases_tested,
                        metrics.threads_used
                    );
                }

                if args.output_format == "json" {
                    output_json(
                        &metrics,
                        json!({
                            "number": n_str.to_string(),
                            "is_prime": result,
                            "probabilistic_bases_used": metrics.bases_tested > 0
                        }),
                    );
                }
            }
            Err(e) => eprintln!("Error parsing number '{}': {}", n_str, e),
        }
    } else if args.batch_test {
        let start = args.start.unwrap_or(2usize);
        let end = args.end.unwrap_or(1000usize);

        print!(
            "Testing range [{}, {}) with {} threads... ",
            start,
            end,
            if args.parallel { threads } else { 1 }
        );
        let _ = io::stdout().flush();

        let total_numbers = end - start;
        let mut primes: Vec<usize> = Vec::with_capacity(1024);

        if !args.parallel {
            for n in start..end {
                if let Some(b) = n.to_biguint() {
                    metrics.bases_tested += get_test_bases_for_size(&b).len();
                    if is_probable_prime(&b) {
                        primes.push(n);
                    }
                }
            }
        } else {
            let chunk_size = (total_numbers + threads - 1) / threads;
            let mut handles: Vec<_> = Vec::with_capacity(threads);
            let start_clone = start.clone();
            let end_clone = end.clone();

            for t in 0..threads {
                let t_start = start_clone + t * chunk_size;
                if t_start >= end_clone {
                    break;
                }
                let t_end = std::cmp::min(t_start + chunk_size, end_clone);

                handles.push(thread::spawn(move || -> (Vec<usize>, usize) {
                    let mut primes_in_chunk: Vec<usize> = Vec::new();
                    let mut bases_total = 0usize;

                    for n in t_start..t_end {
                        if let Some(b) = n.to_biguint() {
                            let bases = get_test_bases_for_size(&b);
                            bases_total += bases.len();

                            if is_probable_prime(&b) {
                                primes_in_chunk.push(n);
                            }
                        }
                    }

                    (primes_in_chunk, bases_total)
                }));
            }

            for handle in handles {
                match handle.join() {
                    Ok((chunk_primes, chunk_bases)) => {
                        primes.extend(chunk_primes);
                        metrics.bases_tested += chunk_bases;
                    }
                    Err(_) => eprintln!("Warning: A worker thread panicked"),
                }
            }

            primes.sort();
        }

        let composite_count = (end - start) - primes.len();

        if !primes.is_empty() {
            print!("Primes: ");
            for (i, p) in primes.iter().enumerate() {
                if i > 0 {
                    print!(", ");
                }
                print!("{}", p);
            }
            println!();
        }

        let elapsed = metrics.elapsed_ms();
        let prime_density = if total_numbers > 0 {
            (primes.len() * 100) as f64 / total_numbers as f64
        } else {
            0.0
        };

        eprintln!(
            "\nPerformance Metrics:\n  Total time: {}\n  Bases tested: {}\n  Threads used: {}\n  Prime density: {:.2}%\n  Throughput: {:.1}/s\n  Avg ms/number: {:.4}",
            format_duration(elapsed),
            metrics.bases_tested,
            metrics.threads_used,
            prime_density,
            metrics.throughput(total_numbers),
            elapsed / total_numbers.max(1) as f64
        );

        if args.verbose && !primes.is_empty() {
            eprintln!("First 10 primes: {:?}", &primes[..primes.len().min(10)]);
        }

        if args.output_format == "json" {
            output_json(
                &metrics,
                json!({
                    "range_start": start,
                    "range_end": end,
                    "total_tested": total_numbers,
                    "primes_found": primes.len(),
                    "composites_found": composite_count,
                    "prime_density_percent": prime_density
                }),
            );
        }
    } else {
        eprintln!("Provide --number <N>, --file <path>, or use --batch-test with --start/--end");
        std::process::exit(1);
    }
}
