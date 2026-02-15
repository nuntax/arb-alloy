#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
NITRO_DIR="${ROOT_DIR}/../nitro"
OUT_PATH="${ROOT_DIR}/crates/consensus/testdata/nitro_tx_fixtures.json"

mkdir -p "$(dirname "${OUT_PATH}")"

(
  cd "${NITRO_DIR}"
  GOCACHE="${GOCACHE:-/tmp/go-build-cache}" \
  GOMODCACHE="${GOMODCACHE:-/tmp/go-mod-cache}" \
  go run ./cmd/arbfixturegen -out "${OUT_PATH}"
)

echo "Wrote ${OUT_PATH}"
