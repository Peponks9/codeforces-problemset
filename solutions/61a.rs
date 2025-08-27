// solution for problem 61A
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lines = input.lines().collect::<Vec<_>>();
    let first = lines[0];
    let second = lines[1];
    let result = operation(first, second);
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
