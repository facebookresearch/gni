// Copyright (c) Meta Platforms, Inc. and affiliates.
// All rights reserved.

// This source code is licensed under the license found in the
// LICENSE file in the root directory of this source tree.

#include "GNI.h"
#include <stdlib.h>
#include <string.h>

extern char* rust_get_gpu_node_id();
extern void rust_free_ptr(char*);

const char* c_get_gpu_node_id() {
    char* rust_string = rust_get_gpu_node_id();
    if (!rust_string) {
        return NULL;
    }
    // Copy the string and free the original string.
    // source: https://en.cppreference.com/w/c/experimental/dynamic/strdup
    const char* result = strdup(rust_string);
    rust_free_ptr(rust_string);

    // Caller is responsible for freeing the result.
    return result;
}
