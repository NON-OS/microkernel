#!/usr/bin/env bash
# Package the verified capsule set `nix build .#capsules` left in ./result
# for the enrollment ceremony: one ELF per capsule at the name make builds
# it under, a BLAKE3 manifest, the commit, the toolchain and the count. The
# list comes from the same make metadata the trust chain uses, so a capsule
# cannot be silently absent. Nothing is signed here.
#
# Usage: enrollment-set.sh [<out dir>]   (default enrollment-build)

set -euo pipefail
cd "$(dirname "$0")/.."

out="${1:-enrollment-build}"
mkdir -p "${out}/elfs"
make -s nonos-mk-print-verified-bins | grep '^userland/' > "${out}/bins.list"
while IFS= read -r bin; do
    name="$(basename "${bin}")"
    built="$(find -L result -mindepth 2 -maxdepth 2 -type f -name "${name}" | head -n1)"
    if [ -z "${built}" ]; then
        echo "::error::nix built no ${name} for ${bin}"
        exit 1
    fi
    cp "${built}" "${out}/elfs/${name}"
done < "${out}/bins.list"
(cd "${out}/elfs" && b3sum -- * | LC_ALL=C sort -k2) > "${out}/BLAKE3SUMS"
git rev-parse HEAD > "${out}/COMMIT"
rustc --version > "${out}/TOOLCHAIN"
wc -l < "${out}/BLAKE3SUMS" | tr -d ' ' > "${out}/COUNT"
echo "capsules: $(cat "${out}/COUNT"), commit $(cut -c1-9 "${out}/COMMIT")"
