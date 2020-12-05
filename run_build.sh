#!/usr/bin/env bash

for i in {1..25}; do
    if [ -d "day$i" ]; then
            cargo fmt --manifest-path=day"$i"/Cargo.toml --verbose --all -- --check
            cargo clippy --manifest-path=day"$i"/Cargo.toml --verbose --all -- -D warnings
            cargo build --manifest-path=day"$i"/Cargo.toml --verbose --all
            cargo test --manifest-path=day"$i"/Cargo.toml --verbose --all
    else
        echo "No solution for day $i found"
    fi 
done