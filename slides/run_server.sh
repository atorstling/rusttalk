#!/bin/bash
# Running the playground server requires a checkout of 
# github.com/integer32llc/rust-playground
# 
# 1. Prepare the playground by doing the following:
# 1a. Run web frontend + rust webservices
# First in ui: 
# cargo build
# Then in ui/frontend: 
# Install a semi-recent version of node, preferably with nvm
# npm -g install yarn
# yarn
# yarn run watch
# 1b. Build required docker images
# in compiler:
# ./build.sh or
# TOOLS_TO_BUILD= CHANNELS_TO_BUILD=nightly ./build.sh (for only nightly, no tools)
# 2. Run this script, to host the slide presentation
dir=~/projects/rust-playground/ui 
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
PLAYGROUND_GITHUB_TOKEN=$1 \
RUST_BACKTRACE=1 \
cargo run
