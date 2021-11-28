// unique decomposition theorem
pub fn prime_factors(mut n: i64) -> String {
    let mut primes = vec![];
    let mut factor = 2;
    while n > 1 {
        let mut count = 0;
        while n % factor == 0 {
            n = n / factor;
            count += 1;
        }
        if count == 1 {
            primes.push(format!("({})", factor));
        } else if count > 1 {
            primes.push(format!("({}**{})", factor, count));
        }
        factor += 1;
    }
    primes.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(n: i64, exp: &str) -> () {
        assert_eq!(&prime_factors(n), exp)
    }

    #[test]
    fn basics_prime_factors() {
        testing(7775460, "(2**2)(3**3)(5)(7)(11**2)(17)");
        testing(17 * 17 * 93 * 677, "(3)(17**2)(31)(677)");
    }
}
