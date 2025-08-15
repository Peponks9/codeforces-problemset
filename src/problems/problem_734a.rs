use crate::utils::io::FastIO;

pub fn solve(io: &mut FastIO) {
    let n: i32 = io.next();
    let s: String = io.next();
    let result = anton_or_danik(&s);
    println!("{}", result);
}

fn anton_or_danik(s: &str) -> String {
    let mut anton_count = 0;
    let mut danik_count = 0;

    for c in s.chars() {
        if c == 'A' {
            anton_count += 1;
        } else if c == 'D' {
            danik_count += 1;
        }
    }
    if anton_count > danik_count {
        return "Anton".to_string();
    } else if danik_count > anton_count {
        return "Danik".to_string();
    } else {
        return "Friendship".to_string();
    }
}
