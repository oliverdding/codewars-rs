fn special_number(n: u64) -> String {
    match format!("{}", n)
        .chars()
        .all(|c| c.to_digit(10).unwrap() <= 5) {
        true => String::from("Special!!"),
        false => String::from("NOT!!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(special_number(2), "Special!!");
        assert_eq!(special_number(3), "Special!!");
        assert_eq!(special_number(6), "NOT!!");
        assert_eq!(special_number(9), "NOT!!");
        assert_eq!(special_number(11), "Special!!");
        assert_eq!(special_number(55), "Special!!");
        assert_eq!(special_number(26), "NOT!!");
        assert_eq!(special_number(92), "NOT!!");
        assert_eq!(special_number(25432), "Special!!");
        assert_eq!(special_number(2783), "NOT!!");
    }
}
