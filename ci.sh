#!/bin/bash

function lint() {
    cargo fmt --all -- --check && cargo clippy --all-features -- -D warnings \
        -D unsafe_code \
        -D trivial_casts \
        -D trivial_numeric_casts \
        -D unused_extern_crates \
        -D unused_import_braces \
        -D unused_qualifications \
        -D unreachable_pub
}

function test() {
    cargo test --workspace --all-features -- --nocapture
}

function help() {
    echo "Usage: $(basename "$0") [OPTIONS]

Commands:
  lint           Run lints
  test           Run all tests
  help           Show help
"
}

if [[ $1 =~ ^(lint|test|help)$ ]]; then
    "$@"
else
    echo "Invalid subcommand '$1'" >&2
    exit 1
fi
