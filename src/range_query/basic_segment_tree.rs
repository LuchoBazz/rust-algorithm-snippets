mod segtree {
    pub struct SegTree<T, F> {
        n: usize,
        neutral: T,
        tree: Vec<T>,
        op: F,
    }

    impl<T, F> SegTree<T, F>
    where
        T: Clone,
        F: Fn(&T, &T) -> T,
    {
        pub fn new(vec: &Vec<T>, neutral: T, op: F) -> SegTree<T, F> {
            let mut seg = SegTree {
                n: vec.len(),
                tree: vec![neutral.clone(); 4 * vec.len()],
                neutral: neutral.clone(),
                op: op,
            };
            for i in 0..seg.n {
                seg.tree[seg.n + i] = vec[i].clone();
            }
            for i in (1..=seg.n - 1).rev() {
                seg.tree[i] = (seg.op)(&seg.tree[i << 1], &seg.tree[i << 1 | 1]);
            }
            seg
        }

        pub fn modify(&mut self, index: usize, value: T) {
            self.tree[index + self.n] = value;
            let mut idx = index.clone() + self.n;
            while idx > 1 {
                self.tree[idx>>1] = (self.op)(&self.tree[idx], &self.tree[idx ^ 1]);
                idx >>= 1;
            }
        }

        pub fn query(&self, mut l: usize, mut r: usize) -> T {
            let mut ans = self.neutral.clone();
            l += self.n;
            r += self.n + 1;
            while l < r {
                if (l & 1) == 1 {
                    ans = (self.op)(&ans, &self.tree[l]);
                    l += 1;
                }
                if (r & 1) == 1 {
                    r -= 1;
                    ans = (self.op)(&ans, &self.tree[r]);
                }
                l >>= 1;
                r >>= 1;
            }
            ans
        }
    }
}

use segtree::*;

// Usage:
// let mut seg = SegTree::new(&v, 0, |x, y| *x + *y);