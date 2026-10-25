# capsule_driver_buslogic

## Role

`capsule_driver_buslogic` drives the BusLogic MultiMaster SCSI host adapter
(PCI 104b:1040, the BT-958): VirtualBox's BusLogic controller and VMware's
BusLogic, the controller older guests are configured with. It serves the
disks and CD/DVD drives on its targets on the block wire (`NBW1`). The SCSI
commands are `nonos_scsi`'s and the block wire is `nonos_scsi_serve`'s,
shared with every SCSI host driver; this capsule only moves a command
control block through the adapter's mailboxes.

```text
kernel block layer / installer --NBW1 (unit)--> driver.buslogic0
       nonos_scsi_serve -> nonos_scsi (CDB, sense, retries) -> BtHba
     CCB -> outgoing mailbox --START MAILBOX--> adapter -> incoming mailbox
```

## Microkernel contract

- `MkDeviceList` finds the first MultiMaster function with its four
  register ports in I/O BAR0.
- `MkDeviceClaim`; `MkPciConfigWrite` sets Bus Master and Interrupt
  Disable.
- `MkPioGrant` on BAR0: every register access is a kernel `in` or `out`.
- `MkDmaMap` once, coherent and below 4 GiB (the mailboxes and every CCB
  field are 32-bit): one mailbox pair, the CCB, the sense buffer, then a
  64 KiB buffer.
- No `MkMmioMap`, no `MkIrqBind`: the adapter's interrupt is off and the
  driver polls the incoming mailbox.
- `MkIpcRecv` serves `service:4296:driver.buslogic0`; replies go to
  `endpoint.4294967358`.

## Interface contract

The block wire as `nonos_scsi_serve` serves it, the same as every SCSI
host driver's: HEALTHCHECK 1, GEOMETRY 2 (blocks, block bytes, max transfer
65536, flags 1 removable / 2 read-only / 4 a medium in), READ 3, WRITE 4,
FLUSH 5, IDENTIFY 6 (vendor and product, serial, medium 6). Units are
numbered in scan order, by target (0 to 15, the adapter's own ID left out)
then LUN (to 7). Statuses beside the wire's own: -5 (the target or the
adapter failed the command, or moved less than asked), -6 (no target, or a
block past the medium), -30 (write protected or a CD drive), -110 (the
adapter did not complete the request in 30 s; it was reset and given its
mailboxes again), -123 (no disc).

## Authority

`CoreExec`, `IPC`, `Memory`, `Driver`, `DeviceEnum`, `Dma`, `Pio`
(`CAPSULE_REQUIRED_CAPS = 0x198019`); `Debug` optional
(`CAPSULE_OPTIONAL_CAPS = 0x100`). No `Mmio`, `Irq` or `Network`. CoreExec
is for the driver's own pid: its serve loop also drains its reply endpoint,
and a reply no caller took is dropped rather than answered.

```text
allowed:   the MultiMaster claim, its port window, one DMA grant
forbidden: any other device, the memory BAR, network, files
```

Only the kernel's own client (sender pid 0) and a sender the kernel says
holds StoreWrite reach the medium; the kernel is asked on every request.

## Privacy and persistence

The driver keeps no block beyond the request it serves. The console names
each unit's address, vendor, product and size, never its serial number.

## Runtime lifecycle

Claim, command bits, the port grant, the DMA grant. A hard reset, its
diagnostics awaited (up to 10 s), the adapter ready and asking for
mailboxes; DISABLE HOST ADAPTER INTERRUPT (an adapter without it is left to
Interrupt Disable); INQUIRE CONFIGURATION for the adapter's own SCSI ID;
INITIALIZE EXTENDED MAILBOX with one 32-bit pair at the grant's start,
which the adapter must take. Each command goes byte by byte through the
command register, its reply bytes read back, its completion awaited and the
interrupt it latched cleared. Then targets 0 to 15 but the adapter's own:
REPORT LUNS on LUN 0 (a selection timeout costs that one request), each
listed LUN up to 7 probed through `nonos_scsi`. An adapter with nothing on
it is given back and the driver leaves.

One request at a time: the CCB (an initiator CCB that reports its residual,
the checked direction, the CDB up to 12 bytes, the data buffer and its
length or none, the sense buffer, the target and LUN), its address in the
outgoing mailbox and then the start action, START MAILBOX COMMAND, then the
incoming mailbox polled: 64 looks a yield apart, then a millisecond between
looks, up to 30 s.

## Failure model

Every bring-up step names its failure and gives back what was taken, the
claim last; the adapter is reset on the way out. A selection timeout ends a
command as no target; an unexpected bus free or a bus reset is tried again,
as is an aborted CCB (four sends at most, with unit attentions and BUSY
status under the same rule); any other adapter status fails it, as does a
completion for a CCB the driver did not send. A data underrun is a residual,
not an error. A request never completed resets the adapter and gives it its
mailboxes again; one that does not take them is given up on and every later
request answers -5.

## Current implemented surface

- Up to 16 units: disks of 512 to 4096-byte blocks up to 2^32 blocks (a
  CCB carries at most a 12-byte CDB, so READ CAPACITY (16) and READ (16)
  cannot be sent), and CD/DVD drives.

## Wire format

The block wire as `nonos_scsi_serve/src/wire` defines it; the 32-bit
mailboxes and CCB as the MultiMaster technical reference and Linux's
`BusLogic.h` lay them out (`src/constants/regs.rs`).

## State ownership

The capsule owns the adapter, its mailboxes and the buffer. The kernel's
block layer owns what is read and written, and when.

## Operating rules

- One CCB out at a time; nothing is sent to an adapter that did not come
  back.
- Never select the adapter's own SCSI ID.
- Never log a serial number.

## Release target

Disks and CD drives on VirtualBox's and VMware's BusLogic controllers, read
and written through the block layer.

## Release evidence

`buslogic_proofs` runs the shipping source, the setup included, and
`nonos_scsi_serve` against a MultiMaster model (VirtualBox's register and
command protocol, the hard reset, the extended mailboxes, each CCB run on
`nonos_scsi`'s simulated units), the broker's port I/O routed into the
model and its DMA grant on host memory: the broker calls and the adapter's
commands in order, every refusal and what it gives back, an adapter without
the interrupt command, the adapter's own ID left out of the scan wherever
it is, a disk past 2 TiB refused, what each CCB carries, many requests
through one mailbox pair, every completion and adapter status, sense,
residuals and an underrun status, a lost request and the adapter brought
back once, the adapter given up on, discovery. A boot is the owner's.

## Release checklist

- Signed manifest and kernel mirror present (`driver_buslogic` keys).
- `buslogic_proofs` and `nonos_scsi` tests pass.
- The kernel's block layer row for `driver.buslogic0` (the maintainers').
- VirtualBox and VMware with a BusLogic controller install to and boot
  from the disk.

## Explicit non-goals today

No 24-bit (ISA) mailboxes, no scatter-gather, no tagged queueing, no
interrupts, no FlashPoint adapters, no disks past 2^32 blocks.

## Verification

- Build: `make -B nonos-mk-driver-buslogic`
- Proofs: `cd userland/buslogic_proofs && cargo test --release`
- Static gate: `bash nonos-ci/run-static-checks.sh`
