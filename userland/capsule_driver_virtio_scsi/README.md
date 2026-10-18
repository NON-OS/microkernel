# capsule_driver_virtio_scsi

## Role

`capsule_driver_virtio_scsi` drives virtio's SCSI host adapter (PCI
1af4:1048, and 1af4:1004 where it carries the modern capabilities, as
QEMU's `virtio-scsi-pci` does) and serves the disks and CD/DVD drives on
its targets on the block wire (`NBW1`) the kernel's block layer and the
installer read. The SCSI commands are `nonos_scsi`'s and the block wire
is `nonos_scsi_serve`'s, shared with every SCSI host driver; this capsule
only moves a command and its data down the request queue.

```text
kernel block layer / installer --NBW1 (unit)--> driver.virtio_scsi0
       nonos_scsi_serve -> nonos_scsi (CDB, sense, retries) -> VirtioHba
             request queue: header | data out | response | data in
```

## Microkernel contract

- `MkDeviceList` finds the first virtio-scsi function.
- `MkDeviceClaim`; `MkPciConfigWrite` sets Memory Space, Bus Master and
  Interrupt Disable.
- `MkPciConfigRead` for the capability walk (`nonos_virtio`), `MkMmioMap`
  for the common, notify and device configuration regions; a function
  without the modern capabilities is refused (no legacy transport).
- `MkDmaMap` once, coherent: the ring in the first page, the request header
  and its response in the second, a 64 KiB buffer after them.
- No `MkIrqBind`: the available ring asks for no interrupt and the driver
  polls the used ring.
- `MkIpcRecv` serves `service:4292:driver.virtio_scsi0`; replies go to
  `endpoint.4294967356`.

## Interface contract

The block wire as `nonos_scsi_serve` serves it:

| Op | Value | Request | Reply body |
|---|---|---|---|
| `HEALTHCHECK` | 1 | none | none |
| `GEOMETRY` | 2 | none | blocks u64, block bytes u32, max transfer u32, flags u32 |
| `READ` | 3 | lba u64, count u32 | count * block bytes |
| `WRITE` | 4 | lba u64, count u32, then the blocks | none |
| `FLUSH` | 5 | none | none |
| `IDENTIFY` | 6 | none | 64 bytes: vendor and product, serial, medium 6 (SCSI) |

Units are numbered in the order the scan finds them: by target, then by
LUN. GEOMETRY's flags: 1 removable, 2 read-only, 4 a medium in; a CD drive
is asked again each time, so a disc changed is served as it is now. Every
op on a unit with no disk answers -6. Statuses beside the wire's own: -5
(the target or the device failed the command, or moved less than asked),
-6 (a block past the medium), -30 (write protected or a CD drive), -110
(the device did not return the request in 30 s; it was reset and brought
back), -123 (no disc in the drive).

## Authority

`CoreExec`, `IPC`, `Memory`, `Driver`, `DeviceEnum`, `Mmio`, `Dma`
(`CAPSULE_REQUIRED_CAPS = 0xB8019`); `Debug` optional
(`CAPSULE_OPTIONAL_CAPS = 0x100`). No `Pio`, `Irq` or `Network`. CoreExec
is for the driver's own pid: its serve loop also drains its reply endpoint,
and a reply no caller took is dropped rather than answered.

```text
allowed:   the virtio-scsi claim, its three register regions, one DMA grant
forbidden: any other device, the control and event queues, network, files
```

Raw sectors hold every partition on a disk, another system's files among
them, and the store the next boot trusts. Only the kernel's own client
(sender pid 0) and a sender the kernel says holds StoreWrite reach the
medium; the kernel is asked on every request. A health check stays open.

## Privacy and persistence

The driver keeps no block beyond the request it serves. The console names
each unit's address, vendor, product and size, never its serial number.

## Runtime lifecycle

Claim, command bits, the capability walk, the three regions, the DMA grant.
The handshake: reset, ACKNOWLEDGE, DRIVER, VERSION_1 (and ACCESS_PLATFORM
when offered) with no device feature, FEATURES_OK, the CDB and sense sizes
set to 32 and 96 and read back, the request queue (2) at up to 16 entries,
DRIVER_OK; the control and event queues stay off. `max_sectors` bounds the
buffer when it is smaller. Then every target from 0 to `max_target`: REPORT
LUNS on LUN 0 (no target answers BAD_TARGET and costs that one request),
each LUN it lists up to `max_lun` probed with INQUIRY, the serial page,
TEST UNIT READY, READ CAPACITY and MODE SENSE (`nonos_scsi`). An adapter
with nothing on it is given back and the driver leaves.

One request at a time: the header (LUN, a fresh id, simple task, the CDB
padded to 32 bytes), the chain built with everything the device reads
first, the doorbell, then the wait: 64 looks a yield apart, then a
millisecond between looks, up to 30 s.

## Failure model

Every bring-up step names its failure and gives back what was taken, the
claim last. BAD_TARGET ends a command as no target; RESET, ABORTED,
BUSY and TRANSPORT_FAILURE are tried again (four sends at most, with unit
attentions and BUSY status under the same rule); any other response fails
it. A request the device never returns resets it, which stops its use of
the ring, and brings it back on a fresh ring; one that does not come back
is given up on and every later request answers -5.

## Current implemented surface

- Up to 16 units: disks of 512 to 4096-byte blocks, READ/WRITE (10) and
  (16), SYNCHRONIZE CACHE (10) and (16); CD/DVD drives read-only with their
  disc tracked.

## Wire format

The block wire header and the ops as `nonos_scsi_serve/src/wire` defines
them (the same items as `nonos_blk_wire`); the virtio-scsi request and
response as virtio 1.2, 5.6.6 lays them out (`src/constants/scsi.rs`).

## State ownership

The capsule owns the adapter, the ring and the buffer. The kernel's block
layer owns what is read and written, and when.

## Operating rules

- One request out at a time; nothing is sent to a device that did not come
  back.
- Never take a device feature: no hot plug, no events, no protection
  information.
- Never log a serial number.

## Release target

Disks and CD drives on QEMU's `virtio-scsi-pci` (`-device virtio-scsi-pci
-device scsi-hd,drive=...`), read and written through the block layer.

## Release evidence

`virtio_scsi_proofs` runs the shipping source, the handshake and request
queue included, and `nonos_scsi_serve` against a virtio-scsi model: the
transport proofs' common configuration, the device configuration and a
request queue answered from `nonos_scsi`'s simulated logical units. The
handshake order and every refusal, the scan order across targets and LUNs,
the LUN field, the chain shapes, reads and writes byte for byte, geometry
of disks and of a CD drive with and without a disc, identify, flush, every
response code, sense and residuals, a request never returned and the
device brought back, the device given up on, every malformed request and
the medium rule. `nonos_scsi`'s own tests hold the commands, the sense
verdicts and the probe. A boot is the owner's.

## Release checklist

- Signed manifest and kernel mirror present (`driver_virtio_scsi` keys).
- `virtio_scsi_proofs` and `nonos_scsi` tests pass.
- The kernel's block layer row for `driver.virtio_scsi0` (the the developers').
- QEMU with `virtio-scsi-pci` and a `scsi-hd` installs to and boots from it.

## Explicit non-goals today

No multiple request queues, no hot plug or event queue, no task management
(abort, reset) beyond the device reset on a lost request, no protection
information, no tape or other device types, no legacy-only transport.

## Verification

- Build: `make -B nonos-mk-driver-virtio-scsi`
- Proofs: `cd userland/virtio_scsi_proofs && cargo test --release`
- Library: `cd userland/nonos_scsi && cargo test`
- Static gate: `bash nonos-ci/run-static-checks.sh`
