fn is_prime(n: usize) -> bool {
    if n <= 1 {
        return false;
    }
    let nl = (n as f64).sqrt() as usize;
    for i in 2..=nl {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

fn num_primorial(mut n: usize) -> u64 {
    let mut result = 2_u64;
    let mut i = 3;
    while n > 1 {
        if is_prime(i) {
            result *= i as u64;
            n -= 1;
        }
        i += 2;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(8), false);
        assert_eq!(is_prime(9), false);
    }

    #[test]
    fn test_basic() {
        assert_eq!(num_primorial(3), 30);
        assert_eq!(num_primorial(4), 210);
        assert_eq!(num_primorial(5), 2310);
        assert_eq!(num_primorial(8), 9699690);
        assert_eq!(num_primorial(9), 223092870);
    }
}
