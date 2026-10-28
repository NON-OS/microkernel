# nonos_disk

The disk writer for the NONOS installer: a whole NONOS disk, written sector by
sector through a `BlockSink`. The installer capsule (`capsule_install`), the
install tool (`tool_install`) and `nonos_blk_client` use it. It is a library,
not a capsule, and holds no capability.

## The disk it writes

```text
  0 .. 34             protective MBR, primary GPT
  256 .. 245760       NONOS-STORE  the package store
  245760 .. 262144    NONOS-PLAN   the disk plan, then the key header
  262144 .. E         NONOS-DATA   the data volume the plan names
  E .. end - 33       NONOS-ESP    bootloader, kernel image, boot.cfg
```

The kernel reads the store, the plan and the key header at fixed sectors
(`nonos_disk_map`), so they stay there and the GPT names them as partitions;
the ESP goes last. The ESP is at least 1 GiB (`ESP_SECTORS`), and a disk
smaller than `MIN_DISK_SECTORS` is refused.

## What is here

| Module | What it does |
|---|---|
| `layout` | `Layout::plan`: the extents above for a disk of a given size |
| `gpt`, `guid` | protective MBR, GPT header and entries, the NONOS partition GUIDs; `written_by_nonos` recognises an earlier NONOS disk |
| `fat32` | the ESP: geometry, boot sector and FSInfo, the FAT, directory entries with long names |
| `image` | `NonosImage`: what goes on the ESP |
| `store` | `StoreBuilder`: the package store in `nonos_disk_map`'s container format |
| `carry` | `gather`: what an install carries from the running system into the new store: first-boot setup's answers and the signed programs under `/capsules/` and `/linux/` |
| `plan_sector` | the disk plan naming the data volume; the key header is cleared so the first boot keys the volume with the TPM, and the volume's header ring is zeroed so that boot formats it |
| `session`, `writer` | `install` and `verify`: write every run, read it back, and report a `Receipt`; `update` writes the ESP and the store alone over an installed disk |
| `keep` | `read_installed`: whether an update keeps an installed disk's data, from its table, plan and key header; `DiskAnswers`: setup's answers out of that disk's own store |
| `boot_media` | `is_boot_media`: whether a disk is the medium this machine booted from, by its ESP GUID, an MBR signature, or the loader file |
| `describe` | the rows the installer shows for what will be written, or kept and written |

A `BlockSink` takes `(lba, bytes)` in whole 512-byte sectors: a block driver in
the installers (`nonos_blk_client::DeviceSink`), memory in the host tests, so
the disk the tests read is the disk that gets written.

## Tests

`tests/` holds 70 `#[test]` functions, one property per file: the GPT is valid
for firmware, the ESP reads back by the FAT specification and through
`mtools`, every write is whole sectors, the layout fits every disk size, the
kernel takes what is written, vfs loads the store, verify catches corruption,
and boot-media detection answers any disk without a panic. The `an_update_*`
files hold an update to the key header, the layout, the plan and the table it
leaves, and show it writes nothing outside the ESP and the store, can be
stopped and run again, and carries the disk's own answers. Run `cargo test`
in this directory.

See [storage](../../docs/handbook/storage.md) and
[installer](../../docs/handbook/apps/installer.md).
