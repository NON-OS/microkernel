#!/usr/bin/env bash
# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
# AGPL-3.0-or-later
#
# Makes the Alpine v3.20 chroot darkfi-app-mesa.sh and darkfi-app-build.sh
# build in, under WORK/root, from Alpine's signed index and its static apk.
#   tools/nix/sources/darkfi-app-root.sh WORK
# Downloads go to WORK/dl. The index must be signed by a key pinned in
# userland/capsule_linux/keys/alpine; apk-tools-static must be the package
# that index names; apk then checks every other package against it. The
# packages whose files go into the app are checked by their index checksum.
set -euo pipefail
WORK=$(realpath -m "$1")
here=$(dirname "$(realpath "$0")")
top=$(realpath "$here/../../..")
MIRROR=https://dl-cdn.alpinelinux.org/alpine/v3.20
APK=apk-tools-static-2.14.4-r1.apk
APK_C=Q1Pb1URvdB7Jsq3hFwnZIGI/kjJpI=
BUILD="alpine-baselayout=3.6.5-r0 busybox=1.36.1-r31 build-base=0.5-r3 meson=1.4.0-r2 samurai=1.2-r5
pkgconf=2.2.0-r0 python3=3.12.13-r0 py3-mako=1.3.3-r0 py3-packaging=24.0-r1 py3-yaml=6.0.1-r3
bison=3.8.2-r1 flex=2.6.4-r6 linux-headers=6.6-r0 patchelf=0.18.0-r3 wayland-dev=1.22.0-r4
wayland-protocols=1.36-r0 libdrm-dev=2.4.120-r0 expat-dev=2.8.1-r0 zlib-dev=1.3.2-r0
alsa-lib-dev=1.2.11-r0 libxkbcommon-dev=1.7.0-r0 wayland-libs-egl=1.22.0-r4 font-noto-emoji=2.042-r0"
# What the app bundles: the package, its index checksum.
SHIPS="musl Q1fq7YZT2csoZ/IsiIVUMTPEUU4kc=
libgcc Q1KgmmS1rsCIWxJPwYy0Eu7UTG2Zo=
libstdc++ Q1tsdDvXUeibsds7ugGfooVkJxBbM=
libffi Q1rkFKW5DOuJf9Smqt4nogEoWsaU8=
zlib Q1QGxV6WTnFdBYf/YIm9WZkgRmDRY=
libdrm Q1brzbi3vw4kdqkF4rFCDS9FDmQrs=
wayland-libs-client Q181+IUqzcW02aA26NgfBJlXngab4=
wayland-libs-server Q1hv64a14l0aT6nYNDO4K+G9ctKBo=
wayland-libs-egl Q1GKValbAPItD8AERtAg+NaOxuYY8=
libxkbcommon Q1Bq1Pu46nhPhWOve/lMyDsnTEhJo=
alsa-lib Q1mAnQNQ+X7y+Lf6azylKgR87aNQA=
font-noto-emoji Q1i+tAVu6rbQdGMD4edKTLLnK2PqE="
dl=$WORK/dl root=$WORK/root
[ ! -e "$root" ] || { echo "$root exists" >&2; exit 1; }
mkdir -p "$dl" "$root/etc/apk/keys"
export SSL_CERT_FILE=${SSL_CERT_FILE:-/etc/ssl/certs/ca-certificates.crt}
curl -fsSL -o "$dl/main.APKINDEX.tar.gz" "$MIRROR/main/x86_64/APKINDEX.tar.gz"
curl -fsSL -o "$dl/$APK" "$MIRROR/main/x86_64/$APK"
# The signed index names the static apk; take apk.static from its data.
python3 -I - "$top/tools" "$dl" "$APK" "$APK_C" "$WORK/apk.static" <<'EOF'
import io, sys, tarfile
sys.path.insert(0, sys.argv[1])
from nonos_market_catalogue import alpine_auth, alpine_index
dl, apk, want, out = sys.argv[2:]
alpine_index.signed_index(open(f"{dl}/main.APKINDEX.tar.gz", "rb").read())
data = alpine_auth.package_files(open(f"{dl}/{apk}", "rb").read(), alpine_auth.checksum(want))
t = tarfile.open(fileobj=io.BytesIO(data))
open(out, "wb").write(t.extractfile("sbin/apk.static").read())
EOF
chmod +x "$WORK/apk.static"
cp "$top"/userland/capsule_linux/keys/alpine/*.rsa.pub "$root/etc/apk/keys/"
printf '%s\n' "$MIRROR/main" "$MIRROR/community" >"$root/etc/apk/repositories"
# shellcheck disable=SC2086
"$WORK/apk.static" --root "$root" --initdb --no-cache --update-cache add $BUILD
cp /etc/resolv.conf "$root/etc/"
# Each shipped package as apk installed it, by the checksum it recorded.
db=$root/lib/apk/db/installed
while read -r p c; do
  got=$(awk -v p="$p" -F: '$1=="C"{c=$2} $1=="P"&&$2==p{print c; exit}' "$db")
  [ "$got" = "$c" ] || { echo "$p: $got is not $c" >&2; exit 1; }
done <<<"$SHIPS"
