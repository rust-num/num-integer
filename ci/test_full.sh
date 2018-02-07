#!/bin/bash

set -ex

echo Testing num-integer on rustc ${TRAVIS_RUST_VERSION}

# num-integer should build and test everywhere.
cargo build --verbose
cargo test --verbose

# test `no_std`
cargo build --verbose --no-default-features
cargo test --verbose --no-default-features
