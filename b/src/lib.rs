use std::{future::Future, pin::Pin};
use c::Adder;
use c::AddMachine;

#[no_mangle]
pub extern fn init() -> Box<Pin<Box<dyn Future<Output = AddMachine>>>> {
    return Box::new(Box::pin(async move {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                AddMachine{}
            })
    }))
}
