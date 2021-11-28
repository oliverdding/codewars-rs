// I'm stupid :(
// fn get_complementary_code(x: i32) -> u32 {
//     if let true = x.is_negative() {
//         let mut x = !(x.abs() as u32);
//         let mut carry = 1_u32;
//         while carry != 0_u32 {
//             if carry == u32::MAX {
//                 panic!("overflow")
//             }
//             if let 0b0 = x & carry {
//                 x |= carry;
//                 carry = 0;
//             } else {
//                 x &= !carry;
//                 carry <<= 1;
//             }
//         }
//         x
//     } else {
//         x as u32
//     }
// }
//
// fn add(x: i32, y: i32) -> i32 {
//     let mut x = get_complementary_code(x);
//     let mut y = get_complementary_code(y);
//     let mut result: u32 = 0;
//     let mut carry = 0_u32;
//     for i in 0_u32..32_u32 {
//         let mask = 1_u32 << i;
//         if 0 == (x & mask) | (y & mask) {
//             if carry != 0 {
//                 result |= carry;
//                 carry = 0_u32;
//             }
//             carry = 0_u32;
//         } else if mask == (x & mask) & (y & mask) {
//             if carry != 0 {
//                 result |= carry;
//                 carry = 0_u32;
//             }
//             carry = mask << 1;
//         } else {
//             if carry == 0 {
//                 result |= mask;
//             } else {
//                 carry = mask << 1;
//             }
//         }
//     }
//     result as i32
// }

pub fn add(mut x: i32, mut y: i32) -> i32 {
    while y != 0 {
        let carry = x & y;
        x = x ^ y;
        y = carry << 1;
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_values() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(5, 19), 24);
        assert_eq!(add(23, 17), 40);
    }

    #[test]
    fn test_negative_values() {
        assert_eq!(add(-14, -16), -30);
        assert_eq!(add(-50, -176), -226);
        assert_eq!(add(-10, -29), -39);
    }

    #[test]
    fn test_mixture_values() {
        assert_eq!(add(-13, 13), 0);
        assert_eq!(add(-27, 18), -9);
        assert_eq!(add(-90, 30), -60);
    }
}
