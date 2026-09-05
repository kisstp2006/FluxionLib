#!/usr/bin/env bash

set -euo pipefail

echo "Formatting workspace..."
cargo fmt --all

echo "Running workspace tests..."
cargo test --workspace

echo "Running Clippy..."
cargo clippy --workspace --all-targets -- -D warnings

echo "FluxionLib verification completed successfully."
