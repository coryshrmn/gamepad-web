#!/bin/bash

set -euo pipefail
IFS=$'\n\t'

CARGO_WEB=${CARGO_WEB:-cargo-web}

set +e
echo "$(rustc --version)" | grep -q "nightly"
if [ "$?" = "0" ]; then
    export IS_NIGHTLY=1
else
    export IS_NIGHTLY=0
fi
set -e

echo "Is Rust from nightly: $IS_NIGHTLY"

function do_test() {
    dir="$1"
    shift

    echo "Testing $dir $@"
    pushd "$dir" > /dev/null
    $CARGO_WEB test $@
    popd > /dev/null
}

function test_targets() {
    dir=$1

    do_test "$dir" --target=asmjs-unknown-emscripten
    do_test "$dir" --target=wasm32-unknown-emscripten

    if [ "$IS_NIGHTLY" = "1" ]; then
        do_test "$dir" --nodejs --target=wasm32-unknown-unknown
    fi
}

# test main library
test_targets ""

EXAMPLES=($(grep members examples/Cargo.toml | sed -r 's/[^"]*"(\w+)"[^"*]*/\1\n/g'))

for example in "${EXAMPLES[@]}"; do
    test_targets "examples/$example"
done
