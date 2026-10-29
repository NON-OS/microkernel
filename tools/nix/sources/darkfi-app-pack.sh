#!/usr/bin/env bash
# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
# AGPL-3.0-or-later
#
# Lays out the DarkFi app's content and bundles it as a source recipe's
# content is bundled (tools/nonos_market_catalogue/source_content.py):
#   tools/nix/sources/darkfi-app-pack.sh SRC WORK OUT
# after darkfi-app-mesa.sh and darkfi-app-build.sh. OUT/darkfi-app.tar is
# the content: usr/bin/darkfi-app, and under usr/lib/nonos-darkfi-app musl's
# loader, every library the program, mesa and miniquad's dlopen reach, each
# a regular file under its soname, mesa's softpipe driver and the
# background image. The tar is checked against the pack's bounds.
set -euo pipefail
src=$(realpath "$1") WORK=$(realpath "$2") out=$(realpath -m "$3")
here=$(dirname "$(realpath "$0")")
top=$(realpath "$here/../../..")
root=$WORK/root
APP=usr/lib/nonos-darkfi-app
mesa=$root/build/mesa-dest/$APP/mesa/lib
# What miniquad and EGL open by name rather than by NEEDED.
DLOPEN="libEGL.so.1 libGLESv2.so.2 libwayland-client.so.0 libwayland-egl.so.1 libxkbcommon.so.0"
tree=$WORK/content/darkfi-app
rm -rf "$WORK/content"
mkdir -p "$tree/usr/bin" "$tree/$APP/lib" "$tree/$APP/mesa/lib/dri" "$tree/$APP/assets"
strip=strip
cp "$WORK/target/x86_64-unknown-linux-musl/release/darkfi-app" "$tree/usr/bin/darkfi-app"
"$strip" "$tree/usr/bin/darkfi-app"
cp "$root/lib/ld-musl-x86_64.so.1" "$tree/$APP/ld-musl-x86_64.so.1"
cp "$mesa/dri/swrast_dri.so" "$tree/$APP/mesa/lib/dri/swrast_dri.so"
cp "$src/bin/app/assets/bg.png" "$tree/$APP/assets/bg.png"
needed() { readelf -d "$1" | sed -n 's/.*(NEEDED).*\[\(.*\)\]/\1/p'; }
# Each soname from mesa's lib, else the chroot's, until nothing is missing.
want=$DLOPEN
while [ -n "$want" ]; do
  next=""
  for so in $want; do
    [ "$so" = libc.musl-x86_64.so.1 ] && continue
    [ -e "$tree/$APP/lib/$so" ] && continue
    for d in "$mesa" "$root/usr/lib" "$root/lib" ""; do
      [ -n "$d" ] || { echo "no $so" >&2; exit 1; }
      [ -e "$d/$so" ] && { cp -L "$d/$so" "$tree/$APP/lib/$so"; break; }
    done
    "$strip" --strip-unneeded "$tree/$APP/lib/$so"
    next="$next $(needed "$tree/$APP/lib/$so")"
  done
  want=$(needed "$tree/usr/bin/darkfi-app"; needed "$tree/$APP/mesa/lib/dri/swrast_dri.so"; echo "$next")
  want=$(for so in $want; do [ -e "$tree/$APP/lib/$so" ] || [ "$so" = libc.musl-x86_64.so.1 ] || echo "$so"; done)
done
mkdir -p "$out"
python3 -I - "$top/tools" "$WORK/content" "$out/darkfi-app.tar" <<'EOF'
import sys
from pathlib import Path
sys.path.insert(0, sys.argv[1])
from nonos_market_catalogue import digest, pack_size, source_content
body = source_content.bundled(Path(sys.argv[2]), "darkfi-app")
pack_size.check(body)
Path(sys.argv[3]).write_bytes(body)
print(len(body), digest.blake3(body), pack_size.pack_len(body, True))
EOF
