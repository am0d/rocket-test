#!/bin/sh

# Ensure that we kill the child processes when we exit
trap "trap -INT && kill -- -$$"
CARGO_INCREMENTAL=1 cargo watch -x check -s 'touch .trigger' &
RUST_BACKTRACE=1 CARGO_INCREMENTAL=1 cargo watch --no-gitignore -w .trigger -x run
