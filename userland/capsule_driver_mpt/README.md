# capsule_driver_mpt

## Role

`capsule_driver_mpt` drives the LSI Fusion-MPT SCSI controllers through
their MPI 1.x message interface: the 53C1030 (PCI 1000:0030; VMware's LSI
Logic Parallel, VirtualBox's LsiLogic) and the SAS1068 (1000:0054; VMware's
LSI Logic SAS, VirtualBox's LsiLogic SAS, QEMU's `mptsas1068`). VMware
configures one of these for most guests. It serves the disks and CD/DVD
drives on its targets on the block wire (`NBW1`). The SCSI commands are
`nonos_scsi`'s and the block wire is `nonos_scsi_serve`'s, shared with every
SCSI host driver; this capsule only moves a SCSI IO request through the IOC.

```text
kernel block layer / installer --NBW1 (unit)--> driver.mpt0
       nonos_scsi_serve -> nonos_scsi (CDB, sense, retries) -> MptHba
  request frame --request FIFO--> IOC --reply FIFO--> context, or a reply frame
```

## Microkernel contract

- `MkDeviceList` finds the first 53C1030 or SAS1068 function and the first
  memory BAR that holds its system interface (BAR1 on every one of these).
- `MkDeviceClaim`; `MkPciConfigWrite` sets Memory Space, Bus Master and
  Interrupt Disable.
- `MkMmioMap` of the register window.
- `MkDmaMap` once, coherent and below 4 GiB (frames, reply frames and the
  sense buffer are named with a zero high half): the request frame, eight
  128-byte reply frames, the sense buffer, then a 64 KiB buffer.
- No `MkIrqBind`: every interrupt is masked and the driver polls the reply
  FIFO.
- `MkIpcRecv` serves `service:4298:driver.mpt0`; replies go to
  `endpoint.4294967359`.

## Interface contract

The block wire as `nonos_scsi_serve` serves it, the same as every SCSI
host driver's: HEALTHCHECK 1, GEOMETRY 2 (blocks, block bytes, max transfer
65536, flags 1 removable / 2 read-only / 4 a medium in), READ 3, WRITE 4,
FLUSH 5, IDENTIFY 6 (vendor and product, serial, medium 6). Units are
numbered in scan order, by target (as many as the IOC's facts give) then
LUN (to 7). Statuses beside the wire's own: -5 (the target or the IOC
failed the command, or moved less than asked), -6 (no target, or a block
past the medium), -30 (write protected or a CD drive), -110 (the IOC did
not complete the request in 30 s; it was reset and brought up again), -123
(no disc).

## Authority

`CoreExec`, `IPC`, `Memory`, `Driver`, `DeviceEnum`, `Mmio`, `Dma`
(`CAPSULE_REQUIRED_CAPS = 0xB8019`); `Debug` optional
(`CAPSULE_OPTIONAL_CAPS = 0x100`). No `Pio`, `Irq` or `Network`. CoreExec
is for the driver's own pid: its serve loop also drains its reply endpoint,
and a reply no caller took is dropped rather than answered.

```text
allowed:   the IOC claim, its register window, one DMA grant
forbidden: any other device, the I/O and diagnostic BARs, events, config
           pages, firmware, network, files
```

Only the kernel's own client (sender pid 0) and a sender the kernel says
holds StoreWrite reach the medium; the kernel is asked on every request.

## Privacy and persistence

The driver keeps no block beyond the request it serves. The console names
each unit's address, vendor, product and size, never its serial number.

## Runtime lifecycle

Claim, command bits, the register window, the DMA grant. Every interrupt
masked; the IOC ready (a message unit reset when it is not, up to 10 s);
IOC FACTS through the doorbell handshake (the start word, each message
dword acked, the reply sixteen bits per doorbell interrupt and one more to
end it): its request frame size checked against the SCSI request's, its
device count, one bus; IOC INIT through the handshake (the host driver,
those devices on one bus, 128-byte reply frames, every high address half
zero, MPI 1.5) and the IOC operational; the eight reply frames handed to
the reply free FIFO; PORT ENABLE through the request FIFO. Then every
target the facts give: REPORT LUNS on LUN 0 (no device there costs that one
request), each listed LUN up to 7 probed through `nonos_scsi`. An IOC with
nothing behind it is given back and the driver leaves.

One request at a time: the frame (target, bus 0, the CDB up to 16 bytes,
96 bytes of sense and its buffer, a fresh context, the LUN in byte 1 of the
LUN field, the direction, the length, one simple 32-bit SGE ending buffer
and list, host to IOC for a write), posted to the request FIFO, then the
reply FIFO polled: 64 looks a yield apart, then a millisecond between
looks, up to 30 s. A context reply is a request that went well, all of
it; an address reply is read from its frame and the frame given back.

## Failure model

Every bring-up step names its failure and gives back what was taken, the
claim last; the IOC is reset on the way out. A missing bus, target or
device ends a command as no target; a busy IOC or a terminated task is
tried again (four sends at most, with unit attentions and BUSY status under
the same rule); any other IOC status fails it, as does a reply for a
context the driver did not send. A recovered error or an underrun is not an
error; the residual is the length less the transfer count. A request never
completed resets the IOC and brings it up again; one that does not come
back is given up on and every later request answers -5.

## Current implemented surface

- Up to 16 units: disks of 512 to 4096-byte blocks and CD/DVD drives, as
  `nonos_scsi` serves them.

## Wire format

The block wire as `nonos_scsi_serve/src/wire` defines it; the MPI 1.x
system interface, the handshake and the IOC FACTS, IOC INIT, PORT ENABLE
and SCSI IO messages and replies as LSI's MPI specification and Linux's
`mpi*.h` lay them out (`src/constants/regs.rs`, `src/constants/msg.rs`).

## State ownership

The capsule owns the IOC, its frames and the buffer. The kernel's block
layer owns what is read and written, and when.

## Operating rules

- One request out at a time; nothing is sent to an IOC that did not come
  back.
- Every reply frame read is given back to the IOC.
- Never unmask an interrupt; never ask for events, config pages or
  firmware.
- Never log a serial number.

## Release target

Disks and CD drives on VMware's and VirtualBox's LSI Logic controllers and
QEMU's `mptsas1068`, read and written through the block layer.

## Release evidence

`mpt_proofs` runs the shipping source and `nonos_scsi_serve` against an
MPI 1.x IOC model (the doorbell and its handshake, the message unit reset,
IOC FACTS and IOC INIT, the request FIFO, the reply post and free FIFOs,
each SCSI IO request run on `nonos_scsi`'s simulated units): the bring-up
in order with every field of IOC INIT, an operational IOC reset first,
every refusal, an older IOC's shorter facts, port enable answered in a
frame, the scan across the IOC's devices, discovery of both parts, what
each frame carries, context and address replies, every reply frame given
back, every IOC status, sense, an underrun and a recovered error, a lost
request and the IOC brought back once, the IOC given up on. A boot is the
owner's.

## Release checklist

- Signed manifest and kernel mirror present (`driver_mpt` keys).
- `mpt_proofs` and `nonos_scsi` tests pass.
- The kernel's block layer row for `driver.mpt0` (the maintainers').
- VMware with LSI Logic Parallel and LSI Logic SAS, VirtualBox with
  LsiLogic and LsiLogic SAS, and QEMU `-device mptsas1068` install to and
  boot from the disk.

## Explicit non-goals today

No MPT2/MPT3 controllers (SAS2008 and later: another interface), no chain
or 64-bit SGEs (one 64 KiB buffer below 4 GiB), no events, no config pages,
no task management beyond the reset on a lost request, no interrupts.

## Verification

- Build: `make -B nonos-mk-driver-mpt`
- Proofs: `cd userland/mpt_proofs && cargo test --release`
- Static gate: `bash nonos-ci/run-static-checks.sh`
