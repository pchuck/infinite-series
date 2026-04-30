use crate::classic::sieve_of_eratosthenes;
use crate::error::PrimeGenError;
use crate::parallel::parallel_segmented_sieve;
use crate::segmented::segmented_sieve;
use crate::utils::{validate_n, validate_segment_size, validate_workers};
use crate::ProgressCallback;
use crate::{DEFAULT_SEGMENT_SIZE, PARALLEL_THRESHOLD};

/// Auto-select algorithm based on n
///
/// # Arguments
/// * `n` - Upper bound (exclusive) for prime generation
/// * `parallel` - Enable parallel processing for large inputs
/// * `workers` - Number of threads (default: all available)
/// * `segment_size` - Segment size in elements (default: DEFAULT_SEGMENT_SIZE)
/// * `progress` - Optional callback receiving segment count updates
///
/// # Examples
///
/// ```
/// use primes::{generate_primes, DEFAULT_SEGMENT_SIZE};
///
/// // Basic usage - classic sieve for small inputs
/// let primes = generate_primes(100, false, None, None, None)?;
/// assert_eq!(primes.len(), 25);
///
/// // Segmented sieve for larger inputs
/// let primes = generate_primes(1_000_000, false, None, None, None)?;
/// assert_eq!(primes.len(), 78498);
///
/// // Parallel processing for large inputs (n >= 5M)
/// let result = generate_primes(100_000_000, true, None, None, None);
/// assert!(result.is_ok());
///
/// // Custom segment size and progress tracking
/// use std::sync::Arc;
/// let progress = Some(Arc::new(|delta: usize| {
///     eprintln!("Processed {} segments", delta);
/// }) as primes::ProgressCallback);
/// let primes = generate_primes(1_000_000, false, Some(4), Some(100_000), progress);
/// assert!(primes.is_ok());
/// # Ok::<_, primes::PrimeGenError>(())
/// ```
pub fn generate_primes(
    n: usize,
    parallel: bool,
    workers: Option<usize>,
    segment_size: Option<usize>,
    progress: Option<ProgressCallback>,
) -> Result<Vec<usize>, PrimeGenError> {
    if n <= 2 {
        return Ok(Vec::new());
    }

    validate_n(n)?;

    let workers = match workers {
        Some(w) => validate_workers(w).map(|_| w)?,
        None => std::thread::available_parallelism()
            .map(|p| p.get())
            .unwrap_or(4),
    };

    let segment_size = match segment_size {
        Some(s) => validate_segment_size(s).map(|_| s)?,
        None => DEFAULT_SEGMENT_SIZE,
    };

    if parallel && n >= PARALLEL_THRESHOLD {
        parallel_segmented_sieve(n, workers, segment_size, progress)
    } else if n >= DEFAULT_SEGMENT_SIZE {
        segmented_sieve(n, segment_size, progress)
    } else {
        sieve_of_eratosthenes(n, progress)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::PrimeGenError;
    use crate::parallel::parallel_segmented_sieve;
    use crate::segmented::segmented_sieve;
    use crate::MAX_N;

    #[test]
    fn test_validation_workers_zero() {
        let result = generate_primes(100, false, Some(0), None, None);
        assert!(
            matches!(result, Err(PrimeGenError::InvalidInput(msg)) if msg.contains("workers=0"))
        );
    }

    #[test]
    fn test_validation_workers_exceeds_max() {
        let result = generate_primes(100, false, Some(crate::MAX_WORKERS + 1), None, None);
        assert!(
            matches!(result, Err(PrimeGenError::InvalidInput(msg)) if msg.contains("exceeds maximum"))
        );
    }

    #[test]
    fn test_validation_segment_size_zero() {
        let result = generate_primes(100, false, None, Some(0), None);
        assert!(
            matches!(result, Err(PrimeGenError::InvalidInput(msg)) if msg.contains("segment_size cannot be zero"))
        );
    }

    #[test]
    fn test_n_exceeds_max() {
        let result = generate_primes(MAX_N + 1, false, None, None, None);
        assert!(
            matches!(result, Err(PrimeGenError::InvalidInput(msg)) if msg.contains("exceeds maximum"))
        );
    }

    #[test]
    fn test_all_algorithms_exclusive_of_n() {
        let n = 7;
        let expected = vec![2, 3, 5];

        let classic = sieve_of_eratosthenes(n, None).unwrap();
        let segmented = segmented_sieve(n, 3, None).unwrap();
        let parallel = parallel_segmented_sieve(n, 2, 3, None).unwrap();

        assert_eq!(classic, expected);
        assert_eq!(segmented, expected);
        assert_eq!(parallel, expected);

        let n = 10;
        let expected = vec![2, 3, 5, 7];

        let classic = sieve_of_eratosthenes(n, None).unwrap();
        let segmented = segmented_sieve(n, 5, None).unwrap();
        let parallel = parallel_segmented_sieve(n, 2, 5, None).unwrap();

        assert_eq!(classic, expected);
        assert_eq!(segmented, expected);
        assert_eq!(parallel, expected);

        let n = 30;
        let expected = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];

        let classic = sieve_of_eratosthenes(n, None).unwrap();
        let segmented = segmented_sieve(n, 10, None).unwrap();
        let parallel = parallel_segmented_sieve(n, 2, 10, None).unwrap();

        assert_eq!(classic, expected);
        assert_eq!(segmented, expected);
        assert_eq!(parallel, expected);
    }
}
