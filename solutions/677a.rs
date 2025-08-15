// solution for problem 677a
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();
    let mut stops = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap();
        let mut parts = line.split_whitespace();
        let a: i32 = parts.next().unwrap().parse().unwrap();
        let b: i32 = parts.next().unwrap().parse().unwrap();
        stops.push((a, b));
    }
    // logic: a: passengers leave, b: passengers enter
    let mut current_passengers = 0;
    let mut max_passengers = 0;

    for (a, b) in stops {
        current_passengers += b - a;
        max_passengers = max_passengers.max(current_passengers);
    }

    println!("{}", max_passengers);
}
