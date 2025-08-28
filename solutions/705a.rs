// solution for problem 705A
use std::io::{self, Read};

/*
input
1
output
I hate it
input
2
output
I hate that I love it
input
3
output
I hate that I love that I hate it
*/

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
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
