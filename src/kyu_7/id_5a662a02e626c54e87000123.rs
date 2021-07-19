fn extra_perfect(n: u32) -> Vec<u32> {
    (1..=n).filter(|&i| i & 1 == 1).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(extra_perfect(3), [1, 3]);
        assert_eq!(extra_perfect(5), [1, 3, 5]);
        assert_eq!(extra_perfect(7), [1, 3, 5, 7]);
        assert_eq!(extra_perfect(28), [1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27]);
        assert_eq!(extra_perfect(39), [1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39]);
    }
}
