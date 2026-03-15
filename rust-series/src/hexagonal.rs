//! Hexagonal number generator
//!
//! Hₙ = n(2n-1)
//! Sequence: 1, 6, 15, 28, 45, 66, 91, 120, 153, 190, ...

pub fn generate_hexagonal(count: usize) -> Vec<usize> {
    (1..=count).map(|n| n * (2 * n - 1)).collect()
}

pub fn generate_hexagonal_up_to(max_value: usize) -> Vec<usize> {
    if max_value < 1 {
        return Vec::new();
    }

    let mut hex: Vec<usize> = Vec::new();
    let mut n = 1;

    loop {
        let h = n * (2 * n - 1);
        if h > max_value {
            break;
        }
        hex.push(h);
        n += 1;
    }

    hex
}

pub fn is_hexagonal(n: usize) -> bool {
    if n < 1 {
        return false;
    }

    let eight_n_plus_1 = n.saturating_mul(8).saturating_add(1);
    if !is_perfect_square(eight_n_plus_1) {
        return false;
    }

    let root = (eight_n_plus_1 as f64).sqrt() as usize;
    (root + 1).is_multiple_of(4)
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
        assert_eq!(generate_hexagonal(0), Vec::<usize>::new());
        assert_eq!(generate_hexagonal(1), vec![1]);
        assert_eq!(generate_hexagonal(5), vec![1, 6, 15, 28, 45]);
        assert_eq!(
            generate_hexagonal(10),
            vec![1, 6, 15, 28, 45, 66, 91, 120, 153, 190]
        );
    }

    #[test]
    fn test_generate_up_to() {
        assert_eq!(generate_hexagonal_up_to(0), Vec::<usize>::new());
        assert_eq!(generate_hexagonal_up_to(10), vec![1, 6]);
        assert_eq!(
            generate_hexagonal_up_to(100),
            vec![1, 6, 15, 28, 45, 66, 91]
        );
    }

    #[test]
    fn test_is_hexagonal() {
        for &n in &[1, 6, 15, 28, 45, 66, 91, 120, 153, 190] {
            assert!(is_hexagonal(n), "{} should be hexagonal", n);
        }

        for &n in &[0, 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14, 16, 17] {
            assert!(!is_hexagonal(n), "{} should NOT be hexagonal", n);
        }
    }
}
