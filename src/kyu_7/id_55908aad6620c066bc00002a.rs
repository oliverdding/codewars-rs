fn xo(string: &'static str) -> bool {
    string.matches(|c| {
        c == 'o' || c == 'O'
    }).count() == string.matches(|c| {
        c == 'x' || c == 'X'
    }).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(xo("xo"), true);
        assert_eq!(xo("Xo"), true);
        assert_eq!(xo("xxOo"), true);
        assert_eq!(xo("xxxm"), false);
        assert_eq!(xo("Oo"), false);
        assert_eq!(xo("ooom"), false);
    }
}