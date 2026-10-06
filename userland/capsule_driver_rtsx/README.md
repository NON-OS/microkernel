# capsule_driver_rtsx

Status: half done, not in the 0.9.2 image. The capsule builds and its
proofs pass, but it is not included in mk/20-build.mk, has no trust
certificate, and the kernel never spawns it. Nothing in it runs on any
machine today.

## Role

`driver.rtsx0` drives a Realtek PCIe SD card reader (RTS5227 10ec:5227,
RTS522A 10ec:522a): it brings the reader chip up, identifies an SD card in
the slot and reads it by DMA. It is compared with Linux's
`drivers/misc/cardreader/rtsx_pcr.c`, `rts5227.c` and
`drivers/mmc/host/rtsx_pci_sdmmc.c`; the SD protocol follows the SD
Physical Layer Simplified Specification.

```text
  kernel hardware broker                    capsule driver.rtsx0
  ----------------------                    --------------------
  MkDeviceList  ---- device records ---->   setup::find (10ec, class FFh)
  MkDeviceClaim ---- claim epoch ------->   setup::run
  MkMmioMap     ---- register BAR ------>   hw::Regs (HCBAR..BIER, HAIMR)
  MkPciConfigWrite - MEM|MASTER|INTx off    (no IRQ: BIPR is polled)
  MkDmaMap x2   ---- DMA32, uncached --->   resv: commands + SG table
                                            data: 64 KiB card data
                                                 |
                                                 v
                         init::init_hw -> card::bring_up -> card::read_blocks
                                                 |
                                                 v
                                        "rtsx:" lines via MkDebug
```

## Microkernel contract

A userland capsule. It reaches hardware only through the broker calls in
the diagram: MkDeviceList, MkDeviceClaim, MkMmioMap, MkDmaMap, and the PCI
command write; it gives the mapping, the DMA grants and the device back on
exit (MkMmioUnmap, MkDmaUnmap, MkDeviceRelease). It binds no interrupt
(no MkIrqBind) and holds no port I/O.

## Interface contract

None yet. Service 4290 and reply 4291 (`endpoint.4294967380`) are reserved
in Capsule.mk, but the capsule serves no request: no block service, no
kernel client. Its only output is its log lines.

## Authority

`CAPSULE_REQUIRED_CAPS` = 0xB8018: IPC, Memory, Driver, DeviceEnum, Mmio,
Dma. No Irq. `CAPSULE_OPTIONAL_CAPS` = 0x100 (Debug), for the "rtsx:" lines.

## Privacy and persistence

It keeps nothing. Card data lands in the DMA buffer only for the read that
asked for it; the only bytes it reports are the last two of block 0. No
card content, serial number or CID is logged or stored.

## Runtime lifecycle

Start: find the reader, then claim, map and init within libc `bring_up`'s
bounded retries. Run: look at the slot every 500 ms. A card that arrives
is powered, identified and its block 0 read; a card that fails or leaves
powers the slot down. Exit: none while a reader is present; with none it
exits EXIT_ABSENT.

## Failure model

Every step that stops is named: `rtsx: <step>: <reason>`. A broker refusal
or chip init failure ends the attempt and bring_up retries, then gives up
by name. A card failure powers the slot down and waits for reinsertion.
Every wait is on mk_uptime_ms; a failed or timed-out command buffer is
stopped as rtsx_pci_stop_cmd does.

## Current implemented surface

Chip bring-up in rtsx_pci_init_hw order with the RTS5227 family's ops;
card power-up as the MMC core drives this host; CMD0, CMD8, ACMD41, CMD2,
CMD3, CMD9, CMD7, CMD16 (byte-addressed cards), ACMD6; 25 MHz default
speed, 4-bit bus; single and multi-block ADMA reads. Other readers in
Linux's rtsx_pci_ids are named in the log and left alone.

## Wire format

Command buffer entries, HAIMR, HCBCTLR, HDBCTLR and scatter-gather words
exactly as rtsx_pcr.c builds them (`src/wire`); SD commands and responses
per the SD specification (`src/sd`). No IPC wire format yet.

## State ownership

The capsule alone owns the claimed reader, its register mapping and two
DMA buffers. The kernel owns the IOMMU domain and the grants.

## Operating rules

32-bit DMA only (a map above 4 GiB is refused by name). Interrupt pin off.
No writes to the card. Vendor settings past config offset 255 are not read;
Linux's defaults apply.

## Release target

0.9.3 at the earliest: owner certificate, mk/20-build.mk include, kernel
spawn, a block service and backend, then a photo from a laptop with an
RTS5227 or RTS522A.

## Release evidence

None on hardware. Proofs and builds only (see Verification).

## Release checklist

- [ ] Trust certificate and manifest from the owner
- [ ] Included in mk/20-build.mk and spawned by the kernel
- [ ] Block service and kernel block backend
- [ ] `log rtsx` photo: reader up, card identified, block 0 read
- [ ] Remaining rtsx chip families ported

## Explicit non-goals today

Writes, high speed and UHS-I, 1.8 V signalling, MemoryStick, SDIO, the
other rtsx chips, power management (LTR, OOBS, CLKREQ#, RTS5227 ASPM).

## Verification

`userland/rtsx_proofs`: 26 tests holding the id table and BAR choice, every
register word, BIPR outcomes, response parsing, the SSC clock values and
the CSD, OCR and status fields against Linux and the SD specification. The
capsule builds for x86_64-nonos-user, clippy -D warnings clean. Not
verified on hardware; QEMU has no model of the chip.
