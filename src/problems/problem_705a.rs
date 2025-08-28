use crate::utils::io::FastIO;

pub fn solve(io: &mut FastIO) {
    let n: i32 = io.next();
    let result = feelings(n);
    println!("{}", result);
}

fn feelings(n: i32) -> String {
    let hate: String = "I hate ".to_string();
    let love: String = "I love ".to_string();
    let mut result = String::new();

    for i in 1..=n {
        if i % 2 == 1 {
            result.push_str(&hate);
        } else {
            result.push_str(&love);
        }
        if i == n {
            result.push_str("it");
        } else {
            result.push_str("that ");
        }
    }
    result
}
