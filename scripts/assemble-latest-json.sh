#!/usr/bin/env bash
#
# Merge per-platform updater fragments (from updater-fragment.sh) into a single
# Tauri `latest.json` manifest.
#
# Usage:
#   assemble-latest-json.sh <frag-dir> [version] [out] [notes]
#
#   frag-dir  Directory containing <platform-key>.json fragments
#   version   Release version without leading 'v' (default: ${GITHUB_REF_NAME#v})
#   out       Output path (default: latest.json)
#   notes     Release notes string (default: "OpenPrompter RS <version>")

set -euo pipefail

FRAG_DIR="${1:?frag-dir required}"
VERSION="${2:-${GITHUB_REF_NAME#v}}"
OUT="${3:-latest.json}"
NOTES="${4:-OpenPrompter RS ${VERSION}}"

shopt -s nullglob
frags=("$FRAG_DIR"/*.json)
if [ ${#frags[@]} -eq 0 ]; then
  echo "assemble-latest-json: no fragments in $FRAG_DIR" >&2
  exit 1
fi

# Deep-merge all fragments into one { "<key>": {...} } platforms object.
platforms=$(jq -s 'reduce .[] as $f ({}; . * $f)' "${frags[@]}")

jq -n \
  --arg v "$VERSION" \
  --arg notes "$NOTES" \
  --arg date "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --argjson platforms "$platforms" \
  '{version: $v, notes: $notes, pub_date: $date, platforms: $platforms}' >"$OUT"

echo "assemble-latest-json: wrote $OUT"
cat "$OUT"
