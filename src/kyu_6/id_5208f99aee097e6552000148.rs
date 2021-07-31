fn solution(s: &str) -> String {
    s.chars().map(|v| {
        match v >= 'A' && v <= 'Z' {
            true => format!(" {}", v),
            false => format!("{}", v),
        }
    }).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution("camelCasing"), "camel Casing");
        assert_eq!(solution("camelCasingTest"), "camel Casing Test");
    }
}
