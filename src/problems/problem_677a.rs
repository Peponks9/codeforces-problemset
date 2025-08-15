use crate::utils::io::FastIO;

pub fn solve(io: &mut FastIO) {
    let n: usize = io.next();
    let mut stops = Vec::new();
    for _ in 0..n {
        let a: i32 = io.next();
        let b: i32 = io.next();
        stops.push((a, b));
    }
    // logic: a: passengers leave, b: passengers enter
    let mut current_passengers = 0;
    let mut max_passengers = 0;

    for (a, b) in stops {
        current_passengers += b - a;
        max_passengers = max_passengers.max(current_passengers);
    }

    println!("{}", max_passengers);
}
