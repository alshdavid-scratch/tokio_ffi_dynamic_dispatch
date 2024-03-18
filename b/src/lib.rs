use std::{future::Future, pin::Pin};
use c::Adder;

#[no_mangle]
pub extern fn init() -> Box<Pin<Box<dyn Future<Output = Adder>>>> {
    return Box::new(Box::pin(async move {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                Adder{}
            })
    }))
}

async fn add_async(left: usize, right: usize) -> usize {
    tokio::task::spawn(async move {
        left + right
    }).await.unwrap()
}