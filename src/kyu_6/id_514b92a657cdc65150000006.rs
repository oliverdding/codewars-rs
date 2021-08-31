fn solution(num: i32) -> i32 {
    (0..num).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(solution(10), 23);
        assert_eq!(solution(11), 33);
        assert_eq!(solution(6), 8);
    }
}
