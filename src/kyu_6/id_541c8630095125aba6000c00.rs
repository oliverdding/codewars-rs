// fn digital_root(n: i64) -> i64 {
//     if n / 10 == 0 {
//         n
//     } else {
//         digital_root(n % 10 + n / 10)
//     }
// }

pub fn digital_root(n: i64) -> i64 {
    match n {
        n if n > 0 => {
            (n - 1) % 9 + 1
        },
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(digital_root(16), 7);
    }

    #[test]
    fn zero_test() {
        assert_eq!(digital_root(0), 0);
    }

    #[test]
    fn huge_number_test() {
        assert_eq!(digital_root(128481828515), 8);
    }
}
