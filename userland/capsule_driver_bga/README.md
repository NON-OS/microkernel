# capsule_driver_bga

Status: not in the image. The capsule builds and its host proofs pass, but
it has no owner certificate and the kernel never spawns it: the kernel
mirror, the spawn on `DisplayBga` and the compositor's graphics list come in
the next unit.

## Role

`capsule_driver_bga` drives the Bochs graphics adapter (QEMU's stdvga,
1234:1111) as Linux `bochs.c` does, and serves it to the compositor on the
gfx wire `driver.virtio_gpu0` speaks. The adapter scans out its VRAM and has
no DMA, so the driver keeps a shadow surface in ordinary memory, shares it
with the compositor, and copies each presented rect into VRAM.

```text
compositor --NVGP--> driver.bga0 --copy--> VRAM (BAR 0) --> scanout
     |                    |
     `-- draws into ------' shadow surface (shared)
```

## Microkernel contract

- `MkDeviceList` finds 1234:1111 with an MMIO register BAR (BAR 2, at least
  0x608 bytes) and a framebuffer BAR (BAR 0).
- `MkDeviceClaim` claims it; `MkPciConfigWrite` sets Memory Space only (no
  bus mastering: the adapter never masters the bus).
- `MkMmioMap` maps BAR 2 and as much of BAR 0 as the mode needs.
- `MkMmap`, `MkSurfaceRegister` and `MkSurfaceShare` make the shadow.
- `MkIpcRecvFrom` serves `service:4240:driver.bga0`; `MkIpcReply` answers.

## Interface contract

NVGP, version 1, as `driver.virtio_gpu0` frames it (its header and codec
files are mounted, not copied):

- `HEALTHCHECK` (0x01);
- `GET_PRIMARY_SURFACE` (0x0C): the shadow's handle, resource 1, width,
  height, stride, format 1 (B8G8R8A8). The first caller holds the screen;
  any other gets -16;
- `TRANSFER_TO_HOST` (0x08): copies the rect, whose rows start at the
  request's offset in the shadow, into VRAM;
- `SET_SCANOUT` (0x09): scanout 0, resource 1, the whole mode;
- `FLUSH` (0x0A): checked and answered. VRAM is what the adapter scans out,
  so the transfer already showed the rect.

Other ops answer -38; a malformed body -22.

## Authority

`CAPSULE_REQUIRED_CAPS = 0x39018`: IPC, Memory, GraphicsSurfaceCreate,
DeviceEnum, Driver, Mmio. No Dma, Irq, Pio, Crypto, Network, filesystem or
admin authority.

## Current implemented surface

- DISPI id check (`(id & 0xfff0) == 0xb0c0`) and VRAM from
  `VIDEO_MEMORY_64K`, capped at BAR 0.
- QEMU's framebuffer byte order set little endian where the register is.
- The firmware's mode is kept when it is enabled at 32 bpp with its linear
  framebuffer, unpanned, and fits. Otherwise 1024x768x32 is set in
  `bochs_hw_setmode`'s order, read back, and cleared.
- One screen, polled by request; no interrupts.

## Wire format

As `driver.virtio_gpu0`: a 20-byte NVGP header (`NVGP`, version, op, flags,
request id, payload length), then a 4-byte status and the op's payload.

## State ownership

The capsule owns the claim, both MMIO grants and the shadow. The compositor
attaches the shadow and draws into it; the driver only reads it.

## Operating rules

No MMIO or framebuffer access outside broker grants. No present from a
process other than the screen's holder.

## Privacy and persistence

The shadow holds what is on screen; nothing is written anywhere else or kept
past the process.

## Runtime lifecycle

Spawned once (next unit). Each request is answered in turn; between them the
driver waits in the kernel.

## Failure model

With no adapter the capsule logs one line and exits `EXIT_ABSENT` (2) before
claiming anything. A failed attempt drops its claim, which releases the
adapter and its grants, and is retried on the shared bounded schedule; running
out exits `EXIT_GAVE_UP` (6) with the last cause: no DISPI id, no mode fits,
mode refused, shadow refused, or a broker refusal.

## Release target

The compositor's gfx backend on QEMU stdvga (and Bochs), in place of the
kernel's GOP blit.

## Release evidence

`bga_proofs` on the host; the QEMU `vga=std` boot below.

## Release checklist

- Kernel mirror, spawn on `DisplayBga`, compositor graphics list.
- Signed by the owner.
- A QEMU stdvga boot showing the desktop through `driver.bga0`.

## Real hardware bring-up checklist

Bochs and QEMU only; there is no such silicon. Under QEMU `-vga std`:

- the firmware's mode (OVMF's) is kept, with no flicker at the switch;
- the compositor moves from GOP to `driver.bga0` and the desktop keeps
  drawing;
- a machine without the adapter shows `[EXIT] <service> status 2`.

## Explicit non-goals today

No mode change after start, no second head, no hardware cursor, no
VirtualBox (80ee:beef) or Bochs I/O-port-only adapter.

## Verification

Static gate: `nonos-ci/run-static-checks.sh`.
Proofs: `(cd userland/bga_proofs && cargo test --release)`.
Handbook: [drivers](../../docs/handbook/drivers.md).
