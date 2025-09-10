// solution for problem 443A
use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let s = input.trim();

    let result = unique_characters(s);
    println!("{}", result);
}

fn unique_characters(s: &str) -> i32 {
    let unique_chars: HashSet<char> = s.chars().filter(|c| c.is_alphabetic()).collect();
    unique_chars.len() as i32
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_characters() {
        assert_eq!(unique_characters("{a, b, c}"), 3);
        assert_eq!(unique_characters("{b, a, b, a}"), 2);
        assert_eq!(unique_characters("{}"), 0);
    }
}
