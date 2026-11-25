#!/usr/bin/env bash
# Scaffold gotigin.com site packs from locale.rs EN.
#
#   ./scripts/scaffold-ui-translations.sh --site gotigin
#
# Bash only. See scripts/scaffold-site-translations.sh.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --site)
      exec "$ROOT/scripts/scaffold-site-translations.sh" "${2:?--site needs gotigin}"
      ;;
    -h|--help)
      sed -n '2,8p' "$0"
      exit 0
      ;;
    *) echo "unknown arg: $1 (use --site gotigin)" >&2; exit 2 ;;
  esac
done

echo "usage: $0 --site gotigin" >&2
exit 2
