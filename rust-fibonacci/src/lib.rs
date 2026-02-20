//! High-performance Fibonacci number generator
//!
//! Provides iterative Fibonacci generation:
//! - `generate_fibonacci(count)` - Generate first N Fibonacci numbers
//! - `generate_fibonacci_up_to(max_value)` - Generate all Fibonacci numbers up to max
//! - `is_fibonacci(n)` - Check if a number is in the Fibonacci sequence

pub fn generate_fibonacci(count: usize) -> Vec<usize> {
    if count == 0 {
        return Vec::new();
    }
    if count == 1 {
        return vec![0];
    }
    if count == 2 {
        return vec![0, 1];
    }

    let mut fibs: Vec<usize> = Vec::with_capacity(count);
    fibs.push(0_usize);
    fibs.push(1_usize);

    for _ in 2..count {
        let next = fibs[fibs.len() - 1].saturating_add(fibs[fibs.len() - 2]);
        fibs.push(next);
    }

    fibs
}

pub fn generate_fibonacci_up_to(max_value: usize) -> Vec<usize> {
    if max_value == 0 {
        return vec![0];
    }
    if max_value == 1 {
        return vec![0, 1, 1];
    }

    let mut fibs: Vec<usize> = vec![0, 1];

    loop {
        let next = fibs[fibs.len() - 1].saturating_add(fibs[fibs.len() - 2]);
        if next > max_value || next < fibs[fibs.len() - 1] {
            break;
        }
        fibs.push(next);
    }

    fibs
}

pub fn is_fibonacci(n: usize) -> bool {
    if n == 0 || n == 1 {
        return true;
    }

    let n_sq = n.saturating_mul(n);
    let five_n_sq_plus_4 = n_sq.saturating_mul(5).saturating_add(4);
    let five_n_sq_minus_4 = n_sq.saturating_mul(5).saturating_sub(4);

    is_perfect_square(five_n_sq_plus_4) || is_perfect_square(five_n_sq_minus_4)
}

fn is_perfect_square(n: usize) -> bool {
    if n == 0 {
        return true;
    }

    let root = (n as f64).sqrt() as usize;
    root.saturating_mul(root) == n || root.saturating_add(1).saturating_mul(root + 1) == n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_small() {
        assert_eq!(generate_fibonacci(0), Vec::<usize>::new());
        assert_eq!(generate_fibonacci(1), vec![0]);
        assert_eq!(generate_fibonacci(2), vec![0, 1]);
        assert_eq!(
            generate_fibonacci(10),
            vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
        );
    }

    #[test]
    fn test_generate_up_to() {
        assert_eq!(generate_fibonacci_up_to(0), vec![0]);
        assert_eq!(generate_fibonacci_up_to(1), vec![0, 1, 1]);
        assert_eq!(generate_fibonacci_up_to(10), vec![0, 1, 1, 2, 3, 5, 8]);
        assert_eq!(
            generate_fibonacci_up_to(34),
            vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
        );
    }

    #[test]
    fn test_is_fibonacci() {
        for &n in &[0, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144] {
            assert!(is_fibonacci(n), "{} should be Fibonacci", n);
        }

        for &n in &[4, 6, 7, 9, 10, 11, 12, 14, 15, 16, 17, 18, 19, 20] {
            assert!(!is_fibonacci(n), "{} should NOT be Fibonacci", n);
        }
    }

    #[test]
    fn test_large_fibonacci() {
        let fibs = generate_fibonacci(50);
        assert_eq!(fibs.len(), 50);
        assert_eq!(fibs[0], 0);
        assert_eq!(fibs[1], 1);
        assert_eq!(fibs[10], 55);
        assert_eq!(fibs[20], 6765);
    }
}
