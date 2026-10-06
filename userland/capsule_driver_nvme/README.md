# capsule_driver_nvme

## Role

`capsule_driver_nvme` is the NVMe controller capsule. It moves NVMe controller
logic out of the kernel and into a signed userland process that receives only
the hardware authority it needs.

The current production slice reaches the admin queue and one NVM IO queue pair:
it claims the PCI NVMe device, maps BAR0, binds MSI-X, allocates broker DMA for
admin queues, IO queues, PRP list, and data buffers, enables the controller,
issues Identify Controller plus Identify Namespace for the first active
namespace, snapshots the controller SMART / health log, and serves
read/write/flush block requests.

```text
driver.nvme0
    |
    | MkDeviceClaim + MkPciConfigWrite(memory space, bus master, INTx off)
    v
NVMe PCI function
    |
    +-- MkMmioMap(BAR0) ----------> controller registers
    +-- MkIrqBind(MSI-X) ---------> completion interrupt
    `-- MkDmaMap -----------------> admin queues / IO queues / data buffers
```

## Microkernel contract

The capsule uses the microkernel as mechanism, not as an NVMe driver:

- `MkDeviceList` finds PCI class `0x010802`.
- `MkDeviceClaim` owns the controller claim and claim epoch.
- `MkPciConfigWrite` sets Memory Space, Bus Master and Interrupt Disable
  through the broker, the three Command bits its allowlist lets a driver set.
- `MkMmioMap` maps BAR0 controller registers.
- `MkIrqBind`, `MkIrqPoll`, and `MkIrqAck` own the MSI-X interrupt path.
- `MkDmaMap` and `MkDmaUnmap` allocate and revoke admin queue, IO queue, PRP,
  identify, health, and sector data DMA.
- `MkIpcRecv` and `MkIpcSend` serve `driver.nvme0` on
  `service:4220:driver.nvme0`.

The kernel never embeds NVMe opcodes, queue policy, namespace interpretation,
or block I/O. It validates the token, grants resources, routes IPC, and
revokes every grant on exit.

The driver guards the medium itself (`src/server/medium.rs`): every op but the
health check answers only the kernel's own block client (sender pid 0) or a
sender the kernel says holds `StoreWrite`, asked with `MkCapCheck` on every
request.

## Interface contract

| Operation | Meaning | Reply payload |
|---|---|---|
| `OP_HEALTHCHECK` | server liveness | status word |
| `OP_CONTROLLER_INFO` | BAR/register and setup snapshot | 52-byte controller record |
| `OP_IDENTIFY_CONTROLLER` | selected Identify Controller fields | 88-byte identity record |
| `OP_IDENTIFY_NAMESPACE` | selected Identify Namespace fields for the served namespace | 36-byte namespace record |
| `OP_SMART_HEALTH` | selected Get Log Page SMART / health fields | 177-byte health record |
| `OP_CAPACITY` | served namespace capacity | status plus LBA count |
| `OP_READ_BLOCKS` | read LBAs from the served namespace | status plus LBA bytes |
| `OP_WRITE_BLOCKS` | write LBAs to the served namespace | status word |
| `OP_FLUSH` | flush the served namespace | status word |

## Authority

The manifest grants `IPC`, `Memory`, `Driver`, `DeviceEnum`, `Mmio`, `Irq`,
and `Dma` (`CAPSULE_REQUIRED_CAPS = 0xF8018`). It has no filesystem, storage
policy, admin, debug, network, or raw physical-memory authority.

```text
allowed:   PCI claim, BAR0 registers, MSI-X, broker DMA, IPC
forbidden: filesystem policy, partition policy, raw physmem, kernel drivers
```

## Privacy and persistence

The capsule reads and writes sector payloads only for explicit block protocol
requests. It does not parse partitions, mount filesystems, cache disk payloads,
or persist metadata. Queue memory and data buffers are broker DMA grants and
are revoked when the capsule exits.

## Runtime lifecycle

The capsule lists every NVMe PCI function (one console line each), tries them
in order with an Intel Optane memory cache module last, and serves one: the
first disk whose namespace gets an I/O queue. For each one tried it claims the
function, sets Memory Space, Bus Master and Interrupt Disable, maps BAR0, binds
MSI-X, allocates admin and IO queue DMA, disables the controller, programs
AQA/ASQ/ACQ, enables the controller, identifies it, asks for one I/O queue
pair (SET FEATURES Number of Queues) and gives a DRAM-less drive the host
memory buffer it asks for (SET FEATURES Host Memory Buffer, up to 64 MiB in
4 MiB pieces, or HMMIN when larger, never past 128 MiB), identifies the
namespace, reads the SMART / health log, and creates one IO queue pair. A
refused queue count or buffer is carried on from; one never answered fails
the attempt, the controller disabled before the buffer's memory goes, and
once an attempt that asked for them fails no later attempt asks; a controller not chosen is disabled
and released. It then serves IPC. Teardown unmaps DMA, unbinds IRQ, unmaps MMIO, and releases the
device claim.

## Failure model

Every setup phase is a barrier with reverse-order rollback. Controller timeout,
admin completion error, stale claim, or DMA allocation failure prevents service
start; a refused MSI-X bind leaves the driver polling, which it does for every
completion anyway. Runtime block requests validate size and capacity before
submitting commands.

The controller's registers and completion entries are treated as hostile:

- Register block. A controller whose CAP or VS reads zero is refused, and so
  is one whose CAP.DSTRD would put a doorbell the driver rings (the I/O
  completion head is the highest) past the part of BAR0 the broker mapped.
  The broker ends that window below an MSI-X table sharing BAR0, so it can be
  shorter than the BAR.
- Enable and disable waits. After writing CC.EN the driver polls CSTS for
  CAP.TO * 500 ms, never less than 5 s, so no CAP can hold one wait past
  127.5 s. A CSTS that reads all ones (a device pulled from the bus) ends
  either wait at once and is never taken as ready or as disabled. CSTS.CFS
  ends the enable wait at once. The disable wait is the reset that clears CFS,
  so it waits for RDY and CFS both to clear: a controller left fatal by
  firmware or an earlier boot is reset and served; a CFS still set when that
  wait runs out is named fatal, not slow. A CSTS that never flips is a
  timeout. Before clearing CC.EN the reset waits, within CAP.TO, for a
  controller found with EN set and RDY clear (mid-enable, as firmware can
  leave it) to show RDY or CFS, as NVMe asks; a clock that cannot be read
  ends every wait as a failed attempt instead of one that never expires.
  Each of these fails the attempt, which releases everything it claimed and
  goes back to the bounded retry schedule.
- Completion waits (admin and I/O share one loop). An entry is taken only
  when its phase tag is this pass's, its SQ id is the queue the command was
  issued on (0 for admin, 1 for I/O) and its command id is the one issued. A
  phase-correct entry for anything else is consumed and the wait goes on, so
  a completion that arrives after its own wait timed out cannot stall the
  queue. Any bit in the status field above the phase tag fails the command.
  The SQ head the controller reports is never read into the driver's state;
  the driver's head and tail stay below the ring sizes, and each ring fits its
  DMA region by a compile-time check. Each wait ends 5 s after it began
  (checked every 1024 polls), whatever the controller writes.
- Namespace. The served NSID is the first one Identify CNS 02h lists as
  active (NSID 1 when the controller refuses the list, as NVMe 1.0 does, or
  its VS is below 1.1). It gets an I/O queue only when the controller reports at
  least one namespace (NN), NSZE is not zero, the format FLBAS selects is one
  the namespace has (index at most NLBAF, FLBAS bits 6:5 clear), that format
  carries no metadata, and its block size is 512 or 4096 bytes. Otherwise the
  controller is still served for identify and health, and block requests
  answer `E_NODEV`.
- Registers. CAP, ASQ and ACQ move as two 32-bit accesses, low dword first.
  With MSI-X refused, INTMS is set to all ones after the enable, so a pin or
  MSI interrupt nothing services cannot stay asserted.
- Bring-up lines. With the Debug capability (`capsule-serial-debug`) the
  capsule says each controller seen, any admin command that failed with its
  SCT, SC and DNR, and why a namespace got no I/O queue (block size, MS,
  FLBAS, NLBAF, MDTS). A namespace that can be served but whose I/O queue
  cannot be created fails the attempt, so the retry schedule runs.
- Transfer size. One command moves at most the 32 KiB data buffer, and less
  when the controller's MDTS is smaller; MDTS 0 and any MDTS too large to
  shift mean the buffer. Larger requests are refused before submission.

`userland/nvme_proofs` runs this code on the host against scripted
controllers, a hostile fuzz and a named boundary set. What it cannot run is
the MMIO and DMA themselves and the order of calls in `setup/sequence.rs`.

With no NVMe controller in the device list the capsule logs one line and
exits `EXIT_ABSENT` (2) before claiming anything. A controller that is present
but fails setup is retried on the shared bounded schedule
(`nonos_libc::bring_up`: seven tries, sleeping between them, each failed try
having released its claim); running out logs the last cause and exits
`EXIT_GAVE_UP` (6).

## Current implemented surface

- Claims a real NVMe PCI function.
- Sets Memory Space, Bus Master and Interrupt Disable through brokered PCI
  config write.
- Maps controller registers.
- Binds MSI-X for admin completion.
- Allocates and zeroes admin queue DMA through the broker.
- Programs AQA/ASQ/ACQ and enables the controller.
- Issues Identify Controller.
- Issues Identify CNS 02h for the active namespace list and Identify Namespace
  for the first active NSID when the controller reports namespaces.
- Issues Get Log Page for the standard SMART / health log; a refusal leaves a
  zeroed snapshot and does not stop the bring-up.
- Creates one IO submission/completion queue pair.
- Allocates PRP list and sector data DMA.
- Serves capacity, read, write, and flush requests over IPC.
- Exposes controller and namespace identity over IPC.
- Exposes selected health counters over IPC without exposing raw log DMA.

## Wire format

Requests use the `NNVM` capsule header, version `1`, and the shared 20-byte
driver envelope. Replies begin with a 4-byte status word. Controller-info
returns 52 bytes. Identify Controller returns 88 bytes of selected fields.
Identify Namespace returns 36 bytes for the served namespace, its NSID first
and its LBA size in bytes at offset 28. Raw 4096-byte identify pages
remain internal DMA data unless a later protocol explicitly exposes them.
SMART / health returns 177 bytes of selected fields, including the controller
warning bits, composite temperature, spare percentage, lifetime counters, media
errors, and error-log count. `OP_CAPACITY` returns an 8-byte LBA count.
`OP_READ_BLOCKS` and `OP_WRITE_BLOCKS` use a 12-byte `lba, count` request
header (u64 then u32, little endian), both in the namespace's own LBAs, never
512-byte sectors: a write carries `count * lba_size` data bytes and a read
answers with as many. `OP_CAPACITY` counts the same LBAs. One request moves at
most what the 32 KiB data buffer holds at that size and what MDTS allows
(64 LBAs of 512 bytes, 8 of 4096, fewer under a small MDTS). A client that
counts 512-byte sectors scales by `lba_size / 512` and reads, patches and
writes back an LBA it covers in part, as the kernel client does
(`src/hardware/nvme_capsule/client/lba_map.rs`).

## State ownership

The capsule owns the controller claim epoch, BAR0 mapping, MSI-X grant, admin
queues, IO queues, identify DMA, health DMA, PRP list, sector data buffer,
controller snapshot, and namespace snapshot. The kernel owns capability
validation, grant records, IRQ routing, and teardown only.

## Operating rules

- Keep namespace and controller command logic inside the capsule.
- Do not parse partitions, filesystems, or encrypted volume headers here.
- Every setup phase must have reverse-order rollback.

## Release target

The next NVMe target is wider validation: namespace scanning, multi-queue
support, PRP boundary stress, timeout/error recovery, MSI-X completion
handling under load, teardown rollback, and one real NVMe controller boot. It
does not parse partitions, filesystems, encryption headers, or application data.

## Release evidence

Release requires QEMU `-device nvme` identify validation, IO queue creation
validation, single read/write/flush proof, PRP boundary tests, teardown DMA
revocation, and one real NVMe controller boot.

## Release checklist

- Signed manifest and publisher trust entries present.
- Kernel mirror embeds and feature-gates `driver.nvme0`.
- QEMU identify validation reports the controller and its first active NSID.
- IO queue creation and single read/write/flush validation pass.
- PRP/SGL boundary tests pass.
- Teardown proof shows admin and IO DMA grants are revoked.

## Real hardware bring-up checklist

What only a boot on real silicon can confirm. The host proofs cover the parsers, the bring-up sequence against a model, and the bounded retry; these do not.

- On a machine without the device: the broker log shows no claim for it and the capsule exits at once with `EXIT_ABSENT` (2); the capsule's own `no controller present, not started` line needs the Debug capability, which this manifest does not grant, so the kernel prints `[EXIT] <service> status 2: no device present, not started` for it (`src/process/exit/end_note.rs`).
- With the device present but failing (disabled in firmware, or a forced setup error): at most seven claim and release rounds in the broker log over about six seconds, then one `device present, bring-up failed` line naming the last cause (with the Debug capability), and exit `EXIT_GAVE_UP` (6), which the kernel names as `[EXIT] <service> status 6: device present, bring-up failed and was given up`. The capsule holds no core while it waits.
- CSTS.RDY follows CC.EN within the controller's CAP.TO on the real drive, including a drive whose CAP.TO is over 10 (5 s) brought up after an unclean shutdown.
- The doorbell window check takes the real drive: BAR0 as the broker maps it (below any MSI-X table) holds the I/O completion doorbell at the drive's CAP.DSTRD.
- A namespace formatted with metadata, or with more than 16 LBA formats and one past the 16th selected, comes up with no I/O queue and identify and health still answer.
- Identify controller, identify namespace and the SMART log read back sensible values; both 512 and 4096 byte LBA formats get an I/O queue.
- Read, write and flush round-trip data on a scratch namespace; an error completion is reported to the caller without wedging the queue.
- With MSI-X refused the driver polls and still completes I/O.
- A laptop SSD whose firmware left Memory Space clear answers CAP and CSTS
  (not all ones) once the Command write sets it.
- On a machine with an Optane memory module beside the SSD, the console lists
  both controllers and the SSD is the one served.
- A DRAM-less SSD (PM991, SN530, BC711) logs `host memory buffer given,
  pages N` and is served; a drive that refuses it logs `refused` and is served
  without one; a drive that never answers costs one attempt, after which
  `later attempts ask for neither` and the drive is served as before.
- With the firmware's SATA mode on Intel RST "RAID On", the AHCI driver logs
  `Intel RST hides N NVMe drive(s)` and this driver finds no controller.
- A 4096-byte-LBA namespace mounts through the kernel client: capacity in
  512-byte sectors is eight times the LBA count, and the store reads and
  writes round trip.

## Explicit non-goals today

No discard, namespaces beyond the first active one, multipath, partition table,
filesystem, encryption, or cache policy is exposed here.

## Verification

- Build: `make -B nonos-mk-driver-nvme`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Architecture check: NVMe must not import kernel driver or memory internals;
  it must use `MkMmioMap`, `MkIrqBind`, and `MkDmaMap`.
- Broker check: setup rollback must unmap DMA, unbind IRQ, unmap MMIO, and
  release the device claim on failure.
- Proofs: `(cd userland/nvme_proofs && cargo test --release)`.
- Handbook: [drivers](../../docs/handbook/drivers.md),
  [storage](../../docs/handbook/storage.md).
- Documentation check: the static gate requires this README and its contract,
  authority, lifecycle, failure model, release evidence, and verification sections.
