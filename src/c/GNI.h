// Copyright (c) Meta Platforms, Inc. and affiliates.
// All rights reserved.

// This source code is licensed under the license found in the
// LICENSE file in the root directory of this source tree.

#pragma once

#ifdef __cplusplus
#include <cstdio>

extern "C" {
#endif
const char* c_get_gpu_node_id();
#ifdef __cplusplus
}
#endif
