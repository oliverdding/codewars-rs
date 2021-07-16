// I'm stupid :(
// fn longest(a1: &str, a2: &str) -> String {
//     let mut occ = [0_usize; 26];
//     a1.chars().for_each(|c| {
//         occ[c as usize - 'a' as usize] += 1;
//     });
//     a2.chars().for_each(|c| {
//         occ[c as usize - 'a' as usize] += 1;
//     });
//     occ.iter().enumerate().filter(|&t| {
//         *t.1 != 0
//     }).map(|t| (t.0 + 'a' as usize) as u8 as char).collect::<String>()
// }

use std::collections::BTreeSet;

fn longest(a1: &str, a2: &str) -> String {
    a1.chars()
        .chain(a2.chars())
        .collect::<BTreeSet<char>>()
        .iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(s1: &str, s2: &str, exp: &str) -> () {
        println!("s1:{:?} s2:{:?}", s1, s2);
        println!("{:?} {:?}", longest(s1, s2), exp);
        println!("{}", longest(s1, s2) == exp);
        assert_eq!(&longest(s1, s2), exp)
    }

    #[test]
    fn basic_tests() {
        testing("aretheyhere", "yestheyarehere", "aehrsty");
        testing("loopingisfunbutdangerous", "lessdangerousthancoding", "abcdefghilnoprstu");
    }
}
