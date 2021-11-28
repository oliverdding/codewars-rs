pub fn disarium_number(n: u32) -> String {
    let n_str = n.to_string();
    match n_str.as_bytes().iter()
        .enumerate()
        .map(|(i, &d)| {
            ((d - '0' as u8) as usize).pow((i + 1) as u32)
        }).sum::<usize>() == n as usize {
        true => String::from("Disarium !!"),
        false => String::from("Not !!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        // assert_eq!(disarium_number(89), "Disarium !!");
        // assert_eq!(disarium_number(564), "Not !!");
        // assert_eq!(disarium_number(1024), "Not !!");
        // assert_eq!(disarium_number(135), "Disarium !!");
        assert_eq!(disarium_number(136586), "Not !!");
    }
}
