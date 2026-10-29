#!/usr/bin/env bash
# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
# AGPL-3.0-or-later
#
# The DarkFi wallet's and the DarkFi local dev chain's content, from drk,
# darkfid and minerd as darkfi-build.sh built them:
#   tools/nix/sources/darkfi-wallet-box.sh SRC BUILT WALLET LOCALNET
# SRC is the DarkFi clone darkfi-build.sh built from; BUILT holds its OUT
# for each, BUILT/<name>/usr/bin/<name>.
# WALLET gets:
# - usr/bin/drk and usr/bin/nonos-drk, the prompt the listing opens
#   (drk-prompt.c);
# - usr/share/nonos-drk/drk_config.toml, drk's own config at the commit,
#   which nonos-drk copies into the home with the passphrase asked for.
# LOCALNET gets the same, plus:
# - usr/bin/darkfid and usr/bin/minerd;
# - usr/bin/nonos-darkfi-localnet, the program that listing opens
#   (darkfi-localnet.c);
# - their configs at the commit, under usr/share/nonos-darkfi-localnet.
# The compiler is the flake's pkgsStatic gcc (darkfi-deps.nix).
set -euo pipefail
COMMIT=2e2de47c699529600085649b69c5c1878164e79a # tag v0.5.0
src=$(realpath "$1") built=$(realpath "$2") wallet=$(realpath -m "$3") localnet=$(realpath -m "$4")
here=$(dirname "$(realpath "$0")")
[ "$(git -C "$src" rev-parse HEAD)" = "$COMMIT" ] || { echo "$src is not $COMMIT" >&2; exit 1; }
# shellcheck disable=SC1090
. "$(nix build --no-link --print-out-paths -f "$here/darkfi-deps.nix")"
t=x86_64-unknown-linux-musl gcc=$cc/bin/$t-gcc strip=$cc/bin/$t-strip
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
flags=(-static -O2 -s -Wl,--build-id=none)
for p in drk-prompt:nonos-drk darkfi-localnet:nonos-darkfi-localnet; do
  "$gcc" "${flags[@]}" -o "$work/${p#*:}" "$here/${p%%:*}.c"
  "$strip" "$work/${p#*:}"
done
rm -rf "$wallet" "$localnet"
for out in "$wallet" "$localnet"; do
  install -D -m 0755 "$built/drk/usr/bin/drk" "$out/usr/bin/drk"
  install -D -m 0755 "$work/nonos-drk" "$out/usr/bin/nonos-drk"
  install -D -m 0644 "$src/bin/drk/drk_config.toml" "$out/usr/share/nonos-drk/drk_config.toml"
done
for n in darkfid minerd; do
  install -D -m 0755 "$built/$n/usr/bin/$n" "$localnet/usr/bin/$n"
done
install -D -m 0755 "$work/nonos-darkfi-localnet" "$localnet/usr/bin/nonos-darkfi-localnet"
s=$localnet/usr/share/nonos-darkfi-localnet
install -D -m 0644 "$src/bin/darkfid/darkfid_config.toml" "$s/darkfid_config.toml"
install -D -m 0644 "$src/bin/minerd/minerd.toml" "$s/minerd.toml"
