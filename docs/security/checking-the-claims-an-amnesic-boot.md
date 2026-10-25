# Checking the security claims yourself: An amnesic boot writes nothing

Part of [Checking the security claims yourself](checking-the-claims.md).

## An amnesic boot writes nothing

Every boot is amnesic until first-boot setup records the choice to keep data. On
such a boot the file store refuses to keep a file with `EACCES` and writes
`[VFS] refused persist: amnesic boot` through `mk_debug`. A write of all zeros,
an empty one included, is taken only as a withdrawal (`keep`,
`userland/capsule_vfs/src/server/handlers/persist_rule.rs`): it replaces a
record the disk already holds, at that record's length, and a name the disk
does not hold is never added (`withdraw`,
`userland/capsule_vfs/src/blk/store_withdraw.rs`). The replacement zeroes the
extent the old record leaves (`userland/capsule_vfs/src/blk/store_replace.rs`). Removing a store entry is refused
and an uninstall drops only the RAM copy on such a boot (`store_remove.rs`,
`store_uninstall.rs` in the same directory). Host-tested:
`userland/fs_proofs/src/vfs_disk/tests_withdraw.rs` holds the disk sector for
sector unchanged for zeros and an empty payload under a new name, and
`userland/fs_proofs/src/vfs_persist_rule_tests.rs` walks the rule; both are
caught by a mutant restoring the old append (2026-10-24).

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

### The boot check

`scripts/amnesic_boot_check.sh` boots a sealed image (`NONOS_AMNESIC_IMAGE`)
on the data disk `tools/nonos-qemu` lays the store on, for
`NONOS_AMNESIC_TIMEOUT` seconds, then boots it a second time for
`NONOS_AMNESIC_SETTLE` seconds (120 by default) on the disk, system partition
and variable store exactly as the first boot left them (`tools/nonos-qemu
--keep-machine`; without it nonos-qemu lays a fresh partition and a fresh
store on every boot, which once erased the first boot's writes before they
were judged). That catches a write left for shutdown, or for the next boot to
find. After each boot, not only the last, it compares with what went in:

- the data disk, byte for byte, naming every changed sector;
- the EFI system partition, which QEMU serves read-write from a directory,
  path for path and file for file;
- the firmware's variable store, variable by variable, with
  `tools/nonos-efi-vars`.

`tools/nonos-efi-vars` refuses a store that is not an edk2 NV-data volume
holding an authenticated, formatted, healthy variable store, and refuses a
store after the boot that holds no variable at all, since a booted firmware
always writes some. A changed variable passes only when it is one of the exact
(vendor GUID, name) pairs the firmware writes on its own:

- the variable error flag, the iSCSI attempt order and `Attempt N`, memory type
  statistics, the DHCPv6 client id and the monotonic counter;
- the per-NIC IPv4 and IPv6 records, named for the NIC's own address
  (`--nic-mac`);
- the boot manager's `Boot####`, `BootOrder`, `Key####`, `ConIn`, `ConOut`,
  `ErrOut`, `Lang`, `PlatformLang` and `Timeout`.

`BootNext` is not allowed: it picks the next boot, and the firmware only ever
deletes it.

A verdict names the boot it fell on, `main` or `settle`, and a write by the
first boot shows under both. `scripts/amnesic_boot_check_selftest.sh` puts the
check to a stand-in machine (`scripts/amnesic_standin.py`, nonos-qemu's own
`disk.prepare` and `disk.vars_store`, and a variable store a real first boot
left, `scripts/testdata/ovmf-vars-first-boot.fd.gz`, under this flake's OVMF):
no write passes; a write to the partition, the disk or the variable store by
the first boot fails under both names; one by the second boot fails under its
own. Mutants: the first boot not judged, the second on a fresh machine, the
store never kept. Each is caught. Run it inside `nix develop`.

The serial log of each boot must show the disk driver up and the store answered
from the loader's copy of this disk (`[STORE] the loader's copy of the
store`), or the run is a gap. An idle amnesic boot reads nothing from the disk
itself, because the copy answers every read, and keeps no disk until a write
asks for one. So the check shows that the disk is reachable, not that it was
read: a write would select it through the driver the log shows, and the byte
comparison catches it. Exit 0 pass, 1 a byte, file or foreign variable
changed, 2 a gap.

`tools/nonos-efi-vars --self-test` builds stores for each rule:

- a firmware first boot and a later one, both passing;
- a NONOS-named and an unknown variable, both failing;
- a near name, another NIC's record, `Attempt 0` and `BootNext`, all failing;
- another volume type, another store format, an unhealthy store and an empty
  after-store, all unreadable.

Five mutants are each caught: the volume type unchecked, `BootNext` allowed,
any NIC's record allowed, an empty after-store judged, and the store's health
unchecked. The flake's `static-abi` check runs the self-test.

