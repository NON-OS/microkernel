# capsule_driver_storvsc

## Role

`capsule_driver_storvsc` drives Hyper-V's synthetic SCSI controller: the
storage VSC a Hyper-V or Azure guest reaches its disks through, over a VMBus
channel. It serves the disks and CD/DVD drives on the controller's targets
on the block wire (`NBW1`). The SCSI commands are `nonos_scsi`'s, the block
wire is `nonos_scsi_serve`'s, and the channel (its rings, the GPADL, the
open and close) is `nonos_vmbus`'s. This capsule only speaks the storage
protocol over the channel.

```text
kernel block layer / installer --NBW1 (unit)--> driver.storvsc0
       nonos_scsi_serve -> nonos_scsi (CDB, sense, retries) -> VmHba
          guest ring --signal--> Hyper-V --> host ring (polled)
```

## Microkernel contract

- The VMBus core (`driver.vmbus0`) learns the SCSI controller's channel
  from the host's offer and has the kernel make it a record
  (`MkHvChannel`): vendor 0x1414, device 0x5343, on the virtual bus, the
  channel's relid in `irq_source`. Only `driver.storvsc0` may claim it.
- `MkDeviceList` finds that record; the driver looks for up to 15 s,
  every 100 ms, while the core brings the bus up, and leaves if none comes.
- `MkDeviceClaim` of the record.
- `MkDmaMap` once, coherent: the guest's ring (32 pages, its header page
  first), the host's ring (the same), then a 64 KiB buffer. A channel
  record has no requester id, so its grants carry physical addresses;
  the broker makes them under the no-IOMMU posture or the development
  opt-in, as for every guest driver.
- `MkHvPost` on the record, connection 0: GPADL_HEADER and GPADL_BODY over
  the 64 ring pages, OPENCHANNEL, and at the end CLOSECHANNEL and
  GPADL_TEARDOWN. The kernel takes only messages naming this channel and
  pages held on this record. `MkHvRecv` on the record hands back the
  host's answers for this channel alone. `MkHvSignal` on the record
  signals the host.
- No `MkIrqBind`: the driver polls the host's ring.
- `MkIpcRecv` serves `service:4304:driver.storvsc0`; replies go to
  `endpoint.4294967394`.

## Interface contract

The block wire as `nonos_scsi_serve` serves it, the same as every SCSI
host driver's: HEALTHCHECK 1, GEOMETRY 2 (blocks, block bytes, max transfer
65536 or the host's smaller limit, flags 1 removable / 2 read-only / 4 a
medium in), READ 3, WRITE 4, FLUSH 5, IDENTIFY 6. Units are numbered in
scan order, by target (0 and 1) then LUN (0 to 254). Statuses beside the
wire's own: -5 (the host failed the command, or moved less than asked), -6
(no target or LUN, or a block past the medium), -30 (write protected or a
CD drive), -110 (the host did not complete the request in 30 s; the bus
was reset), -123 (no disc).

## Authority

`CoreExec`, `IPC`, `Memory`, `Driver`, `DeviceEnum`, `Dma`
(`CAPSULE_REQUIRED_CAPS = 0x98019`); `Debug` optional
(`CAPSULE_OPTIONAL_CAPS = 0x100`). No `Mmio`, `Pio`, `Irq` or `Network`.
CoreExec is for the driver's own pid: its serve loop also drains its reply
endpoint, and a reply no caller took is dropped rather than answered.

```text
allowed:   the SCSI channel's record, its MkHv* calls, one DMA grant
forbidden: any other channel or device, the VMBus root, network, files
```

Only the kernel's own client (sender pid 0) and a sender the kernel says
holds StoreWrite reach the medium; the kernel is asked on every request.

## Privacy and persistence

The driver keeps no block beyond the request it serves. The console names
each unit's address, vendor, product and size, never its serial number.

## Runtime lifecycle

Claim, the DMA grant, both rings' headers set empty (pending_send_sz
honoured), the GPADL over the 64 ring pages and its answer, OPENCHANNEL
naming the host's ring 32 pages in and its answer; a step that fails undoes
the ones before it (`nonos_vmbus::open`). Then the protocol's start, as
Linux's `storvsc_channel_init`: BEGIN_INITIALIZATION, QUERY_PROTOCOL_VERSION
for WIN10 (6.2) then WIN8.1 (6.0), QUERY_PROPERTIES for the host's largest
transfer (the buffer is held to it, in whole pages), END_INITIALIZATION;
each in band on the start's own id, 10 s each. One channel: no
sub-channels. Then targets 0 and 1: REPORT LUNS on LUN 0, each listed LUN
up to 254 probed through `nonos_scsi`. A controller with nothing on it is
closed and given back, and the driver leaves.

One request at a time: EXECUTE_SRB as Linux's `storvsc_queuecommand` fills
it (the 48-byte request, sense of 20 bytes, untagged simple queueing,
synchronous transfer off, 60 s for the host), sent in band when no data
moves and as a GPA-direct packet naming the buffer's pages when it does;
then the host's ring polled for the completion with the request's id: 64
looks a yield apart, then a millisecond between looks, up to 30 s.

## Failure model

Every bring-up step names its failure and gives back what was taken, the
claim last. A completion is read as Linux reads it: success and a short
transfer say how much moved, never more than asked; a CHECK CONDITION
brings its sense; an invalid LUN or a selection timeout is no target; a
bus reset or an abort is tried again; a host timeout is a timeout; any
other failure fails the command, but TEST UNIT READY's, whose status byte
the layer above reads. INQUIRY and MODE SENSE, which Hyper-V answers
wrongly, count as success with what moved (`storvsc_host_mishandles_cmd`).
A completion for another id is dropped; a packet the host sends of its own
accord (a bus enumerated, a device removed) is noted, and one too long to
be an answer is skipped. A request never completed resets the bus, which
the host answers once nothing is out; a host that does not answer that,
or a ring whose index the host put out of bounds, is given up on and every
later request answers -5.

When the driver leaves, it closes the channel and tears the GPADL down. A
host that does not confirm the pages gone leaves them to the kernel, which
keeps them until it does.

## Current implemented surface

- Up to 16 units: disks of 512 to 4096-byte blocks and CD/DVD drives, as
  `nonos_scsi` serves them, on Hyper-V's SCSI controller.

## Wire format

The block wire as `nonos_scsi_serve/src/wire` defines it; the storage
protocol's packet (`struct vstor_packet`, `struct vmscsi_request`) as
Linux's `drivers/scsi/storvsc_drv.c` lays it out
(`src/constants/vstor.rs`); the rings and packets as QEMU's
`hw/hyperv/vmbus.c` and Linux's `include/linux/hyperv.h` (`nonos_vmbus`).

## State ownership

The capsule owns the channel while it is open, its rings and the buffer.
The VMBus core owns the bus; the kernel owns the channel table and takes a
channel's pages back from the host when a driver leaves without. The
kernel's block layer owns what is read and written, and when.

## Operating rules

- One request out at a time; nothing is sent once the host stopped
  answering.
- The buffer is the driver's again only once its request completed or the
  bus was reset.
- Never log a serial number.

## Release target

Disks and CD drives on a Hyper-V Generation 2 guest's SCSI controller (and
Azure's), read and written through the block layer.

## Release evidence

`storvsc_proofs` runs the shipping source and `nonos_scsi_serve` against a
Hyper-V storage host model: `nonos_vmbus`'s host model (written from QEMU's
vmbus.c) for the GPADL, the open and the close, and a storage server on the
channel's rings, written from Linux's storvsc_drv.c, that checks every
request field and answers from `nonos_scsi`'s simulated units through the
buffer's pages. It covers the GPADL over the rings alone; the start, the
version fallback and every refusal; the host's transfer limit; the scan over
both targets; reads and writes page by page; commands with no data in band;
request ids; every SRB status, sense and a short transfer; Hyper-V's
mishandled commands; stale completions, the host's own packets and an
oversized one; a lost request and the bus reset; a host given up on; a ring
the host broke; a guest ring that stays full. QEMU has no storage VSP, so a
boot is on Hyper-V, and the owner's.

## Release checklist

- Signed manifest and kernel mirror present (`driver_storvsc` keys); the
  capsule is in no release profile until then.
- `storvsc_proofs`, `nonos_vmbus` and `nonos_scsi` tests pass.
- The kernel's block layer row for `driver.storvsc0` (the maintainers').
- A Hyper-V Generation 2 guest installs to and boots from its SCSI disk.

## Explicit non-goals today

No sub-channels (one channel, one request at a time), no Fibre Channel
(`FCHBA_DATA`), no rescan on the host's ENUMERATE_BUS (it is noted), no
interrupts. Generation 1's IDE disks over storvsc are left to `piix_ide`.

## Verification

- Build: `make -B nonos-mk-driver-storvsc`
- Proofs: `cd userland/storvsc_proofs && cargo test --release`
- Static gate: `bash nonos-ci/run-static-checks.sh`
