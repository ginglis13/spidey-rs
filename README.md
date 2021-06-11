# spidey-rs

A rewrite of the infamous [Spidey webserver](https://www3.nd.edu/~pbui/teaching/cse.20289.sp18/project.html) from Notre Dame's [Systems Programming Course](https://www3.nd.edu/~pbui/teaching/cse.20289.sp18/) in Rust.

## Build and Run

`cargo build`

All logs:

`RUST_LOG=spidey_rs ./target/debug/spidey-rs`

Just info (and error) logs:

`RUST_LOG=info ./target/debug/spidey-rs`