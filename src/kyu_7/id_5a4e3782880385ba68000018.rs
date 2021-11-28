pub fn balanced_num(n: u64) -> String {
    let number = n.to_string();
    let number = number.as_bytes();
    let length = number.len();
    let (b1, e1, b2, e2) = if let 0_usize = length & 1 {
        (0_usize, length / 2 - 1, length / 2 + 1, length)
    } else {
        (0_usize, length / 2, length / 2 + 1, length)
    };
    match (b1..e1).map(|i| { number[i] }).sum::<u8>() ==
        (b2..e2).map(|i| { number[i] }).sum::<u8>() {
        true => String::from("Balanced"),
        false => String::from("Not Balanced"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balanced_number() {
        assert_eq!(balanced_num(7), "Balanced");
        assert_eq!(balanced_num(959), "Balanced");
        assert_eq!(balanced_num(13), "Balanced");
        assert_eq!(balanced_num(432), "Not Balanced");
        assert_eq!(balanced_num(424), "Balanced");
    }

    #[test]
    fn larger_number() {
        assert_eq!(balanced_num(1024), "Not Balanced");
        assert_eq!(balanced_num(66545), "Not Balanced");
        assert_eq!(balanced_num(295591), "Not Balanced");
        assert_eq!(balanced_num(1230987), "Not Balanced");
        assert_eq!(balanced_num(56239814), "Balanced");
    }
}