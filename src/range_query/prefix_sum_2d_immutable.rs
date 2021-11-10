struct PrefixSum2D<T> {
    n: usize,
    m: usize,
    dp: Vec<Vec<T>>
}

impl<T> PrefixSum2D<T>
where
    T: Clone
        + std::marker::Copy
        + std::ops::Mul<Output = T>
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::convert::From<i32>, {

    fn new(grid: &Vec<Vec<T>>) -> Self {
        let mut prefix = Self {
            n: grid.len(),
            m: grid[0].len(),
            dp: Vec::new()
        };

        prefix.dp = vec![vec![T::from(0); prefix.m+1]; prefix.n+1];

        for i in 1..=prefix.n {
            for j in 1..=prefix.m {
                prefix.dp[i][j] = prefix.dp[i][j-1] + grid[i-1][j-1];
            }
        }

        for j in 1..=prefix.m {
            for i in 1..=prefix.n {
                prefix.dp[i][j] = prefix.dp[i][j] + prefix.dp[i-1][j];
            }
        }
        prefix
    }
    
    fn query(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> T {
        assert!(x1 < self.n && y1 < self.m);
        assert!(x2 < self.n && y2 < self.m);

        let SA = self.dp[x2+1][y2+1];
        let SB = self.dp[x1][y2+1];
        let SC = self.dp[x2+1][y1];
        let SD = self.dp[x1][y1];
        SA-SB-SC+SD
    }
}
// Usage: let mut prefix = PrefixSum2D { prefix: PrefixSum2D::new(&grid) };
// self.prefix.query(x1, y1, x2, y2);
