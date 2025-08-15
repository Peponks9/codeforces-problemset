// solution for problem 110A
// nealy lucky number: if n contains a lucky number of lucky
// numbers
use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n = input.trim();

    let result = nearly_lucky_number(n);
    println!("{}", result);
}

fn nearly_lucky_number(n: &str) -> String {
    let lucky_numbers = HashSet::from(['4', '7']);
    let mut lucky_count = 0;

    for c in n.chars() {
        if lucky_numbers.contains(&c) {
            lucky_count += 1;
        }
    }

    if lucky_count == 4 || lucky_count == 7 {
        return "YES".to_string();
    }
    return "NO".to_string();
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nearly_lucky_number() {
        assert_eq!(nearly_lucky_number("4747"), "YES");
        assert_eq!(nearly_lucky_number("7474"), "YES");
        assert_eq!(nearly_lucky_number("1001"), "NO");
    }
}
