#!/usr/bin/env bash
# Run proof crates' tests on a foreign architecture under qemu-user, so the
# code they hold is executed there, not only built: std_pal_proofs runs the
# std layer's and libc's thread trampolines and thread pointer reads,
# riscv_kernel_proofs the kernel's saved Context on riscv64, and
# loader_arm_proofs the UEFI loader's own set/way walk on aarch64.
#
# Reads TRIPLE (a Rust target), LINKER (its C compiler), RUNNER (the
# qemu-user command line) and CRATES (space-separated crate directories).

set -euo pipefail
cd "$(dirname "$0")/.."

: "${TRIPLE:?}" "${LINKER:?}" "${RUNNER:?}" "${CRATES:?}"
var="$(tr 'a-z-' 'A-Z_' <<<"${TRIPLE}")"
export "CARGO_TARGET_${var}_LINKER=${LINKER}"
export "CARGO_TARGET_${var}_RUNNER=${RUNNER}"
for crate in ${CRATES}; do
    echo "::group::cargo test ${crate} (${TRIPLE})"
    (cd "${crate}" && cargo test --locked --target "${TRIPLE}")
    echo "::endgroup::"
done
