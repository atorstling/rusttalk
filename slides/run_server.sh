#!/bin/bash
# First
# in ui: 
# cargo build
# in ui/frontend: 
# yarn
# in compiler:
# ./build.sh or
# TOOLS_TO_BUILD= CHANNELS_TO_BUILD=nightly ./build.sh (for only nightly, no tools)
dir=~/projects/private/rust-playground/ui 
cd $dir
#TMPDIR=/mnt/playground \
RUST_LOG=ui=debug \
PLAYGROUND_UI_ADDRESS=0.0.0.0 \
PLAYGROUND_UI_PORT=4711 \
PLAYGROUND_UI_ROOT=$dir/frontend/build \
RUST_BACKTRACE=1 \
cargo run
