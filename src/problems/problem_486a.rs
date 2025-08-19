use crate::utils::io::FastIO;

// solution for problem 486A
pub fn solve(io: &mut FastIO) {
    let n: i32 = io.next();
    let mut sum = 0;
    for i in 1..=n {
        if i % 2 == 0 {
            sum += i;
        } else {
            sum -= i;
        }
    }
    println!("{}", sum);
}
