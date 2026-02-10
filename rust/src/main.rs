use clap::Parser;
use std::io::Write;
use std::sync::Arc;
use std::thread;
use std::time::Instant;
use primes::{generate_primes, DEFAULT_SEGMENT_SIZE, PARALLEL_THRESHOLD};
use progress::ProgressBar;

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
    #[arg(short='P', long)]
    progress: bool,
    
    /// Use parallel processing (for n >= 100M)
    #[arg(short, long)]
    parallel: bool,
    
    /// Number of worker threads (default: all available)
    #[arg(short, long)]
    workers: Option<usize>,
    
    /// Segment size for segmented sieve
    #[arg(long, default_value = "1000000")]
    segment: usize,
    
    /// Only print count (no prime list)
    #[arg(long)]
    quiet: bool,
}

fn main() {
    let args = Args::parse();

    let n = match args.n {
        Some(v) => v,
        None => {
            eprint!("Enter upper bound (n): ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            input.trim().parse().unwrap_or(0)
        }
    };

    if n <= 2 {
        println!("No primes less than {}", n);
        return;
    }

    if args.parallel && n < PARALLEL_THRESHOLD {
        eprintln!("[WARN] --parallel ignored: n={} is below threshold {}", n, PARALLEL_THRESHOLD);
    }

    let workers = args.workers.unwrap_or_else(|| {
        thread::available_parallelism().map(|p| p.get()).unwrap_or(4)
    });

    let segment_size_for_progress = if args.progress && args.segment == DEFAULT_SEGMENT_SIZE {
        100_000
    } else {
        args.segment
    };

    let compute_start = Instant::now();

    let primes: Vec<usize> = if args.progress {
        let progress_bar = Arc::new(ProgressBar::new(
            (n / segment_size_for_progress).max(1),
            "Generating primes"
        ));
        
        let progress_callback = Arc::clone(&progress_bar);
        let progress_clone = Arc::clone(&progress_bar);
        
        let handle = thread::spawn(move || {
            let primes = generate_primes(
                n,
                args.parallel && n >= PARALLEL_THRESHOLD,
                Some(workers),
                Some(segment_size_for_progress),
                Some(Arc::new(move |completed: usize| {
                    progress_callback.update(completed);
                })),
            );
            
            progress_clone.finish();
            primes
        });
        
        handle.join().unwrap()
    } else {
        generate_primes(
            n,
            args.parallel && n >= PARALLEL_THRESHOLD,
            Some(workers),
            Some(args.segment),
            None,
        )
    };

    let compute_time = compute_start.elapsed();

    if !primes.is_empty() {
        if !args.quiet {
            println!("Primes less than {}:", n);
            let mut buffer = String::new();
            for (i, &p) in primes.iter().enumerate() {
                if i > 0 {
                    buffer.push_str(", ");
                }
                buffer.push_str(&p.to_string());
            }
            println!("{}", buffer);
            println!("Total primes: {}", primes.len());
        } else {
            println!("{}", primes.len());
        }
    } else {
        println!("No primes less than {}", n);
    }

    let total_time = compute_time;
    let rate = primes.len() as f64 / total_time.as_secs_f64();
    
    if primes.is_empty() {
        eprintln!("Done! Generated 0 primes in {:.3}s (0 primes/s).", total_time.as_secs_f64());
    } else {
        let last_prime = primes[primes.len() - 1];
        let rate_str = format_rate(rate);
        eprintln!("Done! Largest prime < {} is {}. Generated {} primes in {:.3}s ({} primes/s).",
            n, last_prime, primes.len(), total_time.as_secs_f64(), rate_str);
    }
}

fn format_rate(rate: f64) -> String {
    let s = format!("{:.0}", rate);
    let mut result = String::new();
    let mut count = 0;
    for c in s.chars().rev() {
        result.push(c);
        count += 1;
        if count == 3 && result.len() < s.len() {
            result.push(',');
            count = 0;
        }
    }
    result.chars().rev().collect()
}
