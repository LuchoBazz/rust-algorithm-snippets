
// Reference: https://codeforces.com/contest/1523/submission/117891418
struct Random {
    state: usize
}
 
impl Random {
    fn next(&mut self) -> usize {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }
 
    fn next_in_range(&mut self, from: usize, to: usize) -> usize {
        assert!(from < to);
        from + self.next() % (to - from)
    }
 
    fn next_double(&mut self) -> f64 {
        (self.next() as f64) / (std::usize::MAX as f64)
    }
 
    fn new(seed: usize) -> Self {
        assert_ne!(seed, 0);
        Self {
            state: seed,
        }
    }
}
// let seed = std::time::SystemTime::now()
//            .duration_since(std::time::SystemTime::UNIX_EPOCH)
//            .unwrap().as_secs() as usize;
// let mut rand = Random::new(seed);
