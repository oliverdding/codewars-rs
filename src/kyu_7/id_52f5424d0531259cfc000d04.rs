use std::ops::{BitOr, Shl};

pub fn sort_by_bit(list: &[u8]) -> u32 {
    list.iter().fold(0_u32, |acc, &idx| acc.bitor(1_u32.shl(idx)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            sort_by_bit(&[] as &[u8; 0]),
            0b0000,
            "should return 0 if empty array is provided",
        );

        assert_eq!(
            sort_by_bit(&[0]),
            0b0001,
            "should return 1 if array with zero is provided",
        );

        assert_eq!(
            sort_by_bit(&[0, 1]),
            0b0011,
            "should return 3 if array with zero and 1 is provided",
        );

        assert_eq!(
            sort_by_bit(&[1, 0]),
            0b0011,
            "should return 3 if array with zero and 1 is provided, order shouldn't matter",
        );

        assert_eq!(
            sort_by_bit(&[30, 0]),
            0b0100_0000_0000_0000_0000_0000_0000_0001,
            "should return 1073741825 if array with 30 and 0 provided"
        );
    }
}
