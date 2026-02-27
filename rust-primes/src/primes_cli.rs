use clap::Parser;
use std::io::Write;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

use primes::{generate_primes, PARALLEL_THRESHOLD};
use progress::ProgressBar;

pub const DEFAULT_PROGRESS_SEGMENT_SIZE: usize = 100_000;

mod progress;

/// Prime Number Generator - High-performance CLI
#[derive(Parser, Debug)]
#[command(name = "primes")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Upper bound (exclusive) for prime generation
    #[arg(short, long)]
    n: Option<usize>,

    /// Show progress bar
    #[arg(short = 'P', long)]
    progress: bool,

    /// Use parallel processing (for n >= 100M)
    #[arg(short, long)]
    parallel: bool,

    /// Number of worker threads (default: all available)
    #[arg(short, long)]
    workers: Option<usize>,

    /// Segment size for segmented sieve (default: 1M)
    #[arg(long)]
    segment: Option<usize>,

    /// Only print count (no prime list)
    #[arg(long)]
    quiet: bool,
}

const DEFAULT_SEGMENT_SIZE_CLI: usize = 1_000_000;

fn main() {
    let args = Args::parse();

    let segment = args.segment.unwrap_or(DEFAULT_SEGMENT_SIZE_CLI);

    if segment == 0 {
        eprintln!("Error: --segment must be greater than 0");
        std::process::exit(1);
    }

    let n = match args.n {
        Some(v) => v,
        None => {
            eprint!("Enter upper bound (n): ");
            let _ = std::io::stderr().flush();
            let mut input = String::new();
            if let Err(e) = std::io::stdin().read_line(&mut input) {
                eprintln!("Error: Failed to read input: {}", e);
                std::process::exit(1);
            }
            match input.trim().parse() {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Error: Invalid input '{}': {}", input.trim(), e);
                    std::process::exit(1);
                }
            }
        }
    };

    if n <= 2 {
        println!("No primes less than {}", n);
        return;
    }

    if args.parallel && n < PARALLEL_THRESHOLD {
        eprintln!(
            "[WARN] --parallel ignored: n={} is below threshold {}",
            n, PARALLEL_THRESHOLD
        );
    }

    let workers = args.workers.unwrap_or_else(|| {
        thread::available_parallelism()
            .map(|p| p.get())
            .unwrap_or(4)
    });

    let algorithm_segment = args.segment.unwrap_or(DEFAULT_SEGMENT_SIZE_CLI);

    let progress_ticks = if args.progress {
        n.div_ceil(DEFAULT_PROGRESS_SEGMENT_SIZE)
    } else {
        0
    };

    let compute_start = Instant::now();

    let primes: Vec<usize> = if args.progress {
        // note: progress bar completes immediately when classic sieve is used
        // since generate_primes doesn't invoke the callback in that code path.
        let progress_bar = Arc::new(ProgressBar::new(progress_ticks.max(1), "Generating primes"));

        let progress_callback = Arc::clone(&progress_bar);

        let handle = thread::spawn(move || {
            let progress_bar = Arc::clone(&progress_bar);
            let result = generate_primes(
                n,
                args.parallel && n >= PARALLEL_THRESHOLD,
                Some(workers),
                Some(algorithm_segment),
                Some(Arc::new(move |delta: usize| {
                    progress_callback.update(delta);
                })),
            );

            progress_bar.finish();
            result
        });

        match handle.join() {
            Ok(result) => match result {
                Ok(primes) => primes,
                Err(e) => {
                    eprintln!("Error: Prime generation failed: {:?}", e);
                    std::process::exit(1);
                }
            },
            Err(_) => {
                eprintln!("Error: Worker thread panicked during prime generation");
                std::process::exit(1);
            }
        }
    } else {
        match generate_primes(
            n,
            args.parallel && n >= PARALLEL_THRESHOLD,
            Some(workers),
            Some(segment),
            None,
        ) {
            Ok(primes) => primes,
            Err(e) => {
                eprintln!("Error: Prime generation failed: {:?}", e);
                std::process::exit(1);
            }
        }
    };

    let compute_time = compute_start.elapsed();

    if !primes.is_empty() {
        if !args.quiet {
            // Stream output with BufWriter to avoid building a huge String in memory
            let stdout = std::io::stdout();
            let mut writer = std::io::BufWriter::new(stdout.lock());
            writeln!(writer, "Primes less than {}:", n).unwrap();
            for (i, &p) in primes.iter().enumerate() {
                if i > 0 {
                    write!(writer, ", ").unwrap();
                }
                write!(writer, "{}", p).unwrap();
            }
            writeln!(writer).unwrap();
            writeln!(writer, "Total primes: {}", primes.len()).unwrap();
        } else {
            println!("{}", primes.len());
        }
    } else {
        println!("No primes less than {}", n);
    }

    let rate = primes.len() as f64 / compute_time.as_secs_f64();

    if primes.is_empty() {
        eprintln!(
            "Done! Generated 0 primes in {:.3}s (0 primes/s).",
            compute_time.as_secs_f64()
        );
    } else {
        let last_prime = primes[primes.len() - 1];
        let rate_str = format_number(rate as usize);
        eprintln!(
            "Done! Largest prime < {} is {}. Generated {} primes in {:.3}s ({} primes/s).",
            n,
            last_prime,
            primes.len(),
            compute_time.as_secs_f64(),
            rate_str
        );
    }
}

/// Format a number with comma separators (e.g., 1234567 -> "1,234,567")
fn format_number(n: usize) -> String {
    let s = n.to_string();
    let len = s.len();
    if len <= 3 {
        return s;
    }

    let mut result = String::with_capacity(len + len / 3);
    for (i, ch) in s.chars().enumerate() {
        if i > 0 && (len - i).is_multiple_of(3) {
            result.push(',');
        }
        result.push(ch);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(0), "0");
        assert_eq!(format_number(1), "1");
        assert_eq!(format_number(999), "999");
        assert_eq!(format_number(1000), "1,000");
        assert_eq!(format_number(12345), "12,345");
        assert_eq!(format_number(123456), "123,456");
        assert_eq!(format_number(1234567), "1,234,567");
        assert_eq!(format_number(1000000000), "1,000,000,000");
    }
}
