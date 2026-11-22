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
# cargo-makepad writes a .br twin beside every file. Firebase Hosting
# compresses on its own and never serves those, so they stay out of the tree.
rsync -a --exclude='*.br' "$WASM_PKG/" "$OUT/"
"$SITE/scripts/overlay-legal-onto-pkg.sh" "$OUT"
mkdir -p "$OUT/js"
cp "$SITE/static/js/firebase-init.js" "$OUT/js/firebase-init.js"

BUILD_ID="$(date -u +%Y%m%d%H%M%S)-$(git -C "$ROOT" rev-parse --short HEAD 2>/dev/null || echo local)"
INDEX="$OUT/index.html"
if ! grep -qE 'gotigin-website\.wasm|web_gl|full_canvas' "$INDEX"; then
  echo "error: staged index.html is not the Makepad home" >&2
  exit 1
fi

# Identity, link preview and cache-busting in the wasm shell. Crawlers and
# chat apps read this HTML only — they never run the wasm — so the title, the
# description and the preview picture have to be here.
PREVIEW="$SITE/static/media/link-preview.jpg"
if [[ ! -f "$PREVIEW" ]]; then
  echo "error: missing ${PREVIEW#"$ROOT"/} — run sites/gotigin-website/scripts/make-link-preview.sh" >&2
  exit 1
fi
# The picture's address changes when the picture does: WhatsApp, Telegram and
# Facebook keep a preview for days and only fetch again for a new address.
PREVIEW_URL="https://gotigin.com/media/link-preview.jpg?v=$(sha256sum "$PREVIEW" | cut -c1-10)"
HEAD_INJECT='    <title>Gotigin — Where Technology Meets Discovery</title>
    <meta name="description" content="Gotigin — Where Technology Meets Discovery. Trademark of GOTIGIN SOFTWARE &amp; HARDWARE PRIVATE LIMITED. Brands: Kumkaj, Wadhu (Tools · Apps · More), Ghal Bol." />
    <meta name="gotigin-build" content="'"$BUILD_ID"'" />
    <meta name="theme-color" content="#12102a" />
    <meta name="robots" content="max-image-preview:large" />
    <link rel="canonical" href="https://gotigin.com/" />
    <link rel="icon" href="./favicon.png" type="image/png" />
    <link rel="apple-touch-icon" href="./media/gotigin-brand-logo.png" />
    <link rel="manifest" href="./site.webmanifest" />
    <meta property="og:site_name" content="Gotigin" />
    <meta property="og:title" content="Gotigin — Where Technology Meets Discovery" />
    <meta property="og:description" content="GOTIGIN SOFTWARE &amp; HARDWARE PRIVATE LIMITED. Trademark Gotigin. Brands: Kumkaj, Wadhu, Ghal Bol." />
    <meta property="og:url" content="https://gotigin.com/" />
    <meta property="og:type" content="website" />
    <meta property="og:locale" content="en_IN" />
    <meta property="og:image" content="'"$PREVIEW_URL"'" />
    <meta property="og:image:type" content="image/jpeg" />
    <meta property="og:image:width" content="1200" />
    <meta property="og:image:height" content="630" />
    <meta property="og:image:alt" content="The gotigin.com home page: Gotigin — Where Technology Meets Discovery." />
    <meta name="twitter:card" content="summary_large_image" />
    <meta name="twitter:title" content="Gotigin — Where Technology Meets Discovery" />
    <meta name="twitter:description" content="GOTIGIN SOFTWARE &amp; HARDWARE PRIVATE LIMITED. Trademark Gotigin. Brands: Kumkaj, Wadhu, Ghal Bol." />
    <meta name="twitter:image" content="'"$PREVIEW_URL"'" />
    <meta name="twitter:image:alt" content="The gotigin.com home page: Gotigin — Where Technology Meets Discovery." />'
awk -v head="$HEAD_INJECT" -v bid="$BUILD_ID" '
  /<title>/ && !titled {
    print head
    titled = 1
    next
  }
  {
    # A new address per build, so a browser never pairs old wasm with new glue.
    gsub(/\.\/makepad_platform\/web_gl\.js/, "./makepad_platform/web_gl.js?v=" bid)
    gsub(/\.\/gotigin-website\.wasm/, "./gotigin-website.wasm?v=" bid)
  }
  /<\/body>/ && !done {
    print "        <script src=\"./js/firebase-init.js?v=" bid "\" defer></script>"
    done = 1
  }
  { print }
  END { if (!titled || !done) exit 2 }
' "$INDEX" > "$INDEX.tmp" && mv "$INDEX.tmp" "$INDEX"
printf '%s\n' "$BUILD_ID" > "$OUT/build-id.txt"

# The start-up screen (logo, progress ring) replaces cargo-makepad's grey bar.
"$SITE/scripts/patch-web-shell.sh" "$INDEX"
# Fonts: only the characters this site draws (the originals stay whole).
"$SITE/scripts/subset-fonts.sh" "$OUT"

for need in privacy terms cancellation data-deletion support help about cookies security; do
  if [[ ! -f "$OUT/$need/index.html" ]]; then
    echo "error: missing /$need after overlay" >&2
    exit 1
  fi
done
if [[ ! -f "$OUT/js/firebase-init.js" ]]; then
  echo "error: missing $OUT/js/firebase-init.js" >&2
  exit 1
fi
if [[ ! -f "$OUT/robots.txt" || ! -f "$OUT/sitemap.xml" ]]; then
  echo "error: missing robots.txt or sitemap.xml after overlay (static/)" >&2
  exit 1
fi
if [[ ! -f "$OUT/site.webmanifest" ]]; then
  echo "error: missing site.webmanifest after overlay (static/)" >&2
  exit 1
fi
if ! grep -q 'Sitemap: https://gotigin.com/sitemap.xml' "$OUT/robots.txt"; then
  echo "error: robots.txt missing sitemap pointer" >&2
  exit 1
fi
if ! grep -q 'https://gotigin.com/cancellation' "$OUT/sitemap.xml"; then
  echo "error: sitemap.xml missing /cancellation" >&2
  exit 1
fi
if ! grep -q "Gotigin privacy policy" "$OUT/privacy/index.html"; then
  echo "error: /privacy not generated from docs/legal/PRIVACY.md" >&2
  exit 1
fi
if ! grep -q "cancellation and refund policy" "$OUT/cancellation/index.html"; then
  echo "error: /cancellation not generated from docs/legal/CANCELLATION.md" >&2
  exit 1
fi
if ! grep -q "data deletion" "$OUT/data-deletion/index.html"; then
  echo "error: /data-deletion not generated from docs/legal/DATA_DELETION.md" >&2
  exit 1
fi
if ! grep -q 'application/ld+json' "$OUT/index.html" || ! grep -q '<noscript>' "$OUT/index.html"; then
  echo "error: crawlable JSON-LD / noscript missing from staged index.html" >&2
  exit 1
fi
if ! grep -q 'support@gotigin.com' "$OUT/index.html" || ! grep -q 'github.com/gotigin' "$OUT/index.html"; then
  echo "error: Organization JSON-LD facts missing from staged index.html" >&2
  exit 1
fi
test -f "$LEGAL/PRIVACY.md"
test -f "$LEGAL/CANCELLATION.md"
test -f "$LEGAL/HELP.md"
test -f "$LEGAL/SECURITY.md"

# Always pass --project (the default in .firebaserc): without it the CLI uses
# whatever `firebase use` last set for this folder, which can be another site.
PROJECT="$(sed -n 's/.*"default"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' "$SITE/.firebaserc" 2>/dev/null | head -1)"
if [[ -z "$PROJECT" ]]; then
  echo "error: no default project in sites/gotigin-website/.firebaserc" >&2
  exit 1
fi
echo "4/4 Deploy Firebase Hosting → $PROJECT (one site: / + legal pages)"
cd "$SITE"
if [[ "$DRY" -eq 1 ]]; then
  firebase deploy --only hosting --project "$PROJECT" --dry-run
else
  firebase deploy --only hosting --project "$PROJECT"
fi
