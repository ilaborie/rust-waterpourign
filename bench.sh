#!/usr/bin/env bash

# Build
./build.sh

# Bench
cargo bench

# Show result
open target/criterion/report/index.html
