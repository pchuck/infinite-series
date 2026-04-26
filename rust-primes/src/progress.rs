//! Lightweight progress bar implementation
//!
//! Uses ANSI escape codes for in-place terminal updates, no external dependencies.
//! Thread-safe: designed to be shared across threads via `Arc`.
//!
//! # Example
//!
//! ```
//! use std::sync::Arc;
//! use primes::{ProgressBar, generate_primes};
//!
//! let progress_bar = Arc::new(ProgressBar::new(100, "Processing", 10_000));
//! let callback = Arc::new({
//!     let pb = progress_bar.clone();
//!     move |delta: usize| {
//!         pb.update(delta);
//!     }
//! });
//! let _ = generate_primes(1_000_000, false, None, Some(10_000), Some(callback.clone()));
//! progress_bar.finish();
//! ```

use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Default progress bar width in characters
pub const PROGRESS_BAR_WIDTH: usize = 40;

/// Minimum time between progress updates in milliseconds
pub const PROGRESS_UPDATE_INTERVAL_MS: u64 = 50;

#[derive(Debug)]
struct ProgressState {
    completed: usize,
    last_update: Instant,
}

/// A thread-safe progress bar for long-running operations.
///
/// Renders an ANSI escape code-based progress indicator to stderr,
/// showing percentage, completed/total count, and items-per-second rate.
/// Designed to be shared across threads via `Arc<ProgressBar>`.
///
/// # Thread Safety
///
/// The internal state is protected by a `Mutex`, and the struct can be safely
/// shared between threads using `Arc`. If a mutex is poisoned (e.g., due to
/// a thread panic), the progress bar recovers gracefully with a one-time warning.
#[derive(Debug)]
pub struct ProgressBar {
    total: usize,
    state: Mutex<ProgressState>,
    width: usize,
    description: String,
    start_time: Instant,
    update_interval: Duration,
    warned: AtomicBool,
    segment_size: usize,
}

// SAFETY: All fields are Send + Sync:
// - `Mutex<ProgressState>` is Send + Sync
// - `AtomicBool` is Sync (and therefore Send when behind Arc)
// - `usize`, `String`, `Instant`, `Duration` are all Send + Sync
unsafe impl Send for ProgressBar {}
unsafe impl Sync for ProgressBar {}

impl ProgressBar {
    /// Creates a new progress bar.
    ///
    /// # Arguments
    /// * `total` - Total number of update events expected (e.g., total segments)
    /// * `description` - Label displayed before the progress bar (e.g., "Generating primes")
    /// * `segment_size` - Size of each unit for rate calculation (items per update)
    ///
    /// # Example
    ///
    /// ```
    /// use primes::ProgressBar;
    ///
    /// let bar = ProgressBar::new(100, "Processing", 10_000);
    /// ```
    pub fn new(total: usize, description: &str, segment_size: usize) -> Self {
        Self {
            total,
            state: Mutex::new(ProgressState {
                completed: 0,
                last_update: Instant::now(),
            }),
            width: PROGRESS_BAR_WIDTH,
            description: description.to_string(),
            start_time: Instant::now(),
            update_interval: Duration::from_millis(PROGRESS_UPDATE_INTERVAL_MS),
            warned: AtomicBool::new(false),
            segment_size,
        }
    }

    /// Updates the progress bar by the given delta.
    ///
    /// Only renders to stderr at most once per `PROGRESS_UPDATE_INTERVAL_MS` milliseconds,
    /// or immediately when the total is reached.
    ///
    /// # Thread Safety
    ///
    /// This method is safe to call from multiple threads concurrently.
    /// If the internal mutex is poisoned, recovery occurs with a one-time warning to stderr.
    pub fn update(&self, delta: usize) {
        let mut state = self.state.lock().unwrap_or_else(|poisoned| {
            if !self.warned.swap(true, Ordering::Relaxed) {
                eprintln!("[WARN] Progress state recovered from thread panic");
            }
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

    /// Completes the progress bar, rendering 100% and moving to a new line.
    ///
    /// Should be called after all updates are complete. Safe to call from any thread.
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
            (completed * self.segment_size) as f64 / elapsed
        } else {
            0.0
        };
        let rate_str = format_rate(rate);

        eprint!(
            "\r{}: [{}{}] {:3.0}% | {}/{} | {}",
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
