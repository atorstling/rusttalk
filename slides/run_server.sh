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
tmpdir=/tmp/playground
mkdir -p $tmpdir
cd $dir
#TMPDIR=/mnt/playground \
TMPDIR=$tmpdir \
RUST_LOG=ui=debug \
PLAYGROUND_UI_ADDRESS=0.0.0.0 \
PLAYGROUND_UI_PORT=4711 \
PLAYGROUND_UI_ROOT=$dir/frontend/build \
PLAYGROUND_CORS_ENABLED=true \
RUST_BACKTRACE=1 \
cargo run
