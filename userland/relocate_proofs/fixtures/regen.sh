#!/bin/sh
# NONOS Operating System (AGPL-3.0-or-later)
# Rebuild pie.elf with the linker and flags the kernel links with: ld.lld,
# a static PIE, no text relocations.
set -eu
cd "$(dirname "$0")"
LLD="${LLD:-$(ls ~/.rustup/toolchains/*/lib/rustlib/*/bin/rust-lld | head -1)}"
llvm-mc -triple=aarch64 -filetype=obj pie.s -o pie.o
"$LLD" -flavor gnu -pie --no-dynamic-linker -z text -z max-page-size=0x1000 -T pie.ld pie.o -o pie.elf
rm pie.o
