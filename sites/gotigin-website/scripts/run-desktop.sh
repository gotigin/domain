#!/usr/bin/env bash
# Native window for gotigin.com (same crate as the wasm site).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT"
exec cargo run -p gotigin-website --release "$@"
