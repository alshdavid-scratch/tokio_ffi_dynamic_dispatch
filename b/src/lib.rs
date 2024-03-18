use std::{future::Future, pin::Pin};

#[no_mangle]
pub extern fn add(left: usize, right: usize) -> Box<Pin<Box<dyn Future<Output = usize>>>> {
    return Box::new(Box::pin(async move {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(add_async(left, right))
    }))
}

async fn add_async(left: usize, right: usize) -> usize {
    tokio::task::spawn(async move {
        left + right
    }).await.unwrap()
}