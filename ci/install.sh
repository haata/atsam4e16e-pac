set -euxo pipefail

main() {
    rustup component add rustfmt --toolchain nightly
    if [ $TARGET != x86_64-unknown-linux-gnu ]; then
        rustup target add $TARGET
    fi
}

main
