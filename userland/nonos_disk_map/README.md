# nonos_disk_map

Where a NONOS disk keeps what, in 512-byte sectors, and the format of the
package store. A `no_std` library with no dependencies, used by the installer
(`capsule_install`), the disk writer (`nonos_disk`), the vfs server
(`capsule_vfs`) and `fs_proofs`.

## The fixed sectors

| LBA | Constant | What is there |
|---|---|---|
| 0 .. 34 | | protective MBR, primary GPT header and entries |
| 256 .. 131072 | `STORE_BASE_LBA`, `STORE_END_LBA` | the package store: vfs loads it and appends to it |
| 131072 | `PLAN_LBA`, magic `NONOSDP1` | the disk plan: where the data volume lies |
| 131073 | `KEY_LBA`, magic `NONOSDK1` | the key header; a sector without the magic says a TPM keys the volume |
| 262144 .. | `DATA_FLOOR` | the data volume, and anything the plan imports |

`HEADER_RING_SECTORS` (256) is the data volume's header ring; the kernel
formats a volume only over a ring that is all zeros. `MIN_VOLUME_SECTORS` is
the smallest volume the kernel's plan parser takes.

## The store container

From `STORE_BASE_LBA` on (`src/container.rs`):

| Part | Size | Fields |
|---|---|---|
| header | 32 bytes | magic (0..8), version (8..12), entry count (12..16), little-endian |
| entry | 128 bytes | path, NUL padded (0..96); payload offset in bytes from the start of the disk (96..104); length (104..112); `digest16` of the payload, zero for none (112..128) |

Entries follow the header and payloads start on sector boundaries.
`valid_name` holds the rule for a path; `MAX_ENTRIES` and `MAX_TOTAL_BYTES`
bound the store so a full one still ends below `PLAN_LBA`.

## Who else keeps these numbers

The kernel keeps its own copies: it settles on the disk with the store magic at
256 or the plan magic at 131072 (`src/hardware/block_device/identify.rs`),
serves store reads and writes only inside 256..131072
(`src/syscall/microkernel/store_read.rs`, `store_write.rs`), and reads the plan
and key header itself (`src/fs/blockfs_volume/`). The AHCI driver reads the
same two magics to choose a disk. The host tools that pack a store and write a
plan have copies too.

## Tests

`tests/` (10 `#[test]` functions) checks the container format, and reads the
kernel's and the host tools' sources to fail when a number there differs from
the one here: `cargo test` in this directory.

See [storage](../../docs/handbook/storage.md).
