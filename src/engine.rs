/// Quantitative trading engine
pub struct Engine {
    pub left: u64,
}
impl Engine {
    pub fn new() -> Engine {
        Engine { left: 0 }
    }

    pub fn add(&mut self, val: u64) {
        self.left += 1;
    }
}
