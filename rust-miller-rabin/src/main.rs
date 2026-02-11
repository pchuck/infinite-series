use clap::Parser;
use miller_rabin_tester::{
    is_probable_prime, is_probable_prime_parallel, is_probable_prime_parallel_with_bases,
    is_probable_prime_with_bases,
};
use num_bigint::{BigUint, ToBigUint};
use std::fs;
use std::io::{self, Write};
use std::str::FromStr;
use std::thread;

#[derive(Parser, Debug)]
#[command(name = "miller-rabin")]
struct Args {
    #[arg(short, long)]
    number: Option<String>,

    #[arg(short = 'f', long)]
    file: Option<String>,

    #[arg(short = 'p', long)]
    parallel: bool,

    #[arg(short, long)]
    batch_test: bool,

    #[arg(long)]
    start: Option<usize>,

    #[arg(long)]
    end: Option<usize>,

    #[arg(short = 't', long, default_value = "4")]
    threads: usize,

    #[arg(short = 'b', long)]
    bases: Option<String>,
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

    if args.file.is_some() && (args.number.is_some() || args.batch_test) {
        eprintln!("Error: Cannot combine --file with --number or --batch-test");
        std::process::exit(1);
    }

    if let Some(file_path) = &args.file {
        match read_numbers_from_file(file_path) {
            Ok(numbers) => {
                println!("Testing {} numbers from file: {}", numbers.len(), file_path);
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
                        let is_prime = is_probable_prime(&n);
                        println!("{}", if is_prime { "PRIME" } else { "COMPOSITE" });
                        if is_prime {
                            primes_found += 1;
                        } else {
                            composites_found += 1;
                        }
                    }
                }

                println!(
                    "\nSummary: {} primes, {} composites out of {} numbers",
                    primes_found,
                    composites_found,
                    numbers.len()
                );
            }
            Err(e) => eprintln!("Error reading file '{}': {}", file_path, e),
        }
    } else if let Some(n_str) = &args.number {
        match parse_big_uint(n_str) {
            Ok(n) => {
                println!("Testing: {}", n);
                let custom_bases: Vec<u64> =
                    args.bases.as_ref().map_or(Vec::new(), |s| parse_bases(s));
                let result = if !custom_bases.is_empty() && args.parallel && args.threads > 1 {
                    is_probable_prime_parallel_with_bases(&n, args.threads, &custom_bases)
                } else if !custom_bases.is_empty() {
                    is_probable_prime_with_bases(&n, &custom_bases)
                } else if args.parallel && args.threads > 1 {
                    is_probable_prime_parallel(&n, args.threads)
                } else {
                    is_probable_prime(&n)
                };

                if result {
                    println!("Result: PROBABLY PRIME");
                } else {
                    println!("Result: COMPOSITE");
                }
            }
            Err(e) => eprintln!("Error parsing number '{}': {}", n_str, e),
        }
    } else if args.batch_test {
        let start = args.start.unwrap_or(2usize);
        let end = args.end.unwrap_or(1000usize);

        print!(
            "Testing range [{}, {}) with {} threads... ",
            start, end, args.threads
        );
        let _ = io::stdout().flush();

        let total_numbers = end - start;
        let chunk_size = (total_numbers + args.threads - 1) / args.threads;
        let mut handles = Vec::with_capacity(args.threads);
        let start_clone = start.clone();
        let end_clone = end.clone();

        for t in 0..args.threads {
            let t_start = start_clone + t * chunk_size;
            if t_start >= end_clone {
                break;
            }
            let t_end = std::cmp::min(t_start + chunk_size, end_clone);

            handles.push(thread::spawn(move || {
                (t_start..t_end)
                    .into_iter()
                    .filter(|n| n.to_biguint().map_or(false, |b| is_probable_prime(&b)))
                    .collect::<Vec<usize>>()
            }));
        }

        let mut primes: Vec<usize> = Vec::with_capacity(1024);
        for handle in handles {
            match handle.join() {
                Ok(mutex_primes) => primes.extend(mutex_primes),
                Err(_) => eprintln!("Warning: A worker thread panicked"),
            };
        }
        primes.sort();

        let composite_count = (end - start) - primes.len();
        println!("done");
        println!(
            "Found {} primes and {} composites",
            primes.len(),
            composite_count
        );

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
    } else {
        eprintln!("Provide --number <N>, --file <path>, or use --batch-test with --start/--end");
        std::process::exit(1);
    }
}
