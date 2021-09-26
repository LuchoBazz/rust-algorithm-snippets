use std::collections::BinaryHeap;
use std::cmp::{Reverse, Ordering};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct WR {
    val : i32,
    idx : usize
}

impl Ord for WR {
    fn cmp(&self, other: &Self) -> Ordering {
        // use .reverse() for min or Reverse(self.any)
        (self.val, self.idx).cmp(&(other.val, other.idx)).reverse()
    }
}

impl PartialOrd for WR {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for WR {
    fn eq(&self, other: &Self) -> bool {
        (self.val, self.idx) == (other.val, other.idx)
    }
}