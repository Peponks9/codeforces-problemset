// solution for problem 1328A
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t: usize = lines.next().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let line = lines.next().unwrap();
        let mut nums = line.split_whitespace();
        let a: i32 = nums.next().unwrap().parse().unwrap();
        let b: i32 = nums.next().unwrap().parse().unwrap();

        let result = if is_divisible(a, b) { 0 } else { b - (a % b) };
        println!("{}", result);
    }
}

fn is_divisible(a: i32, b: i32) -> bool {
    a % b == 0
}
