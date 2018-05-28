#!/usr/bin/env bash
set -e

for dir in layout log mapping
do
    cd $dir

    cargo web deploy --release
    rm -rf deploy || true
    mv ../target/deploy .

    cd ..
done
