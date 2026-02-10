use clap::Parser;
use miller_rabin_tester::{is_probable_prime, is_probable_prime_parallel};
use num_bigint::{BigUint, ToBigUint};
use std::io::Write;
use std::str::FromStr;
use std::thread;

#[derive(Parser, Debug)]
#[command(name = "miller-rabin")]
struct Args {
    #[arg(short, long)]
    number: Option<String>,

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
}

fn parse_big_uint(s: &str) -> Result<BigUint, String> {
    BigUint::from_str(s).map_err(|e| e.to_string())
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

    if let Some(n_str) = &args.number {
        match parse_big_uint(n_str) {
            Ok(n) => {
                println!("Testing: {}", n);
                if args.parallel && args.threads > 1 {
                    eprintln!("Using parallel mode with {} threads", args.threads);
                }
                let result = is_probable_prime(&n);

                if result {
                    println!("Result: PROBABLY PRIME");
                } else {
                    println!("Result: COMPOSITE");
                }
            }
            Err(e) => eprintln!("Error parsing number: {}", e),
        }
    } else if args.parallel && args.number.is_some() {
        match parse_big_uint(&args.number.unwrap()) {
            Ok(n) => {
                let threads = std::cmp::max(2, args.threads);
                println!("Testing in parallel mode with {} threads: {}", threads, n);
                let result = is_probable_prime_parallel(&n, threads);

                if result {
                    println!("Result: PROBABLY PRIME");
                } else {
                    println!("Result: COMPOSITE");
                }
            }
            Err(e) => eprintln!("Error parsing number: {}", e),
        }
    } else if args.batch_test {
        let start = args.start.unwrap_or(2usize);
        let end = args.end.unwrap_or(1000usize);

        print!(
            "Testing range [{}, {}) with {} threads... ",
            start, end, args.threads
        );
        let _ = std::io::stdout().flush();

        let chunk_size = (end - start + args.threads - 1) / args.threads;
        let mut handles = Vec::new();
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

        let mut primes: Vec<usize> = Vec::new();
        for handle in handles {
            primes.extend(handle.join().unwrap());
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
        eprintln!("Provide --number, or use --batch-test with --start/--end");
        std::process::exit(1);
    }
}
