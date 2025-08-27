use crate::utils::io::FastIO;

pub fn solve(io: &mut FastIO) {
    let first: String = io.next();
    let second: String = io.next();
    let result = operation(&first, &second);
    println!("{}", result);
}

fn operation(first: &str, second: &str) -> String {
    let mut result = String::new();
    for (f, s) in first.chars().zip(second.chars()) {
        if f == s {
            result.push('0'); // XOR: same bits = 0
        } else {
            result.push('1'); // XOR: different bits = 1
        }
    }
    result
}
