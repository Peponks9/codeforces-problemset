// solution for problem 228A
use std::collections::HashSet;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let horseshoes: Vec<u32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let unique: HashSet<_> = horseshoes.iter().collect();
    let to_buy = 4 - unique.len();
    println!("{}", to_buy);
}
