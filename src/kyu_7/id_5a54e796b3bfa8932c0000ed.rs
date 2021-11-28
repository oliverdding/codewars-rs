// I'm stupid :(
// fn jumping_number(n: u64) -> String {
//     let n = n.to_string();
//     let n = n.as_bytes().iter()
//         .map(|&c| c - '0' as u8)
//         .collect::<Vec<u8>>();
//     let mut last = n[0];
//     let mut result = String::from("Jumping!!");
//     n.iter().skip(1).for_each(|&i| {
//         if (i as i8 - last as i8).abs() != 1 {
//             result = String::from("Not!!");
//         }
//         last = i;
//     });
//     return result;
// }

pub fn jumping_number(n: u64) -> String {
    let is_jumping = format!("{}", n)
        .chars()
        .map(|x| (x as u8 - '0' as u8) as i8)
        .collect::<Vec<i8>>()
        .windows(2)
        .map(|xs| (xs[0] - xs[1]).abs() == 1)
        .fold(true, |acc, e| acc && e);
    return if is_jumping {
        String::from("Jumping!!")
    } else {
        String::from("Not!!")
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(jumping_number(1), "Jumping!!");
        assert_eq!(jumping_number(7), "Jumping!!");
        assert_eq!(jumping_number(9), "Jumping!!");
        assert_eq!(jumping_number(23), "Jumping!!");
        assert_eq!(jumping_number(32), "Jumping!!");
        assert_eq!(jumping_number(79), "Not!!");
        assert_eq!(jumping_number(98), "Jumping!!");
        assert_eq!(jumping_number(8987), "Jumping!!");
        assert_eq!(jumping_number(4343456), "Jumping!!");
        assert_eq!(jumping_number(98789876), "Jumping!!");
    }
}
