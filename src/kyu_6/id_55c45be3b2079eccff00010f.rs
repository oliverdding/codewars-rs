// I'm stupid :(
// fn extract_number(str: &str) -> u32 {
//     for c in str.chars() {
//         if c.is_digit(10) {
//             return c.to_digit(10).unwrap();
//         }
//     }
//     return 0;
// }
//
// fn order(sentence: &str) -> String {
//     return if sentence == "" {
//         String::from("")
//     } else {
//         let mut result = String::from("");
//         let mut temp: Vec<&str> = sentence.split_whitespace().collect();
//         temp.sort_by(|str1, str2| {
//             extract_number(*str1).cmp(&extract_number(*str2))
//         });
//         temp.iter().cloned().for_each(|word| {
//             result.push_str(word);
//             result.push_str(" ");
//         });
//         result.trim().to_string()
//     };
// }

fn order(sentence: &str) -> String {
    let mut ws: Vec<_> = sentence.split_whitespace().collect();
    ws.sort_by_key(|s| s.chars().find(|c| c.is_digit(10)).unwrap());
    ws.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
        assert_eq!(order(""), "");
    }
}
