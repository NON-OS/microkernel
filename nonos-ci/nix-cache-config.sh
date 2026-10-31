#!/usr/bin/env bash
# Print the nix.conf lines that read the project's public binary cache, for a
# Nix the cachix action does not set up (inside WSL2 or the devcontainer).
# The cache's signing key is asked of Cachix itself, so no key is copied here.
# Prints nothing when no cache is named, and the build runs from source.
#
# Usage: nix-cache-config.sh <cache name>

set -euo pipefail

cache="${1:-}"
[ -n "${cache}" ] || exit 0

key="$(curl -fsSL --retry 5 --retry-all-errors "https://app.cachix.org/api/v1/cache/${cache}" \
    | grep -oE '"publicSigningKeys":\["[^"]+"' | cut -d'"' -f4)"
[ -n "${key}" ] || { echo "nix-cache-config: Cachix names no signing key for ${cache}" >&2; exit 1; }
echo "extra-substituters = https://${cache}.cachix.org"
echo "extra-trusted-public-keys = ${key}"
