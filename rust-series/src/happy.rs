//! Happy number generator
//!
//! A happy number is one where repeatedly summing the squares of digits
//! eventually reaches 1. Unhappy numbers enter a cycle (4, 16, 37, 58, 89, 145, 42, 20, 4).
//! Sequence: 1, 7, 10, 13, 19, 23, 28, 31, 32, 44, 49, 68, 70, 79, ...

pub fn is_happy(mut n: usize) -> bool {
    let mut seen = [false; 1000];

    while n != 1 && !seen[n % 1000] {
        seen[n % 1000] = true;
        n = sum_of_digit_squares(n);
    }

    n == 1
}

fn sum_of_digit_squares(mut n: usize) -> usize {
    let mut sum = 0;
    while n > 0 {
        let digit = n % 10;
        sum += digit * digit;
        n /= 10;
    }
    sum
}

pub fn generate_happy(count: usize) -> Vec<usize> {
    let mut happy: Vec<usize> = Vec::with_capacity(count);
    let mut n = 1;

    while happy.len() < count {
        if is_happy(n) {
            happy.push(n);
        }
        n += 1;
    }

    happy
}

pub fn generate_happy_up_to(max_value: usize) -> Vec<usize> {
    (1..=max_value).filter(|&n| is_happy(n)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_happy() {
        for &n in &[
            1, 7, 10, 13, 19, 23, 28, 31, 32, 44, 49, 68, 70, 79, 82, 86, 91, 94, 97, 100,
        ] {
            assert!(is_happy(n), "{} should be happy", n);
        }

        for &n in &[
            2, 3, 4, 5, 6, 8, 9, 11, 12, 14, 15, 16, 17, 18, 20, 21, 22, 24, 25, 26,
        ] {
            assert!(!is_happy(n), "{} should NOT be happy", n);
        }
    }

    #[test]
    fn test_generate_small() {
        assert_eq!(generate_happy(0), Vec::<usize>::new());
        assert_eq!(generate_happy(1), vec![1]);
        assert_eq!(generate_happy(5), vec![1, 7, 10, 13, 19]);
        assert_eq!(
            generate_happy(10),
            vec![1, 7, 10, 13, 19, 23, 28, 31, 32, 44]
        );
    }

    #[test]
    fn test_generate_up_to() {
        assert_eq!(generate_happy_up_to(0), Vec::<usize>::new());
        assert_eq!(generate_happy_up_to(10), vec![1, 7, 10]);
        assert_eq!(generate_happy_up_to(20), vec![1, 7, 10, 13, 19]);
    }
}
