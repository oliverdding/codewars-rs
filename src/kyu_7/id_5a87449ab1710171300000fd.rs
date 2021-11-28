pub fn tidy_number(n: u64) -> bool {
    n.to_string().as_bytes()
        .windows(2)
        .all(|pair| pair[0] <= pair[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(tidy_number(12), true);
        assert_eq!(tidy_number(102), false);
        assert_eq!(tidy_number(9672), false);
        assert_eq!(tidy_number(2789), true);
        assert_eq!(tidy_number(2335), true);
    }
}
