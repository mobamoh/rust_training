fn main() {
    let nums: Vec<i64> = Counter::new(10).collect();
    println!("{nums:?}")
}

struct Counter {
    count: i64,
    max: i64,
}

impl Counter {
    fn new(max: i64) -> Self {
        Self { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

impl ExactSizeIterator for Counter {
    fn len(&self) -> usize {
        self.max as usize
    }
}
