// Solution for Codeforces Problem 144A - Arrival of the General

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().trim().parse().unwrap();
    let heights: Vec<i32> = lines
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let result = time_to_correct_positions(&heights, n);
    println!("{}", result);
}

fn time_to_correct_positions(heights: &[i32], n: usize) -> i32 {
    let max_height = *heights.iter().max().unwrap();
    let min_height = *heights.iter().min().unwrap();

    // Find the leftmost position of the maximum height
    let pos_max = heights.iter().position(|&x| x == max_height).unwrap();

    // Find the rightmost position of the minimum height
    let pos_min = heights.iter().rposition(|&x| x == min_height).unwrap();

    let mut time = pos_max as i32 + (n as i32 - 1 - pos_min as i32);

    // If the max is to the right of the min, subtract 1 because moving max left affects min's position
    if pos_max > pos_min {
        time -= 1;
    }

    time
}
