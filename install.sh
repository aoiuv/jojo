#!/bin/bash

CFG_PATH="/Users/sulirc/Desktop/cfg.jo"
echo "current cfg.jo"
cat $CFG_PATH

JOJO_BIN=$(which jojo)
JOJO_BIN_DIR=$(dirname $JOJO_BIN)
rm $JOJO_BIN
echo "jojo clean"

cargo build --release --bins -q
echo "jojo built"

cp target/release/jojo $JOJO_BIN_DIR 
echo "jojo setup"

jojo l 