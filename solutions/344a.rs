// solution for problem 344A
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lines = input.lines().collect::<Vec<_>>();
    let n: usize = lines[0].parse().unwrap();
    let magnets = &lines[1..=n];
    let result = count_magnet_groups(magnets);
    println!("{}", result);
}

fn count_magnet_groups(magnets: &[&str]) -> i32 {
    let mut groups = 1;
    let mut second_pole = magnets[0].chars().nth(1).unwrap();

    for magnet in magnets.iter().skip(1) {
        let current_first_pole = magnet.chars().next().unwrap();
        let current_second_pole = magnet.chars().nth(1).unwrap();
        if current_first_pole == second_pole {
            groups += 1;
        }
        second_pole = current_second_pole;
    }

    groups
}
