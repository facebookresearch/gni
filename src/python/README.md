# GNI - Python pkg

GNI uses [pyo3](https://pyo3.rs/main/getting-started.html?#installation) to
write a native python library, and
[maturin](https://pyo3.rs/main/getting-started.html?#building) is the
recommended build tool.

## Building

1. Install maturin (see
   [maturin installation guide](https://www.maturin.rs/installation))

   ```
   pip install maturin
   ```

1. Builds the package
   ```
   maturin build
   ```

> You can view the built wheels in the target/wheels directory

## Installation

1. Builds the crate and installs it in the current env

   ```
   maturin develop --features python
   ```

1. Run gni
   ```
   python src/python/main.py
   ```
