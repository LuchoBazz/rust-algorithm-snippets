trait PrefixSum {
    fn build(&mut self);
    fn query(&self, left: usize, rigth: usize) -> i64;
}

impl PrefixSum for Vec<i64> {
    fn build(&mut self) {
        let mut prefix = vec![0; self.len() + 1];
        for i in 1..=self.len() {
            prefix[i] = prefix[i-1] + self[i-1];
        }
        *self = prefix;
    }

    fn query(&self, left: usize, right: usize) -> i64 {
        assert!(0 <= left && left <= right && right < (self.len()-1));
        self[right+1] - self[left]
    }
}