//! Powers of 2 generator
//!
//! Sequence: 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, ...

pub fn generate_powers_of_2(count: usize) -> Vec<usize> {
    if count == 0 {
        return Vec::new();
    }

    let mut powers: Vec<usize> = Vec::with_capacity(count);
    let mut current: usize = 1;

    for _ in 0..count {
        powers.push(current);
        current = current.saturating_mul(2);
        if current == 0 {
            break;
        }
    }

    powers
}

pub fn generate_powers_of_2_up_to(max_value: usize) -> Vec<usize> {
    if max_value < 1 {
        return Vec::new();
    }

    let mut powers: Vec<usize> = Vec::new();
    let mut current: usize = 1;

    while current <= max_value {
        powers.push(current);
        current = match current.checked_mul(2) {
            Some(v) => v,
            None => break,
        };
    }

    powers
}

pub fn is_power_of_2(n: usize) -> bool {
    n > 0 && (n & (n - 1)) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_small() {
        assert_eq!(generate_powers_of_2(0), Vec::<usize>::new());
        assert_eq!(generate_powers_of_2(1), vec![1]);
        assert_eq!(generate_powers_of_2(5), vec![1, 2, 4, 8, 16]);
        assert_eq!(
            generate_powers_of_2(10),
            vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512]
        );
    }

    #[test]
    fn test_generate_up_to() {
        assert_eq!(generate_powers_of_2_up_to(0), Vec::<usize>::new());
        assert_eq!(generate_powers_of_2_up_to(1), vec![1]);
        assert_eq!(generate_powers_of_2_up_to(10), vec![1, 2, 4, 8]);
        assert_eq!(
            generate_powers_of_2_up_to(100),
            vec![1, 2, 4, 8, 16, 32, 64]
        );
    }

    #[test]
    fn test_is_power_of_2() {
        for &n in &[1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024] {
            assert!(is_power_of_2(n), "{} should be power of 2", n);
        }

        for &n in &[0, 3, 5, 6, 7, 9, 10, 12, 15, 17, 31, 100] {
            assert!(!is_power_of_2(n), "{} should NOT be power of 2", n);
        }
    }
}
