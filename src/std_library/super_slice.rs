// Reference: https://codeforces.com/contest/1558/submission/126988527
pub trait super_slice {
    type Item;
    fn lower_bound(&self, key: &Self::Item) -> usize
    where
        Self::Item: Ord;
    fn lower_bound_by<F>(&self, f: F) -> usize
    where
        F: FnMut(&Self::Item) -> std::cmp::Ordering;
    fn lower_bound_by_key<K, F>(&self, key: &K, f: F) -> usize
    where
        K: Ord,
        F: FnMut(&Self::Item) -> K;
    fn upper_bound(&self, key: &Self::Item) -> usize
    where
        Self::Item: Ord;
    fn upper_bound_by<F>(&self, f: F) -> usize
    where
        F: FnMut(&Self::Item) -> std::cmp::Ordering;
    fn upper_bound_by_key<K, F>(&self, key: &K, f: F) -> usize
    where
        K: Ord,
        F: FnMut(&Self::Item) -> K;
}

impl<T> super_slice for [T] {
    type Item = T;
    fn lower_bound(&self, key: &Self::Item) -> usize
    where
        T: Ord,
    {
        self.lower_bound_by(|p| p.cmp(key))
    }
    fn lower_bound_by<F>(&self, mut f: F) -> usize
    where
        F: FnMut(&Self::Item) -> std::cmp::Ordering,
    {
        self.binary_search_by(|p| f(p).then(std::cmp::Ordering::Greater))
            .unwrap_err()
    }
    fn lower_bound_by_key<K, F>(&self, key: &K, mut f: F) -> usize
    where
        K: Ord,
        F: FnMut(&Self::Item) -> K,
    {
        self.lower_bound_by(|p| f(p).cmp(key))
    }
    fn upper_bound(&self, key: &Self::Item) -> usize
    where
        T: Ord,
    {
        self.upper_bound_by(|p| p.cmp(key))
    }
    fn upper_bound_by<F>(&self, mut f: F) -> usize
    where
        F: FnMut(&Self::Item) -> std::cmp::Ordering,
    {
        self.binary_search_by(|p| f(p).then(std::cmp::Ordering::Less))
            .unwrap_err()
    }
    fn upper_bound_by_key<K, F>(&self, key: &K, mut f: F) -> usize
    where
        K: Ord,
        F: FnMut(&Self::Item) -> K,
    {
        self.upper_bound_by(|p| f(p).cmp(key))
    }
}

#[cfg(test)]
mod tests {
    use crate::super_slice;
    #[test]
    fn test_lower_bound() {
        let mut v = vec![1, 2, 3, 3, 3, 3, 4, 5, 7, 10, 20];
        let it = v.lower_bound(&-100);
        assert_eq!(it, 0usize);
        let it = v.lower_bound(&100);
        assert_eq!(it, 11usize);
        let it = v.lower_bound(&1);
        assert_eq!(it, 0usize);
        let it = v.lower_bound(&4);
        assert_eq!(it, 6usize);
        let it = v.lower_bound(&6);
        assert_eq!(it, 8usize);
        let it = v.lower_bound(&3);
        assert_eq!(it, 2usize);
    }

    #[test]
    fn test_upper_bound() {
        let mut v = vec![1, 2, 3, 3, 3, 3, 4, 5, 7, 10, 20];
        let it = v.upper_bound(&-100);
        assert_eq!(it, 0usize);
        let it = v.upper_bound(&100);
        assert_eq!(it, 11usize);
        let it = v.upper_bound(&1);
        assert_eq!(it, 1usize);
        let it = v.upper_bound(&4);
        assert_eq!(it, 7usize);
        let it = v.upper_bound(&6);
        assert_eq!(it, 8usize);
        let it = v.upper_bound(&3);
        assert_eq!(it, 6usize);
    }
}