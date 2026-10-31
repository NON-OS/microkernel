#!/usr/bin/env bash
# Install the provers that pin their own toolchains outside the flake, each
# at the version named here, and put them on $GITHUB_PATH.
#
#   elan        the Lean toolchain manager; each project's lean-toolchain
#               picks its Lean
#   verus       Verus and the Rust toolchain it is built against
#   extraction  Charon and Aeneas, the nightly Rust Charon drives, and elan
#
# Usage: install-verifiers.sh <elan|verus|extraction>

set -euo pipefail

VERUS_VERSION="0.2026.06.28.1847ab3"
VERUS_RUST="1.96.0-x86_64-unknown-linux-gnu"
CHARON_RELEASE="nightly-2026.07.02"
AENEAS_RELEASE="nightly-2026.07.06-45061fa"
CHARON_RUST="nightly-2026-06-01"

fetch() { curl --retry 5 --retry-all-errors --proto '=https' --tlsv1.2 -fsSL -o "$2" "$1"; }
on_path() {
    if [ -n "${GITHUB_PATH:-}" ]; then echo "$1" >> "${GITHUB_PATH}"; fi
    export PATH="$1:${PATH}"
}

elan() {
    fetch https://elan.lean-lang.org/elan-init.sh "${RUNNER_TEMP:-/tmp}/elan-init.sh"
    sh "${RUNNER_TEMP:-/tmp}/elan-init.sh" -y --default-toolchain none
    on_path "${HOME}/.elan/bin"
}

case "${1:?usage: install-verifiers.sh <elan|verus|extraction>}" in
    elan)
        elan
        ;;
    verus)
        rustup toolchain install "${VERUS_RUST}" --profile minimal
        fetch "https://github.com/verus-lang/verus/releases/download/release/${VERUS_VERSION}/verus-${VERUS_VERSION}-x86-linux.zip" \
            "${RUNNER_TEMP:-/tmp}/verus.zip"
        unzip -q "${RUNNER_TEMP:-/tmp}/verus.zip" -d "${HOME}/verus"
        on_path "${HOME}/verus/verus-x86-linux"
        ;;
    extraction)
        rustup toolchain install "${CHARON_RUST}" --profile minimal \
            --component rustc-dev,llvm-tools-preview,rust-src
        mkdir -p "${HOME}/exttools"
        fetch "https://github.com/AeneasVerif/charon/releases/download/${CHARON_RELEASE}/charon-linux-x86_64.tar.gz" \
            "${HOME}/exttools/charon.tar.gz"
        fetch "https://github.com/AeneasVerif/aeneas/releases/download/${AENEAS_RELEASE}/aeneas-linux-x86_64.tar.gz" \
            "${HOME}/exttools/aeneas.tar.gz"
        tar -C "${HOME}/exttools" -xzf "${HOME}/exttools/charon.tar.gz"
        tar -C "${HOME}/exttools" -xzf "${HOME}/exttools/aeneas.tar.gz"
        on_path "${HOME}/exttools"
        elan
        ;;
    *)
        echo "install-verifiers: unknown tool $1" >&2
        exit 2
        ;;
esac
