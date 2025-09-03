use crate::utils::io::FastIO;

pub fn solve(io: &mut FastIO) {
    let n: usize = io.next();
    let s: String = io.next();
    if is_pangram(&s) {
        println!("YES");
    } else {
        println!("NO");
    }
}

fn is_pangram(s: &str) -> bool {
    let mut seen = std::collections::HashSet::new();
    for c in s.chars() {
        if c.is_alphabetic() {
            seen.insert(c.to_ascii_lowercase());
        }
    }
    seen.len() == 26
}
