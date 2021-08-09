// I'm not that stupid :)
// use std::ops::{BitAnd, Shr};
//
// fn int32_to_ip(int: u32) -> String {
//     return format!("{}.{}.{}.{}",
//         int.bitand(0xFF000000_u32).shr(24),
//         int.bitand(0xFF0000_u32).shr(16),
//         int.bitand(0xFF00_u32).shr(8),
//         int.bitand(0xFF_u32),
//     );
// }

use std::net::Ipv4Addr;

fn int32_to_ip(int: u32) -> String {
    Ipv4Addr::from(int).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(int32_to_ip(2154959208), "128.114.17.104");
        assert_eq!(int32_to_ip(2149583361), "128.32.10.1");
        assert_eq!(int32_to_ip(0), "0.0.0.0");
    }
}
