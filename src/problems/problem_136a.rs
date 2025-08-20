use crate::utils::io::FastIO;
// problem 136A

pub fn solve(io: &mut FastIO) {
    let n: usize = io.next();
    let mut presents = vec![0; n];

    for i in 0..n {
        presents[i] = io.next();
    }

    let mut result = vec![0; n];

    for i in 0..n {
        let giver = i + 1; // Friend number (1-indexed)
        let receiver = presents[i]; // Who they gave to (1-indexed)
        result[receiver - 1] = giver; // Convert to 0-indexed for array
    }

    for i in 0..n {
        print!("{}", result[i]);
        if i < n - 1 {
            print!(" ");
        }
    }
    println!();
}
