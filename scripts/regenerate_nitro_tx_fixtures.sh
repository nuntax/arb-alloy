#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_PATH="${ROOT_DIR}/crates/consensus/testdata/nitro_tx_fixtures.json"
RECEIPT_OUT_PATH="${ROOT_DIR}/crates/consensus/testdata/nitro_receipt_fixtures.json"
HEADER_OUT_PATH="${ROOT_DIR}/crates/consensus/testdata/nitro_header_fixtures.json"
SCRIPTS_DIR="${ROOT_DIR}/scripts"

mkdir -p "$(dirname "${OUT_PATH}")"

(
  cd "${SCRIPTS_DIR}"
  GOCACHE="${GOCACHE:-/tmp/go-build-cache}" \
  GOMODCACHE="${GOMODCACHE:-/tmp/go-mod-cache}" \
  go run . \
    -tx-out "${OUT_PATH}" \
    -receipt-out "${RECEIPT_OUT_PATH}" \
    -header-out "${HEADER_OUT_PATH}"
)

echo "Wrote ${OUT_PATH}"
echo "Wrote ${RECEIPT_OUT_PATH}"
echo "Wrote ${HEADER_OUT_PATH}"
