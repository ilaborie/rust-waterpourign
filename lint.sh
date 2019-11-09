#!/usr/bin/env bash

cargo clean -p rust-waterpouring
cargo clippy -- -W clippy::pedantic -A clippy::module-name-repetitions
