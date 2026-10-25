#!/usr/bin/env bash
# Build every NONOS capsule make knows for one user target, through make's
# own rule, and check each ELF is for that target's machine. A capsule that
# cannot build for the target has a row in nonos-ci/arch-reach/capsules.txt
# saying why, and is left out; arch_reach_proofs holds that file's shape.
#
# Usage: build-every-capsule.sh <user target> [<shard>/<shards>] [--recheck]
#   <shard>/<shards>  build only every <shards>th capsule from <shard> (1-based)
#   --recheck         build only the excused capsules, and fail on any that
#                     builds: its row is stale and must go
#
# Each capsule's own target directory is removed once its ELF is checked,
# so the lane fits a runner's disk.

set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "${repo_root}"

target="${1:?usage: build-every-capsule.sh <user target> [k/n] [--recheck]}"
shard="1/1" recheck=0
for arg in "${@:2}"; do
    case "${arg}" in
        --recheck) recheck=1 ;;
        */*) shard="${arg}" ;;
        *) echo "build-every-capsule: unknown argument ${arg}" >&2; exit 2 ;;
    esac
done
case "${target}" in
    x86_64-nonos-user) machine=62 ;;     # EM_X86_64
    aarch64-nonos-user) machine=183 ;;   # EM_AARCH64
    riscv64-nonos-user) machine=243 ;;   # EM_RISCV
    *) echo "build-every-capsule: unknown user target ${target}" >&2; exit 2 ;;
esac
k="${shard%/*}" n="${shard#*/}"
[[ "${k}" =~ ^[0-9]+$ && "${n}" =~ ^[0-9]+$ && "${k}" -ge 1 && "${k}" -le "${n}" ]] \
    || { echo "build-every-capsule: bad shard ${shard}" >&2; exit 2; }

export NONOS_IN_FLAKE="${NONOS_IN_FLAKE:-1}"
excused="$(awk -v t="${target}" '/^[^#[:space:]]/ && index("," $2 ",", "," t ",") { print $1 }' \
    nonos-ci/arch-reach/capsules.txt)"

known="$(bash nonos-ci/list-capsules.sh "${target}")"
[ -n "${known}" ] || { echo "build-every-capsule: make builds nothing for ${target}" >&2; exit 1; }

built=0 bad=() stale=() i=0
while read -r _ dir bin; do
    i=$((i + 1))
    [ $(( (i - 1) % n + 1 )) -eq "${k}" ] || continue
    # The ELF itself, make's own rule for it: signing and attesting it is
    # the image's business, and that chain is x86_64's alone today.
    elf="${dir}/target/${target}/release/${bin}"
    if grep -qxF "${dir}" <<< "${excused}"; then
        [ "${recheck}" -eq 1 ] || { echo "excused: ${dir} (${target})"; continue; }
        if make --no-print-directory NONOS_USER_TARGET="${target}" "${elf}" \
            < /dev/null > /dev/null 2>&1; then stale+=("${dir}"); fi
    elif [ "${recheck}" -eq 1 ]; then
        continue
    elif ! make --no-print-directory NONOS_USER_TARGET="${target}" "${elf}" < /dev/null; then
        bad+=("${dir}: the build failed")
    elif why="$(python3 nonos-ci/elf-machine.py "${machine}" "${elf}")"; then
        built=$((built + 1))
    else
        bad+=("${why}")
    fi
    rm -rf "${dir}/target"
done <<< "${known}"

for line in "${bad[@]}"; do echo "::error::${line}"; done
for dir in "${stale[@]}"; do
    echo "::error::${dir} builds for ${target}: drop its row in nonos-ci/arch-reach/capsules.txt"
done
echo "build-every-capsule: ${built} capsules are ELFs for machine ${machine} (shard ${shard})"
[ "${#bad[@]}" -eq 0 ] && [ "${#stale[@]}" -eq 0 ]
