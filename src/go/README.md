# GNI - go

GNI for Go uses a C library via cgo to bridge Rust functionality into Go.

## Building

1. Build, and compile gni:

   ```
   make build_go
   ```

1. Build, compile and run gni:

   ```
   make run_go
   ```

1. Release mode:

   For all make cmds you can also pass `BUILD_MODE=release` for
   [rust's release mode](https://doc.rust-lang.org/book/ch14-01-release-profiles.html#customizing-builds-with-release-profiles):

   ```
   make run_go BUILD_MODE=release
   ```

1. Remove build artifacts:
   ```
   make clean
   ```

## Usage

### Binary

    ```
    $ make build_go
    $ ./src/go/go
    <ID>
    ```

### go library

To install the pkg as a go library:

```
$ cd src/go && go install
$ go list -m gni/src/go
```
