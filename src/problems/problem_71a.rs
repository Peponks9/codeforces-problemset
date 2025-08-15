use crate::utils::io::FastIO;

pub fn solve(io: &mut FastIO) {
    let n: i32 = io.next();
    for _ in 0..n {
        let word: String = io.next();
        println!("{}", abbreviation(&word));
    }
}

fn abbreviation(word: &str) -> String {
    if word.len() <= 10 {
        word.to_string()
    } else {
        let first_char = word.chars().next().unwrap();
        let last_char = word.chars().last().unwrap();
        let middle_length = word.len() - 2;
        format!("{}{}{}", first_char, middle_length, last_char)
    }
}
