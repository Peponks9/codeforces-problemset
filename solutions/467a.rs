// solution for problem 467A

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let n: u32 = lines.next().unwrap().parse().unwrap();

    let mut rooms = Vec::new(); // Declare rooms as a vector first

    for _ in 0..n {
        let room: Vec<u32> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        rooms.push((room[0], room[1]));
    }

    let mut available_rooms = 0; // Rename to avoid conflict
    const GEORGE_AND_ALEX: u32 = 2;

    // iterate over data structure that stores p and q
    for (p, q) in rooms {
        available_rooms += if p + GEORGE_AND_ALEX <= q { 1 } else { 0 };
    }

    println!("{}", available_rooms);
}
