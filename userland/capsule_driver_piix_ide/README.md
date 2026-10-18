# capsule_driver_piix_ide

## Role

`capsule_driver_piix_ide` drives the Intel PIIX3 and PIIX4 IDE controllers
(PCI 8086:7010, 8086:7111) and the ATA disks on their two channels: the
disk controller of QEMU's `pc` machine, VirtualBox's default (IDE, PIIX4
or PIIX3) and Hyper-V Generation 1. It serves each disk on the block wire
(`NBW1`) the kernel's block layer and the installer read.

```text
kernel block layer / installer --NBW1 (unit = slot)--> driver.piix_ide0
                                                           |
          task file per command -> bus master engine -> 64 KiB buffer in DMA
```

## Microkernel contract

- `MkDeviceList` finds the controller: 8086:7010 or 8086:7111, class 01h
  subclass 01h, the bus master window in BAR4 (16 ports), and for each
  channel its command block (BAR `2c`, 8 ports) and control register (BAR
  `2c + 1`). A channel in compatibility mode decodes the ISA addresses
  (0x1F0/0x3F6, 0x170/0x376); the kernel lists them in those BARs
  (`src/hardware/broker/table/legacy_ide.rs`), never the floppy
  controller's ports beside 0x3F6.
- `MkDeviceClaim`; `MkPciConfigWrite` sets Bus Master and Interrupt Disable.
- `MkPioGrant` on BAR4 and on each listed channel's two windows: every
  register access is a kernel `in` or `out` through a grant.
- `MkDmaMap` once, coherent and below 4 GiB: the descriptor table in the
  first page, a 64 KiB transfer buffer after it.
- No `MkMmioMap`, no `MkIrqBind`: nIEN holds each channel's interrupt down
  and the driver polls the engine and the disk.
- `MkIpcRecv` serves `service:4288:driver.piix_ide0`; replies go to
  `endpoint.4294967355`.

## Interface contract

The block wire, `NBW1` (magic 0x4E425731, version 1, 20-byte header: magic,
version, op, flags, unit, request id, payload length). The reply echoes op,
flags, unit and id, then an `i32` status and the op's body.

| Op | Value | Request | Reply body |
|---|---|---|---|
| `HEALTHCHECK` | 1 | none | none |
| `GEOMETRY` | 2 | none | sectors u64, 512 u32, 65536 u32, flags u32 (4: present) |
| `READ` | 3 | lba u64, count u32 (1..=128) | count * 512 bytes |
| `WRITE` | 4 | lba u64, count u32, then count * 512 bytes | none |
| `FLUSH` | 5 | none | none |
| `IDENTIFY` | 6 | none | 64 bytes: lengths, medium 5 (ATA), model, serial |

Units are drive slots: 0 the primary master, 1 the primary slave, 2 the
secondary master, 3 the secondary slave. Every op on a unit with no disk,
the health check among them, answers -6, which is how a client learns the
units. Driver statuses: -13 (the sender may not reach the medium), -22 (an
unknown op, a payload where none belongs, a count of 0 or over 128, a frame
that is not the wire), -90 (a payload whose length is not what the op
carries), -6 (a range past the disk), -19 (IDENTIFY of a name that is not
text). Device statuses: -5 (the disk or the engine reported an error, or
the engine was left with table unused), -110 (the disk did not finish in
10 s; the channel is reset).

## Authority

`CoreExec`, `IPC`, `Memory`, `Driver`, `DeviceEnum`, `Dma`, `Pio`
(`CAPSULE_REQUIRED_CAPS = 0x198019`); `Debug` optional
(`CAPSULE_OPTIONAL_CAPS = 0x100`). No `Mmio`, `Irq` or `Network`. CoreExec
is for the driver's own pid: its serve loop also drains its reply
endpoint, and a reply no caller took comes back there; it is dropped
rather than answered, which would start a flood.

```text
allowed:   the PIIX IDE claim, its port windows, one DMA grant
forbidden: any other device, the floppy controller's ports, network, files
```

Raw sectors hold every partition on a disk, another system's files among
them, and the store the next boot trusts. Only the kernel's own client
(sender pid 0) and a sender the kernel says holds StoreWrite (the
installer, vfs) reach the medium; the kernel is asked on every request.
A health check stays open.

## Privacy and persistence

The driver keeps no sector beyond the request it serves; the buffer is
overwritten by the next. The console names each disk's model and size and
never its serial number, which identifies the one machine.

## Runtime lifecycle

Claim, command bits, the port grants, the DMA grant; both engines stopped
in case firmware left one running. Each listed channel: a status that
floats high means nothing is attached and the channel is left; otherwise a
software reset with nIEN held, then up to 5 s for the devices to leave
busy. Each slot: selected, IDENTIFY DEVICE; a slot that answers zero is
empty, one with the packet signature (14h/EBh) is a CD or DVD drive and is
named and left, an ATA disk's 256 words are read and checked: LBA and DMA
required, 512-byte logical sectors, the capacity from words 100..103 with
LBA48 or 60..61 without, flush from words 82..83. A controller with no disk
behind it is given back whole and the driver leaves.

One request at a time: the range checked against the disk, the task file
worked out (LBA28 when the range ends at or below 2^28, LBA48 otherwise),
the descriptor table written for exactly the bytes moved and cut at every
64 KiB boundary, the engine stopped and pointed at it and its latched
status cleared, the command written, the engine started. The wait yields
for 64 looks, then sleeps a millisecond between them, until the disk holds
neither busy nor data or the engine faults; then up to 5 ms for the engine
to go inactive. Its status is read before it is stopped, since stopping
clears active.

## Failure model

Every bring-up step names its failure and gives back what was taken, the
claim last. A disk that reports an error, or an engine still active with
table unused, answers -5. A disk still busy after 10 s answers -110 and
its channel is reset, so the next request finds it free. Bring-up is tried
a bounded number of times (`nonos_libc::start_driver`).

## Current implemented surface

- Up to four ATA disks, LBA28 and LBA48, bus master DMA, 64 KiB a request.
- FLUSH CACHE or FLUSH CACHE EXT as the disk offers; a disk with no write
  cache answers FLUSH at once.

## Wire format

```text
offset  size  request header        reply header
     0     4  magic 0x4E425731      magic
     4     2  version 1             version
     6     2  op                    op (echoed)
     8     2  flags                 flags (echoed)
    10     2  unit                  unit (echoed)
    12     4  request id            request id (echoed)
    16     4  payload length        4 + body length
    20        payload               i32 status, then the body
```

All fields little endian. The driver's copy of the wire is `src/wire`, the
same items under the same names as `nonos_blk_wire`; at integration it is
replaced by the crate.

## State ownership

The capsule owns the controller, its two channels and the buffer. The
kernel's block layer owns what is read and written, and when.

## Operating rules

- Never let a channel raise its interrupt: nIEN stays set.
- Never write a command to a disk that is busy or holds data.
- Read the engine's status before stopping it.
- Never log a serial number.

## Release target

Disks in QEMU's `pc` machine (`-hda`, `-drive if=ide`), VirtualBox's IDE
controller and Hyper-V Generation 1, read and written through the block
layer the other disk drivers serve.

## Release evidence

`piix_ide_proofs` runs the shipping source against a model of the PIIX bus
master function and the ATA and packet drives on its channels, as ATA, the
82371AB data sheet and QEMU's piix answer: the order of the grants, the
resets with nIEN held, every slot probed, a CD drive and a disk without DMA
left by name, a floating channel and a channel stuck busy, every refusal
and what it gives back, reads and writes byte for byte through the engine,
the 64 KiB cut, LBA48 above and across 2^28, the slave of the secondary,
a slow transfer, a draining engine, a disk error mid-transfer and after it,
a short transfer, a hung disk timed out and its channel reset, the medium
rule, the units, every malformed request, and the kernel's listing of
compatibility-mode channels. A boot is the owner's.

## Release checklist

- Signed manifest and kernel mirror present (`driver_piix_ide` keys).
- `piix_ide_proofs` passes.
- The kernel's block layer row for `driver.piix_ide0` (the lead's).
- QEMU `pc` with `-hda`, VirtualBox IDE and Hyper-V Gen 1 install to and
  boot from the disk.

## Explicit non-goals today

No packet devices (CD and DVD drives: SCSI commands in packets, for
`nonos_scsi`), no PIO data transfers, no transfer mode programming (a
virtual controller needs none), no other vendors' IDE controllers, no
command queueing, no hot plug.

## Verification

- Build: `make -B nonos-mk-driver-piix-ide`
- Proofs: `cd userland/piix_ide_proofs && cargo test --release`
- Static gate: `bash nonos-ci/run-static-checks.sh`
