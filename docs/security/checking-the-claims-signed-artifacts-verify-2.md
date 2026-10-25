# Checking the security claims yourself: Signed artifacts verify and tampered ones are refused, continued

Part of [Checking the security claims yourself](checking-the-claims.md).

## Signed artifacts verify and tampered ones are refused, continued

```sh
make nonos-mk-attest-refusal-run
```

Not tested in this release.

`tools/nonos-flip-byte` copies a file with one byte inverted, the middle one
unless `--at` names another, and keeps the length, so a size check alone cannot
catch it (`main`, `tools/nonos-flip-byte:29-45`).

```sh
python3 tools/nonos-flip-byte original.bin flipped.bin
```

On a scratch file it exited 0 with one byte changed. An offset past the end
prints `nonos-flip-byte: offset <n> outside <size> bytes` and exits 1. The guest
image uses it to place a tampered copy of the guest suite at
`/linux/bin/tampered` and of `libprobe.so` at `/linux/lib/libprobe_bad.so`, each
beside the good file's proofs, which the personality must refuse however they
are reached (`LINUX_GUEST_TAMPERED` and `LINUX_GUEST_BAD_LIB`,
`userland/linux_guests/GuestFiles.mk:23-43`). The `dyn` guest prints `[GUEST]
dyn ESCAPED: the tampered library loaded` if the bad library loads
(`userland/linux_guests/c/dyn.c:26`).

The tamper targets are broken at this commit. `nonos-mk-tamper`,
`nonos-mk-tamper-restore` and `nonos-mk-tamper-status` run `tamper_kernel.sh`
from `nonos-utils` on the kernel in `ESP_DIR` (`mk/40-run.mk:237-245`), and that
script is not in the tree, so each fails before it touches anything.
`nonos-mk-tamper-run` still boots the current
[ESP](../overview/glossary-dma-pool.md#esp) under `QEMU` without rebuilding it
(`mk/40-run.mk:247-253`). By hand:

```sh
cp target/esp/EFI/nonos/kernel.bin target/kernel.bin.good
python3 tools/nonos-flip-byte target/kernel.bin.good target/esp/EFI/nonos/kernel.bin
make nonos-mk-tamper-run
```

Not tested in this release.

What the loader does with a changed kernel is on [Boot chain and
signatures](boot-chain-and-signatures.md). Copy `target/kernel.bin.good` back
afterwards.
