# Contributing to gni

## Development Workflow

After cloning gni, and installing Rust ([rustup.rs](https://rustup.rs/)). You can run:

- `cargo fmt --all -- --check` checks code formatting
- `cargo clippy --all-features` [clippy](https://github.com/rust-lang/rust-clippy) runs a collection of lints
- `cargo build` builds the rust project
- `cargo run` runs the rust binary

For other supported languages, please refer to their respective READMEs:

- [c](/src/c/README.md)
- [cpp](/src/cpp/README.md)
- [go](/src/go/README.md)
- [python](/src/python/README.md)

## Codebase structure

- `src`[./src] Core Rust library
    - `c`[./src/c] C bindings
    - `cpp`[./src/cpp] Cpp bindings
    - `go`[./src/go] Go bindings
    - `python`[./src/python] Python bindings

## Pull Requests
We welcome your pull requests.

1. Fork the repo and create your feature branch from `main`.
1. If you've added code add suitable tests.
1. Ensure the test suite and lint pass.
1. If you haven't already, complete the Contributor License Agreement ("CLA").

## Contributor License Agreement ("CLA")
In order to accept your pull request, we need you to submit a CLA. You only need
to do this once to work on any of Facebook's open source projects.

Complete your CLA here: <https://code.facebook.com/cla>

## Issues
We use GitHub issues to track public bugs. Please ensure your description is
clear and has sufficient instructions to be able to reproduce the issue.

Facebook has a [bounty program](https://www.facebook.com/whitehat/) for the safe
disclosure of security bugs. In those cases, please go through the process
outlined on that page and do not file a public issue.

## License
By contributing to gni, you agree that your contributions will be licensed
under the LICENSE file in the root directory of this source tree.