sudo apt install musl-tools #ubuntu 18.04
rustup target add x86_64-unknown-linux-musl
cargo build --target=x86_64-unknown-linux-musl --release
