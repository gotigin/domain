#!/usr/bin/env bash
# Copy canonical site chrome from sites/_shared/chrome into gotigin-website static/
# (and public/js when present). Run after editing _shared chrome.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
SHARED="$ROOT/sites/_shared/chrome"
SITE="$ROOT/sites/gotigin-website"

for f in auth-nav.js lang-nav.js site-i18n.js auth-nav.css; do
  if [[ ! -f "$SHARED/$f" ]]; then
    echo "error: missing $SHARED/$f" >&2
    exit 1
  fi
done

mkdir -p "$SITE/static/js" "$SITE/static/css"
cp "$SHARED/auth-nav.js" "$SITE/static/js/auth-nav.js"
cp "$SHARED/lang-nav.js" "$SITE/static/js/lang-nav.js"
cp "$SHARED/site-i18n.js" "$SITE/static/js/site-i18n.js"
cp "$SHARED/auth-nav.css" "$SITE/static/css/auth-nav.css"

if [[ -d "$SITE/public/js" ]]; then
  cp "$SHARED/auth-nav.js" "$SITE/public/js/auth-nav.js"
  cp "$SHARED/lang-nav.js" "$SITE/public/js/lang-nav.js"
fi

echo "Synced _shared/chrome → $SITE/static/js (+ css); public/js updated if present."