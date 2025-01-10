#include "mod.rs.h"
#include "GNI.h"

std::string cpp_get_gpu_node_id() noexcept {
  // Convert from rust::String to cpp string
  rust::String rust_str = cxx_get_gpu_node_id();
  std::string cpp_str(rust_str.data(), rust_str.size());
  return cpp_str;
}
