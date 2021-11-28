// I'm stupid :(
// // Sliding windows
// fn consecutive_ducks(n: u32) -> bool {
//     let mut start: u32 = 0;
//     let mut sum: u64 = 0;
//     for i in 1..n {
//         sum += i as u64;
//         while sum > n as u64 {
//             sum -= start as u64;
//             start += 1_u32;
//         }
//         if sum == n as u64 {
//             return true;
//         }
//     };
//     return false;
// }

pub fn consecutive_ducks(n: u32) -> bool {
    !n.is_power_of_two()
}

// or:
// 1.
// fn consecutive_ducks(n: u32) -> bool {
//     (n&(n-1))!=0
// }
// 2.
// fn consecutive_ducks(n: u32) -> bool {
//     n.count_ones() != 1
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_values() {
        assert_eq!(consecutive_ducks(69), true);
        assert_eq!(consecutive_ducks(8), false);
        assert_eq!(consecutive_ducks(57), true);
        assert_eq!(consecutive_ducks(6), true);
        assert_eq!(consecutive_ducks(13), true);
        assert_eq!(consecutive_ducks(16), false);
        assert_eq!(consecutive_ducks(91), true);
        assert_eq!(consecutive_ducks(75), true);
        assert_eq!(consecutive_ducks(38), true);
        assert_eq!(consecutive_ducks(25), true);
        assert_eq!(consecutive_ducks(32), false);
        assert_eq!(consecutive_ducks(65), true);
        assert_eq!(consecutive_ducks(13), true);
        assert_eq!(consecutive_ducks(16), false);
        assert_eq!(consecutive_ducks(99), true);
    }

    #[test]
    fn medium_values() {
        assert_eq!(consecutive_ducks(522), true);
        assert_eq!(consecutive_ducks(974), true);
        assert_eq!(consecutive_ducks(755), true);
        assert_eq!(consecutive_ducks(512), false);
        assert_eq!(consecutive_ducks(739), true);
        assert_eq!(consecutive_ducks(1006), true);
        assert_eq!(consecutive_ducks(838), true);
        assert_eq!(consecutive_ducks(1092), true);
        assert_eq!(consecutive_ducks(727), true);
        assert_eq!(consecutive_ducks(648), true);
        assert_eq!(consecutive_ducks(1024), false);
        assert_eq!(consecutive_ducks(851), true);
        assert_eq!(consecutive_ducks(541), true);
        assert_eq!(consecutive_ducks(1011), true);
        assert_eq!(consecutive_ducks(822), true);
    }

    #[test]
    fn large_values() {
        assert_eq!(consecutive_ducks(382131), true);
        assert_eq!(consecutive_ducks(118070), true);
        assert_eq!(consecutive_ducks(17209), true);
        assert_eq!(consecutive_ducks(32768), false);
        assert_eq!(consecutive_ducks(161997), true);
        assert_eq!(consecutive_ducks(400779), true);
        assert_eq!(consecutive_ducks(198331), true);
        assert_eq!(consecutive_ducks(325482), true);
        assert_eq!(consecutive_ducks(88441), true);
        assert_eq!(consecutive_ducks(648), true);
        assert_eq!(consecutive_ducks(65536), false);
        assert_eq!(consecutive_ducks(323744), true);
        assert_eq!(consecutive_ducks(183540), true);
        assert_eq!(consecutive_ducks(65271), true);
        assert_eq!(consecutive_ducks(5263987), true);
    }

    #[test]
    fn huge_values() {
        assert_eq!(consecutive_ducks(u32::MAX), true);
    }
}
