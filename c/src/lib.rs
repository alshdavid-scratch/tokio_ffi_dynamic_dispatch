pub struct Adder {}

impl Adder {
    pub async fn add(&self, a: usize, b: usize) -> usize {
        return a + b + a
    }
}
