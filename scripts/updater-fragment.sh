#!/usr/bin/env bash
#
# Emit a per-platform updater fragment for the Tauri `latest.json` manifest.
#
# Each release job calls this after `cargo tauri build` (with
# createUpdaterArtifacts=true) to capture one platform's updater bundle URL +
# minisign signature. A later job merges the fragments into `latest.json`.
#
# Usage:
#   updater-fragment.sh <platform-key> <bundle-glob> [tag] [outdir] [repo]
#
#   platform-key  Tauri updater target key, e.g. windows-x86_64, linux-x86_64,
#                 darwin-x86_64, darwin-aarch64
#   bundle-glob   Glob to the updater bundle (NOT the .sig), e.g.
#                 "target/release/bundle/nsis/*-setup.exe"
#   tag           Release tag (default: $GITHUB_REF_NAME)
#   outdir        Where to write <platform-key>.json (default: updater-frag)
#   repo          owner/name (default: JHenriquesss/OpenTeleprompter)
#
# The GitHub release-asset URL is derived deterministically: GitHub replaces
# spaces in asset filenames with dots, so "OpenPrompter RS_1.0.0_x64-setup.exe"
# is served as "OpenPrompter.RS_1.0.0_x64-setup.exe".

set -euo pipefail

PLATFORM_KEY="${1:?platform-key required}"
BUNDLE_GLOB="${2:?bundle-glob required}"
TAG="${3:-${GITHUB_REF_NAME:?tag required}}"
OUTDIR="${4:-updater-frag}"
REPO="${5:-JHenriquesss/OpenTeleprompter}"

# shellcheck disable=SC2086  # intentional glob expansion
bundle=$(ls $BUNDLE_GLOB | head -n1)
if [ -z "${bundle:-}" ] || [ ! -f "$bundle" ]; then
  echo "updater-fragment: bundle not found for glob: $BUNDLE_GLOB" >&2
  exit 1
fi

sig="${bundle}.sig"
if [ ! -f "$sig" ]; then
  echo "updater-fragment: signature not found: $sig" >&2
  echo "  (is bundle.createUpdaterArtifacts=true and TAURI_SIGNING_PRIVATE_KEY set?)" >&2
  exit 1
fi

base=$(basename "$bundle")
asset="${base// /.}" # GitHub normalizes spaces to dots in asset names
url="https://github.com/${REPO}/releases/download/${TAG}/${asset}"
signature=$(cat "$sig")

mkdir -p "$OUTDIR"
jq -n \
  --arg k "$PLATFORM_KEY" \
  --arg s "$signature" \
  --arg u "$url" \
  '{($k): {signature: $s, url: $u}}' >"$OUTDIR/${PLATFORM_KEY}.json"

echo "updater-fragment: wrote $OUTDIR/${PLATFORM_KEY}.json"
cat "$OUTDIR/${PLATFORM_KEY}.json"
