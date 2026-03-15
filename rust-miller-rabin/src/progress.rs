//! Lightweight progress bar implementation for Miller-Rabin testing
//!
//! Uses ANSI escape codes with no external dependencies beyond the standard library.

use std::io::Write;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Internal state for the progress bar
struct ProgressState {
    completed: usize,
    last_update: Instant,
}

/// A thread-safe progress bar for tracking Miller-Rabin test progress
///
/// This implementation uses a Mutex for thread safety and supports
/// updating from multiple threads during parallel execution.
///
/// # Examples
/// ```
/// use miller_rabin_tester::progress::ProgressBar;
/// use std::sync::Arc;
///
/// let progress = Arc::new(ProgressBar::new(100, "Testing primality"));
/// progress.update(10);
/// progress.update(20);
/// progress.finish();
/// ```
pub struct ProgressBar {
    total: usize,
    state: Mutex<ProgressState>,
    width: usize,
    description: String,
    start_time: Instant,
    update_interval: Duration,
}

impl ProgressBar {
    /// Creates a new progress bar with the given total steps and description
    ///
    /// # Arguments
    /// * `total` - The total number of steps to completion
    /// * `description` - A description shown before the progress bar
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

    /// Updates the progress bar by adding 'delta' completed steps
    ///
    /// This method is thread-safe and can be called concurrently from multiple threads.
    pub fn update(&self, delta: usize) {
        let mut state = self.state.lock().unwrap();
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

    /// Marks the progress bar as complete
    ///
    /// This renders the bar at 100% and prints a newline.
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

/// Callback type for progress reporting during Miller-Rabin testing
///
/// The callback receives (current_progress, total_work) where progress
/// is typically measured in bit operations or bases tested.
pub type ProgressCallback = dyn Fn(usize, usize) + Send + Sync;

/// Creates a callback that drives a ProgressBar from (current, total) values.
///
/// The callback computes the delta from the last reported position and
/// forwards it to `ProgressBar::update`.
pub fn create_progress_callback(
    progress: std::sync::Arc<ProgressBar>,
    total: usize,
) -> impl Fn(usize, usize) + Send + Sync {
    let last = std::sync::atomic::AtomicUsize::new(0);
    move |current, _total| {
        // Scale current into the ProgressBar's total range
        if total == 0 {
            return;
        }
        let scaled = (current as u128 * progress.total as u128 / total as u128) as usize;
        let prev = last.swap(scaled, std::sync::atomic::Ordering::Relaxed);
        if scaled > prev {
            progress.update(scaled - prev);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_bar_creation() {
        let pb = ProgressBar::new(100, "Test");
        // Just verify it doesn't panic
        pb.update(10);
        pb.finish();
    }

    #[test]
    fn test_format_rate() {
        assert_eq!(format_rate(0.5), "0/s");
        assert_eq!(format_rate(1000.0), "1.0K/s");
        assert_eq!(format_rate(1500000.0), "1.5M/s");
    }
}
