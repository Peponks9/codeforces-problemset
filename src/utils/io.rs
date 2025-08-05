use std::io::{self, Read, Write};

pub struct FastIO {
    input: String,
    pos: usize,
}

impl FastIO {
    pub fn new() -> Self {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        Self { input, pos: 0 }
    }

    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        while self.pos < self.input.len() && self.input.as_bytes()[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
        let start = self.pos;
        while self.pos < self.input.len() && !self.input.as_bytes()[self.pos].is_ascii_whitespace()
        {
            self.pos += 1;
        }
        self.input[start..self.pos].parse().ok().unwrap()
    }
}

impl Default for FastIO {
    fn default() -> Self {
        Self::new()
    }
}
