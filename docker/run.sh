#!/bin/bash
set -euo pipefail

# Build (from repo root): docker build -t rusty_blog .
# Usage:
#   CONTENT=/path/to/content OUTPUT=/path/to/site ./docker/run.sh
# Defaults match a sibling content checkout and this repo as the site root.

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

CONTENT="${CONTENT:-$REPO_ROOT/../content}"
OUTPUT="${OUTPUT:-$REPO_ROOT}"
IMAGE="${IMAGE:-rusty_blog}"

docker run --rm \
  -v "$CONTENT:/content:ro" \
  -v "$OUTPUT:/output" \
  -w /app \
  "$IMAGE" \
  /content /output
