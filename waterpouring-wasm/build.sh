#!/usr/bin/env bash

wasm-pack build

pushd www || exit
  npm run build
popd || exit
