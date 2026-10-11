# capsule_driver_pvscsi

## Role

`capsule_driver_pvscsi` drives VMware's paravirtual SCSI adapter (PCI
15ad:07c0): VMware's PVSCSI, the controller VMware recommends for its
guests, and QEMU's `pvscsi`. It serves the disks and CD/DVD drives on its
targets on the block wire (`NBW1`). The SCSI commands are `nonos_scsi`'s and
the block wire is `nonos_scsi_serve`'s, shared with every SCSI host driver;
this capsule only moves a command and its data through the adapter's rings.

```text
kernel block layer / installer --NBW1 (unit)--> driver.pvscsi0
       nonos_scsi_serve -> nonos_scsi (CDB, sense, retries) -> PvHba
          request ring --kick--> adapter --> completion ring (polled)
```

## Microkernel contract

- `MkDeviceList` finds the first PVSCSI function and its first memory BAR
  of at least 32 KiB (BAR1 behind an I/O BAR0 on VMware, BAR0 on QEMU).
- `MkDeviceClaim`; `MkPciConfigWrite` sets Memory Space, Bus Master and
  Interrupt Disable.
- `MkMmioMap` of the 32 KiB register window.
- `MkDmaMap` once, coherent: the rings state, the request ring and the
  completion ring a page each, a sense page, then a 64 KiB buffer.
- No `MkIrqBind`: every interrupt is masked and the driver polls the
  completion ring.
- `MkIpcRecv` serves `service:4294:driver.pvscsi0`; replies go to
  `endpoint.4294967357`.

## Interface contract

The block wire as `nonos_scsi_serve` serves it, the same as every SCSI
host driver's: HEALTHCHECK 1, GEOMETRY 2 (blocks, block bytes, max transfer
65536, flags 1 removable / 2 read-only / 4 a medium in), READ 3, WRITE 4,
FLUSH 5, IDENTIFY 6 (vendor and product, serial, medium 6). Units are
numbered in scan order, by target (0 to 63) then LUN. Statuses beside the
wire's own: -5 (the target or the adapter failed the command, or moved less
than asked), -6 (no target, or a block past the medium), -30 (write
protected or a CD drive), -110 (the adapter did not complete the request in
30 s; it was reset and given fresh rings), -123 (no disc).

## Authority

`CoreExec`, `IPC`, `Memory`, `Driver`, `DeviceEnum`, `Mmio`, `Dma`
(`CAPSULE_REQUIRED_CAPS = 0xB8019`); `Debug` optional
(`CAPSULE_OPTIONAL_CAPS = 0x100`). No `Pio`, `Irq` or `Network`. CoreExec
is for the driver's own pid: its serve loop also drains its reply endpoint,
and a reply no caller took is dropped rather than answered.

```text
allowed:   the PVSCSI claim, its register window, one DMA grant
forbidden: any other device, the I/O BAR, the message ring, network, files
```

Only the kernel's own client (sender pid 0) and a sender the kernel says
holds StoreWrite reach the medium; the kernel is asked on every request.

## Privacy and persistence

The driver keeps no block beyond the request it serves. The console names
each unit's address, vendor, product and size, never its serial number.

## Runtime lifecycle

Claim, command bits, the register window, the DMA grant. ADAPTER_RESET,
every interrupt masked, the three ring pages zeroed, SETUP_RINGS with one
page each (32 request and 128 completion descriptors) and its status read;
the ring sizes the adapter writes into the rings state are checked against
the layout. Then targets 0 to 63: REPORT LUNS on LUN 0 (a selection timeout
costs that one request), each listed LUN up to 255 probed through
`nonos_scsi`. An adapter with nothing on it is given back and the driver
leaves.

One request at a time: the descriptor (a fresh context, the data buffer
and its length or none, the sense page, the direction, the CDB up to 16
bytes, the LUN in byte 1 of the LUN field, bus 0, the target), the request
producer moved, the read/write doorbell for a READ or WRITE and the other
for everything else, then the completion ring polled: 64 looks a yield
apart, then a millisecond between looks, up to 30 s.

## Failure model

Every bring-up step names its failure and gives back what was taken, the
claim last. A selection timeout ends a command as no target; a bus or
device reset, a lost bus, an aborted queue or an adapter timeout is tried
again (four sends at most, with unit attentions and BUSY status under the
same rule); any other host status fails it, as does a completion for a
context the driver did not send. A request never completed resets the
adapter and gives it fresh rings; one that does not take them is given up
on and every later request answers -5.

## Current implemented surface

- Up to 16 units: disks of 512 to 4096-byte blocks and CD/DVD drives, as
  `nonos_scsi` serves them.

## Wire format

The block wire as `nonos_scsi_serve/src/wire` defines it; the PVSCSI
request and completion descriptors and the rings state as Linux's
`vmw_pvscsi.h` lays them out (`src/constants/regs.rs`).

## State ownership

The capsule owns the adapter, its rings and the buffer. The kernel's block
layer owns what is read and written, and when.

## Operating rules

- One request out at a time; nothing is sent to an adapter that did not
  come back.
- Never unmask an interrupt; never use the message ring.
- Never log a serial number.

## Release target

Disks and CD drives on VMware's PVSCSI controller and QEMU's `pvscsi`,
read and written through the block layer.

## Release evidence

`pvscsi_proofs` runs the shipping source and `nonos_scsi_serve` against a
PVSCSI model (QEMU's command register with its data words, SETUP_RINGS
writing the ring sizes, each doorbell walking the request ring and
answering on the completion ring from `nonos_scsi`'s simulated units):
the reset and the rings handed over word for word, every refusal, the scan
across 64 targets, discovery of both BAR layouts, what each descriptor
carries and which doorbell rings, contexts and ring wrap, a full transfer,
every host status, sense and a short transfer, a grant that starts as
garbage, a lost request and the adapter brought back once, the adapter
given up on. A boot is the owner's.

## Release checklist

- Signed manifest and kernel mirror present (`driver_pvscsi` keys).
- `pvscsi_proofs` and `nonos_scsi` tests pass.
- The kernel's block layer row for `driver.pvscsi0` (the lead's).
- VMware with a PVSCSI controller, and QEMU `-device pvscsi`, install to
  and boot from the disk.

## Explicit non-goals today

No scatter-gather lists (one 64 KiB buffer), no message ring (hot plug),
no interrupts, no request call threshold, no task management beyond the
adapter reset on a lost request.

## Verification

- Build: `make -B nonos-mk-driver-pvscsi`
- Proofs: `cd userland/pvscsi_proofs && cargo test --release`
- Static gate: `bash nonos-ci/run-static-checks.sh`
