#!/bin/sh
cargo build --release
# cp -r target/release/gw2cli $HOME/.local/bin/
cp -r target/release/gw2cli $HOME/.cargo/bin/