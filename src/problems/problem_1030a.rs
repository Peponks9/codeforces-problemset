use crate::utils::io::FastIO;
use std::collections::HashSet;

pub fn solve(io: &mut FastIO) {
    let n: usize = io.next();
    let mut votes = Vec::new();
    for _ in 0..n {
        let vote: usize = io.next();
        votes.push(vote);
    }

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
