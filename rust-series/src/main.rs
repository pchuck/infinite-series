use clap::{Parser, ValueEnum};
use std::io::{self, BufWriter, Write};

mod progress;
use series::{
    collatz_stopping_time, generate_catalan, generate_collatz_times, generate_fibonacci,
    generate_happy, generate_hexagonal, generate_lucas, generate_powers_of_2, generate_triangular,
};

#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Series {
    Fib,
    Lucas,
    Tri,
    Collatz,
    Pow2,
    Catalan,
    Hex,
    Happy,
}

impl std::fmt::Display for Series {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Series::Fib => write!(f, "fibonacci"),
            Series::Lucas => write!(f, "lucas"),
            Series::Tri => write!(f, "triangular"),
            Series::Collatz => write!(f, "collatz"),
            Series::Pow2 => write!(f, "powers-of-2"),
            Series::Catalan => write!(f, "catalan"),
            Series::Hex => write!(f, "hexagonal"),
            Series::Happy => write!(f, "happy"),
        }
    }
}

#[derive(Parser)]
#[command(name = "series_cli")]
#[command(about = "Infinite series generators", long_about = None)]
struct Args {
    #[arg(short, long, default_value = "10")]
    count: usize,

    #[arg(short = 's', long, value_enum, default_value = "fib")]
    series: Series,

    #[arg(short, long)]
    quiet: bool,

    #[arg(short = 'P', long)]
    progress: bool,
}

fn main() {
    let args = Args::parse();

    let values: Vec<usize> = match args.series {
        Series::Fib => {
            if args.progress {
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
            }
        }
        Series::Lucas => {
            if args.progress {
                let mut prev: usize = 2;
                let mut curr: usize = 1;
                let mut result = Vec::with_capacity(args.count);

                let mut bar = progress::ProgressBar::new(args.count);
                for i in 0..args.count {
                    if i == 0 {
                        result.push(2);
                    } else if i == 1 {
                        result.push(1);
                    } else {
                        let next = prev.saturating_add(curr);
                        prev = curr;
                        curr = next;
                        result.push(next);
                    }
                    bar.inc(1);
                }
                bar.finish();
                result
            } else {
                generate_lucas(args.count)
            }
        }
        Series::Tri => {
            if args.progress {
                let mut bar = progress::ProgressBar::new(args.count);
                let result: Vec<usize> = (0..args.count)
                    .map(|n| {
                        bar.inc(1);
                        n * (n + 1) / 2
                    })
                    .collect();
                bar.finish();
                result
            } else {
                generate_triangular(args.count)
            }
        }
        Series::Collatz => {
            if args.progress {
                let mut bar = progress::ProgressBar::new(args.count);
                let result: Vec<usize> = (0..args.count)
                    .map(|n| {
                        bar.inc(1);
                        collatz_stopping_time(n)
                    })
                    .collect();
                bar.finish();
                result
            } else {
                generate_collatz_times(args.count)
            }
        }
        Series::Pow2 => {
            if args.progress {
                let mut current: usize = 1;
                let mut result = Vec::with_capacity(args.count);

                let mut bar = progress::ProgressBar::new(args.count);
                for _ in 0..args.count {
                    result.push(current);
                    current = current.saturating_mul(2);
                    bar.inc(1);
                }
                bar.finish();
                result
            } else {
                generate_powers_of_2(args.count)
            }
        }
        Series::Catalan => {
            if args.progress {
                let mut result = Vec::with_capacity(args.count);
                let mut c = 1usize;

                let mut bar = progress::ProgressBar::new(args.count);
                for i in 0..args.count {
                    result.push(c);
                    if i > 0 {
                        c = c.saturating_mul(2 * (2 * i - 1)) / (i + 1);
                    }
                    bar.inc(1);
                }
                bar.finish();
                result
            } else {
                generate_catalan(args.count)
            }
        }
        Series::Hex => {
            if args.progress {
                let mut bar = progress::ProgressBar::new(args.count);
                let result: Vec<usize> = (1..=args.count)
                    .map(|n| {
                        bar.inc(1);
                        n * (2 * n - 1)
                    })
                    .collect();
                bar.finish();
                result
            } else {
                generate_hexagonal(args.count)
            }
        }
        Series::Happy => {
            if args.progress {
                let mut result = Vec::with_capacity(args.count);
                let mut n = 1;

                let mut bar = progress::ProgressBar::new(args.count);
                while result.len() < args.count {
                    if series::is_happy(n) {
                        result.push(n);
                        bar.inc(1);
                    }
                    n += 1;
                }
                bar.finish();
                result
            } else {
                generate_happy(args.count)
            }
        }
    };

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    if args.quiet {
        writeln!(writer, "{}", values.len()).unwrap();
    } else {
        for val in &values {
            writeln!(writer, "{}", val).unwrap();
        }
    }

    writer.flush().unwrap();
}
