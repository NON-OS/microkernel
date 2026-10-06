# nvme_proofs

Host-runnable proofs for the NVMe driver's untrusted-input parsers and for
its initialisation and completion waits against a hostile controller. The
real driver source is included through `#[path]` and run on the host.

## Block I/O bounds

A block read or write request carries an attacker-controlled logical block
address and sector count over IPC. The proof establishes that the parser never
panics, a short request body is rejected, and an accepted request stays on the
disk: the sector count is within range and `lba + count` neither overflows nor
exceeds the device capacity. A Kani harness proves the bounds over every
request body and every capacity. This is the same isolation property the AHCI
proof establishes for the other block driver.

## Wire header decode

The IPC request header is decoded from attacker-controlled bytes. The proof
establishes that decoding is total, that a short buffer or a wrong magic or
version is rejected, that every accepted field is read from its wire offset in
little-endian order, and that an encoded response header decodes back to the
request's fields. A Kani harness proves totality and field faithfulness over
every buffer up to a full header plus slack.

## Device-controlled identify and SMART pages

The identify controller page, the identify namespace page, and the SMART log
page are written by the device over DMA, so a hostile controller chooses every
byte. The driver always parses them from fixed-size buffers: 4096 bytes for
identify, 512 for the SMART log page. Over those buffer sizes the proofs
establish that parsing never panics for any contents and that every field is
read from its NVMe spec offset in little-endian order. For the namespace page
the device also steers which of the 16 LBA-format slots is read; the proof
covers every slot and shows the reported block size is zero or a power of two
(an absurd LBA shift yields zero rather than a wrapped shift) and the
formatted-count arithmetic saturates instead of wrapping. Kani harnesses prove
totality over every page of the parsed sizes.

The parsers index fixed offsets without a length guard, so these guarantees
hold for the buffer sizes the driver actually passes, not for arbitrary short
slices. The call sites keep that precondition: both identify paths return a
4096-byte DMA slice and the SMART path a 512-byte one.

## Doorbells inside the register window (`doorbell_tests.rs`)

CAP.DSTRD spaces the doorbells, and the broker may map less of BAR0 than the
BAR (it stops below an MSI-X table that shares it). The proof checks the four
doorbells the driver rings against the spec formula for every stride, shows
`ControllerInfo::doorbells_fit` accepts a window exactly when all four fit
(every stride, every 4-byte window length up to 448 KiB), and pins named
layouts: QEMU's, an MSI-X table at 0x1000, a 16 KiB BAR at strides 9, 10 and
15, the one-byte edge.

## Enable and disable waits (`ready_tests.rs`)

The decision (`admin/ready_step.rs`) and the polling loop
(`admin/ready_wait.rs`) take CAP, the CSTS read and the clock from the
caller, so the proofs run the driver's loop against scripted controllers on
a scripted clock. For every CAP.TO the wait is `max(CAP.TO * 500, 5000)` ms
and at most the spec's 127.5 s. CFS ends the enable wait and an all-ones CSTS
either wait at the first read, never as a success; a CFS the disable wait
outlives is named fatal (`ControllerFatal`), not a timeout; a CSTS that never flips times out no
sooner than the bound and within two clock ticks of it, for CAP.TO 0, 255 and
edges between; a controller that becomes ready a millisecond inside its
CAP.TO is taken. 200,000 random CAP, CSTS and elapsed values never wait past
the bound or report ready for a CSTS that is not.

## Completion waits (`completion_tests.rs`)

The admin and I/O queues share one loop (`admin/completion_wait.rs`) over a
cursor (`admin/cq_cursor.rs`) and a per-entry decision
(`admin/completion_step.rs`). Over scripted rings of 1, 2, 8 and 64 entries,
with every slot read and doorbell value checked against the ring: an entry
with last pass's phase is never taken; an entry with another command id or
another SQ id never completes the command and is consumed, so a late
completion does not wedge the queue; each of the 15 status bits fails the
command; SQ head values up to 0xffff never move the cursor; an empty ring
times out after exactly `(n + 1) * 1024` reads; a controller flooding stray
entries is still cut off at the deadline; a well-behaved controller is
followed three times round each ring.

## Namespace acceptance and the transfer ceiling (`geometry_tests.rs`)

Identify pages are built byte by byte and run through the real parsers and
`NamespaceGeometry::check`. NN 0 (absent namespace), NSID 0 and NSZE 0 are
refused; of all 256 LBADS values only 9 and 12 are taken; any metadata, an
FLBAS index past NLBAF and an index that needs FLBAS bits 6:5 are refused.
For all 256 MDTS values the per-command ceiling is at least one block, never
more than the 32 KiB data buffer, and exactly the spec's limit; the request
parser under that ceiling never takes more.

## Hostile controller fuzz and named boundaries (`hostile_tests.rs`)

200,000 xorshift rounds draw CAP, CSTS, elapsed time, the mapped window, a
completion ring and identify pages, run the real decisions and loops, and
compare each with an oracle written from the spec apart from the driver: no
panic, no success the spec does not allow, no index out of range, and every
branch reached in more than one round in a hundred. Named sets then pin 12
ready wait edges, 12 completion entry edges on every ring size and 16 identify
edges.

## Real hardware bring-up decisions (`bringup_tests.rs`)

The pure pieces a laptop SSD exercises and QEMU never did. The active
namespace list (`admin/active_ns.rs`): the first listed NSID is served, a
broadcast or past-NN entry is skipped, a zero ends the list, random pages
never yield 0, broadcast or past NN; only NVMe 1.1 and later (or a blank VS)
are asked. The reset (`admin/disable_step.rs`): EN set with RDY and CFS clear
waits for RDY or CFS before EN is cleared, bounded by CAP.TO, after which the
reset goes ahead; all ones is a device gone. A clock read that fails
(`clock/budget.rs`, the ready and pre-disable loops) ends the wait as
`ClockFailed` at whichever read failed, never spinning. CAP, ASQ and ACQ split
low dword first and join back for 100,000 values (`regs/lo_hi.rs`). An Optane
cache module is tried after every disk (`discover/rank.rs`), and the choice
(`discover/choice.rs`) serves a disk with I/O first, retries while a disk
failed, and never serves a controller that failed. A failed completion's SCT,
SC and DNR are read from their bits, each namespace refusal names its field,
and a console line (`log/line.rs`) reads as written and never overruns.

## 512-byte sectors on native LBAs (`lba_map_tests.rs`)

The kernel client (`src/hardware/nvme_capsule/client/lba_map.rs`) maps the
block layer's sectors onto the namespace's LBAs. For LBA sizes 512 to 8192
and every start and length up to 40 sectors the plan covers exactly the bytes
asked for, each unaligned end lies in one LBA, the body is LBA aligned, and a
512-byte namespace never needs a read-modify-write. The commands a body is
cut into never pass the per-command ceiling, which equals the capsule's own
for all 256 MDTS values at 512 and 4096. A simulated 4096-byte-LBA disk,
driven through the plan as the client drives the capsule, matches a 512-byte
model over 2,000 random reads and writes per geometry, and capacity is
reported in 512-byte sectors.

## What stays unproven on the host

The proofs run the driver's decisions and loops, not its MMIO and DMA: the
volatile reads and writes, the order of calls in `setup/sequence.rs` (that
the doorbell check and the namespace check are made, and that disable comes
before enable) and the kernel's uptime clock advancing are read from the
code, not executed. So are the console lines reaching serial (they need the
Debug capability, granted only by a `capsule-serial-debug` build), which
controller a real machine lists first, and the kernel client's IPC round
trips around the map. They need a boot.

## Run

```sh
cd userland/nvme_proofs
cargo test --release
cargo kani                # all-input totality and bounds (requires Kani)
```

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
