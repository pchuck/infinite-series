//! High-performance prime number generator
//!
//! Provides three algorithms:
//! - Classic Sieve of Eratosthenes (best for n < 1M)
//! - Segmented Sieve (best for n >= 1M)
//! - Parallel Segmented Sieve (best for n >= 5M, see scripts/README.md)
//!
//! All algorithms use odd-only sieves for 2x memory and work reduction.

use std::sync::Arc;

mod classic;
mod dispatcher;
pub mod progress;
mod error;
mod parallel;
mod segmented;
mod utils;

pub use classic::sieve_of_eratosthenes;
pub use dispatcher::generate_primes;
pub use error::PrimeGenError;
pub use parallel::parallel_segmented_sieve;
pub use progress::{ProgressBar, PROGRESS_BAR_WIDTH, PROGRESS_UPDATE_INTERVAL_MS};
pub use segmented::segmented_sieve;
pub use utils::estimate_prime_count;

/// Type alias for progress callbacks used throughout the library.
/// Receives the number of segments processed as a progress update.
pub type ProgressCallback = Arc<dyn Fn(usize) + Send + Sync>;

/// Default segment size for segmented sieve (1M elements)
pub const DEFAULT_SEGMENT_SIZE: usize = 1_000_000;

/// Maximum number of worker threads.
/// This is a safeguard against accidental extreme values, not a practical limit.
/// 1024 workers at the default 1M segment size uses ~512MB of memory.
pub const MAX_WORKERS: usize = 1024;

/// Minimum input size for parallel processing (5M)
/// See scripts/README.md for benchmark-based threshold analysis
pub const PARALLEL_THRESHOLD: usize = 5_000_000;

/// Maximum input size (1 quadrillion)
/// Beyond this, time required exceeds practical limits
#[cfg(target_arch = "wasm32")]
pub const MAX_N: usize = 4_294_967_295;

#[cfg(not(target_arch = "wasm32"))]
pub const MAX_N: usize = 1_000_000_000_000_000;
