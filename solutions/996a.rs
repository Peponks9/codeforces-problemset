// solution for problem 996A
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let amount: i32 = input.trim().parse().unwrap();

    // logic
    let bills = [100, 20, 10, 5, 1];
    let mut min_bills = 0;
    let mut current = amount;
    for &bill in &bills {
        if current == 0 {
            break;
        }
        let count = current / bill;
        min_bills += count;
        current -= count * bill;
    }
    println!("{}", min_bills);
}
