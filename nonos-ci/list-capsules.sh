#!/usr/bin/env bash
# Print "slug dir bin" for every NONOS capsule make builds for one user
# target, asked of make itself rather than read off the Capsule.mk files: a
# Capsule.mk that mk/ never includes is one make never builds. A capsule
# built for a foreign triple (the Linux userland on musl) keeps its own
# target and is left out.
#
# Usage: list-capsules.sh <user target>

set -euo pipefail
cd "$(dirname "$0")/.."
target="${1:?usage: list-capsules.sh <user target>}"
row='$(info NONOS-CI-CAPSULE $(s) $($(s)_DIR) $($(s)_BIN_NAME) $($(s)_TARGET))'
each="\$(foreach s,\$(NONOS_VERIFIED_CAPSULES) \$(NONOS_DEV_CAPSULES),${row})"
NONOS_IN_FLAKE="${NONOS_IN_FLAKE:-1}" make --no-print-directory \
    NONOS_USER_TARGET="${target}" --eval="nonos-ci-capsules: ; @:${each}" nonos-ci-capsules \
    | sed -n 's/^NONOS-CI-CAPSULE //p' | awk -v t="${target}" '$4 == t { print $1, $2, $3 }'
