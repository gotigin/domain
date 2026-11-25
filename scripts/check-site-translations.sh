#!/usr/bin/env bash
# Structural check for gotigin.com packs (cargo test is the full gate).
#
#   ./scripts/check-site-translations.sh
#   ./scripts/scaffold-ui-translations.sh --site gotigin && ./scripts/check-site-translations.sh
#
# Confirms every table language has a pack whose keys match EN order, and
# that hi.tsv / ur.tsv are not English and do not contain @@EN@@.
# Bash + gawk only.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIR="$ROOT/sites/gotigin-website"
LOC="$DIR/src/locale.rs"
LANGS="$ROOT/sites/_shared/i18n/languages.rs"

en_pairs() {
  gawk '
    /^pub const EN: &\[\(&str, &str\)\] = &\[/ { on = 1; next }
    on && /^\];/ { exit }
    on && match($0, /^[[:space:]]*\("([a-z0-9_]+)", "((\\.|[^"\\])*)"\),[[:space:]]*$/, m) {
      v = m[2]; gsub(/\\"/, "\"", v); print m[1] "\t" v
    }
  ' "$LOC"
}

PAIRS="$(mktemp)"
trap 'rm -f "$PAIRS"' EXIT
en_pairs > "$PAIRS"
n_keys="$(wc -l < "$PAIRS" | tr -d ' ')"
if [[ "$n_keys" -lt 5 ]]; then
  echo "error: read only $n_keys EN rows" >&2
  exit 1
fi

fail=0
while IFS= read -r code; do
  case "$code" in en|pt-BR|es-MX|yue|gom) continue ;; esac
  file="$DIR/translations/$code.tsv"
  if [[ ! -f "$file" ]]; then
    echo "missing $file" >&2
    fail=1
    continue
  fi
  got="$(mktemp)"
  gawk -F'\t' 'BEGIN{OFS="\t"} $0=="" || $0 ~ /^#/ {next} {print $1}' "$file" > "$got"
  if ! cut -f1 "$PAIRS" | cmp -s - "$got"; then
    echo "$code: keys differ from EN order" >&2
    fail=1
  fi
  rm -f "$got"
done < <(gawk 'match($0, /code: "([^"]+)"/, m) { print m[1] }' "$LANGS")

for code in hi ur; do
  file="$DIR/translations/$code.tsv"
  if grep -q '@@EN@@' "$file"; then
    echo "$code: contains @@EN@@ stand-in" >&2
    fail=1
  fi
  same="$(gawk -F'\t' -v en="$PAIRS" '
    NR==FNR { if ($0 != "" && $0 !~ /^#/) e[$1]=substr($0, index($0,"\t")+1); next }
    $0=="" || $0 ~ /^#/ { next }
    {
      key=$1; val=substr($0, index($0,"\t")+1)
      if (key in e && val==e[key]) n++
    }
    END { print n+0 }
  ' "$PAIRS" "$file")"
  if [[ "$same" != 0 ]]; then
    echo "$code: $same rows still equal English" >&2
    fail=1
  fi
done

if [[ "$fail" -ne 0 ]]; then
  exit 1
fi
echo "gotigin packs: $n_keys keys; hi and ur are translated"
