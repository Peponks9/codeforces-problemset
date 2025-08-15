use crate::utils::io::FastIO;
// problem 266b solution/266b.rs

pub fn solve(io: &mut FastIO) {
    let n: usize = io.next();
    let t: usize = io.next();
    let mut queue: Vec<char> = io.next::<String>().chars().collect();

    for _ in 0..t {
        let mut i = 0;
        while i < n - 1 {
            if queue[i] == 'B' && queue[i + 1] == 'G' {
                queue.swap(i, i + 1);
                i += 1; // skip next position as it is now occupied by a boy
            }
            i += 1;
        }
    }

    // output the final arrangement
    let result: String = queue.iter().collect();
    println!("{}", result);
}
