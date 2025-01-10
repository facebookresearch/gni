# gni - c

## Building

1. Build, and compile gni:

   ```
   make compile_c
   ```

1. Build, compile and run gni:

   ```
   make run_c
   ```

1. Release mode:

   For all make cmds you can also pass `BUILD_MODE=release` for
   [rust's release mode](https://doc.rust-lang.org/book/ch14-01-release-profiles.html#customizing-builds-with-release-profiles):

   ```
   make run_c BUILD_MODE=release
   ```

1. Remove build artifacts:
   ```
   make clean
   ```

## Usage

### Binary

    ```
    $ make compile_c
    $ ./main_c
    <ID>
    ```

### c library

There are 2 steps to make use of GNI on an existing c project:

1. Add a dependency on `c_get_gpu_node_id`. See how [main.c](./main.c) depends
   on `c_get_gpu_node_id` from [GNI.c](./GNI.c).

1. You'll need a similar linking to the one done on [Makefile](../../Makefile),
   see "C Targets" section.
