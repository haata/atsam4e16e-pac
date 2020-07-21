set -ex

main() {
    cargo check --target $TARGET
    cargo check --target $TARGET --features rt

    if [ $TARGET = x86_64-unknown-linux-gnu ]; then
        # Generate the PAC
        bash update.sh
        cargo clean
        cargo build
    fi
}

main