#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct MinMaxHeap {
    values: std::collections::BTreeMap<i32, usize>,
    size: usize,
}

impl Default for MinMaxHeap {
    fn default() -> Self {
        Self {
            values: BTreeMap::new(),
            size: 0usize,
        }
    }
}

impl From<&Vec<i32>> for MinMaxHeap {
    fn from(values: &Vec<i32>) -> Self {
        let mut to_return = Self::default();
        for val in values.iter() {
            *to_return.values.entry(*val).or_insert(0) += 1;
            to_return.size += 1;
        }
        return to_return;
    }
}

impl MinMaxHeap {
    fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, value: i32) -> () {
        *self.values.entry(value).or_insert(0) += 1;
        self.size += 1;
    }

    fn get_max(&self) -> i32 {
        assert!(!self.is_empty());
        let it: (&i32, &usize) = self.values.iter().next_back().unwrap();
        *it.0
    }

    fn get_min(&self) -> i32 {
        assert!(!self.is_empty());
        let it: (&i32, &usize) = self.values.iter().next().unwrap();
        *it.0
    }

    fn count_min(&self) -> usize {
        let mut option: Option<&usize> = self.values.get(&self.get_min());
        if let Some(val) = option {
            return *val;
        }
        0usize
    }

    fn count_max(&self) -> usize {
        let mut option: Option<&usize> = self.values.get(&self.get_max());
        if let Some(val) = option {
            return *val;
        }
        0usize
    }

    fn count(&self, key: i32) -> usize {
        let mut option: Option<&usize> = self.values.get(&key);
        if let Some(val) = option {
            return *val;
        }
        0usize
    }

    fn pop_min(&mut self) -> i32 {
        assert!(!self.is_empty());
        let to_return = self.get_min();
        *self.values.entry(to_return).or_insert(0) -= 1;
        if *self.values.get(&to_return).unwrap() == 0 {
            self.values.remove(&to_return);
        }
        self.size -= 1;
        to_return
    }

    fn pop_max(&mut self) -> i32 {
        assert!(!self.is_empty());
        let to_return = self.get_max();
        *self.values.entry(to_return).or_insert(0) -= 1;
        if *self.values.get(&to_return).unwrap() == 0 {
            self.values.remove(&to_return);
        }
        self.size -= 1;
        to_return
    }

    fn remove(&mut self, value: i32) -> bool {
        if self.count(value) == 0 {
            return false;
        }
        *self.values.entry(value).or_insert(0) -= 1;
        if *self.values.get(&value).unwrap() == 0 {
            self.values.remove(&value);
        }
        self.size -= 1;
        return true;
    }

    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    fn size(&self) -> usize {
        self.size
    }

    fn clear(&mut self) -> () {
        self.values.clear();
        self.size = 0;
    }
}