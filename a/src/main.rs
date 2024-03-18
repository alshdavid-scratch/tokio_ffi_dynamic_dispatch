fn main() {
    let exe_path = std::env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let lib_path = exe_dir.join("libb.so");

    unsafe {
        let lib = libloading::Library::new(&lib_path).unwrap();
        let add_fn: libloading::Symbol<unsafe fn(usize, usize) -> usize> = lib.get(b"add").unwrap();
        println!("{}", add_fn(1, 1));
    }
}
