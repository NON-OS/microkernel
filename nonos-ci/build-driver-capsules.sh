#!/usr/bin/env bash
# Build every driver capsule for one user target through make's own capsule
# rule, then check each ELF is for that target's machine, not only that
# cargo exited 0. C-23 found the rule building x86_64 for the aarch64 lanes
# with every step green; the machine check is what would have caught it.
#
# Usage: build-driver-capsules.sh [aarch64-nonos-user|x86_64-nonos-user]
#
# A driver make never builds (no Capsule.mk, or one mk/ does not include) is
# in no image; it is built here with the rule's own cargo line all the same,
# so a driver cannot rot on one architecture by being left out of make.

set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "${repo_root}"

target="${1:-aarch64-nonos-user}"
case "${target}" in
    aarch64-nonos-user) machine=183 ;;   # EM_AARCH64
    x86_64-nonos-user) machine=62 ;;     # EM_X86_64
    *) echo "build-driver-capsules: unknown user target ${target}" >&2; exit 2 ;;
esac

export NONOS_IN_FLAKE="${NONOS_IN_FLAKE:-1}"

# The capsules make itself builds, as "slug dir bin", asked of make rather
# than read off the Capsule.mk files: a Capsule.mk that mk/ never includes
# is one make never builds.
known="$(make --no-print-directory NONOS_USER_TARGET="${target}" \
    --eval='nonos-ci-capsules: ; @:$(foreach s,$(NONOS_VERIFIED_CAPSULES) $(NONOS_DEV_CAPSULES),$(info NONOS-CI-CAPSULE $(s) $($(s)_DIR) $($(s)_BIN_NAME)))' \
    nonos-ci-capsules | sed -n 's/^NONOS-CI-CAPSULE //p')"

made=() elfs=() direct=()
for dir in userland/capsule_driver_*/; do
    dir="${dir%/}"
    entry="$(awk -v d="${dir}" '$2 == d { print $1, $3 }' <<< "${known}")"
    if [ -n "${entry}" ]; then
        made+=("nonos-mk-${entry%% *}")
        elfs+=("${dir}/target/${target}/release/${entry##* }")
    else
        direct+=("${dir}")
    fi
done
[ "${#made[@]}" -gt 0 ] || { echo "build-driver-capsules: make builds no driver capsules" >&2; exit 1; }

make --no-print-directory NONOS_USER_TARGET="${target}" "${made[@]}"

for dir in "${direct[@]}"; do
    echo "Building ${dir} directly: make never builds it..."
    (cd "${dir}" && cargo build --release --target "../${target}.json" \
        -Zbuild-std=core,alloc -Zbuild-std-features=compiler-builtins-mem)
    # The binary target: a crate may also carry a library, listed first.
    bin="$(cd "${dir}" && cargo metadata --no-deps --format-version 1 \
        | python3 -c 'import json,sys; print(next(t["name"] for t in json.load(sys.stdin)["packages"][0]["targets"] if "bin" in t["kind"]))')"
    elfs+=("${dir}/target/${target}/release/${bin}")
done

python3 - "${machine}" "${elfs[@]}" <<'EOF'
import struct, sys
machine, bad = int(sys.argv[1]), []
for path in sys.argv[2:]:
    try:
        with open(path, "rb") as f:
            head = f.read(20)
    except OSError as e:
        bad.append(f"{path}: {e.strerror}")
        continue
    # ELF64 little-endian, e_machine at offset 18 (System V ABI, ELF header).
    if head[:4] != b"\x7fELF" or head[4] != 2 or head[5] != 1:
        bad.append(f"{path}: not a little-endian ELF64")
    elif struct.unpack_from("<H", head, 18)[0] != machine:
        bad.append(f"{path}: e_machine {struct.unpack_from('<H', head, 18)[0]}, want {machine}")
for line in bad:
    print(f"::error::{line}")
print(f"build-driver-capsules: {len(sys.argv) - 2 - len(bad)} of {len(sys.argv) - 2} "
      f"driver capsules are ELFs for machine {machine}")
sys.exit(1 if bad else 0)
EOF
