//! Lightweight progress bar implementation
//! Uses ANSI escape codes, no external dependencies

use std::io::Write;
use std::time::{Duration, Instant};

pub struct ProgressBar {
    total: usize,
    completed: usize,
    width: usize,
    description: String,
    start_time: Instant,
    last_update: Instant,
    update_interval: Duration,
}

impl ProgressBar {
    pub fn new(total: usize) -> Self {
        Self {
            total,
            completed: 0,
            width: 40,
            description: "Progress".to_string(),
            start_time: Instant::now(),
            last_update: Instant::now(),
            update_interval: Duration::from_millis(50),
        }
    }

    pub fn inc(&mut self, delta: usize) {
        self.completed += delta;

        let now = Instant::now();
        if now.duration_since(self.last_update) >= self.update_interval
            || self.completed >= self.total
        {
            self.last_update = now;
            self.render();
        }
    }

    pub fn finish(&self) {
        self.render();
        eprintln!();
    }

    fn render(&self) {
        if self.total == 0 {
            return;
        }

        let percent = (self.completed as f64 / self.total as f64).min(1.0);
        let filled = (percent * self.width as f64) as usize;

        let filled_str = "=".repeat(filled);
        let empty_str = " ".repeat(self.width.saturating_sub(filled));

        let elapsed = self.start_time.elapsed().as_secs_f64();
        let rate = if elapsed > 0.0 {
            self.completed as f64 / elapsed
        } else {
            0.0
        };
        let rate_str = format_rate(rate);

        eprint!(
            "\r{}: [{}{}] {:3.0}% | {}/{} | {}    ",
            self.description,
            filled_str,
            empty_str,
            percent * 100.0,
            self.completed,
            self.total,
            rate_str,
        );
        let _ = std::io::stderr().flush();
    }
}

fn format_rate(rate: f64) -> String {
    if rate >= 1_000_000.0 {
        format!("{:.1}M/s", rate / 1_000_000.0)
    } else if rate >= 1_000.0 {
        format!("{:.1}K/s", rate / 1_000.0)
    } else {
        format!("{:.0}/s", rate)
    }
}
