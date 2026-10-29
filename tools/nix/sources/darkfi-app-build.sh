#!/usr/bin/env bash
# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
# AGPL-3.0-or-later
#
# Builds DarkFi's desktop app (bin/app) as a dynamic x86_64 musl program,
# linked by the gcc of the chroot darkfi-app-root.sh made:
#   tools/nix/sources/darkfi-app-build.sh SRC WORK
# SRC is a clone of the commit below, patched here (darkfi-app.patch):
# - no tor or i2p transport (darkfi.patch's net-defaults hunk);
# - miniquad waits for events rather than drawing every frame on the CPU;
# - no multisampled EGL config asked: Mesa's softpipe has none, and miniquad's
#   default of one sample found no config at all (eglChooseConfig, egl.rs);
# - the background image read from the content's absolute path;
# - miniquad from SRC/../miniquad, which starts with no wl_data_device_manager
#   and asks EGL for single-sampled configs, the only ones softpipe has.
# The fonts it embeds are IBM Plex Mono from doc/src/fonts at the commit and
# Noto Color Emoji from Alpine's font-noto-emoji, both pinned below. The
# program is WORK/target/x86_64-unknown-linux-musl/release/darkfi-app; its
# interpreter and RUNPATH are under /usr/lib/nonos-darkfi-app, where
# darkfi-app-pack.sh puts musl and the libraries.
set -euo pipefail
COMMIT=2e2de47c699529600085649b69c5c1878164e79a # tag v0.5.0
RUST=1.97.0
MQ=a48953ab76c8f4d918633bd65becdbceeceedacc MQ_URL=https://github.com/narodnik/miniquad
PLEX=dd9f8c93a6a2f9050ec125953b6292ca2ac7828a18157a5d5d8dc08a5b4d1c36
EMOJI=c2f19f6a404baa7da7a710b018c2892d7b51386983ddca146811f76aea0b6861 # font-noto-emoji 2.042-r0
APP=/usr/lib/nonos-darkfi-app
src=$(realpath "$1") WORK=$(realpath "$2")
here=$(dirname "$(realpath "$0")")
root=$WORK/root t=x86_64-unknown-linux-musl
home=${CARGO_HOME:-$HOME/.cargo} rh=${RUSTUP_HOME:-$HOME/.rustup}
[ "$(git -C "$src" rev-parse HEAD)" = "$COMMIT" ] || { echo "$src is not $COMMIT" >&2; exit 1; }
if git -C "$src" diff --quiet; then git -C "$src" apply "$here/darkfi-app.patch"; fi
git -C "$src" apply --reverse --check "$here/darkfi-app.patch"
# The tag's bin/app/Cargo.lock does not match its manifests; this one does.
cp "$here/../locks/darkfi-app-0.5.0.Cargo.lock" "$src/bin/app/Cargo.lock"
# The app's miniquad, beside SRC, made to start with no data device manager.
mq=$(dirname "$src")/miniquad
[ -d "$mq" ] || { git init -q "$mq" && git -C "$mq" fetch -q --depth 1 "$MQ_URL" "$MQ" && git -C "$mq" checkout -q FETCH_HEAD; }
[ "$(git -C "$mq" rev-parse HEAD)" = "$MQ" ] || { echo "$mq is not $MQ" >&2; exit 1; }
if git -C "$mq" diff --quiet; then git -C "$mq" apply "$here/darkfi-app-miniquad.patch"; fi
git -C "$mq" apply --reverse --check "$here/darkfi-app-miniquad.patch"
cp "$src/doc/src/fonts/IBMPlexMono-Regular.otf" "$src/bin/app/ibm-plex-mono-regular.otf"
cp "$root/usr/share/fonts/noto/NotoColorEmoji.ttf" "$src/bin/app/NotoColorEmoji.ttf"
(cd "$src/bin/app" && sha256sum -c --quiet) <<<"$PLEX  ibm-plex-mono-regular.otf
$EMOJI  NotoColorEmoji.ttf"
# The chroot's gcc and g++, run in the caller's directory; the source, the
# target, cargo's sources, rustup's std and the temp dir are bound into the
# chroot at the paths they have here.
mkdir -p "$WORK/bin" "$WORK/target" "$WORK/tmp"
for c in gcc g++; do
  printf '#!/bin/sh\nexec chroot %s /usr/bin/env -i PATH=/usr/bin:/bin /bin/sh -c '\''cd "$1" && shift && exec "$@"'\'' sh "$PWD" %s "$@"\n' \
    "$root" "$c" >"$WORK/bin/musl-$c"
  chmod +x "$WORK/bin/musl-$c"
done
binds=(/dev "$src" "$WORK/target" "$WORK/tmp" "$home" "$rh")
for d in "${binds[@]}"; do
  mkdir -p "$root$d"
  mountpoint -q "$root$d" || mount --bind "$d" "$root$d"
done
trap 'for d in "${binds[@]}"; do umount "$root$d"; done' EXIT
cd "$src/bin/app"
[[ $(rustc -V) == "rustc $RUST "* ]] || { echo "rustc is not $RUST" >&2; exit 1; }
export CARGO_TARGET_DIR=$WORK/target TMPDIR=$WORK/tmp
export CC_x86_64_unknown_linux_musl=$WORK/bin/musl-gcc CXX_x86_64_unknown_linux_musl=$WORK/bin/musl-g++
export AR_x86_64_unknown_linux_musl=ar
SOURCE_DATE_EPOCH=$(git -C "$src" log -1 --format=%ct)
export SOURCE_DATE_EPOCH CARGO_PROFILE_RELEASE_DEBUG=0 CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1
map="-ffile-prefix-map=$src=/darkfi -ffile-prefix-map=$home=/cargo -ffile-prefix-map=$WORK/target=/target"
export CFLAGS_x86_64_unknown_linux_musl="$map" CXXFLAGS_x86_64_unknown_linux_musl="$map"
remap="--remap-path-prefix=$src=/darkfi --remap-path-prefix=$home=/cargo --remap-path-prefix=$WORK/target=/target"
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=$WORK/bin/musl-gcc
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-C target-feature=-crt-static -C link-self-contained=no \
-C link-arg=-Wl,--dynamic-linker=$APP/ld-musl-x86_64.so.1 -C link-arg=-Wl,--enable-new-dtags \
-C link-arg=-Wl,-rpath,$APP/lib -C link-arg=-Wl,--build-id=none $remap"
cargo build --release --locked --target "$t" --features enable-plugins
