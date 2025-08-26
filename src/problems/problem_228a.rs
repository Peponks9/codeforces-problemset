use crate::utils::io::FastIO;
use std::collections::HashSet;

pub fn solve(io: &mut FastIO) {
    let mut horseshoes: Vec<u32> = Vec::new();
    for _ in 0..4 {
        horseshoes.push(io.next());
    }
    let unique: HashSet<_> = horseshoes.iter().collect();
    let to_buy = 4 - unique.len();
    println!("{}", to_buy);
}
