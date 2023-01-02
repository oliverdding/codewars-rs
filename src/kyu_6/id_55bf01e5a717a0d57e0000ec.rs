pub fn persistence(num: u64) -> u64 {
    let mut cnt:u64 = 0;
    let mut result: u64 = num;
    while result >= 10 {
        result = result.to_string().chars().map(|x| x as u64 - 48).product::<u64>();
        cnt +=1;
    }
    cnt
}

#[cfg(test)]
mod tests {

    #[test]
    fn sample_tests() {
        assert_eq!(super::persistence(39), 3);
        assert_eq!(super::persistence(4), 0);
        assert_eq!(super::persistence(25), 2);
        assert_eq!(super::persistence(999), 4);
    }
}
