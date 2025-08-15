use crate::utils::io::FastIO;

pub fn solve(io: &mut FastIO) {
    let s: String = io.next();
    let t: String = io.next();

    if s == t.chars().rev().collect::<String>() {
        println!("YES");
    } else {
        println!("NO");
    }
}
