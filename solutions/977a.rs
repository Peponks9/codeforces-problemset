// solution for problem 977A
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n = iter.next().unwrap().parse::<i32>().unwrap();
    let k = iter.next().unwrap().parse::<i32>().unwrap();

    let result = wrong_subtraction(n, k);
    println!("{}", result);
}

fn wrong_subtraction(n: i32, k: i32) -> i32 {
    let mut n = n;
    for _ in 0..k {
        if n % 10 == 0 {
            n /= 10;
        } else {
            n -= 1;
        }
    }
    n
}
