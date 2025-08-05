// solution for problem 59A
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let s = input.trim();

    let result = convert_string(s);
    println!("{}", result);
}

fn convert_string(s: &str) -> String {
    let uppercase_count = s.chars().filter(|c| c.is_uppercase()).count();
    let lowercase_count = s.chars().filter(|c| c.is_lowercase()).count();

    if uppercase_count > lowercase_count {
        s.to_uppercase()
    } else {
        s.to_lowercase()
    }
}
