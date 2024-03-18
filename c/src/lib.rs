use std::{future::Future, pin::Pin};

pub trait Adder {
    fn add(&self, a: usize, b: usize) -> Box<Pin<Box<dyn Future<Output = usize>>>>;
}

pub struct AddMachine {}

impl Adder for AddMachine {
    fn add(&self, a: usize, b: usize) -> Box<Pin<Box<dyn Future<Output = usize>>>> {
        Box::new(Box::pin(async move {
            tokio::task::spawn(async move {
                return a + b + a
            }).await.unwrap()
        }))
    }
}
