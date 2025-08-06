use crate::utils::io::FastIO;

pub fn solve(io: &mut FastIO) {
    let n: i32 = io.next();
    let k: i32 = io.next();

    let result = wrong_subtraction(n, k);
    println!("{}", result);
}

fn wrong_subtraction(n: i32, k: i32) -> i32 {
    let mut n = n;
    for _ in 0..k {
        if n % 10 == 0 {
            n /= 10;
        } else {
            n -= 1;
        }
    }
    n
}
