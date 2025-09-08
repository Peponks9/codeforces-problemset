use std::io::{self, BufRead};

fn count_killed_days(k: i32, l: i32, m: i32, n: i32, d: i32) -> i32 {
    let mut count = 0;
    for day in 1..=d {
        if day % k == 0 || day % l == 0 || day % m == 0 || day % n == 0 {
            count += 1;
        }
    }
    count
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();

    let k: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let l: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let m: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let d: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let result = count_killed_days(k, l, m, n, d);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        // Sample: k=2, l=3, m=4, n=5, d=24 -> 17
        assert_eq!(count_killed_days(2, 3, 4, 5, 24), 17);
    }

    #[test]
    fn test_no_kills() {
        // All large numbers, d small -> 0
        assert_eq!(count_killed_days(100, 200, 300, 400, 10), 0);
    }

    #[test]
    fn test_all_days_killed() {
        // k=1 -> every day killed
        assert_eq!(count_killed_days(1, 2, 3, 4, 12), 12);
    }

    #[test]
    fn test_edge_case_d_one() {
        // d=1, k=1 -> 1
        assert_eq!(count_killed_days(1, 2, 3, 4, 1), 1);
    }
}
