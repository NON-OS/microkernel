#!/usr/bin/env bash
# The files a release publishes, beside the bundle nonos-verify release
# writes, run in the flake's shell after `nix build .#standard`.
#
#   web    the attestation surface's verifier: nox_verify_wasm from
#          NON-OS/STARKs at the commit flake.lock pins, and the proof
#          verifier's page (nonos-proof-verify in wasm), one tarball with
#          its order and times fixed
#   sums   the release's sums over exactly the files a person downloads;
#          the bundle's own sums, which name the files inside the tarball,
#          are kept as BUNDLE-*SUMS
#
# Usage: release-assets.sh <web|sums>

set -euo pipefail
cd "$(dirname "$0")/.."

case "${1:?usage: release-assets.sh <web|sums>}" in
    web)
        mkdir -p release
        cp "$(nix build .#verifier-wasm --no-link --print-out-paths)/verifier.wasm" release/verifier.wasm
        web="$(nix build .#proof-verify-web --no-link --print-out-paths)"
        tar --sort=name --mtime=@0 --owner=0 --group=0 --numeric-owner \
            -C "${web}" -czf release/proof-verify-web.tar.gz .
        ;;
    sums)
        mv release/SHA256SUMS release/BUNDLE-SHA256SUMS
        mv release/BLAKE3SUMS release/BUNDLE-BLAKE3SUMS
        cp result/nonos.cdx.json result/nonos-build.json release/
        cd release
        assets=(nonos-release-bundle.tar.gz BUNDLE-SHA256SUMS BUNDLE-BLAKE3SUMS provenance.json
                nonos.cdx.json nonos-build.json verifier.wasm proof-verify-web.tar.gz)
        sha256sum "${assets[@]}" > SHA256SUMS
        b3sum "${assets[@]}" > BLAKE3SUMS
        ;;
    *)
        echo "release-assets: unknown step $1" >&2
        exit 2
        ;;
esac
