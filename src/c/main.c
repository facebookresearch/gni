#include <stdio.h>

extern char* c_get_gpu_node_id();

int main() {
    char* gpu_node_id = c_get_gpu_node_id();
    printf("%s\n", gpu_node_id);
    return 0;
}
