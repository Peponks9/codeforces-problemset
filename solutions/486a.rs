// solution for problem 486A
use std::io::{self, Read};

fn main() {
    // input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    let mut sum = 0;
    for i in 1..=n {
        if i % 2 == 0 {
            sum += i;
        } else {
            sum -= i;
        }
    }
    sum
}
