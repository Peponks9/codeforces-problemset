use crate::utils::io::FastIO;
use std::collections::HashSet;

pub fn solve(io: &mut FastIO) {
    let year: i32 = io.next();
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
