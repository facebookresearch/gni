// Copyright (c) Meta Platforms, Inc. and affiliates.
// All rights reserved.

// This source code is licensed under the license found in the
// LICENSE file in the root directory of this source tree.

use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn rust_get_gpu_node_id() -> *mut c_char {
    let message: String = crate::get_gpu_node_id(None).unwrap();

    // Convert Rust String to CString
    let c_string: CString = match CString::new(message) {
        Ok(c_str) => c_str,
        Err(_) => return std::ptr::null_mut(),
    };

    // Consumes the CString and transfers pointer ownership to the caller.
    // Not calling CString::from_raw will lead to a memory leak. See rust_free_ptr below.
    // source: https://doc.rust-lang.org/std/ffi/struct.CString.html#method.into_raw
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn rust_free_ptr(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }

    // Convert the raw pointer back into a CString and drop it.
    // source: https://doc.rust-lang.org/std/ffi/struct.CString.html#method.from_raw
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}
