#![allow(warnings, unused)]
/**
 * @created     : `!v strftime("%B %d, %Y")`
 * @handle      : 🇨🇴 @SorKierkegaard
 */

use std::cmp::{min, max};
use std::io::Write;
use std::collections::{BTreeSet, BTreeMap, HashSet, HashMap, BinaryHeap, VecDeque};

fn solve_one<W: Write>(sc: &mut Scanner, out: &mut W) -> () {
    
}

fn main() {
    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new(std::io::stdin().lock());
    input!(sc=sc, t: usize);
    for _ in 1..=t {
        solve_one(&mut sc, &mut out);
    }
}

// reference: https://qiita.com/takeda_SE/items/65fd9f2985beb627bcf2
#[macro_export]
macro_rules! input{
    (sc=$sc:expr,$($r:tt)*)=>{ input_inner!{$sc,$($r)*} };
    ($($r:tt)*)=>{let mut sc=fast_input::Scanner::new(std::io::stdin().lock());input_inner!{sc,$($r)*}};
}
#[macro_export]
macro_rules! input_inner{
    ($sc:expr)=>{};
    ($sc:expr,)=>{};
    ($sc:expr,$var:ident:$t:tt$($r:tt)*)=>{let $var=read_value!($sc,$t);input_inner!{$sc $($r)*}};
    ($sc:expr,mut $var:ident:$t:tt$($r:tt)*)=>{let mut $var=read_value!($sc,$t);input_inner!{$sc $($r)*}};
}
#[macro_export]
macro_rules! read_value{
    ($sc:expr,($($t:tt),*))=>{($(read_value!($sc,$t)),*)};
    ($sc:expr,[$t:tt;$len:expr])=>{(0..$len).map(|_|read_value!($sc,$t)).collect::<Vec<_>>()};
    ($sc:expr,Chars)=>{read_value!($sc,String).chars().collect::<Vec<char>>()};
    ($sc:expr,Usize1)=>{read_value!($sc,usize)-1};
    ($sc:expr,$t:ty)=>{$sc.next::<$t>()};
}
pub struct Scanner {s: Box<str>, input: std::iter::Peekable<std::str::SplitAsciiWhitespace<'static>>,}
impl Scanner {
    pub fn new<R: std::io::Read>(mut reader: R) -> Self {let s = {let mut s = String::new();reader.read_to_string(&mut s).unwrap();s.into_boxed_str()};let mut sc = Scanner {s,input: "".split_ascii_whitespace().peekable(),};use std::mem;let s: &'static str = unsafe { mem::transmute(&*sc.s) };sc.input = s.split_ascii_whitespace().peekable();sc}
    #[inline]
    pub fn next<T: std::str::FromStr>(&mut self) -> T where T::Err: std::fmt::Debug,{self.input.next().unwrap().parse::<T>().expect("Parse Error")}
}
#[macro_export]
macro_rules! debug {
    ($($a:expr),*) => {
        #[cfg(debug_assertions)]
        writeln!(&mut std::io::stderr(), concat!("[DEBUG] ", $(stringify!($a), "={:?} "),*), $($a),*);
    }
}
