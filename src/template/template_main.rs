/**
 * @created     : `!v strftime("%B %d, %Y")`
 * @handle      : 🇨🇴 @SorKierkegaard
 */

#![allow(warnings, unused)]
use std::cmp::{min, max};
use std::io::Write;

fn solve_one<W: Write>(scan : &mut Scanner, out : &mut W) -> () {
    
}

fn main() {
    let stdout = std::io::stdout();
    let mut scan = Scanner::default();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let t : usize = scan.next();
    for _ in 0..t {
        solve_one(&mut scan, &mut out);
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
#[macro_export]
macro_rules! debug {
    ($($a:expr),*) => {
        #[cfg(debug_assertions)]
        writeln!(&mut std::io::stderr(), concat!("[DEBUG] ", $(stringify!($a), "={:?} "),*), $($a),*);
    }
}