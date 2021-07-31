// I'm stupid :(
// use std::ops::Add;
//
// fn print(n: i32) -> Option<String> {
//     match n.is_positive() && (n & 1 == 1) {
//         true => {
//             let mut result = String::new();
//             for i in 1..=(n / 2 + 1) {
//                 result = result.add(&*" ".repeat(((n - 2 * i + 1) / 2) as usize));
//                 result = result.add(&*"*".repeat((2 * i - 1) as usize));
//                 result = result.add("\n");
//             }
//             for i in (n / 2 + 2)..=n {
//                 result = result.add(&*" ".repeat(((2 * i - n - 1) / 2) as usize));
//                 result = result.add(&*"*".repeat((2 * n - 2 * i + 1) as usize));
//                 result = result.add("\n");
//             }
//             Some(result)
//         }
//         false => None
//     }
// }

fn print(n: i32) -> Option<String> {
    match n.is_positive() && (n & 1 == 1) {
        true => Some((1..=(n as usize))
            .chain((1..(n as usize)).rev())
            .step_by(2)
            .map(|i| format!("{}{}\n", " ".repeat((n as usize - i) / 2), "*".repeat(i)))
            .collect::<String>()),
        false => None,
    }
}

#[test]
fn basic_test() {
    assert_eq!(print(3), Some(" *\n***\n *\n".to_string()));
    assert_eq!(print(5), Some("  *\n ***\n*****\n ***\n  *\n".to_string()));
    assert_eq!(print(-3), None);
    assert_eq!(print(2), None);
    assert_eq!(print(0), None);
    assert_eq!(print(1), Some("*\n".to_string()));
}
