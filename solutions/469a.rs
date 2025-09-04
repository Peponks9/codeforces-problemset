// solution for problem 469A
use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    /*
    Input
    The first line contains a single integer n (1 ≤  n ≤ 100).

    The next line contains an integer p (0 ≤ p ≤ n) at first, then follows p distinct integers a1, a2, ..., ap (1 ≤ ai ≤ n). These integers denote the indices of levels Little X can pass. The next line contains the levels Little Y can pass in the same format. It's assumed that levels are numbered from 1 to n.
    */
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let n: i32 = lines.next().unwrap().trim().parse().unwrap();
    let x_levels: Vec<i32> = lines
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    let y_levels: Vec<i32> = lines
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    if pass_the_game(n, x_levels, y_levels) {
        println!("I become the guy.");
    } else {
        println!("Oh, my keyboard!");
    }
}

fn pass_the_game(n: i32, x_levels: Vec<i32>, y_levels: Vec<i32>) -> bool {
    let game_passed: HashSet<i32> = (1..=n).collect();
    let mut current_levels = HashSet::new();
    for &level in &x_levels {
        current_levels.insert(level);
    }
    for &level in &y_levels {
        current_levels.insert(level);
    }

    current_levels == game_passed
}
