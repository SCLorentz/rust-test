if [ "$1" = "arm" ]; then
    RUSTFLAGS="-C link-arg=-nostartfiles" cargo run --profile arm --verbose
else
    RUSTFLAGS="-C link-arg=-nostartfiles" cargo run  --release
fi