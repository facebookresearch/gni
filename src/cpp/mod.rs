#[cxx::bridge]
mod ffi {
    // Expose functions to cpp
    extern "Rust" {
        fn cxx_get_gpu_node_id() -> String;
    }
}

pub fn cxx_get_gpu_node_id() -> String {
    crate::get_gpu_node_id(None).unwrap()
}
