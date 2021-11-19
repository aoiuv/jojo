#!/bin/bash
set -x

CFG_PATH="/Users/$USER/jojo"
mkdir -p $CFG_PATH 
touch "$CFG_PATH/cfg.jo"

JOJO_BIN=$(which jojo)
JOJO_BIN_DIR=$(dirname $JOJO_BIN)
rm $JOJO_BIN
echo "jojo clean"

cargo build --release --bins -q
echo "jojo built"

cp target/release/jojo $JOJO_BIN_DIR 
echo "jojo setup"