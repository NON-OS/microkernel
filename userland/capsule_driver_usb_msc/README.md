# capsule_driver_usb_msc

## Role

`capsule_driver_usb_msc` is the USB Mass Storage class capsule. It finds one
SCSI-transparent BOT device on driver.xhci0's ports, runs BOT/SCSI over the bulk
pipes driver.xhci0 keeps for it, and serves whole 512-byte sectors to the
kernel's block layer (`src/hardware/usb_msc_capsule`).

```text
USB flash / disk
        |
        v
driver.xhci0 -- descriptors + bulk transfers
        |
        v
driver.usb_msc0 -- BOT/SCSI framing --> the kernel's block layer
```

The capsule is not a host-controller driver and is not a filesystem. PCI
ownership, MMIO, IRQ routing, DMA, xHCI rings, endpoint configuration, and bulk
transfer scheduling stay in `driver.xhci0`. Filesystems, partitioning, caching,
and encryption stay above the block layer.

## Microkernel contract

The manifest grants only `CoreExec`, `IPC` and `Memory`:

```text
CAPSULE_REQUIRED_CAPS = 0x19
```

The service receives requests with `MkIpcRecvFrom`, replies with `MkIpcReply`
(or `MkIpcSend` to the kernel's reply inbox), and calls driver.xhci0 with
`MkIpcCall`. It makes no enumeration, MMIO, IRQ, DMA, or PIO broker syscalls.
Its only state is in process memory: the bound device, BOT tags, counters.

No capsule may send to `driver.usb_msc0`: the kernel holds the endpoint to an
empty list (`src/services/registry/held.rs`), so only the kernel's own block
client reaches it, by name or by pid. Every op but the health check also
checks for sender pid 0 itself (`src/server/admits.rs`).

## Authority

This capsule has no hardware authority. It cannot enumerate USB controllers,
claim PCI devices, map controller registers, bind interrupts, allocate DMA, or
touch I/O ports. It receives descriptor bytes and command status wrappers over
IPC, then returns class-driver decisions and BOT command wrappers to its caller.

```text
allowed:   descriptor classification, BOT/SCSI framing, status accounting
forbidden: xHCI ownership, USB scheduling, DMA buffers, block cache, filesystem policy
```

## Interface contract

| Operation | Input | Output |
|---|---|---|
| `OP_HEALTHCHECK` | none | status |
| `OP_PROBE_CONFIG` | raw USB configuration descriptor | MSC bindings |
| `OP_BUILD_INQUIRY` | none | BOT CBW for SCSI INQUIRY |
| `OP_BUILD_READ_CAPACITY10` | none | BOT CBW for READ CAPACITY(10) |
| `OP_BUILD_READ10` | `lba_le32, blocks_le16` | BOT CBW for READ(10) |
| `OP_BUILD_WRITE10` | `lba_le32, blocks_le16` | BOT CBW for WRITE(10) |
| `OP_BUILD_TEST_UNIT_READY` / `OP_BUILD_REQUEST_SENSE` | none | BOT CBW for that command |
| `OP_ACCEPT_CSW` | 13-byte BOT CSW | status |
| `OP_DECODE_INQUIRY` / `OP_DECODE_CAPACITY` / `OP_DECODE_SENSE` | the SCSI data-in bytes | decoded fields |
| `OP_GET_STATE` | none | counters, the search report, then the re-bind generation (`u32`) |
| `OP_BLK_CAPACITY` / `OP_BLK_FLUSH` | none; kernel (pid 0) only | blocks, block length / status |
| `OP_BLK_READ` / `OP_BLK_WRITE` | `lba_le64, sectors_le32` (+ data), at most 64 sectors | data / status |

Unknown operations reply `E_BAD_OP`. Malformed descriptors or command bodies
reply `E_INVAL`. Valid descriptors without a SCSI-transparent BOT interface
reply `E_NO_MSC`.

## Privacy and persistence

The capsule does not store product strings, serial numbers, raw descriptors,
SCSI payloads, or block data. It keeps only the current endpoint binding table,
the last issued BOT tag, diagnostic counters, and a digest of the served
device's INQUIRY and serial number, in process memory. Capsule teardown
drops that memory through normal userland process cleanup.

## Runtime lifecycle

The search for the device runs one bounded step at a time between requests, so
every request is answered within its own window while it lasts. The capsule
waits up to 30 s for `driver.xhci0` to register, then looks for its device for
at most 10 s, answering the block surface `E_AGAIN` meanwhile, and `E_NODEV`
once every connected port is decided, 1.5 s at the soonest; ports are listed
again every 5 s after, so a late stick is still taken. It addresses each free
connected port, keeps the first SCSI-transparent BOT interface, reads the serial
number (none when it does not read), and asks each logical unit INQUIRY, TEST
UNIT READY and READ CAPACITY, a command a step, for 30 s at most. An empty card
reader is passed a second after it was configured, and its port waits 5 s, then
twice that. The first unit ready is served: blocks of 512 to 4096 bytes, as
512-byte sectors; any other length is answered `E_NOTSUP`, and the kernel passes
the device over by name.

A device that fails a request is checked in the next step: still on its port,
ready, and of its size. One gone or not ready is let go and the search restarts.
After a let-go only the same unit, size and serial number is served again, and
one let go without leaving its port is asked on that port alone. Serving ends
for the boot (`E_NODEV`) when the driver cannot tell what it would serve from
what it served: a card taken out, a size that changed, a medium change (UNIT
ATTENTION 28h) said by the unit served when it is asked again after a let-go or
heard by the check once the kernel has read or written a sector, and a device
with no serial number let go. Any other UNIT ATTENTION but no medium (a power-on
or reset, 29h), a sense that does not read, and a lost phase may be a change
too: a reset reader may say 29h for 28h, and a reader clears its 28h once it
fails a command for it, which a lost CSW hides. The check then serves the medium
afresh, raising the re-bind generation. A device the check keeps after three
failed requests in a row, no READ or WRITE moving data between them, is let go:
one with a serial number is searched for again, the kernel told there is no
device while its port waits (5 s, doubling to 80 s), then bound again; one
without ends serving. A request out of its 18 s answers `E_TIMEDOUT`, which the
kernel takes as a refusal and asks again.

The trade-offs. Until the kernel's first READ or WRITE succeeds it holds nothing
of the medium, so a request that fails with a UNIT ATTENTION then is answered
`E_AGAIN`, which the kernel's disk selection asks again where it passed the
stick over on `E_IO`, and the check asks TEST UNIT READY and READ CAPACITY again
and serves what is there, its identity recorded afresh: another card may be
served, before the kernel has chosen its disk by what it reads. After that read,
a reader that says its medium changed loses the boot disk even with the same
card in it. A stick with no serial number is served once a boot: a let-go of it
ends serving rather than risk the kernel's writes landing on another stick. The
state reply ends with a re-bind generation, raised each time a device is served
after the first or a medium is served afresh, so the kernel can read the store
header again before it writes. A reader that says a power-on after a transport
reset raises it with the same card in it, at the cost of that read.

## Failure model

Malformed descriptor records fail closed without mutating the endpoint snapshot.
Oversized transfer counts are rejected before a CBW is emitted. Invalid CSW
signatures or illegal CSW status values are reported as protocol errors. A CSW
tag mismatch increments the phase-error counter so recovery code can reset the
USB mass-storage transport before issuing more commands.

## Current implemented surface

- USB configuration descriptor walk.
- SCSI-transparent BOT interface detection.
- Bulk IN / bulk OUT endpoint extraction.
- BOT command block wrapper construction.
- BOT command status wrapper validation.
- SCSI INQUIRY, TEST UNIT READY, REQUEST SENSE, READ CAPACITY(10), READ(10),
  and WRITE(10) CDB construction, and decoding of the INQUIRY, capacity and
  sense data.
- The block surface for the kernel: capacity, read, write and flush of whole
  512-byte sectors over the bulk pipes `driver.xhci0` keeps for this driver.
- Bounded transfer-length validation.
- Kernel-spawnable capsule metadata and stable endpoint contract.

## Wire format

Requests use the `NUMS` capsule header, version `1`, and the shared 20-byte
driver envelope. Replies begin with a 4-byte signed status word.

`OP_PROBE_CONFIG` returns a 32-bit binding count followed by 8-byte binding
records:

```text
interface, bulk_in, bulk_out, pad, max_packet_in_le16, max_packet_out_le16
```

CBW replies are the USB BOT 31-byte command block wrapper. CSW inputs are the
USB BOT 13-byte command status wrapper. READ(10) and WRITE(10) request bodies
are:

```text
lba_le32, block_count_le16
```

## State ownership

`driver.usb_msc0` owns only class-local state: endpoint bindings, BOT tag
generation, CSW pass/fail counters, phase-error counters, and residue totals.
`driver.xhci0` owns USB device slots, endpoint contexts, transfer rings, and
bulk scheduling. Storage capsules own block device registration, filesystems,
encryption, cache policy, and mount lifecycle.

## Operating rules

- Keep endpoint scheduling and all USB transfer mechanics in `driver.xhci0`.
- Keep filesystems, partitions, mount policy, and encryption above this driver.
- Do not add MMIO, PIO, IRQ, DMA, or device-enumeration authority here.
- Do not persist product strings, serial numbers, descriptors, or block data.
- Do not accept unbounded reads or writes; callers must provide bounded counts.

## Release target

The intended runtime chain is:

```text
driver.xhci0 -> driver.usb_msc0 -> block service -> filesystem capsules
```

The release target is a USB stick on QEMU xHCI carrying the NONOS store or disk
plan, chosen by the kernel's block layer and read and written through here.

## Release evidence

Build evidence covers the signed capsule ELF, endpoint contract, descriptor
parser, BOT/SCSI framing, and architecture gates. Runtime release evidence
requires a QEMU `qemu-xhci` boot with a USB storage device, descriptor
classification on serial, successful INQUIRY and READ CAPACITY(10), and a
bounded READ(10) transfer through the xHCI bulk-transfer service.

## Release checklist

- Capsule builds with zero warnings.
- Static gates confirm README, capability boundary, matrix row, and endpoint.
- Kernel profile `microkernel-driver-usb-msc` resolves with the signed capsule.
- Descriptor parser rejects malformed record lengths and missing bulk endpoints.
- CBW builders emit fixed-length, bounded BOT command wrappers.
- CSW validator rejects bad signatures, bad status values, and tag drift.
- QEMU xHCI USB storage validation passes INQUIRY, capacity, and bounded read.

## Real hardware bring-up checklist

What only a boot on real silicon can confirm. The host proofs cover the parsers, the bring-up sequence against a model, and the bounded retry; these do not.

- A stick plugged in at boot is found inside the ten second window; with none the block layer is told "no device".
- A slow medium passes TEST UNIT READY within its 30 s.
- Read, write and flush round-trip data; a stalled endpoint recovers.

## Explicit non-goals today

Not here: UASP, more than one logical unit served at a time, write-protect
detection and UNMAP, filesystems, partitions, caching and encryption.
Descriptor-format sense is not decoded: one counts as a UNIT ATTENTION, so a
reader that uses it still has a change seen, and the generation rises after any
request that failed with one. Two sticks of one make, size and serial number
cannot be told apart once the one served has left its port. Stall and
phase-error recovery follow BOT 6.6 and 5.3.4; a failed command is not retried
beyond that and the check after it.

## Verification

- Build: `make -B nonos-mk-driver-usb-msc`
- Kernel profile: `cargo check --no-default-features --features
  microkernel-driver-usb-msc`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Runtime proof target: QEMU xHCI USB storage probe with INQUIRY, capacity, and
  one bounded read.
- Proofs: `(cd userland/usb_msc_proofs && cargo test --release)`.
- Handbook: [drivers](../../docs/handbook/drivers.md),
  [storage](../../docs/handbook/storage.md).
