// I'm stupid :(
// fn automorphic(n: u64) -> String {
//     let length = n.to_string().len();
//     let temp = n.pow(2).to_string();
//     match temp.chars().skip(temp.len() - length).collect::<String>().parse::<u64>().unwrap() == n {
//         true => String::from("Automorphic"),
//         false => String::from("Not!!"),
//     }
// }

pub fn automorphic(n: u64) -> String {
    match n.pow(2).to_string().ends_with(&n.to_string()) {
        true => String::from("Automorphic"),
        false => String::from("Not!!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(automorphic(1), "Automorphic");
        assert_eq!(automorphic(3), "Not!!");
        assert_eq!(automorphic(6), "Automorphic");
        assert_eq!(automorphic(9), "Not!!");
        assert_eq!(automorphic(25), "Automorphic");
        assert_eq!(automorphic(53), "Not!!");
        assert_eq!(automorphic(76), "Automorphic");
        assert_eq!(automorphic(95), "Not!!");
        assert_eq!(automorphic(625), "Automorphic");
        assert_eq!(automorphic(225), "Not!!");
    }
}
