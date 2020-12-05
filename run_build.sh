#!/usr/bin/env bash

RET=0
for i in {1..25}; do
    if [ -d "day$i" ]; then
            cargo fmt --manifest-path=day"$i"/Cargo.toml --verbose --all -- --check || RET=1;
            cargo clippy --manifest-path=day"$i"/Cargo.toml --verbose --all -- -D warnings || RET=1;
            cargo build --manifest-path=day"$i"/Cargo.toml --verbose --all || RET=1;
            cargo test --manifest-path=day"$i"/Cargo.toml --verbose --all || RET=1;
    else
        echo "No solution for day $i found"
    fi 
done

exit $RET