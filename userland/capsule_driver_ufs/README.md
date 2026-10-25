# capsule_driver_ufs

Status: wired, not yet signed. The kernel mirror (`src/hardware/ufs_capsule`)
embeds and spawns it under `nonos-capsule-driver-ufs` (the
`microkernel-driver-ufs` build) when the inventory sees a UFS host, and the
kernel's block wire client and the installer (`nonos_blk_client`,

## Role

`capsule_driver_ufs` is the UFS host controller (UFSHCI) driver capsule,
written from JESD223 and JESD220. It owns the PCI function, BAR0, the UTP transfer and
task management lists, one command descriptor and a 64 KiB data region, and
serves each UFS logical unit as a unit on the block wire. QEMU's `ufs`
device (with `ufs-lu` units) drives this path.

```text
kernel block wire client / installer
    |
    | block wire (NBW1), units 0..7 = the LUs REPORT LUNS names
    v
driver.ufs0 -- broker MMIO + DMA --> UFSHCI --> UFS device (SCSI LUs)
    |
    `-- polled: no interrupt is bound
```

## Microkernel contract

- `MkDeviceList` finds the first UFS function (class 01h/09h) whose ID is
  in `platform/hosts.rs`; any other is named in the log and left.
- `MkDeviceClaim` claims it; `MkPciConfigWrite` turns on Memory Space and
  Bus Master Enable and sets INTx Disable.
- `MkMmioMap` maps BAR0; `MkDmaMap` takes the four regions.
- `MkIpcRecv` serves `service:4238:driver.ufs0`; replies go to
  `endpoint.4294967325`.

## Interface contract

The block wire as `driver.sdhci0` serves it (`nonos_blk_wire`): HEALTHCHECK,
GEOMETRY (blocks, 512 or 4096-byte block size, 64 KiB per request, media
present), READ, WRITE, FLUSH (SYNCHRONIZE CACHE) and IDENTIFY (INQUIRY
vendor and product, VPD serial, medium 3 UFS). A unit the device does not
have answers -6.

## Authority

`CAPSULE_REQUIRED_CAPS = 0xB8018`: IPC, Memory, Driver, DeviceEnum, Mmio,
Dma, as `driver.sdhci0` holds. No Irq, Crypto, Network, filesystem or admin
authority. Raw sectors reach only the kernel's client and senders the
kernel says hold StoreWrite (`nonos_blk_wire::medium_rule`).

## Current implemented surface

Host table for 12 IDs; UFSHCI 1.x and 2.0+ command types; one slot, polled
completion with UTRLCLR on timeout; NOP OUT and fDeviceInit; REPORT LUNS,
TEST UNIT READY, READ CAPACITY (10, 16), INQUIRY (standard, VPD 0x80),
READ and WRITE (10, 16), SYNCHRONIZE CACHE; up to eight units.

## Wire format

As `driver.sdhci0` (`nonos_blk_wire`): 20-byte NBW1 header carrying the
unit, 4-byte status, per-op data.

## State ownership

The capsule owns the lists, the command descriptor and the data region; the
kernel owns the grants; the filesystems above own what the sectors mean.

## Operating rules

One request at a time, in slot 0. Every wait is on the uptime clock,
bounded as `ufs/facts/bounds.rs` sets out with each bound's source. Every
step that can fail logs one `driver.ufs:` line saying what the host or
device answered.
A response is taken only when its transaction type, unit, tag and (for a
query) opcode and IDN match what was asked; the data region is zeroed
before every read, so a device that answers GOOD without data hands back
zeroes, not the last request's bytes; a unit whose block count times block
size overflows 64 bits is refused.

## Privacy and persistence

Sectors pass through the data region and IPC only while a request is
served. Nothing is stored by the capsule.

## Runtime lifecycle

Discover, claim, map, take the regions, then bring the host up stage by
stage (`ufs/bring_up/stage.rs`): HCE reset and enable, DME_LINKSTARTUP
(three retries, LCC cleared first on Intel hosts), the lists run, NOP OUT, fDeviceInit set and seen
cleared, REPORT LUNS, and for each unit TEST UNIT READY, READ CAPACITY and
INQUIRY. Then serve the units until the process ends.

## Failure model

No listed host: one line and `EXIT_ABSENT`. A failing step logs which and
why, resets the host, gives back every grant, and the attempt is retried on
the shared bounded schedule; running out is `EXIT_GAVE_UP`. A unit that is
not ready within five seconds, or whose blocks are not 512 or 4096 bytes,
is named and not served. A request the device never finishes is cleared
through UTRLCLR and answered -110.

## Release target

A signed, spawned block driver for the UFS storage of Intel UFS laptops and
tablets (Elkhart Lake to Panther Lake) and for QEMU's `ufs` device.

## Release evidence

A boot with QEMU's `ufs` and `ufs-lu` showing the `LU 0:` line, the
installer listing the unit, and a write read back; then one UFS machine.

## Release checklist

- A QEMU UFS lane, and one machine photographed with the `driver.ufs:` lines.

## Explicit non-goals today

No high-speed gear: the link stays in PWM-G1, one lane, as link startup
leaves it, which is slow on hardware. No interrupts, MCQ, task management,
inline crypto, write booster, RPMB, power management or descriptor writes.
No ACPI-only UFS host. The installer lists unit 0 alone, as it asks every
block wire driver; the kernel's client reaches all eight.

## Verification

- Build: `cargo build --release --target ../x86_64-nonos-user.json
  -Zbuild-std=core,alloc -Zbuild-std-features=compiler-builtins-mem`
- Proofs: `cd userland/ufs_proofs && cargo test`
- Not verified under QEMU or on any hardware yet.
