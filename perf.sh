#!/usr/bin/env bash

cargo build --release

hyperfine --warmup 3 --export-json results/perf.json --parameter-list type rec,rec2,imp "./target/release/waterpouring-{type} 3"
