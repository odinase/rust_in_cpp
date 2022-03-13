// build.rs

fn main() {
    cxx_build::bridge("src/lib.rs")  // returns a cc::Build
        .file("hello.cpp")
        .flag_if_supported("-std=c++17")
        .compile("cxxbridge");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=hello.cpp");
    // println!("cargo:rerun-if-changed=include/demo.h");
}