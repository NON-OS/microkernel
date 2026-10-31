#!/usr/bin/env bash
# Provision the trust material a job signs and boots with, inside the
# flake's shell (`nix develop --command bash nonos-ci/provision-trust.sh`).
#
#   scratch     a throwaway chain made on this runner
#               (scratch-trust-bootstrap.sh) over the flake's capsules,
#               which come from the binary cache rather than a cargo build
#               of each in turn
#   production  the committed trust set, checked first; its STARK
#               enrollment is reused (NONOS_TRUST_REUSE=1) rather than
#               proven afresh, and make checks it against the flake's
#               capsules, the bytes the seal enrolls
#
# Either way the tools the kernel embeds and the Linux userland are laid
# where make reads them (lay-prebuilt.sh). NONOS_CAPSULE_BINS and
# NONOS_TRUST_REUSE are exported to later steps through $GITHUB_ENV.
#
# Usage: provision-trust.sh <scratch|production>
# Reads SIGNING_KEY_BASE64 when set (setup-signing-key.sh).

set -euo pipefail
cd "$(dirname "$0")/.."

mode="${1:?usage: provision-trust.sh <scratch|production>}"
export NONOS_CI_TRUST_MODE="${mode}"

keep() {
    export "$1=$2"
    if [ -n "${GITHUB_ENV:-}" ]; then
        echo "$1=$2" >> "${GITHUB_ENV}"
    fi
}

case "${mode}" in
    scratch)
        bash nonos-ci/setup-signing-key.sh
        keep NONOS_CAPSULE_BINS "$(nix build .#capsules --no-link --print-out-paths)"
        bash nonos-ci/scratch-trust-bootstrap.sh
        ;;
    production)
        bash nonos-ci/setup-signing-key.sh
        make nonos-mk-check-trust-manifest
        keep NONOS_TRUST_REUSE 1
        keep NONOS_CAPSULE_BINS "$(nix build .#capsules --no-link --print-out-paths)"
        ;;
    *)
        echo "::error::unknown trust mode '${mode}'" >&2
        exit 2
        ;;
esac

bash nonos-ci/lay-prebuilt.sh
