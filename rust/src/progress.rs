//! Lightweight progress bar implementation
//! Uses ANSI escape codes, no external dependencies

use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

struct ProgressState {
    completed: usize,
    last_update: Instant,
}

#[allow(dead_code)]
pub struct ProgressBar {
    total: Arc<AtomicUsize>,
    state: Arc<Mutex<ProgressState>>,
    width: usize,
    description: String,
    start_time: Instant,
    update_interval: Duration,
}

impl ProgressBar {
    pub fn new(total: usize, description: &str) -> Self {
        Self {
            total: Arc::new(AtomicUsize::new(total)),
            state: Arc::new(Mutex::new(ProgressState {
                completed: 0,
                last_update: Instant::now(),
            })),
            width: 40,
            description: description.to_string(),
            start_time: Instant::now(),
            update_interval: Duration::from_millis(50),
        }
    }

    pub fn update(&self, delta: usize) {
        let mut state = self.state.lock().unwrap();
        state.completed += delta;
        let completed = state.completed;
        
        let now = Instant::now();
        if now.duration_since(state.last_update) >= self.update_interval 
            || completed >= self.total.load(Ordering::Relaxed) {
            state.last_update = now;
            drop(state);
            self.render(completed);
        }
    }

    #[allow(dead_code)]
    pub fn set_total(&self, total: usize) {
        self.total.store(total, Ordering::Relaxed);
    }

    #[allow(dead_code)]
    pub fn set_description(&self, desc: &str) {
        let _ = desc;
    }

    pub fn finish(&self) {
        let total = self.total.load(Ordering::Relaxed);
        self.render(total);
        eprintln!();
    }

    fn render(&self, completed: usize) {
        let total = self.total.load(Ordering::Relaxed);
        if total == 0 {
            return;
        }

        let percent = (completed as f64 / total as f64).min(1.0);
        let filled = (percent * self.width as f64) as usize;

        let filled_str = "=".repeat(filled);
        let empty_str = " ".repeat(self.width.saturating_sub(filled));
        
        eprint!(
            "\r{}: [{}{}] {:3.0}% | {}/{}    ",
            self.description,
            filled_str,
            empty_str,
            (percent * 100.0) as usize,
            completed,
            total
        );
        let _ = std::io::stderr().flush();
    }
}

#[allow(dead_code)]
fn format_rate(rate: f64) -> String {
    if rate >= 1_000_000.0 {
        format!("{:.1}M/s", rate / 1_000_000.0)
    } else if rate >= 1_000.0 {
        format!("{:.1}K/s", rate / 1_000.0)
    } else {
        format!("{:.0}/s", rate)
    }
}

#[allow(dead_code)]
fn format_duration(seconds: f64) -> String {
    if seconds >= 3600.0 {
        let hours = (seconds / 3600.0) as u64;
        let mins = ((seconds % 3600.0) / 60.0) as u64;
        format!("{}h{}m", hours, mins)
    } else if seconds >= 60.0 {
        let mins = (seconds / 60.0) as u64;
        let secs = (seconds % 60.0) as u64;
        format!("{}m{}s", mins, secs)
    } else {
        format!("{}s", seconds as u64)
    }
}

#[allow(dead_code)]
pub fn format_number(n: usize) -> String {
    let s = n.to_string();
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
