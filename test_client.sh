#!/bin/sh
cd server
RUST_LOG=info cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
