#!/usr/bin/env bash
# Deploy the one gotigin.com Hosting tree (Khaynjo / Wadhu pattern):
#   Makepad SPA at /  +  /privacy /terms /about /help …  +  Firebase client
#
# Firebase project: gotigin-com (.firebaserc). Custom domain: gotigin.com.
#
#   ./sites/gotigin-website/scripts/deploy-firebase.sh
#   ./sites/gotigin-website/scripts/deploy-firebase.sh --dry-run
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
SITE="$ROOT/sites/gotigin-website"
OUT="$SITE/public"
LEGAL="$ROOT/docs/legal"
WASM_PKG="$ROOT/target/makepad-wasm-app/release/gotigin-website"
DRY=0

for arg in "$@"; do
  case "$arg" in
    --dry-run) DRY=1 ;;
    -h|--help) sed -n '2,12p' "$0"; exit 0 ;;
    *) echo "Unknown arg: $arg" >&2; exit 1 ;;
  esac
done

export PATH="${HOME}/.local/bin:${PATH}"

if ! command -v firebase >/dev/null 2>&1; then
  echo "error: firebase CLI not found (npm i -g firebase-tools)" >&2
  exit 1
fi

echo "1/4 Bundle Firebase client → static/js/firebase-init.js"
(
  cd "$SITE"
  if [[ ! -d node_modules/firebase ]]; then
    npm install
  fi
  npm run build:firebase
)
if [[ ! -f "$SITE/static/js/firebase-init.js" ]]; then
  echo "error: missing $SITE/static/js/firebase-init.js after build:firebase" >&2
  exit 1
fi

echo "2/4 Build Makepad wasm (gotigin-website, release)"
(
  cd "$ROOT"
  cargo run -q --manifest-path vendor/makepad-full/tools/cargo_makepad/Cargo.toml -- \
    wasm --no-threads --strip --brotli build -p gotigin-website --release
)
if [[ ! -f "$WASM_PKG/index.html" || ! -f "$WASM_PKG/gotigin-website.wasm" ]]; then
  echo "error: wasm package incomplete at $WASM_PKG" >&2
  exit 1
fi

echo "3/4 Stage public/ = one tree (wasm home + legal + Firebase client)"
"$SITE/scripts/present-legal.sh"
rm -rf "$OUT"
mkdir -p "$OUT"
rsync -a "$WASM_PKG/" "$OUT/"
"$SITE/scripts/overlay-legal-onto-pkg.sh" "$OUT"
mkdir -p "$OUT/js"
cp "$SITE/static/js/firebase-init.js" "$OUT/js/firebase-init.js"

BUILD_ID="$(date -u +%Y%m%d%H%M%S)-$(git -C "$ROOT" rev-parse --short HEAD 2>/dev/null || echo local)"
INDEX="$OUT/index.html"
if ! grep -qE 'gotigin-website\.wasm|web_gl|full_canvas' "$INDEX"; then
  echo "error: staged index.html is not the Makepad home" >&2
  exit 1
fi

# SEO / company identity in the wasm shell (reviewers read the HTML source).
if ! grep -q 'og:site_name' "$INDEX"; then
  awk -v bid="$BUILD_ID" '
    /<title>/ && !titled {
      print "    <title>Gotigin — Where Technology Meets Discovery</title>"
      print "    <meta name=\"description\" content=\"GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED — house mark Gotigin (GTG). Parent of Kumkaj, Wadhu, Ghal Bol. Privacy, terms, help, and data deletion for store verification.\" />"
      print "    <meta name=\"theme-color\" content=\"#12102a\" />"
      print "    <link rel=\"icon\" href=\"./favicon.png\" type=\"image/png\" />"
      print "    <link rel=\"apple-touch-icon\" href=\"./media/gotigin-brand-logo.png\" />"
      print "    <meta property=\"og:site_name\" content=\"Gotigin\" />"
      print "    <meta property=\"og:title\" content=\"Gotigin — Where Technology Meets Discovery\" />"
      print "    <meta property=\"og:description\" content=\"Company site for GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED. Brands: Kumkaj, Wadhu, Ghal Bol.\" />"
      print "    <meta property=\"og:url\" content=\"https://gotigin.com/\" />"
      print "    <meta property=\"og:image\" content=\"https://gotigin.com/media/gotigin-brand-logo.png\" />"
      print "    <meta property=\"og:type\" content=\"website\" />"
      titled=1
      next
    }
    /<\/body>/ && !done {
      print "        <script src=\"./js/firebase-init.js?v=" bid "\" defer></script>"
      done=1
    }
    { print }
  ' "$INDEX" > "$INDEX.tmp" && mv "$INDEX.tmp" "$INDEX"
elif ! grep -q 'firebase-init.js' "$INDEX"; then
  awk -v bid="$BUILD_ID" '
    /<\/body>/ && !done {
      print "        <script src=\"./js/firebase-init.js?v=" bid "\" defer></script>"
      done=1
    }
    { print }
  ' "$INDEX" > "$INDEX.tmp" && mv "$INDEX.tmp" "$INDEX"
fi

for need in privacy terms data-deletion support help about cookies security; do
  if [[ ! -f "$OUT/$need/index.html" ]]; then
    echo "error: missing /$need after overlay" >&2
    exit 1
  fi
done
if [[ ! -f "$OUT/js/firebase-init.js" ]]; then
  echo "error: missing $OUT/js/firebase-init.js" >&2
  exit 1
fi
if ! grep -q "Gotigin privacy policy" "$OUT/privacy/index.html"; then
  echo "error: /privacy not generated from docs/legal/PRIVACY.md" >&2
  exit 1
fi
if ! grep -q "data deletion" "$OUT/data-deletion/index.html"; then
  echo "error: /data-deletion not generated from docs/legal/DATA_DELETION.md" >&2
  exit 1
fi
test -f "$LEGAL/PRIVACY.md"
test -f "$LEGAL/HELP.md"
test -f "$LEGAL/SECURITY.md"

echo "4/4 Deploy Firebase Hosting → gotigin-com (one site: / + legal pages)"
cd "$SITE"
if [[ "$DRY" -eq 1 ]]; then
  firebase deploy --only hosting --dry-run
else
  firebase deploy --only hosting
fi
