#!/usr/bin/env bash
# Run gotigin.com locally — one tree like deploy (Khaynjo / Wadhu pattern):
#   1) build debug wasm
#   2) overlay /privacy /terms /about … beside Makepad home
#   3) python http.server
#
# Serves at http://127.0.0.1:8030
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
SITE="$ROOT/sites/gotigin-website"
PKG="$ROOT/target/makepad-wasm-app/debug/gotigin-website"
PORT="${PORT:-8030}"

cd "$ROOT"
if [[ ! -d vendor/makepad-full/tools/cargo_makepad ]]; then
  echo "error: vendor/makepad-full missing — run ./scripts/setup-makepad.sh" >&2
  echo "       and ensure vendor/makepad-full exists (symlink from another Gotigin workspace is fine locally)." >&2
  exit 1
fi
if [[ ! -d vendor/makepad/widgets ]]; then
  echo "error: vendor/makepad missing — run ./scripts/setup-makepad.sh" >&2
  exit 1
fi

cargo run -q --manifest-path vendor/makepad-full/tools/cargo_makepad/Cargo.toml -- \
  wasm --no-threads build -p gotigin-website "$@"

if [[ ! -f "$PKG/index.html" ]]; then
  echo "error: missing $PKG/index.html" >&2
  exit 1
fi

"$SITE/scripts/overlay-legal-onto-pkg.sh" "$PKG"
# Same start-up screen as the deployed site (the debug wasm is bigger, so
# the ring simply takes longer).
"$SITE/scripts/patch-web-shell.sh" "$PKG/index.html"

if command -v fuser >/dev/null 2>&1; then
  if fuser "${PORT}/tcp" >/dev/null 2>&1; then
    echo "Port ${PORT} in use — stopping previous listener."
    fuser -k "${PORT}/tcp" >/dev/null 2>&1 || true
    sleep 0.5
  fi
fi

echo "Serving $PKG at http://127.0.0.1:${PORT}/ (Ctrl+C to stop)"
echo "    One tree: / = Makepad SPA; /privacy /terms /about /help … = same site"
cd "$PKG"
exec python3 -m http.server "$PORT" --bind 127.0.0.1
