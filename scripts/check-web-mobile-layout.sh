#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
fail=0
echo "== unit: mobile_layout_size.js =="
node "$ROOT/scripts/web/mobile_layout_size.js"
WEBJS="$ROOT/vendor/makepad/platform/src/os/web/web.js"
[[ -f "$WEBJS" ]] || { echo "error: missing vendor web.js" >&2; exit 1; }
for needle in css_layout_size_for_canvas "visualViewport.addEventListener" "Desktop site" \
  "fire_resize_debounced" "canvas.width !== sw"; do
  grep -q "$needle" "$WEBJS" || { echo "error: web.js missing $needle" >&2; fail=1; }
done
if grep -q "visualViewport.addEventListener('scroll'" "$WEBJS"; then
  echo "error: web.js must not listen to visualViewport scroll for resize" >&2
  fail=1
fi
grep -q web_mobile_layout_width "$ROOT/scripts/setup-makepad.sh" || fail=1
[[ -f "$ROOT/patches/makepad/web_mobile_layout_width.rs.patch" ]] || fail=1
GUARD="$ROOT/sites/gotigin-website/shell/mobile-viewport-guard.js"
[[ -f "$GUARD" ]] || { echo "error: missing $GUARD" >&2; fail=1; }
grep -q 'maximum-scale=1' "$GUARD" || fail=1
grep -q 'mobile-viewport-guard' "$ROOT/sites/gotigin-website/scripts/patch-web-shell.sh" || {
  echo "error: patch-web-shell must inject mobile-viewport-guard" >&2; fail=1; }

# Crawlable shell + Hosting static (gotigin.com)
SITE="$ROOT/sites/gotigin-website"
for f in \
  "$SITE/shell/noscript.html" \
  "$SITE/shell/json-ld.json" \
  "$SITE/static/robots.txt" \
  "$SITE/static/sitemap.xml" \
  "$SITE/static/site.webmanifest" \
  "$SITE/static/favicon.ico" \
  "$SITE/static/cancellation/index.html"
do
  [[ -f "$f" ]] || { echo "error: missing $f" >&2; fail=1; }
done
grep -q 'application/ld+json\|json-ld.json' "$SITE/scripts/patch-web-shell.sh" || {
  echo "error: patch-web-shell must inline JSON-LD" >&2; fail=1; }
grep -q 'noscript.html' "$SITE/scripts/patch-web-shell.sh" || {
  echo "error: patch-web-shell must inline noscript" >&2; fail=1; }
grep -q 'Sitemap: https://gotigin.com/sitemap.xml' "$SITE/static/robots.txt" || {
  echo "error: robots.txt missing sitemap pointer" >&2; fail=1; }
grep -q 'https://gotigin.com/cancellation' "$SITE/static/sitemap.xml" || {
  echo "error: sitemap.xml missing /cancellation" >&2; fail=1; }
grep -q 'support@gotigin.com' "$SITE/shell/json-ld.json" || {
  echo "error: JSON-LD missing support@gotigin.com" >&2; fail=1; }
grep -q 'github.com/gotigin' "$SITE/shell/json-ld.json" || {
  echo "error: JSON-LD missing sameAs github.com/gotigin" >&2; fail=1; }

[[ "$fail" -eq 0 ]] || { echo "check-web-mobile-layout: FAILED" >&2; exit 1; }
echo "check-web-mobile-layout: OK"
