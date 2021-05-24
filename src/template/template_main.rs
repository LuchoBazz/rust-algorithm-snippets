#![allow(warnings, unused)]
use std::cmp::{min, max};
use std::io::Write;

fn solveOne<W: Write>(scan : &mut Scanner, out : &mut W) -> () {
    
}

fn main() {
    let stdout = std::io::stdout();
    let mut scan = Scanner::default();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let t : usize = scan.next();
    for _ in 0..t {
        solveOne(&mut scan, &mut out);
    }
}

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}