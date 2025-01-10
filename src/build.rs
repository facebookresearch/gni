use cxx_build;

fn main() {
    cxx_build::bridge("src/cpp/mod.rs")
        .include("src/cpp")
        .flag("-std=c++17")
        .compile("cxx");

    println!("cargo:rerun-if-changed=src/cpp/mod.rs");
    println!("cargo:rerun-if-changed=src/cpp/GNI.h");
}
