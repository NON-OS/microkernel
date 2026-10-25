#!/usr/bin/env bash
# Type-check every proof crate, tests included, for one foreign host triple,
# so kernel and capsule source a proof mounts by #[path] is compiled for that
# architecture's cfg too, not only for the x86_64 host the proofs run on.
# A C build script (blake3's) needs that triple's C compiler on PATH.
#
# Usage: check-proofs-every-arch.sh <aarch64-unknown-linux-gnu|riscv64gc-unknown-linux-gnu>

set -euo pipefail
cd "$(dirname "$0")/.."

triple="${1:?usage: check-proofs-every-arch.sh <triple>}"
case "${triple}" in
    aarch64-unknown-linux-gnu|riscv64gc-unknown-linux-gnu) ;;
    *) echo "check-proofs-every-arch: no lane for ${triple}" >&2; exit 2 ;;
esac
# One target directory for every crate: the dependencies they share are
# checked once, and the runner's disk holds one tree rather than two hundred.
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-${PWD}/target/proofs-${triple}}"

failed=() passed=0
for dir in userland/*_proofs; do
    if (cd "${dir}" && cargo check --quiet --locked --tests --target "${triple}" < /dev/null); then
        passed=$((passed + 1))
    else
        failed+=("${dir}")
    fi
done
for dir in "${failed[@]}"; do echo "::error::${dir} does not type-check for ${triple}"; done
echo "check-proofs-every-arch: ${passed} of $((passed + ${#failed[@]})) proof crates for ${triple}"
[ "${#failed[@]}" -eq 0 ]
