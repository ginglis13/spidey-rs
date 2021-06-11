# spidey-rs

A rewrite of the infamous [Spidey webserver](https://www3.nd.edu/~pbui/teaching/cse.20289.sp18/project.html) from Notre Dame's [Systems Programming Course](https://www3.nd.edu/~pbui/teaching/cse.20289.sp18/) in Rust.

Unix only. Uses the [nix crate](https://docs.rs/nix/0.13.0/nix/unistd/fn.fork.html) in order to use the `fork` system call, so this won't run on Windows. This is intentional as the original project only compiled for Unix.

## Build and Run

`cargo build`

All logs:

`RUST_LOG=spidey_rs ./target/debug/spidey-rs`

Just info (and error) logs:

`RUST_LOG=info ./target/debug/spidey-rs`

You can also just run with

`RUST_LOG=spidey_rs cargo run`
