#!/bin/bash
# First
# in ui: 
# cargo build
# in ui/frontend: 
# fist install semi-recent version of node, preferably with nvm
# yarn
# yarn run watch
# in compiler:
# ./build.sh or
# TOOLS_TO_BUILD= CHANNELS_TO_BUILD=nightly ./build.sh (for only nightly, no tools)
dir=~/projects/private/rust-playground/ui 
ui_root_dir=$dir/frontend/build
tmpdir=/tmp/playground
mkdir -p $ui_root_dir
mkdir -p $tmpdir
cd $dir
#TMPDIR=/mnt/playground \
TMPDIR=$tmpdir \
RUST_LOG=ui=debug \
PLAYGROUND_UI_ADDRESS=0.0.0.0 \
PLAYGROUND_UI_PORT=4711 \
PLAYGROUND_UI_ROOT=$ui_root_dir \
PLAYGROUND_CORS_ENABLED=true \
RUST_BACKTRACE=1 \
cargo run
