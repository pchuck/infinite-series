use clap::Parser;
use std::io::{ErrorKind, Write};
use std::sync::Arc;
use std::time::Instant;

use primes::{
    format_number, generate_primes, ProgressBar, ProgressCallback, DEFAULT_SEGMENT_SIZE,
    PARALLEL_THRESHOLD,
};

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

    /// Use parallel processing (for n >= 5M)
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

    /// Print one prime per line instead of comma-separated
    #[arg(short = 'l', long)]
    lines: bool,
}

fn main() {
    let args = Args::parse();

    let segment = args.segment.unwrap_or(DEFAULT_SEGMENT_SIZE);

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
        std::thread::available_parallelism()
            .map(|p| p.get())
            .unwrap_or(4)
    });

    let progress_ticks = if args.progress {
        n.div_ceil(segment)
    } else {
        0
    };

    let compute_start = Instant::now();

    let progress_bar = args.progress.then(|| {
        Arc::new(ProgressBar::new(
            progress_ticks.max(1),
            "Generating primes",
            segment,
        ))
    });

    let progress_callback = progress_bar.as_ref().map(|bar| {
        let bar = Arc::clone(bar);
        Arc::new(move |delta: usize| bar.update(delta)) as ProgressCallback
    });

    let result = generate_primes(
        n,
        args.parallel,
        Some(workers),
        Some(segment),
        progress_callback,
    );

    let primes = match result {
        Ok(primes) => primes,
        Err(e) => {
            if let Some(bar) = &progress_bar {
                bar.finish();
            }
            eprintln!("Error: Prime generation failed: {}", e);
            std::process::exit(1);
        }
    };

    if let Some(bar) = &progress_bar {
        bar.finish();
    }

    let compute_time = compute_start.elapsed();

    if !primes.is_empty() {
        if !args.quiet {
            let stdout = std::io::stdout();
            let mut writer = std::io::BufWriter::new(stdout.lock());
            write_or_exit(&mut writer, format_args!("Primes less than {}:\n", n));
            if args.lines {
                for &p in primes.iter() {
                    write_or_exit(&mut writer, format_args!("{}\n", p));
                }
            } else {
                for (i, &p) in primes.iter().enumerate() {
                    if i > 0 {
                        write_or_exit(&mut writer, format_args!(", "));
                    }
                    write_or_exit(&mut writer, format_args!("{}", p));
                }
            }
            write_or_exit(
                &mut writer,
                format_args!("\nTotal primes: {}\n", primes.len()),
            );
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
        let last_prime = *primes.last().unwrap();
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

fn write_or_exit<W: Write>(writer: &mut W, args: std::fmt::Arguments) {
    if let Err(e) = writer.write_fmt(args) {
        if e.kind() != ErrorKind::BrokenPipe {
            eprintln!("Write error: {}", e);
            std::process::exit(1);
        }
    }
}
