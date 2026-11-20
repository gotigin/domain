#!/usr/bin/env bash
# Present docs/legal/*.md as formatted HTML for gotigin.com.
# Do not hand-edit the generated HTML under static/.
#
#   ./sites/gotigin-website/scripts/present-legal.sh
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
SITE="$ROOT/sites/gotigin-website"
LEGAL="$ROOT/docs/legal"
TMPL="$SITE/legal-template.html"

if [[ ! -f "$TMPL" ]]; then
  echo "error: missing $TMPL" >&2
  exit 1
fi
if ! command -v pandoc >/dev/null 2>&1; then
  echo "error: pandoc required to present legal Markdown" >&2
  exit 1
fi

present() {
  local slug="$1" title="$2" description="$3" canonical="$4" md_name="$5"
  local md="$LEGAL/$md_name"
  if [[ ! -f "$md" ]]; then
    echo "error: missing $md" >&2
    exit 1
  fi
  local out_dir="$SITE/static/$slug"
  mkdir -p "$out_dir"
  pandoc "$md" \
    -f markdown \
    -t html5 \
    --template="$TMPL" \
    --metadata title="$title" \
    -V lang=en \
    -V description="$description" \
    -V canonical="$canonical" \
    -o "$out_dir/index.html"
  cp "$md" "$SITE/static/${slug}.md"
  if [[ -d "$SITE/public" ]]; then
    mkdir -p "$SITE/public/$slug"
    cp "$out_dir/index.html" "$SITE/public/$slug/index.html"
    cp "$md" "$SITE/public/${slug}.md"
  fi
  echo "  /$slug ← $md"
}

echo "Presenting docs/legal → sites/gotigin-website/static/"
present privacy "Privacy — Gotigin" \
  "Privacy policy for GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED and gotigin.com." \
  "https://gotigin.com/privacy" "PRIVACY.md"
present terms "Terms — Gotigin" \
  "Terms of use for gotigin.com and company-level Gotigin services." \
  "https://gotigin.com/terms" "TERMS.md"
present data-deletion "Data deletion — Gotigin" \
  "How to ask Gotigin to delete your personal data, for the company and for each product." \
  "https://gotigin.com/data-deletion" "DATA_DELETION.md"
present support "Support — Gotigin" \
  "Support contact for Gotigin company and brand routing." \
  "https://gotigin.com/support" "SUPPORT.md"
present help "Help — Gotigin" \
  "Help for Gotigin — names, brands, products, and company pages." \
  "https://gotigin.com/help" "HELP.md"
present about "About — Gotigin" \
  "About Gotigin and GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED." \
  "https://gotigin.com/about" "ABOUT.md"
present cookies "Cookies — Gotigin" \
  "Cookie policy for gotigin.com." \
  "https://gotigin.com/cookies" "COOKIES.md"
present security "Security — Gotigin" \
  "Security practices and vulnerability reporting for Gotigin." \
  "https://gotigin.com/security" "SECURITY.md"

echo "Done. Deploy with ./sites/gotigin-website/scripts/deploy-firebase.sh"
