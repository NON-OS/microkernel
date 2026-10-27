# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
# AGPL-3.0-or-later
#
# The environment darkfi-build.sh builds in, sourced with $src, $here and
# $RUST set: the flake's pkgsStatic gcc and static libraries
# (darkfi-deps.nix) for the C side, rustup's Rust for the rest.
# shellcheck shell=bash disable=SC1090
. "$(nix build --no-link --print-out-paths -f "$here/darkfi-deps.nix")"
[[ $(rustc -V) == "rustc $RUST "* ]] || { echo "rustc is not $RUST" >&2; exit 1; }
t=x86_64-unknown-linux-musl
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-$src/target}
home=${CARGO_HOME:-$HOME/.cargo}
work=$CARGO_TARGET_DIR/darkfi-tools
mkdir -p "$work/lib"
# RandomX's build.rs runs cmake with ARCH=native, which would fit the code
# to this machine's CPU. These compilers drop -march=native for -maes, the
# flag RandomX's own default build uses; it picks AES at run time.
for c in gcc g++; do
  printf '#!/bin/sh\nfor a; do shift; [ "$a" = -march=native ] && a=-maes; set -- "$@" "$a"; done\nexec %s "$@"\n' \
    "$cc/bin/$t-$c" >"$work/$c"
  chmod +x "$work/$c"
done
# The pinned RandomX commit is gone from codeberg; GitHub's mirror holds it.
export CARGO_NET_GIT_FETCH_WITH_CLI=true GIT_CONFIG_COUNT=1
export GIT_CONFIG_KEY_0=url.https://github.com/darkrenaissance/RandomX.insteadOf
export GIT_CONFIG_VALUE_0=https://codeberg.org/darkrenaissance/RandomX
# RandomX builds in its git checkout, not under the target; start it clean.
rm -rf "$home"/git/checkouts/RandomX-*/91cd628/build
export CC="$work/gcc" CXX="$work/g++" HOST_CC=gcc HOST_CXX=g++
export CC_x86_64_unknown_linux_musl="$work/gcc" CXX_x86_64_unknown_linux_musl="$work/g++"
export AR_x86_64_unknown_linux_musl="$cc/bin/$t-ar"
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER="$cc/bin/$t-gcc"
export X86_64_UNKNOWN_LINUX_MUSL_OPENSSL_LIB_DIR="$ssl/lib" X86_64_UNKNOWN_LINUX_MUSL_OPENSSL_INCLUDE_DIR="$ssldev/include"
export PKG_CONFIG_ALLOW_CROSS=1 PKG_CONFIG_ALL_STATIC=1 PKG_CONFIG_PATH="$alsa/lib/pkgconfig"
# No build path, time or debug info in the program.
SOURCE_DATE_EPOCH=$(git -C "$src" log -1 --format=%ct)
export SOURCE_DATE_EPOCH CARGO_PROFILE_RELEASE_DEBUG=0 CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1
map="-ffile-prefix-map=$src=/darkfi -ffile-prefix-map=$home=/cargo -ffile-prefix-map=$CARGO_TARGET_DIR=/target"
export CFLAGS_x86_64_unknown_linux_musl="$map" CXXFLAGS_x86_64_unknown_linux_musl="$map" CFLAGS="$map" CXXFLAGS="$map"
remap="--remap-path-prefix=$src=/darkfi --remap-path-prefix=$home=/cargo --remap-path-prefix=$CARGO_TARGET_DIR=/target"
# musl's libc and crt are the gcc's, and so is the unwinder: Rust's std asks
# for -lunwind, here gcc's libgcc.a, whose __register_frame takes the whole
# .eh_frame wasmer hands it (LLVM's libunwind takes one FDE and refuses).
# __rust_probestack, which wasmer calls, is darkfi-probestack.c.
cp "$("$cc/bin/$t-gcc" -print-libgcc-file-name)" "$work/lib/libunwind.a"
"$cc/bin/$t-gcc" -c -o "$work/lib/probestack.o" "$here/darkfi-probestack.c"
# pkgsStatic's libstdc++.a is not PIC, so the program is a static ET_EXEC,
# which the personality loads at its own addresses (image/interp.rs).
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-C target-feature=+crt-static -C link-self-contained=no \
-C relocation-model=static -L native=$work/lib -C link-arg=$work/lib/probestack.o -C link-arg=-l:libc.a $remap"
# The contracts import their host calls; rustc no longer lets wasm-ld leave
# a symbol undefined unless asked.
export CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS="-C link-arg=--allow-undefined $remap"
