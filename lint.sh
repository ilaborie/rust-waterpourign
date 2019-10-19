#!/usr/bin/env bash

cargo clippy -- -W clippy::pedantic -A clippy::module_name_repetitions
