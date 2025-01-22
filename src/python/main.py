# Copyright (c) Meta Platforms, Inc. and affiliates.
# All rights reserved.

# This source code is licensed under the license found in the
# LICENSE file in the root directory of this source tree.

import gni_lib

def main():
    gpu_node_id = gni_lib.get_gpu_node_id()
    print(f"{gpu_node_id=}")


if __name__ == "__main__":
    main()
