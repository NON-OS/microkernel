# capsule_driver_ahci

## Role

`capsule_driver_ahci` is the SATA AHCI controller capsule. Its job is to own
the AHCI PCI function in userland, expose the controller's identity and port
state over IPC, and keep SATA command policy out of the kernel.

driver.ahci0 also serves eMMC hosts until it gets its own publisher keys.
On Atom, Celeron and Pentium Silver laptops the only disk is often soldered
eMMC behind an SD Host Controller (Intel eMMC hosts from Bay Trail to Jasper
Lake and Elkhart Lake, and any SDHCI slot its capabilities call embedded).
When no SATA disk comes up, the capsule brings that eMMC up instead and
serves it over the same wire: the same ops, 512-byte sectors and 64-sector
requests, with IDENTIFY's medium byte set to 1 so the installer names it
eMMC. The eMMC code is the self-contained `src/emmc/` tree (SDHCI host with
ADMA2, High Speed at up to 52 MHz on the widest bus that reads back intact,
CMD23 multi-block transfers, cache flush), proved by `userland/emmc_proofs`
against a register-level model of a host with an eMMC behind it. It moves
to a capsule of its own with that directory. It asks for no capability the
SATA path did not already hold.

This slice is a block-controller milestone. It proves discovery, claim, MMIO
mapping, IRQ ownership, AHCI-mode enable, ATA identify, command-list/FIS/PRDT
DMA setup, read/write transfer, flush, and live per-port status telemetry.

```text
signed capsule
    |
    | MkDeviceList / MkDeviceClaim
    v
AHCI PCI function -- MkMmioMap(BAR5 / ABAR) --> user VA
    |
    +-- MkIrqBind / MkIrqPoll / MkIrqAck --> controller events
    `-- MkDmaMap -------------------------> command/FIS/PRDT/data buffers
```

## Microkernel contract

The capsule talks to hardware only through the broker:

- `MkDeviceList` locates SATA AHCI controller records.
- `MkDeviceClaim` binds the controller to this capsule's process.
- `MkMmioMap` maps BAR5, the AHCI ABAR register window.
- `MkIrqBind` binds the INTx line when it can; a refused bind leaves a zero
  grant and changes nothing, since every command completion is polled
  (`src/setup/irq.rs`). `MkIrqPoll` and `MkIrqAck` serve a bound line.
- `MkDmaMap` and `MkDmaUnmap` allocate command-list, received-FIS, command
  table, PRDT, and sector data buffers.
- `MkIpcRecv` and `MkIpcSend` serve `driver.ahci0` on
  `service:4216:driver.ahci0`.

The kernel validates the capability token, owns address spaces, owns broker
revocation, and tears grants down on exit. It does not contain SATA command
logic, ATA identify logic, block scheduling, or filesystem policy.

The driver guards the medium itself (`src/server/medium.rs`): every op but the
health check answers only the kernel's own block client (sender pid 0) or a
sender the kernel says holds `StoreWrite`, asked with `MkCapCheck` on every
request.

## Interface contract

| Operation | Meaning | Reply payload |
|---|---|---|
| `OP_HEALTHCHECK` | server liveness | status word |
| `OP_CONTROLLER_INFO` | AHCI global register summary | 24-byte controller record |
| `OP_PORT_LIST` | implemented ports, signatures, and live status | count plus 36-byte entries |
| `OP_CAPACITY` | selected block port capacity | status plus sector count |
| `OP_READ_BLOCKS` | read sectors from selected block port | status plus sector bytes |
| `OP_WRITE_BLOCKS` | write sectors to selected block port | status word |
| `OP_FLUSH` | flush selected block port | status word |
| `OP_IDENTIFY` (8) | the served disk's identity | status plus 76-byte record |

## Authority

The manifest grants `IPC`, `Memory`, `Driver`, `DeviceEnum`, `Mmio`, `Irq`,
and `Dma` (`CAPSULE_REQUIRED_CAPS = 0xf8018`).

```text
allowed:   device enumeration, one device claim, ABAR MMIO, IRQ, DMA, IPC
forbidden: PIO, filesystem, admin, debug, raw kernel memory
```

## Privacy and persistence

The capsule reads and writes sector payloads only for explicit block protocol
requests. It does not parse partitions, mount filesystems, cache disk payloads,
or persist controller state. All broker grants are process-lifetime resources
and are revoked by kernel teardown.

## Runtime lifecycle

The capsule opens and resets every AHCI controller the device list reports,
brings up each port with a link, keeps those whose signature after link-up is an
ATA disk, and reads (never writes) LBA 256 and 131072 for the NONOS store and
plan magics. It serves the first disk carrying either, else the first that came up.

## Failure model

With no AHCI controller in the device list the capsule logs one line and exits
`EXIT_ABSENT` (2) before claiming anything. A controller whose claim,
memory and bus-master enable, MMIO map or HBA reset fails is skipped and
released; when no disk comes up on any controller (none opened, or every port
empty or failing), the whole walk is retried on the shared bounded schedule
(`nonos_libc::bring_up`: seven tries, sleeping between them), and running out
logs the last cause and exits `EXIT_GAVE_UP` (6). A port whose link, DMA,
signature, or ATA identify fails is stopped; unserved ports are parked, other
controllers released. Requests never touch a port that is not the one served.

Taken for an HBA: any SATA function (class 01h subclass 06h, whatever its
prog-if) and an Intel RST RAID-mode function (01h/04h, which is AHCI
underneath), never an Intel VMD (`src/discover/rule.rs`, whose id list the
proof crate holds to the kernel's `src/hardware/inventory/vmd.rs`). The ABAR
may be as small as one port's registers (0x180 bytes): Intel PCH ABARs are
commonly 2 KiB and AMD FCH ones 1 KiB, often off a page boundary; the broker
maps such a BAR by its page and hands back the VA of its first byte.

Bring-up, per controller: BIOS/OS handoff when CAP2.BOH; GHC.HR within 1 s
(a controller that does not reset is left alone), with CAP and PI written
back when the reset cleared them; then PxCMD.SUD (and POD with CAP.CPD) on
every implemented port, since with staggered spin-up a port sends no COMRESET
until SUD is set and reads DET = 0 with a disk attached. Every implemented
port is then tried: its own COMRESET decides whether a disk is there (the link
quiet for 200 ms is an empty port), a disk gets up to 10 s to clear BSY while
it spins up, and only an ATA signature is identified. Every wait is on the
monotonic clock (`src/constants/timing.rs`). On an HBA without CAP.S64A every
DMA buffer is asked for below 4 GiB (`MK_DMA_MAP_DMA32`). Each port tried
logs one `[ahci]` serial line with PxSSTS, PxTFD, PxSERR, PxSIG and PxCMD.

PI is the controller's own word, and its bits may be sparse: the walk covers
up to CAP.NP + 1 ports or the highest PI bit, whichever is higher, and a zero
PI falls back to the first CAP.NP + 1 ports. A port is walked only when its
whole register block lies inside the ABAR window the broker actually mapped
(`controller/window.rs`): a 1 KiB BAR reaches ports 0 to 5, 2 KiB ports 0 to
13, 4 KiB ports 0 to 29, and the broker clamps a window that would expose an
MSI-X table. A port PI names past the window is never read.

The IDENTIFY DEVICE block is the drive's own bytes, so the driver copies it out
of the DMA buffer once and holds the copy to fixed rules (`src/identity/`)
before it serves anything. A block that breaks a rule refuses the disk: its
port is parked as if IDENTIFY had failed, and nothing is clamped or guessed.

- The 48-bit Address feature set must be supported and enabled (word 83 bit
  10 and word 86 bit 10, each counted only when its validity bits read 01b).
  Every command the driver issues is a 48-bit one, so a disk without it is
  refused, and the 28-bit count in words 60-61 is never read.
- The logical sector must be 512 bytes: word 106, under its validity bits,
  with words 117-118 when bit 12 says the sector is longer. 512 is the one
  size the wire format, the PRD byte count and every copy are sized by, and
  the one the kernel block layer addresses, so a 4096-byte disk is refused
  like any other figure. The figure is compared, never used to size anything.
- Words 100-103 must count at least one sector and fewer than 2^48, the most
  a FIS can address. That count is the capacity served.

Every read and write is held to the served capacity before a command is built.
The request parser refuses a zero count, more than 64 sectors, an LBA plus
count that overflows u64, and a span past the end; `transfer` checks the span
again (`engine/span.rs`, also bounded by 2^48), whoever asked for it, so the
probe reads at bring-up pass the same gate. The one PRD entry's byte count comes
from `engine/prd_count.rs`: nonzero, even, at most the 32 KiB data buffer the
driver owns and within the 4 MiB a PRD entry can carry. A count it refuses
builds nothing.

A command's end is judged from the port registers alone
(`engine/completion.rs`). Before issue the device must have cleared BSY and
DRQ and the port must show no slot issued or queued. While waiting, each look
reads PxCI first, then PxSACT, PxIS and PxTFD. A fatal PxIS bit, an overflow
(PxIS.OFS), ERR in PxTFD, a PxCI bit for any slot but 0, or any PxSACT bit
fails the command. Slot 0 clear while PxTFD still shows BSY or DRQ is waited
on, and a status that never settles times out. Only a clear slot 0 with a
clean status read after it counts as done, so a failed or unfinished command
never hands its buffer to a client: the reply is `E_IO`. The port is then
recovered: its errors cleared, its engine stopped, a device stuck in BSY or
DRQ freed by Command List Override (with CAP.SCLO) or else a COMRESET, and the
engine started again with FIS receive back on, so the next command's status
reaches PxTFD. A command, with any kick before it, has 3 s in all, inside the
kernel's 5 s wait for a reply.

## Current implemented surface

- Claims the AHCI controller through the broker.
- Maps ABAR through `MkMmioMap`.
- Binds the controller interrupt when it can, and polls either way.
- Enables AHCI mode and reads controller-global registers.
- Identifies each SATA disk and records its block capacity, refusing a disk
  whose IDENTIFY block breaks a rule in the failure model.
- Allocates command-list, received-FIS, command-table, PRDT, and data DMA.
- Serves capacity, read, write, and flush requests over IPC.
- Reports implemented ports, signatures, PxIS, PxCMD, PxTFD, PxSERR, PxSACT,
  and PxCI through the service endpoint.
- Fails closed when discovery, claim, MMIO, DMA, or identify setup fails.

## Wire format

Requests use the capsule's 20-byte protocol header with magic `NAHC`, version
`1`, operation id, request id, and payload length. Replies use the same header
shape and begin with a 4-byte status word. `OP_CONTROLLER_INFO` returns a
24-byte fixed register summary. `OP_CAPACITY` returns an 8-byte sector count.
`OP_READ_BLOCKS` and `OP_WRITE_BLOCKS` use a 12-byte `lba, sector_count`
request header and fixed 512-byte sectors. `OP_PORT_LIST` returns a 4-byte
count followed by fixed 36-byte port records:

```text
u8 index, u8 implemented, u8 present, u8 kind,
u32 PxSSTS, u32 PxSIG, u32 PxIS, u32 PxCMD,
u32 PxTFD, u32 PxSERR, u32 PxSACT, u32 PxCI
```

`OP_IDENTIFY` takes no payload and returns, after the status word, the record
`src/protocol/identify_reply.rs` lays out: u64 sectors, u32 logical sector
size (512), u8 model length, u8 serial length, two zero bytes, the model in
40 bytes and the serial in 20, printable ASCII trimmed of padding and
zero-filled. With no disk served the reply is `E_NODEV`.

## State ownership

The capsule owns the AHCI claim epoch, ABAR mapping, IRQ grant id, DMA grants,
controller snapshot, port snapshot, command state, and data buffer. The kernel
owns only the broker records and address-space mappings.

## Operating rules

- Keep partition, filesystem, encryption, and cache policy above this driver.
- Any setup failure must unwind DMA, IRQ, MMIO, and device claim in reverse order.

## Release target

The next AHCI target is broader validation: NCQ where supported, serving more
than one disk, timeout recovery, device reset, and repeated real-controller boot
evidence. It remains a driver only: partitions, filesystems, encryption, and
cache policy stay in separate storage capsules.

## Release evidence

Release requires `ich9-ahci` boot validation, port signature proof, teardown
grant-revocation proof, read/write/flush validation, and one real SATA
controller boot dossier.

## Release checklist

- Signed manifest and publisher keys present.
- Kernel mirror embeds and feature-gates `driver.ahci0`.
- QEMU controller probe passes on `ich9-ahci`.
- Teardown proof shows no leaked MMIO/IRQ/device claim.
- Read/write/flush proof passes through the IPC block endpoint.

## Real hardware bring-up checklist

What only a boot on real silicon can confirm. The host proofs cover the parsers, the bring-up sequence against a model, and the bounded retry; these do not.

- On a machine without the device: the broker log shows no claim for it and the capsule exits at once with `EXIT_ABSENT` (2); the capsule's own `no controller present, not started` line needs the Debug capability, which this manifest does not grant, so the kernel prints `[EXIT] <service> status 2: no device present, not started` for it (`src/process/exit/end_note.rs`).
- With the device present but failing (disabled in firmware, or a forced setup error): at most seven claim and release rounds in the broker log over about six seconds, then one `device present, bring-up failed` line naming the last cause (with the Debug capability), and exit `EXIT_GAVE_UP` (6), which the kernel names as `[EXIT] <service> status 6: device present, bring-up failed and was given up`. The capsule holds no core while it waits.
- GHC.HR clears on the real HBA; every implemented port's link settles.
- On a machine with several controllers the disk that carries NONOS is the one served; a blank target falls back to the first port that came up.
- IDENTIFY capacity matches the disk; read, write and flush round-trip data.
- A real disk passes the IDENTIFY rules: words 83, 86 and 87 report the 48-bit
  feature set supported and enabled, and a 512e disk reports 512-byte logical
  sectors. A 4096-byte native disk is refused and its port parked; another
  disk, if any, is served.
- Commands finish without stalling on the real HBA: PxTFD shows an idle status
  by the time PxCI clears, so the completion wait never runs to its bound on
  a healthy disk.
- After a command fails (a forced error, or a disk pulled and reseated), the
  next command on the port succeeds: recovery turns FIS receive back on.
- A controller whose ABAR is shorter than a full 32-port register file is
  walked without a fault, and ports past the mapped window are not touched.
- A disk still spinning up links within the two second link window and clears
  BSY within ten.
- On an HBA with staggered spin-up (Intel Gemini Lake 8086:31E3 among them) a
  disk is found though PxSSTS read DET = 0 before the spin-up.
- A 2 KiB Intel ABAR and a 1 KiB AMD FCH ABAR map, off a page boundary too.

## Explicit non-goals today

No NCQ, multi-disk serving, partition parsing, filesystem, encryption policy,
or disk cache lives in this capsule.

## Verification

- Build: `make -B nonos-mk-driver-ahci`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Architecture check: the capsule must not import `crate::drivers`,
  `crate::hardware`, `crate::memory`, `crate::paging`, or use inline PIO/DMA.
- Documentation check: this README is required by the static gate and must
  describe authority, privacy, current surface, release evidence, and non-goals.
- Proofs: `(cd userland/ahci_link_proofs && cargo test --release)`.
- Handbook: [drivers](../../docs/handbook/drivers.md),
  [storage](../../docs/handbook/storage.md).
