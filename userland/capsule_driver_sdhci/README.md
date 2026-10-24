# capsule_driver_sdhci

## Role

`capsule_driver_sdhci` is the SD Host Controller capsule. Its job is to own
the SDHCI PCI function in userland, bring up the soldered eMMC behind it,
and serve that part as a disk on the block wire, `driver.sdhci0`. On a
machine with no eMMC host, it serves a card reader's removable slot
instead: an SD memory card, or an MMC card, while one is in it.

On Atom, Celeron and Pentium Silver laptops the only disk is often eMMC
behind an SD Host Controller: the Intel eMMC hosts from Bay Trail to Jasper
Lake and Elkhart Lake, and any SDHCI slot whose capabilities call it
embedded. The eMMC engine is the self-contained `src/emmc/` tree, moved
unchanged from the AHCI capsule, which served it until it had a capsule of
its own: SDHCI 3.0/4.x host with ADMA2, JEDEC eMMC 5.1 card, High Speed at up
to 52 MHz on the widest bus that reads back intact, CMD23 multi-block
transfers, cache flush.

```text
signed capsule
    |
    | MkDeviceList / MkDeviceClaim
    v
SDHCI PCI function -- MkMmioMap(slot BAR) --> user VA
    |
    +-- MkPciConfigWrite --> memory decode, bus master, INTx off
    `-- MkDmaMap --------> ADMA2 descriptor page, 32 KiB data buffer
                               |
kernel block layer <-- block wire (driver.sdhci0) --'
```

## Microkernel contract

The capsule talks to hardware only through the broker:

- `MkDeviceList` lists PCI functions; class 08h subclass 05h are matched
  here (`src/emmc/pci/`), since the broker files them under OTHER.
- `MkDeviceClaim` binds one host to this process; `MkDeviceRelease` gives
  it back when its card does not come up.
- `MkPciConfigRead` reads Slot Information; `MkPciConfigWrite` sets memory
  decoding with the INTx pin off at the claim, bus mastering only once the
  host reset is seen to finish, and bus mastering off again before the
  release.
- `MkMmioMap` maps the slot's register BAR; `MkMmioUnmap` on release.
- `MkDmaMap` maps the ADMA2 descriptor page and the data buffer every
  transfer goes through, below 4 GiB (`MK_DMA_MAP_DMA32`) on a host without
  64-bit addressing, which 32-bit ADMA2 reaches. Neither is unmapped by
  hand: `MkDeviceRelease` frees them once it reads Bus Master back off.
- `MkIpcRecv` and `MkIpcSend` serve `driver.sdhci0` on
  `service:4260:driver.sdhci0`, replying to `endpoint.4294967324`.

The kernel validates the capability token, owns address spaces and the IOMMU
domain, and tears every grant down on exit. It holds no SDHCI or MMC logic.

The driver guards the medium itself (`src/server/medium.rs`, the AHCI
capsule's rule): every op but the health check answers only the kernel's own
block client (sender pid 0) or a sender the kernel says holds `StoreWrite`,
asked with `MkCapCheck` on every request.

## Interface contract

| Operation | Meaning | Reply payload |
|---|---|---|
| `OP_HEALTHCHECK` (1) | server liveness | status word |
| `OP_GEOMETRY` (2) | the disk's size and shape | 20-byte geometry |
| `OP_READ` (3) | read `count` blocks at `lba` | `count * 512` bytes |
| `OP_WRITE` (4) | write `count` blocks at `lba` | status word |
| `OP_FLUSH` (5) | flush the card's cache | status word |

## Authority

The manifest grants `IPC`, `Memory`, `Driver`, `DeviceEnum`, `Mmio` and
`Dma` (`CAPSULE_REQUIRED_CAPS = 0xb8018`). No `Irq`: completions are polled
and the INTx pin is off.

```text
allowed:   device enumeration, one device claim, slot MMIO, DMA, IPC
forbidden: IRQ, PIO, network, filesystem, admin, debug, raw kernel memory
```

## Privacy and persistence

The capsule reads and writes sector payloads only for explicit block wire
requests. It does not parse partitions, mount filesystems, cache disk
payloads, or persist host or card state. All broker grants are process
lifetime resources and are revoked by kernel teardown.

## Runtime lifecycle

Discovery lists every SDHCI function, Intel eMMC hosts first, then any other
host in device list order. Each is opened in turn: claim, memory decode on
with INTx off, slot BAR mapped, a generic host refused unless its slot type
is embedded, then the host reset (Software Reset For All read back clear,
Present State then idle, ADMA2 offered), so ADMA a dead driver left running
reaches nothing. Only then are the two DMA regions mapped and bus mastering
turned on, so a host refused before then never gets it; then the card is
brought from power on to transfer state, and sector 0 read once to prove
the data path. The first host whose card comes up is served for the life of
the capsule. Only when every host was refused for not being embedded is the
first removable slot served, card or no card. Before each request such a slot
reads its card detect and the removal the host latched: a card taken out is
dropped, a card put in is brought up once on a host reset for it (SD
first, then MMC, the host reset again between the two), and READ, WRITE
and FLUSH wait until GEOMETRY has described the card in the slot.

## Failure model

With no SDHCI function in the device list the capsule logs one line and
exits `EXIT_ABSENT` (2) before claiming anything. A host whose claim,
command write, mapping, reset, DMA or card bring-up fails is quiesced
(reset, power off), has bus mastering written off (a no-op until the broker
stores Command words as written, K6), and is released; a host still busy
after its reset is refused (`HostBusy`). When no host gives a card, the
whole walk is retried on the shared bounded schedule
(`nonos_libc::bring_up`), and running out logs the first host's reason and
exits `EXIT_GAVE_UP` (6). Every host and card
wait is on the monotonic clock with its own deadline (`src/emmc/sdhci/`,
`src/emmc/mmc/`). A failed request answers -110 for a timeout, a status
carrying the host's Error Interrupt Status or the card's R1 error bit
(`src/emmc/error/wire_codes.rs`), or -5; the kernel client reads every
status it does not name as an I/O error. A removable slot with no card, or
with one GEOMETRY has not described, answers -123; a card whose write
protect switch is on refuses WRITE with -30.

## Current implemented surface

- Intel eMMC hosts by id and generic embedded SDHCI slots; a removable slot
  on a machine with neither.
- SDHCI reset, power, clock, ADMA2 descriptors and the polled command engine.
- eMMC bring-up: OCR, CID, CSD, EXT_CSD, bus width and High Speed.
- SD memory card bring-up: CMD8, ACMD41 with HCS, the card's own RCA, CSD
  versions 1.0 and 2.0, four lines at Default Speed.
- A card's answers held to what it may say: each bring-up command must
  find the card in the state the sequence put it in; an SD CSD version
  that disagrees with the card's CCS, an RCA of 0, and a sector-mode eMMC
  CSD that does not defer to SEC_COUNT are refused, not followed.
- 512-byte block read, write and cache flush, up to 64 blocks a request.
- The block wire: health, geometry, read, write, flush, identify.

## Wire format

Requests use the block wire's 20-byte header under magic `0x4E425731`
(`NBW1`), version `1`. Replies repeat the header, then a 4-byte status, then
the payload. GEOMETRY answers lba_count u64, lba_bytes u32 (512),
max_transfer_bytes u32 (32768) and flags u32 (bit 0 removable, bit 1 read
only, bit 2 media present). READ and WRITE carry lba u64 and count u32;
WRITE follows them with the data. IDENTIFY answers the part's model and
serial and its medium: 1 eMMC, 2 SD.

## State ownership

The capsule owns the host claim, the slot window, the descriptor page, the
data buffer and the card's state. The kernel block layer owns sector
mapping, disk selection and every filesystem above it.

## Operating rules

- Serve an embedded slot or a known Intel eMMC host; a removable slot only
  on a machine with neither.
- Never send data to a card GEOMETRY has not described.
- Never answer a request outside the disk or larger than one buffer.
- Reset the host before any DMA region is mapped or bus mastering goes on.
- Quiesce the host before any DMA region is freed, and never unmap one by
  hand: the release frees them.
- Report every failure upward; never retry a write silently.

## Release target

A signed SD host capsule serving eMMC and SD cards, on any SDHCI function
and on ACPI-declared hosts, on the block wire.

## Release evidence

Release requires the eMMC path proved against the host model, a boot on an
Intel eMMC laptop that installs to and boots from the eMMC, and QEMU
`sdhci-pci` with an SD card once SD cards are served.

## Release checklist

- Signed manifest, publisher keys and kernel mirror present.
- `userland/emmc_proofs` passes.
- An eMMC laptop boots from its eMMC through `driver.sdhci0`.
- Teardown stops the host, writes bus mastering off, unmaps the window and
  releases the claim, which frees both DMA regions.

## Explicit non-goals today

No SD or SDIO cards, no HS200, HS400 or DDR52, no interrupts, no ACPI-only
hosts, no partition parsing.

## Verification

- Build: `make -B nonos-mk-driver-sdhci`
- Proofs: `cd userland/emmc_proofs && cargo test --release`
- Static gate: `bash nonos-ci/run-static-checks.sh`
