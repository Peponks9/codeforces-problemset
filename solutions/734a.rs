// solution for problem 734A
/*
Input
The first line of the input contains a single integer n (1 ≤ n ≤ 100 000) — the number of games played.

The second line contains a string s, consisting of n uppercase English letters 'A' and 'D' — the outcome of each of the games. The i-th character of the string is equal to 'A' if the Anton won the i-th game and 'D' if Danik won the i-th game.
*/
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let s = input.trim();

    let result = anton_or_danik(&s);
    println!("{}", result);
}

fn anton_or_danik(s: &str) -> String {
    let mut anton_count = 0;
    let mut danik_count = 0;

    for c in s.chars() {
        if c == 'A' {
            anton_count += 1;
        } else if c == 'D' {
            danik_count += 1;
        }
    }
    if anton_count > danik_count {
        return "Anton".to_string();
    } else if danik_count > anton_count {
        return "Danik".to_string();
    } else {
        return "Friendship".to_string();
    }
}
