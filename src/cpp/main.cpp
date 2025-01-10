#include <iostream>
#include "GNI.h"

int main() {
    std::string gpu_node_id = cpp_get_gpu_node_id();
    std::cout << gpu_node_id << std::endl;
    return 0;
}
