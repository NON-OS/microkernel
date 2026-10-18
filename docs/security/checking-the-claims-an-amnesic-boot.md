# Checking the security claims yourself: An amnesic boot writes nothing

Part of [Checking the security claims yourself](checking-the-claims.md).

## An amnesic boot writes nothing

Every boot is amnesic until first-boot setup records the choice to keep data. On
such a boot the file store refuses to keep a file with `EACCES` and writes
`[VFS] refused persist: amnesic boot` through `mk_debug`; a write of all zeros,
which withdraws a record, is let through (`require_persistent`,
`userland/capsule_vfs/src/server/handlers/persist_gate.rs:33-41`).

`tools/nonos-amnesic-check` reads the [package
store](../overview/glossary-linux-personality.md#package-store) table of a raw
disk image: the `NONOSTR1` container at LBA 256, one row per entry with name,
offset, length and digest (`STORE_LBA` and `rows`,
`tools/nonos-amnesic-check:28-48`). `--record FILE` saves the rows before a
boot. `--against FILE` compares after it, names every row the boot added or
changed, and exits 1 when there is one (`added`,
`tools/nonos-amnesic-check:62-71`).

The tool reads at most 64 rows and stops with `<image>: <n> entries, above 64`
when the table holds more (`MAX`, `tools/nonos-amnesic-check:29-40`). The store
format allows 512, and the comment beside that limit puts the image's store at
some 220 entries (`MAX_ENTRIES`,
`userland/nonos_disk_map/src/container.rs:35-41`). The QEMU disk packs the Linux
userland: 19 programs at four files each, 76 rows before Perl's and Tcl's
library files (`LINUX_USERLAND_STORE_ENTRIES`,
`userland/linux_userland/Userland.mk:135-138`,
`userland/linux_userland/Userland.mk:153-181`), so by the code the check stops
at its first command on that disk. A scratch store of 70 rows gave exactly that
error here.

The QEMU boots use `target/qemu-virtio-blk.img`, with the store packed at LBA
256 (`QEMU_BLK_STORE_STAMP`, `mk/40-run.mk:88-89`). The intended use: boot once
so make packs the disk, record it, boot again, stop QEMU, compare.

```sh
python3 tools/nonos-amnesic-check target/qemu-virtio-blk.img --record target/amnesic-before.txt
make nonos-mk-run-serial-log
python3 tools/nonos-amnesic-check target/qemu-virtio-blk.img --against target/amnesic-before.txt
```

Not tested in this release.

A pass ends with `[amnesic] 0 rows added or changed, <n> in all` and exits 0. A
failure first prints `[amnesic] the boot left <name> on disk` for each row. A
disk with no store gives `no NONOSTR1 container at LBA 256` and exit 1. The pass
line and the no-store error both showed on scratch images here. The tool reads
the table, not the data behind it: a write that leaves every row as it was is
not seen, and a row the boot removed is not reported. No make target, workflow
or flake check runs it.
