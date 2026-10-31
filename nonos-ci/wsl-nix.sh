#!/usr/bin/env bash
# The Windows job, run inside WSL2 as a Windows contributor works: Nix in an
# Ubuntu distribution, a clone in the Linux filesystem, and the same make
# commands as on Linux.
#
#   install [cache]   single-user Nix, the version and installer checksum the
#                     devcontainer pins (.devcontainer/Dockerfile); with a
#                     cache name, it also reads that binary cache
#   clone             copy the checkout into ~/nonos, line endings as committed
#   make <target>...  make each target in turn in ~/nonos, one rustc at a time
#
# One rustc at a time: the default fan-out runs several kernel profiles'
# rustc at once, which overruns the WSL2 VM's memory, and rustc takes SIGBUS
# (signal 7) faulting in a memory-mapped file, which reads as a compiler
# crash. NONOS_JOBS and CARGO_BUILD_JOBS are ?= in mk/00-config.mk.

set -euo pipefail

step="${1:?usage: wsl-nix.sh <install [cache]|clone|make <target>...>}"
shift

case "${step}" in
    install)
        here="$(cd "$(dirname "$0")/.." && pwd)"
        dockerfile="${here}/.devcontainer/Dockerfile"
        sha="$(sed -nE 's/^ARG NIX_INSTALLER_SHA256=([0-9a-f]{64})$/\1/p' "${dockerfile}")"
        url="$(grep -oE 'https://releases\.nixos\.org/nix/nix-[0-9.]+/install' "${dockerfile}" | head -n1)"
        [ -n "${sha}" ] && [ -n "${url}" ] || { echo "wsl-nix: no pinned installer in ${dockerfile}" >&2; exit 1; }
        # Nix run as root builds as the nixbld users, and the single-user
        # installer stops when the group is missing.
        if ! getent group nixbld >/dev/null; then
            groupadd --system nixbld
            for i in $(seq 1 8); do
                useradd --system --no-create-home --home-dir /var/empty \
                    --shell /usr/sbin/nologin --gid nixbld --groups nixbld "nixbld${i}"
            done
        fi
        curl -fsSL --retry 5 --retry-all-errors -o /tmp/nix-install "${url}"
        echo "${sha}  /tmp/nix-install" | sha256sum -c -
        sh /tmp/nix-install --no-daemon --yes
        mkdir -p ~/.config/nix
        {
            echo "experimental-features = nix-command flakes"
            bash "${here}/nonos-ci/nix-cache-config.sh" "${1:-}"
        } > ~/.config/nix/nix.conf
        cat ~/.config/nix/nix.conf
        ;;
    clone)
        git config --global --add safe.directory '*'
        git clone -q --no-hardlinks . ~/nonos
        ;;
    make)
        # shellcheck disable=SC1090,SC1091
        . ~/.nix-profile/etc/profile.d/nix.sh
        cd ~/nonos
        for target in "$@"; do
            NONOS_JOBS=1 CARGO_BUILD_JOBS=1 make "${target}"
        done
        df -h / | tail -1
        ;;
    *)
        echo "wsl-nix: unknown step ${step}" >&2
        exit 2
        ;;
esac
