//! Lucas number generator
//!
//! L₀=2, L₁=1, Lₙ=Lₙ₋₁+Lₙ₋₂
//! Sequence: 2, 1, 3, 4, 7, 11, 18, 29, 47, 76, ...

pub fn generate_lucas(count: usize) -> Vec<usize> {
    if count == 0 {
        return Vec::new();
    }
    if count == 1 {
        return vec![2];
    }
    if count == 2 {
        return vec![2, 1];
    }

    let mut lucas: Vec<usize> = Vec::with_capacity(count);
    lucas.push(2_usize);
    lucas.push(1_usize);

    for _ in 2..count {
        let next = lucas[lucas.len() - 1].saturating_add(lucas[lucas.len() - 2]);
        lucas.push(next);
    }

    lucas
}

pub fn generate_lucas_up_to(max_value: usize) -> Vec<usize> {
    if max_value < 1 {
        return vec![2];
    }
    if max_value < 2 {
        return vec![2, 1];
    }

    let mut lucas: Vec<usize> = vec![2, 1];

    loop {
        let next = lucas[lucas.len() - 1].saturating_add(lucas[lucas.len() - 2]);
        if next > max_value || next < lucas[lucas.len() - 1] {
            break;
        }
        lucas.push(next);
    }

    lucas
}

pub fn is_lucas(n: usize) -> bool {
    if n == 1 || n == 2 {
        return true;
    }

    let n_sq = n.saturating_mul(n);
    let five_n_sq_plus_20 = n_sq.saturating_mul(5).saturating_add(20);
    let five_n_sq_minus_20 = n_sq.saturating_mul(5).saturating_sub(20);

    is_perfect_square(five_n_sq_plus_20) || is_perfect_square(five_n_sq_minus_20)
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
        assert_eq!(generate_lucas(0), Vec::<usize>::new());
        assert_eq!(generate_lucas(1), vec![2]);
        assert_eq!(generate_lucas(2), vec![2, 1]);
        assert_eq!(generate_lucas(10), vec![2, 1, 3, 4, 7, 11, 18, 29, 47, 76]);
    }

    #[test]
    fn test_generate_up_to() {
        assert_eq!(generate_lucas_up_to(0), vec![2]);
        assert_eq!(generate_lucas_up_to(2), vec![2, 1]);
        assert_eq!(generate_lucas_up_to(10), vec![2, 1, 3, 4, 7]);
        assert_eq!(
            generate_lucas_up_to(50),
            vec![2, 1, 3, 4, 7, 11, 18, 29, 47]
        );
    }

    #[test]
    fn test_is_lucas() {
        for &n in &[1, 2, 3, 4, 7, 11, 18, 29, 47, 76] {
            assert!(is_lucas(n), "{} should be Lucas", n);
        }

        for &n in &[5, 6, 8, 9, 10, 12, 13, 14, 15, 16] {
            assert!(!is_lucas(n), "{} should NOT be Lucas", n);
        }
    }
}
