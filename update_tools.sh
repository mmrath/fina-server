rustup update nightly
rustup update stable
rustup component add rustfmt-preview --toolchain=nightly
rustup component add clippy-preview --toolchain=nightly
cargo install cargo-fix --force
