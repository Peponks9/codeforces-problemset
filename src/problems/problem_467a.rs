use crate::utils::io::FastIO;

// solution for problem 467A
pub fn solve(io: &mut FastIO) {
    let n: u32 = io.next();
    let mut rooms = 0;
    const GEORGE_AND_ALEX: u32 = 2;

    for _ in 0..n {
        let p: u32 = io.next();
        let q: u32 = io.next();
        rooms += if p + GEORGE_AND_ALEX <= q { 1 } else { 0 };
    }

    println!("{}", rooms);
}
