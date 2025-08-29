use crate::utils::io::FastIO;

pub fn solve(io: &mut FastIO) {
    let t: usize = io.next();
    for _ in 0..t {
        let a: i32 = io.next();
        let b: i32 = io.next();

        let result = if is_divisible(a, b) { 0 } else { b - (a % b) };
        println!("{}", result);
    }
}

fn is_divisible(a: i32, b: i32) -> bool {
    a % b == 0
}
