#!/usr/bin/env bash
# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
# AGPL-3.0-or-later
#
# Builds one of DarkFi's drk, darkirc or darkfid as a static x86_64 musl
# program from the pinned release, stripped, into OUT/usr/bin/<name>.
#   tools/nix/sources/darkfi-build.sh SRC NAME OUT
# SRC is a clone of the commit below, which this patches (darkfi.patch):
# - no tor or i2p transport, so every peer is a plain tcp or tls one;
# - drk's SQLCipher built in, not the system's;
# - darkirc's library an rlib, since static musl makes no cdylib;
# - darkirc builds no RLN proving key, 137 MB each, at its first start.
# The toolchain is rustup's stable, the version below, with its musl and
# wasm32-unknown-unknown std; the rest is darkfi-env.sh. The nix recipe that
# would do this offline from vendored crates is pending.
set -euo pipefail
COMMIT=2e2de47c699529600085649b69c5c1878164e79a # tag v0.5.0
RUST=1.97.0
src=$(realpath "$1") name=$2 out=$(realpath -m "$3")
here=$(dirname "$(realpath "$0")")
case $name in drk | darkirc | darkfid) ;; *) echo "not a program here: $name" >&2; exit 2 ;; esac
[ "$(git -C "$src" rev-parse HEAD)" = "$COMMIT" ] || { echo "$src is not $COMMIT" >&2; exit 1; }
if git -C "$src" diff --quiet; then git -C "$src" apply "$here/darkfi.patch"; fi
git -C "$src" apply --reverse --check "$here/darkfi.patch"
# shellcheck source=darkfi-env.sh
. "$here/darkfi-env.sh"
cd "$src"
# The programs embed zkas circuits and, but for darkirc, the native contracts
# as wasm, which upstream's Makefile builds first.
cargo build --release --locked -p zkas
find src/contract bin/darkirc/proof -name '*.zk' | sort | while read -r zk; do
  "$CARGO_TARGET_DIR/release/zkas" "$zk" -o "$zk.bin"
done
if [ "$name" != darkirc ]; then
  for c in money dao deployooor; do
    w=darkfi_${c}_contract.wasm
    cargo build --release --locked --target wasm32-unknown-unknown -p "darkfi_${c}_contract"
    "$wabt/bin/wasm-strip" -o "src/contract/$c/$w" "$CARGO_TARGET_DIR/wasm32-unknown-unknown/release/$w"
  done
fi
cargo build --release --locked --target "$t" -p "$name"
mkdir -p "$out/usr/bin"
"$cc/bin/$t-strip" -o "$out/usr/bin/$name" "$CARGO_TARGET_DIR/$t/release/$name"
