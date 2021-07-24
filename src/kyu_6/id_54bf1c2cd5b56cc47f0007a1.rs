fn count_duplicates(text: &str) -> u32 {
    let mut occ_alphabets = [0; 26];
    let mut occ_numeric = [0; 10];

    text.to_lowercase()
        .chars()
        .for_each(|c| {
            if 'a' <= c {
                occ_alphabets[(c as u8 - 'a' as u8) as usize] += 1;
            } else {
                occ_numeric[(c as u8 - '0' as u8) as usize] += 1;
            }
        });
    occ_alphabets.iter().chain(occ_numeric.iter())
        .filter(|&&i| i > 1)
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abcde() {
        assert_eq!(count_duplicates("abcde"), 0);
    }

    #[test]
    fn test_abcdea() {
        assert_eq!(count_duplicates("abcdea"), 1);
    }

    #[test]
    fn test_indivisibility() {
        assert_eq!(count_duplicates("indivisibility"), 1);
    }
}
