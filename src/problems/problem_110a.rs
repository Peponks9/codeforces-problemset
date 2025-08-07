use crate::utils::io::FastIO;
use std::collections::HashSet;

pub fn solve(io: &mut FastIO) {
    let n: String = io.next();
    let result = nearly_lucky_number(&n);
    println!("{}", result);
}

fn nearly_lucky_number(n: &str) -> String {
    let lucky_numbers = HashSet::from(['4', '7']);
    let mut lucky_count = 0;

    for c in n.chars() {
        if lucky_numbers.contains(&c) {
            lucky_count += 1;
        }
    }

    if lucky_count == 4 || lucky_count == 7 {
        return "YES".to_string();
    }
    return "NO".to_string();
}
