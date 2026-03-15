//! Catalan number generator
//!
//! Cₙ = (2n)!/(n!(n+1)!) = binomial(2n, n) / (n+1)
//! Sequence: 1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862, ...

pub fn generate_catalan(count: usize) -> Vec<usize> {
    if count == 0 {
        return Vec::new();
    }

    let mut catalan: Vec<usize> = Vec::with_capacity(count);
    catalan.push(1);

    for i in 1..count {
        let prev = catalan[i - 1];
        let next = prev.saturating_mul(2 * (2 * i - 1)) / (i + 1);
        catalan.push(next);
    }

    catalan
}

pub fn generate_catalan_up_to(max_value: usize) -> Vec<usize> {
    if max_value < 1 {
        return Vec::new();
    }

    let mut catalan: Vec<usize> = vec![1];
    let mut i = 1;

    loop {
        let prev = catalan[i - 1];
        let next = prev.saturating_mul(2 * (2 * i - 1)) / (i + 1);
        if next > max_value || next == 0 {
            break;
        }
        catalan.push(next);
        i += 1;
    }

    catalan
}

pub fn is_catalan(n: usize) -> bool {
    if n < 1 {
        return false;
    }
    if n == 1 {
        return true;
    }

    let mut catalan = 1usize;
    let mut i = 1;

    while catalan < n {
        catalan = catalan.saturating_mul(2 * (2 * i - 1)) / (i + 1);
        if catalan == n {
            return true;
        }
        if catalan == 0 {
            break;
        }
        i += 1;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_small() {
        assert_eq!(generate_catalan(0), Vec::<usize>::new());
        assert_eq!(generate_catalan(1), vec![1]);
        assert_eq!(generate_catalan(5), vec![1, 1, 2, 5, 14]);
        assert_eq!(
            generate_catalan(10),
            vec![1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862]
        );
    }

    #[test]
    fn test_generate_up_to() {
        assert_eq!(generate_catalan_up_to(0), Vec::<usize>::new());
        assert_eq!(generate_catalan_up_to(1), vec![1, 1]);
        assert_eq!(generate_catalan_up_to(50), vec![1, 1, 2, 5, 14, 42]);
    }

    #[test]
    fn test_is_catalan() {
        for &n in &[1, 2, 5, 14, 42, 132, 429, 1430] {
            assert!(is_catalan(n), "{} should be Catalan", n);
        }

        for &n in &[0, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 15, 16] {
            assert!(!is_catalan(n), "{} should NOT be Catalan", n);
        }
    }
}
