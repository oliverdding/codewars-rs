pub fn square_sum(vec: Vec<i32>) -> i32 {
    vec.iter().map(|i| i * i).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(square_sum(vec![1, 2]), 5);
        assert_eq!(square_sum(vec![-1, -2]), 5);
        assert_eq!(square_sum(vec![5, 3, 4]), 50);
    }
}