//! Collatz stopping time calculator
//!
//! For n: count steps to reach 1 using 3n+1 rule
//! - If n is even: n → n/2
//! - If n is odd: n → 3n+1
//!
//! Output is stopping times for 1, 2, 3, 4, 5, ...
//! Sequence: 0, 1, 7, 2, 5, 8, 16, 3, 19, 6, 14, 9, 9, 17, 17, 4, 12, 20, ...

pub fn collatz_stopping_time(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 0;
    }

    let mut current = n as u64;
    let mut steps = 0;

    while current != 1 {
        if current.is_multiple_of(2) {
            current /= 2;
        } else {
            current = 3 * current + 1;
        }
        steps += 1;
    }

    steps
}

pub fn generate_collatz_times(count: usize) -> Vec<usize> {
    (0..count).map(collatz_stopping_time).collect()
}

pub fn generate_collatz_times_up_to(max_value: usize) -> Vec<usize> {
    (0..=max_value).map(collatz_stopping_time).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stopping_time() {
        assert_eq!(collatz_stopping_time(0), 0);
        assert_eq!(collatz_stopping_time(1), 0);
        assert_eq!(collatz_stopping_time(2), 1);
        assert_eq!(collatz_stopping_time(3), 7);
        assert_eq!(collatz_stopping_time(4), 2);
        assert_eq!(collatz_stopping_time(5), 5);
        assert_eq!(collatz_stopping_time(6), 8);
        assert_eq!(collatz_stopping_time(7), 16);
        assert_eq!(collatz_stopping_time(8), 3);
        assert_eq!(collatz_stopping_time(9), 19);
    }

    #[test]
    fn test_generate_times() {
        assert_eq!(generate_collatz_times(0), Vec::<usize>::new());
        assert_eq!(generate_collatz_times(1), vec![0]);
        assert_eq!(generate_collatz_times(5), vec![0, 0, 1, 7, 2]);
        assert_eq!(
            generate_collatz_times(10),
            vec![0, 0, 1, 7, 2, 5, 8, 16, 3, 19]
        );
    }

    #[test]
    fn test_generate_up_to() {
        assert_eq!(generate_collatz_times_up_to(0), vec![0]);
        assert_eq!(generate_collatz_times_up_to(4), vec![0, 0, 1, 7, 2]);
    }
}
