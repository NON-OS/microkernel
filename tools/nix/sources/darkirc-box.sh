#!/usr/bin/env bash
# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
# AGPL-3.0-or-later
#
# The DarkIRC listing's content, from darkirc as darkfi-build.sh built it:
#   tools/nix/sources/darkirc-box.sh SRC DARKIRC OUT
# SRC is the DarkFi clone darkfi-build.sh built from, and DARKIRC its OUT
# for darkirc. OUT gets:
# - usr/bin/darkirc, those bytes;
# - usr/bin/sic, suckless sic, an IRC client, built static from the
#   release pinned below;
# - usr/bin/nonos-darkirc, the program the listing opens
#   (darkirc-launch.c), which joins the two on the box's loopback;
# - usr/share/nonos-darkirc/darkirc_config.toml, darkirc's own default
#   config at the commit, so its first run does not stop to write one.
# The compiler is the flake's pkgsStatic gcc (darkfi-deps.nix).
set -euo pipefail
COMMIT=2e2de47c699529600085649b69c5c1878164e79a # tag v0.5.0
SIC=1.3 SIC_SHA256=30478fab3ebc75f2eb5d08cbb5b2fedcaf489116e75a2dd7197e3e9c733d65d2
src=$(realpath "$1") irc=$(realpath "$2") out=$(realpath -m "$3")
here=$(dirname "$(realpath "$0")")
[ "$(git -C "$src" rev-parse HEAD)" = "$COMMIT" ] || { echo "$src is not $COMMIT" >&2; exit 1; }
# shellcheck disable=SC1090
. "$(nix build --no-link --print-out-paths -f "$here/darkfi-deps.nix")"
t=x86_64-unknown-linux-musl gcc=$cc/bin/$t-gcc strip=$cc/bin/$t-strip
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
curl -fsSL -o "$work/sic.tar.gz" "https://dl.suckless.org/tools/sic-$SIC.tar.gz"
echo "$SIC_SHA256  $work/sic.tar.gz" | sha256sum -c --quiet
tar -C "$work" -xzf "$work/sic.tar.gz"
s=$work/sic-$SIC
sed 's/"sic - 250 LOC are too much!"/"bye"/' "$s/config.def.h" >"$s/config.h"
flags=(-static -O2 -s -ffile-prefix-map="$work"=. -Wl,--build-id=none)
"$gcc" "${flags[@]}" -D_GNU_SOURCE -DVERSION="\"$SIC\"" -I"$s" -o "$work/sic" "$s/sic.c"
"$gcc" "${flags[@]}" -o "$work/nonos-darkirc" "$here/darkirc-launch.c"
rm -rf "$out"
install -D -m 0755 "$irc/usr/bin/darkirc" "$out/usr/bin/darkirc"
install -D -m 0755 "$work/sic" "$out/usr/bin/sic"
install -D -m 0755 "$work/nonos-darkirc" "$out/usr/bin/nonos-darkirc"
"$strip" "$out/usr/bin/sic" "$out/usr/bin/nonos-darkirc"
install -D -m 0644 "$src/bin/darkirc/darkirc_config.toml" "$out/usr/share/nonos-darkirc/darkirc_config.toml"
