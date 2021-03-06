use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::generate(crate_dir).unwrap()
    .write_to_file("./include/rust_interface.h");
}