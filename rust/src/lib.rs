//! High-performance prime number generator
//! 
//! Provides three algorithms:
//! - Classic Sieve of Eratosthenes (best for n < 1M)
//! - Segmented Sieve (best for n >= 1M)
//! - Parallel Segmented Sieve (best for n >= 100M)

use std::cmp::min;
use std::f64;
use std::sync::Arc;
use std::thread;

pub const DEFAULT_SEGMENT_SIZE: usize = 1_000_000;
pub const PARALLEL_THRESHOLD: usize = 100_000_000;

/// Classic Sieve of Eratosthenes
/// Best for n < 1,000,000
pub fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    if n <= 2 {
        return Vec::new();
    }

    let mut sieve = vec![true; n];
    sieve[0] = false;
    sieve[1] = false;

    let limit = (n as f64).sqrt() as usize;
    for i in 2..=limit {
        if sieve[i] {
            let start = i * i;
            let step = i;
            for j in (start..n).step_by(step) {
                sieve[j] = false;
            }
        }
    }

    sieve
        .into_iter()
        .enumerate()
        .filter_map(|(i, is_prime)| if is_prime { Some(i) } else { None })
        .collect()
}

/// Segmented Sieve of Eratosthenes
/// Best for n >= 1,000,000
/// Uses O(√n + segment_size) memory
pub fn segmented_sieve(
    n: usize,
    segment_size: usize,
    progress: Option<Arc<dyn Fn(usize) + Send + Sync>>,
) -> Vec<usize> {
    if n <= 2 {
        return Vec::new();
    }

    let base_limit = (n as f64).sqrt() as usize;
    let base_primes = sieve_of_eratosthenes(base_limit + 1);

    let segments = (n + segment_size - 1) / segment_size;
    let mut primes = Vec::with_capacity(n / (n as f64).ln() as usize);

    // Reusable buffer for segments - allocate once to max segment size
    let mut is_prime = vec![true; segment_size];

    for seg_idx in 0..segments {
        let low = seg_idx * segment_size;
        let high = min(low + segment_size, n);

        if high <= 2 {
            continue;
        }

        let segment_low = if low < 2 { 2 } else { low };
        let seg_len = high - segment_low;
        // Reuse buffer: reset only the portion we need
        is_prime[..seg_len].fill(true);

        for &p in &base_primes {
            let mut start = ((low + p - 1) / p) * p;
            if start < p * p {
                start = p * p;
            }

            let adjusted_start = start - segment_low;
            if adjusted_start >= seg_len {
                continue;
            }

            let step = p;
            for j in (adjusted_start..seg_len).step_by(step) {
                is_prime[j] = false;
            }
        }

        // Only iterate over the actual segment length, not the full buffer
        for (i, &is_p) in is_prime[..seg_len].iter().enumerate() {
            if is_p {
                primes.push(segment_low + i);
            }
        }

        if let Some(ref callback) = progress {
            callback(seg_idx + 1);
        }
    }

    primes
}

struct SegmentResult {
    seg_idx: usize,
    primes: Vec<usize>,
}

/// Parallel Segmented Sieve
/// Best for n >= 100,000,000
/// Uses multiple threads for concurrent segment processing
pub fn parallel_segmented_sieve(
    n: usize,
    workers: usize,
    segment_size: usize,
    progress: Option<Arc<dyn Fn(usize) + Send + Sync>>,
) -> Vec<usize> {
    if n <= 2 {
        return Vec::new();
    }

    let base_limit = (n as f64).sqrt() as usize;
    let base_primes = sieve_of_eratosthenes(base_limit + 1);

    let segments = (n + segment_size - 1) / segment_size;
    let num_workers = min(workers, segments);

    let chunk_size = (segments + num_workers - 1) / num_workers;

    std::thread::scope(|s| {
        let mut handles = Vec::new();
        
        for worker_idx in 0..num_workers {
            let start_seg = worker_idx * chunk_size;
            let end_seg = min(start_seg + chunk_size, segments);
            
            if start_seg >= segments {
                break;
            }

            let base_primes = base_primes.clone();
            let progress = progress.clone();
            
            handles.push(s.spawn(move || {
                let mut results: Vec<SegmentResult> = Vec::new();
                // Reusable buffer for this worker's segments
                let mut is_prime = vec![true; segment_size];
                
                for seg_idx in start_seg..end_seg {
                    let low = seg_idx * segment_size;
                    let high = min(low + segment_size, n);
                    
                    if high <= 2 {
                        if let Some(ref callback) = progress {
                            callback(seg_idx + 1);
                        }
                        continue;
                    }
                    
                    let segment_low = if low < 2 { 2 } else { low };
                    let seg_len = high - segment_low;
                    // Reuse buffer: reset only the portion we need
                    is_prime[..seg_len].fill(true);
                    
                    for &p in &base_primes {
                        let mut start = ((low + p - 1) / p) * p;
                        if start < p * p {
                            start = p * p;
                        }
                        
                        let adjusted_start = start - segment_low;
                        if adjusted_start >= seg_len {
                            continue;
                        }
                        
                        let step = p;
                        for j in (adjusted_start..seg_len).step_by(step) {
                            is_prime[j] = false;
                        }
                    }
                    
                    // Extract primes without consuming the buffer
                    // Use a slice to limit iteration to actual segment length
                    let segment_slice = &is_prime[..seg_len];
                    let mut primes = Vec::with_capacity(seg_len / 10);
                    for (i, &is_p) in segment_slice.iter().enumerate() {
                        if is_p {
                            primes.push(segment_low + i);
                        }
                    }
                    
                    results.push(SegmentResult {
                        seg_idx,
                        primes,
                    });
                    
                    if let Some(ref callback) = progress {
                        callback(seg_idx + 1);
                    }
                }
                
                results
            }));
        }
        
        // Pre-allocate results indexed by segment to avoid sorting
        let mut all_primes: Vec<Vec<usize>> = vec![Vec::new(); segments];
        for handle in handles {
            let results = handle.join().unwrap();
            for result in results {
                all_primes[result.seg_idx] = result.primes;
            }
        }
        
        // Flatten results in segment order
        all_primes.into_iter().flatten().collect()
    })
}

/// Auto-select algorithm based on n
pub fn generate_primes(
    n: usize,
    parallel: bool,
    workers: Option<usize>,
    segment_size: Option<usize>,
    progress: Option<Arc<dyn Fn(usize) + Send + Sync>>,
) -> Vec<usize> {
    if n <= 2 {
        return Vec::new();
    }

    let workers = workers.unwrap_or_else(|| thread::available_parallelism().map(|p| p.get()).unwrap_or(4));
    let segment_size = segment_size.unwrap_or(DEFAULT_SEGMENT_SIZE);

    if parallel && n >= PARALLEL_THRESHOLD {
        parallel_segmented_sieve(n, workers, segment_size, progress)
    } else if n >= DEFAULT_SEGMENT_SIZE {
        segmented_sieve(n, segment_size, progress)
    } else {
        sieve_of_eratosthenes(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sieve_small() {
        assert_eq!(sieve_of_eratosthenes(10), vec![2, 3, 5, 7]);
        assert_eq!(sieve_of_eratosthenes(30), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn test_sieve_empty() {
        assert_eq!(sieve_of_eratosthenes(0), Vec::new());
        assert_eq!(sieve_of_eratosthenes(1), Vec::new());
        assert_eq!(sieve_of_eratosthenes(2), Vec::new());
    }

    #[test]
    fn test_segmented_matches_classic() {
        for &n in &[100, 500, 1000, 5000] {
            let classic = sieve_of_eratosthenes(n);
            let segmented = segmented_sieve(n, 100, None);
            assert_eq!(classic, segmented, "Failed for n={}", n);
        }
    }

    #[test]
    fn test_parallel_matches_segmented() {
        for &n in &[100, 500, 1000, 5000] {
            let segmented = segmented_sieve(n, 100, None);
            let parallel = parallel_segmented_sieve(n, 2, 100, None);
            assert_eq!(segmented, parallel, "Failed for n={}", n);
        }
    }

    #[test]
    fn test_large_input() {
        let primes = segmented_sieve(1_000_000, DEFAULT_SEGMENT_SIZE, None);
        assert_eq!(primes.len(), 78498);
        assert_eq!(primes[0], 2);
        assert_eq!(primes.last().unwrap(), &999983);
    }
}
