mod problems;
mod utils;

use utils::io::FastIO;

fn main() {
    let mut io = FastIO::new();
    problems::problem_520a::solve(&mut io);
}
