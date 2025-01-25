#!/usr/bin/env bash

# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH

cd ..

if [ -d "tests/assets/" ]; then
    echo "wiping tests assets cache"
    rm -rf tests/assets/
fi

mkdir tests/assets
echo "test assets cache created"


cargo build --release -p test-lib-wrapper
echo "test-lib-wrapper built"

cp target/release/libtest_lib_wrapper.dylib tests/assets/
echo "test-lib-wrapper copied to tests/assets"

cargo run -p test-runner
