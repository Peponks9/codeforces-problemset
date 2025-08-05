use crate::utils::io::FastIO;

pub fn solve(io: &mut FastIO) {
    // Read the input string
    let s: String = io.next();

    // Count uppercase and lowercase letters
    let uppercase_count = s.chars().filter(|c| c.is_uppercase()).count();
    let lowercase_count = s.chars().filter(|c| c.is_lowercase()).count();

    // Convert string based on which case has more letters
    let result = if uppercase_count > lowercase_count {
        s.to_uppercase()
    } else {
        s.to_lowercase()
    };

    println!("{}", result);
}
