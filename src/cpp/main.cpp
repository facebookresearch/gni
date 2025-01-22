// Copyright (c) Meta Platforms, Inc. and affiliates.
// All rights reserved.

// This source code is licensed under the license found in the
// LICENSE file in the root directory of this source tree.

#include <iostream>
#include "GNI.h"

int main() {
    std::string gpu_node_id = cpp_get_gpu_node_id();
    std::cout << gpu_node_id << std::endl;
    return 0;
}
