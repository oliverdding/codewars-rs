pub fn delete_nth(lst: &[u8], n: usize) -> Vec<u8> {
    let mut occurrencces = [0; u8::MAX as usize + 1];
    lst.iter().cloned().filter(|i| {
        occurrencces[*i as usize] += 1;
        occurrencces[*i as usize] <= n as u8
    }).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(delete_nth(&[20, 37, 20, 21], 1), vec![20, 37, 21]);
        assert_eq!(delete_nth(&[1, 1, 3, 3, 7, 2, 2, 2, 2], 3), vec![1, 1, 3, 3, 7, 2, 2, 2]);
    }
}