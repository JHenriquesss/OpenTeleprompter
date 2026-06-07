#!/usr/bin/env bash
set -euo pipefail

echo "=== Formatting check ==="
cargo fmt --all -- --check

echo "=== Backend cargo check ==="
cargo check -p openprompter-rs-tauri --all-targets

echo "=== Backend tests ==="
cargo test -p openprompter-rs-tauri

echo "=== Backend clippy ==="
cargo clippy -p openprompter-rs-tauri --all-targets --all-features -- -D warnings

echo "=== Frontend WASM build ==="
trunk build

echo ""
echo "=== All checks passed ==="
