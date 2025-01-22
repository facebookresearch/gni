// Copyright (c) Meta Platforms, Inc. and affiliates.
// All rights reserved.

// This source code is licensed under the license found in the
// LICENSE file in the root directory of this source tree.

#include <stdio.h>

extern char* c_get_gpu_node_id();

int main() {
    char* gpu_node_id = c_get_gpu_node_id();
    printf("%s\n", gpu_node_id);
    return 0;
}
