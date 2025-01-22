# GNI - cpp

We're using [cxx](https://cxx.rs/#cxx--safe-interop-between-rust-and-c) to
expose cpp bindings.

## Building

1. Build, and compile gni:

   ```
   make compile_cpp
   ```

1. Build, compile and run gni:

   ```
   make run_cpp
   ```

1. Release mode:

   For all make cmds you can also pass `BUILD_MODE=release` for
   [rust's release mode](https://doc.rust-lang.org/book/ch14-01-release-profiles.html#customizing-builds-with-release-profiles):

   ```
   make run_cpp BUILD_MODE=release
   ```

1. Remove build artifacts:
   ```
   make clean
   ```

## Usage

### Binary

    ```
    $ make compile_cpp
    $ ./main_cpp
    <ID>
    ```

### cpp library

There are 2 steps to make use of GNI on an existing cpp project:

1. Add a dependency on `cpp_get_gpu_node_id`. See how [main.cpp](./main.cpp)
   depends on `cpp_get_gpu_node_id` from [GNI.cpp](./GNI.cpp).

1. You'll need a similar linking to the one done on [Makefile](../../Makefile).
