#!/usr/bin/env bash
# Resolve a 32-byte Ed25519 dev-signing seed for a CI lane.
#
# Upstream `main` on `NON-OS/nonos-micro-kernel` carries the production
# `SIGNING_KEY_BASE64` secret; that key is what we use for every
# image whose hash is recorded in the production-ledger.
#
# Forks (e.g. `eKisNonos/nonos-micro-kernel`) do not have access to that
# secret. Without a fallback every fork CI run goes red on the
# first build step. To keep the contract honest the fallback is
# narrow: a deterministic 32-byte string, marked plainly as a fork
# dev seed. The resulting kernel image is signature-shaped but does
# not chain to the upstream signing key — anything the
# production-ledger trusts must be re-signed by the upstream lane.

set -euo pipefail

mode="${NONOS_CI_TRUST_MODE:-scratch}"

mkdir -p .keys

if [ -n "${SIGNING_KEY_BASE64:-}" ]; then
    printf '%s' "${SIGNING_KEY_BASE64}" | base64 -d > .keys/dev-signing.seed
else
    if [ "${mode}" = "production" ]; then
        echo "::error::SIGNING_KEY_BASE64 is required for production CI trust mode"
        exit 1
    fi
    echo "::warning::SIGNING_KEY_BASE64 not set; using deterministic fork dev seed"
    printf 'NONOS-CI-FORK-DEV-SEED-32-BYTE!!' > .keys/dev-signing.seed
fi

chmod 600 .keys/dev-signing.seed

if [ "$(wc -c < .keys/dev-signing.seed)" -ne 32 ]; then
    echo "::error::signing key must be 32 bytes"
    exit 1
fi

cp .keys/dev-signing.seed .keys/signing_key_v1.bin
chmod 600 .keys/signing_key_v1.bin

if [ -n "${GITHUB_ENV:-}" ]; then
    printf 'SIGNING_KEY=%s/.keys/signing_key_v1.bin\n' "$(pwd)" >> "$GITHUB_ENV"
fi
