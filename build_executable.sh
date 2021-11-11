sudo apt install musl-tools #ubuntu 18.04
sudo snap install rustup --classic
rustup target add x86_64-unknown-linux-musl
cargo build --target=x86_64-unknown-linux-musl --release
