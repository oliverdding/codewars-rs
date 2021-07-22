fn num_primorial(n: usize) -> u64 {
    // TODO
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(num_primorial(3), 30);
        assert_eq!(num_primorial(4), 210);
        assert_eq!(num_primorial(5), 2310);
        assert_eq!(num_primorial(8), 9699690);
        assert_eq!(num_primorial(9), 223092870);
    }
}
