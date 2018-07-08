rustup update nightly
rustup update stable
cargo +nightly install --force clippy
rustup component add rustfmt-preview --toolchain nightly
rustup component add rustfmt-preview