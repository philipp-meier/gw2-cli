#!/bin/sh
cargo build --release
# cp -r target/release/gw2-cli $HOME/.local/bin/
cp -r target/release/gw2-cli $HOME/.cargo/bin/