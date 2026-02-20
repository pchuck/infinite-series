use clap::Parser;
use fibonacci::generate_fibonacci;
use std::io::{self, BufWriter, Write};

mod progress;

#[derive(Parser)]
#[command(name = "fibonacci")]
#[command(about = "High-performance Fibonacci number generator", long_about = None)]
struct Args {
    /// Number of Fibonacci numbers to generate
    #[arg(short, long, default_value = "10")]
    count: usize,

    /// Quiet mode - only print count
    #[arg(short, long)]
    quiet: bool,

    /// Show progress bar
    #[arg(short = 'P', long)]
    progress: bool,
}

fn main() {
    let args = Args::parse();

    let fibs = if args.progress {
        let mut current: usize = 0;
        let mut next: usize = 1;
        let mut result = Vec::with_capacity(args.count);

        let mut bar = progress::ProgressBar::new(args.count);
        for i in 0..args.count {
            if i == 0 {
                result.push(0);
            } else if i == 1 {
                result.push(1);
            } else {
                let new_next = current.saturating_add(next);
                current = next;
                next = new_next;
                result.push(new_next);
            }
            bar.inc(1);
        }
        bar.finish();
        result
    } else {
        generate_fibonacci(args.count)
    };

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    if args.quiet {
        writeln!(writer, "{}", fibs.len()).unwrap();
    } else {
        for fib in &fibs {
            writeln!(writer, "{}", fib).unwrap();
        }
    }

    writer.flush().unwrap();
}
