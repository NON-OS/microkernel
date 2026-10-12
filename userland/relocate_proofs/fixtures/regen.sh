#!/bin/sh
# NONOS Operating System (AGPL-3.0-or-later)
# Rebuild pie.elf with the linker and flags the kernel links with: ld.lld,
# a static PIE, no text relocations. Prints the result's sha256 and compares
# it with the one recorded in pie.elf.sha256, so anyone can show the
# checked-in fixture is exactly what pie.s and pie.ld produce. A difference
# fails and leaves pie.elf as it was; --update takes the new one.
set -eu
cd "$(dirname "$0")"
LLD="${LLD:-$(ls ~/.rustup/toolchains/*/lib/rustlib/*/bin/rust-lld | head -1)}"
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
llvm-mc -triple=aarch64 -filetype=obj pie.s -o "$work/pie.o"
"$LLD" -flavor gnu -pie --no-dynamic-linker -z text -z max-page-size=0x1000 \
    -T pie.ld "$work/pie.o" -o "$work/pie.elf"
got="$(sha256sum "$work/pie.elf" | cut -d' ' -f1)"
want="$(cut -d' ' -f1 pie.elf.sha256)"
echo "pie.elf sha256 $got"
if [ "$got" = "$want" ]; then
    echo "matches the recorded fixture"
elif [ "${1:-}" = "--update" ]; then
    cp "$work/pie.elf" pie.elf
    echo "$got  pie.elf" > pie.elf.sha256
    echo "updated pie.elf and pie.elf.sha256 (was $want)"
else
    echo "differs from the recorded $want; rerun with --update to take it" >&2
    exit 1
fi
