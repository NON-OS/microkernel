# capsule_driver_rtsx

Status: not in the 0.9.2 image (`CAPSULE_NOT_IN_IMAGE := 1`). The capsule
builds and its proofs pass, but it has no trust certificate, is not in
mk/20-build.mk, and the kernel neither embeds nor spawns it. Nothing in it
runs on any machine today.

## Role

`driver.rtsx0` drives a Realtek PCIe SD card reader (RTS5227 10ec:5227,
RTS522A 10ec:522a) and serves the SD card in its slot as a block device:
read, write, flush, capacity and identity. It is compared with Linux's
`drivers/misc/cardreader/rtsx_pcr.c`, `rts5227.c` and
`drivers/mmc/host/rtsx_pci_sdmmc.c`; the SD protocol follows the SD
Physical Layer Simplified Specification.

```text
  kernel hardware broker                    capsule driver.rtsx0
  MkDeviceList  ---- device records ---->   setup::find (10ec, class FFh)
  MkDeviceClaim ---- claim epoch ------->   setup::run (Claim, released last)
  MkMmioMap     ---- register BAR ------>   hw::Regs (HCBAR..BIER, HAIMR)
  MkPciConfigWrite - MEM, INTx off ----->   no MASTER yet (no IRQ: polled)
  MkDmaMap x2   ---- DMA32, uncached --->   resv: commands + SG table
                                            data: 64 KiB card data
  MkPciConfigWrite - MEM|MASTER|INTx off    once engine::stopped is seen
  init::init_hw -> serve::run: slot::look -> card::bring_up (CID kept)
  MkIpcRecv/Reply <- requests ->  admits -> card::read_blocks / write_blocks
```

## Microkernel contract

A userland capsule. It reaches hardware only through the broker calls in
the diagram. It binds no interrupt (no MkIrqBind) and holds no port I/O.
It unmaps nothing itself: MkDeviceRelease clears Bus Master, then tears
down the register map and drains the DMA grants.

## Interface contract

Service `driver.rtsx0` on port 4290; the kernel's client takes replies at
`endpoint.4294967380` (port 4291), a capsule caller by MkIpcReply.
`nonos_blk_client` names it `Driver::Rtsx`. Requests reach the medium only
from the kernel's block layer (pid 0) and a StoreWrite holder, the
installer (`serve/admits.rs`, C6), as the SATA, NVMe and virtio-blk
drivers rule; a health check stays open.

## Wire format

Header: magic "NSDR" (0x4E534452), version 1, op, flags, a reserved half
word, request id, payload length; 20 bytes, little endian. A reply echoes
it, then an i32 status, then the payload. Ops: 1 HEALTHCHECK, 2 CAPACITY
(u64 blocks), 3 READ and 4 WRITE (lba u64, count u32 of 1..=128 blocks,
then the blocks for a write), 5 FLUSH (answers once the card has
programmed everything), 6 IDENTIFY (76 bytes in the SATA driver's layout:
blocks, 512, model and serial lengths, medium 2 for SD, the CID's product
name, its serial as 8 hex digits). Statuses: 0; -5 I/O; -6 past the end;
-13 sender refused; -19 service ended, another card; -22 bad op or count;
-30 write-protect switch; -90 bad length; -110 timed out; -123 no card.
Below it, command buffer, HAIMR, HCBCTLR, HDBCTLR and scatter-gather words
exactly as rtsx_pcr.c builds them (`src/wire`).

## Authority

`CAPSULE_REQUIRED_CAPS` = 0xB8018: IPC, Memory, Driver, DeviceEnum, Mmio,
Dma. No Irq. `CAPSULE_OPTIONAL_CAPS` = 0x100 (Debug), for the "rtsx:" lines.

## Privacy and persistence

It keeps nothing. Each request's data passes through the DMA buffer and
the IPC buffers and is wiped once answered (C8). The card is named in the
log by its size alone; its CID never reaches a log line or a Debug print,
and its serial goes only to a StoreWrite holder's IDENTIFY, for the
installer's erase confirmation (C5).

## Runtime lifecycle

Start: find the reader, claim, map and init within libc `bring_up`'s
bounded retries. Bus Master goes on only once the command and DMA engines
a dead owner may have left armed are stopped and the chip has answered
the reset and flush after the stops (`setup/master.rs`); never before,
and never on a refusal. Run: the slot is looked at before every request that
reaches the card and every 500 ms between them. The first card identified
is served. The served card taken out answers -123 until it is back; a card
of another CID in its place, or one that cannot be identified there, ends
service (-19) for good, since two cards of one size cannot be told apart by
size. A card-detect change latched in BIPR has the card identified again.
Teardown powers a card down, stops both engines, masks BIER and writes the
PCI command with Bus Master clear before the claim is released (C2). The
release is the real guard; the driver's own clear takes effect once the
kernel stores exact bits (K6).

## Failure model

Every step that stops is named: `rtsx: <step>: <reason>`. A request the
card fails has it power-cycled and identified again before the next (C1).
Every wait is by elapsed time, parked between polls past its first
millisecond, with a named failure (C7). A card pulled during a transfer
ends it at once and no CMD12 goes to an empty slot; an all-ones BIPR stops
both engines. A failed write still gets the programming wait (after
CMD12 if multi-block). The longest request, the card identified again
(about 10.5 s) and then a write with every step at its limit (23.3 s),
ends inside the kernel's 35 s wait for a reply (`worst_case_tests.rs`).

## Current implemented surface

Chip bring-up in rtsx_pci_init_hw order with the RTS5227 family's ops;
card power-up as the MMC core drives this host; CMD0, 8, ACMD41, CMD2, 3,
9, 7, 16, ACMD6; 25 MHz, 4-bit bus; ADMA reads (CMD17, 18) and writes
(CMD24, 25), CMD12, and CMD13 until programmed.

## State ownership

The capsule alone owns the claimed reader, its register mapping, the two
DMA buffers and the medium state. The kernel owns the IOMMU domain, the
grants and the choice of its disk.

## Operating rules

32-bit DMA only. Interrupt pin off. A card past what a 32-bit argument
reaches is refused. Vendor settings past config offset 255 are not read.

## Release target

0.9.3 at the earliest: owner certificate, mk/20-build.mk include, kernel
spawn and block backend (track A), then a photo from a laptop.

## Release evidence

None on hardware. Proofs and builds only (see Verification).

## Release checklist

- [ ] Trust certificate and manifest from the owner
- [ ] mk/20-build.mk, kernel spawn and a kernel block backend
- [ ] `log rtsx` photo: reader up, card served, read and written

## Explicit non-goals today

High speed and UHS-I, 1.8 V, SDUC past 2 TiB, MMC, SDIO, MemoryStick, the
other rtsx chips, card-detect by interrupt, power management.

## Verification

`userland/rtsx_proofs`: the pure modules against Linux and the SD
specification, and the setup, engine, card and block-service flows over a
modelled RTS5227 and SD card (libc_shim: virtual clock, broker, DMA, IPC):
teardown order and a late completion, waits, removal, the medium rules,
the wire, deadlines, and hostile sweeps of every parser. Not verified on
hardware; QEMU has no model of the chip.
