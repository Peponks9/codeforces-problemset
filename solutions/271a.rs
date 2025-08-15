// solution for problem 271A
use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let year: i32 = input.trim().parse().unwrap();
    let next_year = next_minimum_year(year);
    println!("{}", next_year);
}

fn next_minimum_year(year: i32) -> i32 {
    let mut candidate = year + 1;

    loop {
        let mut digits = HashSet::new();
        let mut temp = candidate;

        // Extract all digits
        if temp == 0 {
            digits.insert(0);
        } else {
            while temp > 0 {
                digits.insert(temp % 10);
                temp /= 10;
            }
        }

        // If all digits are distinct, return this year
        if digits.len() == 4 {
            return candidate;
        }

        candidate += 1;
    }
}
