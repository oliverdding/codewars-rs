static DEC: [u64; 10] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

pub fn strong(n: u64) -> String {
    let n_str = n.to_string();
    match n_str.as_bytes().iter().map(|&i| DEC[(i - ('0' as u8)) as usize]).sum::<u64>() == n {
        true => String::from("STRONG!!!!"),
        false => String::from("Not Strong !!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        // Testing for 1
        assert_eq!(strong(1), "STRONG!!!!");

        // Testing for 2
        assert_eq!(strong(2), "STRONG!!!!");

        // Testing for 145
        assert_eq!(strong(145), "STRONG!!!!");

        // Testing for 7
        assert_eq!(strong(7), "Not Strong !!");

        // Testing for 93
        assert_eq!(strong(93), "Not Strong !!");

        // Testing for 185
        assert_eq!(strong(185), "Not Strong !!");
    }
}
