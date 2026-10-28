#!/usr/bin/env bash
# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
# AGPL-3.0-or-later
#
# Builds mesa for the DarkFi app in the chroot darkfi-app-root.sh made:
#   tools/nix/sources/darkfi-app-mesa.sh WORK
# EGL and GLES2 for Wayland only, with gallium's softpipe and no LLVM, so
# nothing is compiled at run time and no page is mapped executable
# (execmem). With no DRM device, EGL's Wayland swrast path draws into
# wl_shm buffers. The prefix is where the app's content puts it, so the
# driver path is built in. The result is WORK/root/build/mesa-dest.
set -euo pipefail
WORK=$(realpath "$1")
VER=24.0.9
SHA256=51aa686ca4060e38711a9e8f60c8f1efaa516baf411946ed7f2c265cd582ca4c
URL=https://archive.mesa3d.org/mesa-$VER.tar.xz
PREFIX=/usr/lib/nonos-darkfi-app/mesa
FLAGS="--prefix=$PREFIX --libdir=lib --buildtype=release -Db_ndebug=true -Dstrip=true
-Dplatforms=wayland -Dgallium-drivers=swrast -Dvulkan-drivers= -Dllvm=disabled
-Ddraw-use-llvm=false -Dexecmem=false -Dglx=disabled -Dgbm=disabled -Ddri3=disabled
-Dxmlconfig=disabled -Dshader-cache=disabled -Dzstd=disabled -Dexpat=enabled -Degl=enabled
-Dgles1=disabled -Dgles2=enabled -Dopengl=true -Dglvnd=false -Dosmesa=false
-Dvalgrind=disabled -Dlibunwind=disabled -Dlmsensors=disabled -Dselinux=false
-Dgallium-vdpau=disabled -Dgallium-va=disabled -Dgallium-xa=disabled -Dgallium-nine=false
-Dgallium-omx=disabled -Dgallium-opencl=disabled -Dgallium-rusticl=false -Dvideo-codecs=
-Dbuild-tests=false -Dtools= -Dperfetto=false -Dandroid-libbacktrace=disabled
-Dmicrosoft-clc=disabled -Dintel-clc=disabled"
root=$WORK/root dl=$WORK/dl/mesa
mkdir -p "$dl" "$root/build"
[ -f "$dl/mesa-$VER.tar.xz" ] || curl -fsSL -o "$dl/mesa-$VER.tar.xz" "$URL"
echo "$SHA256  $dl/mesa-$VER.tar.xz" | sha256sum -c --quiet
rm -rf "$root/build/mesa-$VER" "$root/build/mesa-out" "$root/build/mesa-dest"
tar -C "$root/build" -xf "$dl/mesa-$VER.tar.xz"
mountpoint -q "$root/dev" || mount --bind /dev "$root/dev"
mountpoint -q "$root/proc" || mount -t proc proc "$root/proc"
trap 'umount "$root/proc" "$root/dev"' EXIT
# shellcheck disable=SC2086
chroot "$root" /usr/bin/env -i PATH=/usr/bin:/bin SOURCE_DATE_EPOCH=1717632000 \
  meson setup /build/mesa-out /build/mesa-$VER $FLAGS
chroot "$root" /usr/bin/env -i PATH=/usr/bin:/bin ninja -C /build/mesa-out -j"$(nproc)"
chroot "$root" /usr/bin/env -i PATH=/usr/bin:/bin DESTDIR=/build/mesa-dest \
  ninja -C /build/mesa-out install
