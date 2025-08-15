use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    for _ in 0..n {
        let mut word = String::new();
        io::stdin().read_line(&mut word).unwrap();
        let word = word.trim();
        println!("{}", abbreviation(word));
    }
}

fn abbreviation(word: &str) -> String {
    if word.len() <= 10 {
        word.to_string()
    } else {
        let first_char = word.chars().next().unwrap();
        let last_char = word.chars().last().unwrap();
        let middle_length = word.len() - 2; // Number of letters between first and last
        format!("{}{}{}", first_char, middle_length, last_char)
    }
}
