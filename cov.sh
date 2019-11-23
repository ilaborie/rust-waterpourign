#!/usr/bin/env bash

# Testing with cov
export RUSTFLAGS="-Zprofile";
cargo +nightly cov clean
cargo +nightly cov test
cargo +nightly cov report --open


# Testing with grcov

#export CARGO_INCREMENTAL=0
#
## -Zprofile -Zno-landing-pads
#export RUSTFLAGS="-Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off"
#
#cargo build --verbose $CARGO_OPTIONS
#cargo test --verbose $CARGO_OPTIONS
#
#zip -0 ccov.zip `find . \( -name "rust-waterpouring*.gc*" \) -print`;
#
#./grcov ccov.zip -s . -t lcov --llvm --branch --ignore-not-existing --ignore-dir "/*" -o lcov.info;
#
#bash <(curl -s https://codecov.io/bash) -f lcov.info;
