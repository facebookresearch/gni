import gni_lib

def main():
    gpu_node_id = gni_lib.get_gpu_node_id()
    print(f"{gpu_node_id=}")


if __name__ == "__main__":
    main()
