//! Lightweight progress bar implementation
//! Uses ANSI escape codes, no external dependencies

use std::io::Write;
use std::sync::Mutex;
use std::time::{Duration, Instant};

struct ProgressState {
    completed: usize,
    last_update: Instant,
}

pub struct ProgressBar {
    total: usize,
    state: Mutex<ProgressState>,
    width: usize,
    description: String,
    start_time: Instant,
    update_interval: Duration,
}

impl ProgressBar {
    pub fn new(total: usize, description: &str) -> Self {
        Self {
            total,
            state: Mutex::new(ProgressState {
                completed: 0,
                last_update: Instant::now(),
            }),
            width: 40,
            description: description.to_string(),
            start_time: Instant::now(),
            update_interval: Duration::from_millis(50),
        }
    }

    pub fn update(&self, delta: usize) {
        let mut state = self.state.lock().unwrap_or_else(|poisoned| {
            // Recover from poisoned mutex - log warning and continue
            eprintln!("[WARN] Progress bar mutex poisoned, recovering state");
            poisoned.into_inner()
        });
        state.completed += delta;
        let completed = state.completed;

        let now = Instant::now();
        if now.duration_since(state.last_update) >= self.update_interval || completed >= self.total
        {
            state.last_update = now;
            drop(state);
            self.render(completed);
        }
    }

    pub fn finish(&self) {
        self.render(self.total);
        eprintln!();
    }

    fn render(&self, completed: usize) {
        if self.total == 0 {
            return;
        }

        let percent = (completed as f64 / self.total as f64).min(1.0);
        let filled = (percent * self.width as f64) as usize;

        let filled_str = "=".repeat(filled);
        let empty_str = " ".repeat(self.width.saturating_sub(filled));

        let elapsed = self.start_time.elapsed().as_secs_f64();
        let rate = if elapsed > 0.0 {
            completed as f64 / elapsed
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
            completed,
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
