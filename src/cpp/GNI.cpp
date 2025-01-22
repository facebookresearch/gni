// Copyright (c) Meta Platforms, Inc. and affiliates.
// All rights reserved.

// This source code is licensed under the license found in the
// LICENSE file in the root directory of this source tree.

#include "mod.rs.h"
#include "GNI.h"

std::string cpp_get_gpu_node_id() noexcept {
  // Convert from rust::String to cpp string
  rust::String rust_str = cxx_get_gpu_node_id();
  std::string cpp_str(rust_str.data(), rust_str.size());
  return cpp_str;
}
