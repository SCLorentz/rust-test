if [ "$1" = "arm" ]; then
    RUSTFLAGS="-C link-arg=-nostartfiles" cargo run --target aarch64-unknown-linux-gnu
else
    RUSTFLAGS="-C link-arg=-nostartfiles" cargo run --target x86_64-unknown-linux-gnu
fi