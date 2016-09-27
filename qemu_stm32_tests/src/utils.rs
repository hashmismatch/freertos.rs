use prelude::v1::*;

pub struct Rnd { seed: u64 }
impl Rnd {
    pub fn new(seed: u64) -> Rnd {
        Rnd { seed: seed }
    }
    pub fn next(&mut self) -> u64 {
        let r = self.seed;
        self.seed = (1103515245 * r + 12345) % (1 << 31);
        r
    }
    pub fn next_num(&mut self, max: u64) -> u64 {
        self.next() % max
    }
    pub fn next_num_range(&mut self, min: u64, max: u64) -> u64 {
        let r = max - min;
        (self.next() % r) + min
    }
}
impl Iterator for Rnd {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        Some(self.next())
    }
}



#[test]
fn test_rnd() {
    let rnd = Rnd::new(1);
    let numbers: Vec<_> = rnd.take(1000).collect();
    assert_eq!([1, 1103527590, 377401575], &numbers[0..3]);
}
