use crate::utils::io::FastIO;

pub fn solve(io: &mut FastIO) {
    let n: usize = io.next();
    let mut percentages: Vec<i32> = (0..n).map(|_| io.next()).collect();

    for _ in 0..n {
        percentages.push(io.next());
    }

    // Calculate sum as float to get precise division
    let sum: f64 = percentages.iter().map(|&x| x as f64).sum();
    let average = sum / n as f64;

    println!("{}", average);
}
