use std::{future::Future, pin::Pin};

async fn main_async() {
    let exe_path = std::env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let lib_path = exe_dir.join("libb.so");

    unsafe {
        let lib = libloading::Library::new(&lib_path).unwrap();
        let add_fn: libloading::Symbol<unsafe fn(usize, usize) -> Box<Pin<Box<dyn Future<Output = usize>>>>> = lib.get(b"add").unwrap();
        println!("{}", add_fn(1, 1).await);
    }
}

pub fn main() {
    tokio::runtime::Builder::new_multi_thread()
      .enable_all()
      .build()
      .unwrap()
      .block_on(main_async());
}