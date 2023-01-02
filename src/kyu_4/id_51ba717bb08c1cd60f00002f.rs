// Should make good use of iteration...
// pub fn range_extraction(a: &[i32]) -> String {
//     let mut slices = Vec::with_capacity(a.len());
//     for i in 0..(a.len()-1) {
//         slices.push(a[i+1] - a[i]);
//     }
//     slices.push(1);
//     let mut result: Vec<String> = Vec::new();
//     let mut i = 0;
//     while i < a.len() {
//         if slices[i] == 1 {
//             // jump to next not 1
//             let mut j = i;
//             while j < (a.len()-1) && slices[j] == 1 {
//                 j += 1;
//             }
//             if j - i > 1 {
//                 result.push(format!("{}-{}", a[i], a[j]))
//             } else {
//                 for x in i..=j {
//                     result.push(format!("{}", a[x]));
//                 }
//             }
//             i = j + 1;
//         } else {
//             result.push(format!("{}", a[i]));
//             i += 1;
//         }
//     }
//     result.join(",")
// }

pub fn range_extraction(a: &[i32]) -> String {
    let mut ans = vec![];
    let mut start = a[0];
    let mut current = a[0];
    for i in a.iter().skip(1) {
        if *i != current + 1 {
            ans.push((start, current));
            start = *i;
        }
        current = *i;
    }
    ans.push((start, current));
    ans.iter()
        .map(|(start, end)| {
            if end == start {
                end.to_string()
            } else if *end == start + 1 {
                format!("{},{}", start, end)
            } else {
                format!("{}-{}", start, end)
            }
        })
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            range_extraction(&[
                -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20
            ]),
            "-6,-3-1,3-5,7-11,14,15,17-20"
        );
        assert_eq!(
            range_extraction(&[-3, -2, -1, 2, 10, 15, 16, 18, 19, 20]),
            "-3--1,2,10,15,16,18-20"
        );
    }

    #[test]
    fn one_by_one() {
        assert_eq!(
            range_extraction(&[-3, 1, 3]),
            "-3,1,3"
        );
    }
}
