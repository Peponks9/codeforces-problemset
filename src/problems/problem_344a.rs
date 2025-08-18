use crate::utils::io::FastIO;

pub fn solve(io: &mut FastIO) {
    // solution to 344a.rs
    let n: usize = io.next();
    let mut magnets = Vec::new();

    for _ in 0..n {
        let magnet: String = io.next();
        magnets.push(magnet);
    }

    let result = count_magnet_groups(&magnets);
    println!("{}", result);
}

fn count_magnet_groups(magnets: &[String]) -> i32 {
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
