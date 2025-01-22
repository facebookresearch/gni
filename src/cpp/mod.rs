// Copyright (c) Meta Platforms, Inc. and affiliates.
// All rights reserved.

// This source code is licensed under the license found in the
// LICENSE file in the root directory of this source tree.

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
