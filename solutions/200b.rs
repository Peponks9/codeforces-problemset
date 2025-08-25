// solution for problem 200B
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let lines: Vec<&str> = input.lines().collect();
    let n: usize = lines[0].parse().unwrap();
    let percentages: Vec<i32> = lines[1]
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate sum as float to get precise division
    let sum: f64 = percentages.iter().map(|&x| x as f64).sum();
    let average = sum / n as f64;

    println!("{}", average);
}
