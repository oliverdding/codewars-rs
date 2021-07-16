fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_multiply() {
        assert_eq!(multiply(3, 5), 15);
    }

    #[test]
    fn negative_multiply() {
        assert_eq!(multiply(-5, 5), -25);
    }

    #[test]
    fn zero_multiply() {
        assert_eq!(multiply(0, 54), 0)
    }
}
