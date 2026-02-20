//! Triangular number generator
//!
//! Tₙ = n(n+1)/2
//! Sequence: 0, 1, 3, 6, 10, 15, 21, 28, 36, 45, ...

pub fn generate_triangular(count: usize) -> Vec<usize> {
    (0..count).map(|n| n * (n + 1) / 2).collect()
}

pub fn generate_triangular_up_to(max_value: usize) -> Vec<usize> {
    if max_value == 0 {
        return vec![0];
    }

    let mut triangular: Vec<usize> = Vec::new();
    let mut n = 0;

    loop {
        let t = n * (n + 1) / 2;
        if t > max_value {
            break;
        }
        triangular.push(t);
        n += 1;
    }

    triangular
}

pub fn is_triangular(n: usize) -> bool {
    if n == 0 {
        return true;
    }

    let eight_n_plus_1 = n.saturating_mul(8).saturating_add(1);
    if !is_perfect_square(eight_n_plus_1) {
        return false;
    }

    let root = (eight_n_plus_1 as f64).sqrt() as usize;
    (root - 1) % 2 == 0
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
        assert_eq!(generate_triangular(0), Vec::<usize>::new());
        assert_eq!(generate_triangular(1), vec![0]);
        assert_eq!(generate_triangular(5), vec![0, 1, 3, 6, 10]);
        assert_eq!(
            generate_triangular(10),
            vec![0, 1, 3, 6, 10, 15, 21, 28, 36, 45]
        );
    }

    #[test]
    fn test_generate_up_to() {
        assert_eq!(generate_triangular_up_to(0), vec![0]);
        assert_eq!(generate_triangular_up_to(5), vec![0, 1, 3]);
        assert_eq!(generate_triangular_up_to(20), vec![0, 1, 3, 6, 10, 15]);
    }

    #[test]
    fn test_is_triangular() {
        for &n in &[0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55] {
            assert!(is_triangular(n), "{} should be triangular", n);
        }

        for &n in &[2, 4, 5, 7, 8, 9, 11, 12, 13, 14, 16, 17, 18, 19, 20] {
            assert!(!is_triangular(n), "{} should NOT be triangular", n);
        }
    }
}
