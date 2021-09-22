
pub mod dsu {
    pub struct DisjointSet {
        pub n: usize,
        pub comps: usize,
        pub parents: Vec<usize>,
        pub sizes: Vec<usize>,
        pub mx: usize
    }

    impl DisjointSet {
        pub fn new(n: usize) -> DisjointSet {
            DisjointSet {
                n: n,
                comps: n,
                parents: (0..n).collect(),
                sizes: vec![1; n],
                mx: 1usize
            }
        }

        pub fn find(&mut self, x: usize) -> usize {
            assert!(0 <= x && x < self.n);
            if self.parents[x] != x {
                self.parents[x] = self.find(self.parents[x]);
            }
            self.parents[x].clone()
        }

        pub fn unite(&mut self, left: usize, right: usize) -> bool {
            assert!(0 <= left && left < self.n);
            assert!(0 <= right && right < self.n);
            let mut x = self.find(left);
            let mut y = self.find(right);
            if x == y {
                return false;
            }
            if self.sizes[x] < self.sizes[y] {
                std::mem::swap(&mut x, &mut y);
            }
            self.sizes[x] += self.sizes[y];
            self.mx = std::cmp::max(self.mx, self.sizes[x]);
            self.parents[y] = x;
            self.comps -= 1;
            true
        }

        pub fn united(&mut self, left: usize, right: usize) -> bool {
            assert!(0 <= left && left < self.n);
            assert!(0 <= right && right < self.n);
            self.find(left) == self.find(right)
        }

        pub fn size(&mut self, x: usize) -> usize {
            assert!(0 <= x && x < self.n);
            let parent = self.find(x);
            self.sizes[parent].clone()
        }

        pub fn id(&mut self, x: usize) -> usize {
            assert!(0 <= x && x < self.n);
            let parent = self.find(x);
            self.parents[parent]
        }

        pub fn groups(&mut self) -> Vec<Vec<usize>> {
            let mut parent_id = vec![0usize; self.n];
            let mut group_size = vec![0usize; self.n];
            for i in 0..self.n {
                parent_id[i] = self.find(i);
                group_size[parent_id[i]] += 1;
            }
            let mut result = vec![Vec::new(); self.n];
            for i in 0..self.n {
                result[i].reserve(group_size[i]);
            }
            for i in 0..self.n {
                result[parent_id[i]].push(i);
            }
            result.retain(|x| !x.is_empty());
            result
        }
    }
}
use dsu::DisjointSet;
type DSU = DisjointSet;

// Usage: 
// let mut dsu = DSU::new(n);