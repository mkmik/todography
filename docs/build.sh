#!/bin/sh
# Builds the book into docs/book/, with the example map's page at
# docs/book/example/index.html, as it's published on GitHub Pages.
#
# Needs mdbook 0.4 on the PATH and a Rust toolchain.
set -eu
cd "$(dirname "$0")/.."

mdbook build docs

# The build time the page records: the last commit's, so rebuilding the
# same commit gives the same file.
SOURCE_DATE_EPOCH=$(git log -1 --format=%ct 2>/dev/null || date +%s)
export SOURCE_DATE_EPOCH
cargo run --quiet --locked -- --map examples/todography build --out docs/book/example/index.html
