use std::io;

fn main() {
    let mut input = String::new();

    // Read number of friends
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    // Clear input buffer for next line
    input.clear();

    // Read the presents array
    io::stdin().read_line(&mut input).unwrap();
    let presents: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Create result array
    let mut result = vec![0; n];

    // For each friend i (1-indexed), they gave a present to friend presents[i-1]
    // So friend presents[i-1] received a present from friend i
    for i in 0..n {
        let giver = i + 1; // Friend number (1-indexed)
        let receiver = presents[i]; // Who they gave to (1-indexed)
        result[receiver - 1] = giver; // Convert to 0-indexed for array
    }

    // Print the result
    for i in 0..n {
        print!("{}", result[i]);
        if i < n - 1 {
            print!(" ");
        }
    }
    println!();
}
