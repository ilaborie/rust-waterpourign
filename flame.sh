#!/usr/bin/env bash

for elt in waterpouring-rec waterpouring-rec2 waterpouring-imp
do
  sudo cargo flamegraph --dev --bin $elt -o "flamegraph-$elt.svg" 3
done
