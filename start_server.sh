#!/bin/sh
cd server
RUST_LOG=info cargo watch -q -c -w src/ -x run
