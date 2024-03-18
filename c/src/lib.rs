use async_trait::async_trait;

#[async_trait]
pub trait Adder {
    async fn add(&self, a: usize, b: usize) -> usize;
}

pub struct AddMachine {}

#[async_trait]
impl Adder for AddMachine {
    async fn add(&self, a: usize, b: usize) -> usize {
        tokio::task::spawn(async move {
            return a + b + a
        }).await.unwrap()
    }
}
