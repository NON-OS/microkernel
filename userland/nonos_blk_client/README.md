# nonos_blk_client

A block device behind whichever storage driver capsule owns it, for userland
callers. The installer capsule (`capsule_install`) and the install tool
(`tool_install`) use it to list disks and write the chosen one; the kernel has
its own clients under `src/hardware/` and does not use this crate.

## What is here

- `Driver` (`src/driver/table.rs`): the three drivers it speaks to, with the
  service name, magic and capacity, read, write and flush opcodes copied from
  each driver's `protocol` module:

  | Driver | Services | Magic | Ops (capacity, read, write, flush) |
  |---|---|---|---|
  | NVMe | `driver.nvme0` to `driver.nvme3` | 0x4E4E_564D | 6, 7, 8, 9 |
  | AHCI | `driver.ahci0` to `driver.ahci3` | 0x4E41_4843 | 4, 5, 6, 7 |
  | virtio-blk | `driver.virtio_blk0` | 0x4E42_4C4B | 2, 3, 4, 5 |

  Each driver registers its `0` name today; the others are looked up as well,
  so a kernel that registers more instances has their disks listed.

- `wire`: the shared 20-byte request header, encoded and decoded once.
- `BlockDevice` and `discover`: capacity, read, write and flush in whole
  512-byte sectors. A driver that is registered but does not answer is
  reported, not skipped, so a broken disk shows in the list. So is a
  controller on the bus with no service behind it (`src/device/present.rs`,
  `src/driver/pci.rs`): a driver that gave up exits and its name goes with it.
- Block sizes (`src/device/span.rs`): the NVMe driver takes addresses, counts
  and the capacity in the namespace's own blocks, 512 or 4096 bytes. The
  client asks for the block size (identify namespace) and MDTS (identify
  controller), turns sector ranges into whole blocks, splits them at what one
  command moves, and writes a range that starts or ends inside a block by
  reading that block first. AHCI and virtio-blk are 512-byte, 64 to a request.
  `fs_proofs/src/blk_span_tests.rs` drives this code against a disk in memory.
- `survey`, `scan` and `Disk`: the disks a person chooses from, each with its
  bus, model (NVMe), size, what it holds now (`Contents`), and a confirm word
  made from its serial. The disk the loader's record names as the boot media
  is left off the list (`src/disks/booted.rs`, `nonos_disk::is_boot_media`).
  One found only by a copy of the running loader, with no record, is listed
  but never offered: it may be the boot disk or an earlier install of the
  same build. `Survey::raid` says an Intel RST or VMD controller is on the
  bus; with no disk found, the firmware's storage mode is the likely cause.
  `Disk::refusal` refuses a disk with 4096-byte blocks for an install: the
  writer lays the GPT out in 512-byte blocks, which firmware would not find.
- `DeviceSink`: a `nonos_disk` block sink that writes to a `BlockDevice`, so
  the installer writes the same image the host tests check.
- Every failed request logs one `[BLK] <op> failed ... lba=... sectors=... code=...`
  line (`src/device/refused.rs`); a driver request gives its lba and count in
  the disk's own blocks, with the block size.

## Authority

The crate holds no capability; the capsule that links it does. All three
drivers answer the medium only to the kernel or to a sender holding
`StoreWrite`, which they ask the kernel for on every request, so a caller
without `StoreWrite` gets refusals. Naming the controllers on the bus reads
the kernel's device table, which needs `DeviceEnum`; a caller without it is
refused the table and its list has no rows for missing drivers and no RST
note.

## What it does not do

USB mass storage is not in the table. The three protocols stay separate: a
change in a driver's opcodes has to be made here as well.

See [storage](../../docs/handbook/storage.md) and
[drivers](../../docs/handbook/drivers.md).
