// solution for problem 1030A
use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    // input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();
    let mut votes = Vec::new();
    for _ in 0..n {
        let vote: usize = lines.next().unwrap().parse().unwrap();
        votes.push(vote);
    }

    // votes -> if hashset.len() > 1 - it's hard
    let mut count = HashSet::new(); // 0 easy, 1 hard
    for vote in votes {
        count.insert(vote);
    }
    if count.len() > 1 {
        println!("HARD");
    } else {
        println!("EASY");
    }
}
