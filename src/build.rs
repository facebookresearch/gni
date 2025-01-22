fn main() {
    if std::env::var("CARGO_FEATURE_CPP").is_ok() {
        // C++ build
        #[cfg(feature = "cpp")]
        use cxx_build;

        #[cfg(feature = "cpp")]
        cxx_build::bridge("src/cpp/mod.rs")
            .include("src/cpp")
            .flag("-std=c++17")
            .compile("cxx");

        println!("cargo:rerun-if-changed=src/cpp/mod.rs");
        println!("cargo:rerun-if-changed=src/cpp/GNI.h");
    } else {
        // C build
        println!("cargo:rerun-if-changed=src/c/GNI.c");
        println!("cargo:rerun-if-changed=src/c/GNI.h");
    }
}