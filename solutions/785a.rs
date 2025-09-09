// solution for problem 785A
// Anton and Polyhedrons
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let n: i32 = lines.next().unwrap().trim().parse().unwrap();
    let mut total_faces = 0;
    for _ in 0..n {
        let shape = lines.next().unwrap().trim();
        total_faces += count_faces(shape);
    }
    println!("{}", total_faces);
}

fn count_faces(shape: &str) -> i32 {
    match shape {
        "Tetrahedron" => 4,
        "Cube" => 6,
        "Octahedron" => 8,
        "Dodecahedron" => 12,
        "Icosahedron" => 20,
        _ => 0,
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_total_faces() {
        let shapes = vec!["Icosahedron", "Cube", "Tetrahedron", "Dodecahedron"];
        let expected_faces = vec![20, 6, 4, 12]; // Fixed: corrected to match shapes (removed extra 4, now sums to 42)
        let total: i32 = shapes.iter().map(|s| count_faces(s)).sum();
        assert_eq!(total, expected_faces.iter().sum());
    }
}
