#!/usr/bin/env bash

# Bench
cargo bench

# Show result
open target/criterion/report/index.html
