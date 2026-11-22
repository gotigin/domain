#!/usr/bin/env bash
# Put gotigin.com's own start-up screen into a staged Makepad wasm index.html.
#
# cargo-makepad writes a grey bar that says "Loading.." and nothing else. This
# replaces it with shell/boot.{css,html,js}: the logo in a ring that fills as
# the wasm arrives, the name, a line that changes, and an honest message when
# the page is slow or cannot start. Everything is inlined, so the screen is
# there with the first response and costs no request.
#
#   ./sites/gotigin-website/scripts/patch-web-shell.sh <path/to/index.html>
#
# The wasm must sit beside index.html (its size drives the progress ring).
# Safe to run twice. Used by run-web.sh and deploy-firebase.sh.
set -euo pipefail

INDEX="${1:?usage: patch-web-shell.sh <path/to/index.html>}"
SITE="$(cd "$(dirname "$0")/.." && pwd)"
SHELL_SRC="$SITE/shell"
WASM_NAME="gotigin-website.wasm"

if [[ ! -f "$INDEX" ]]; then
  echo "error: missing $INDEX" >&2
  exit 1
fi
PKG="$(cd "$(dirname "$INDEX")" && pwd)"
for f in boot.css boot.html boot.js logo.webp json-ld.json noscript.html; do
  if [[ ! -f "$SHELL_SRC/$f" ]]; then
    echo "error: missing $SHELL_SRC/$f" >&2
    exit 1
  fi
done
if [[ ! -f "$PKG/$WASM_NAME" ]]; then
  echo "error: missing $PKG/$WASM_NAME (needed for the progress ring)" >&2
  exit 1
fi
if grep -q 'id="boot"' "$INDEX"; then
  echo "Already patched: $INDEX"
  exit 0
fi
if ! grep -q "class='canvas_loader'" "$INDEX"; then
  echo "error: $INDEX has no canvas_loader block — cargo-makepad changed its shell" >&2
  exit 1
fi

WASM_BYTES="$(stat -c %s "$PKG/$WASM_NAME")"
BODY_TMP="$(mktemp)"
JSONLD_TMP="$(mktemp)"
trap 'rm -f "$BODY_TMP" "$JSONLD_TMP" "$INDEX.tmp"' EXIT
# The logo travels inside the page (about 3 KB) so it shows at once.
{
  printf 's|__WASM_BYTES__|%s|\n' "$WASM_BYTES"
  printf 's|__LOGO_DATA_URI__|data:image/webp;base64,%s|\n' "$(base64 -w0 "$SHELL_SRC/logo.webp")"
} > "$BODY_TMP.sed"
sed -f "$BODY_TMP.sed" "$SHELL_SRC/boot.html" > "$BODY_TMP"
rm -f "$BODY_TMP.sed"
# One line for the script tag (minified whitespace from the source file).
tr -d '\n' < "$SHELL_SRC/json-ld.json" | sed 's/  */ /g' > "$JSONLD_TMP"

GUARD_SRC="$SHELL_SRC/mobile-viewport-guard.js"
if [[ ! -f "$GUARD_SRC" ]]; then
  echo "error: missing $GUARD_SRC" >&2
  exit 1
fi

awk -v css="$SHELL_SRC/boot.css" -v body="$BODY_TMP" -v js="$SHELL_SRC/boot.js" \
    -v guard="$GUARD_SRC" -v jsonld="$JSONLD_TMP" -v noscript="$SHELL_SRC/noscript.html" '
  function cat(file,   line) {
    while ((getline line < file) > 0) print line
    close(file)
  }
  # cargo-makepad: <div class="canvas_loader"> <div> Loading.. </div> </div>
  skip > 0 {
    if ($0 ~ /<\/div>/) skip--
    next
  }
  /<html[> ]/ && !htmllang {
    sub(/<html/, "<html lang=\"en\"")
    htmllang = 1
  }
  # cargo-makepad writes name='viewport'; either quote style is fine.
  /name=.viewport./ {
    print "        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1, maximum-scale=1, user-scalable=no, viewport-fit=cover\" />"
    next
  }
  # Drop a thin/earlier Organization JSON-LD so we inject the full one once.
  /application\/ld\+json/ { next }
  /<\/head>/ && !styled {
    print "        <link rel=\"manifest\" href=\"./site.webmanifest\" />"
    print "        <script type=\"application/ld+json\">"
    cat(jsonld)
    print "</script>"
    print "        <style>"
    cat(css)
    print "        </style>"
    print "        <script>"
    cat(guard)
    print "        </script>"
    styled = 1
  }
  /class=.canvas_loader./ && !booted {
    cat(body)
    print "        <script>"
    cat(js)
    print "        </script>"
    booted = 1
    skip = 2
    next
  }
  /<\/body>/ && !ns {
    cat(noscript)
    ns = 1
  }
  { print }
  END { if (!styled || !booted || !ns) exit 3 }
' "$INDEX" > "$INDEX.tmp"
mv "$INDEX.tmp" "$INDEX"

if ! grep -q 'id="boot"' "$INDEX" || grep -q 'Loading\.\.$' "$INDEX"; then
  echo "error: start-up screen was not placed in $INDEX" >&2
  exit 1
fi
if ! grep -q 'application/ld+json' "$INDEX" || ! grep -q '<noscript>' "$INDEX"; then
  echo "error: crawlable JSON-LD / noscript missing from $INDEX" >&2
  exit 1
fi
if ! grep -q 'support@gotigin.com' "$INDEX" || ! grep -q 'github.com/gotigin' "$INDEX"; then
  echo "error: Organization JSON-LD facts missing from $INDEX" >&2
  exit 1
fi
echo "Patched $INDEX (start-up screen, crawlable summary, wasm $WASM_BYTES bytes)"
