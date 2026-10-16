# capsule_driver_bga

Status: not in the 0.9.2 image. The capsule builds and its host proofs

## Role

`driver.bga0` drives the Bochs/QEMU standard VGA adapter (1234:1111) and
serves the compositor the display wire it speaks to virtio-gpu. The
compositor looks it up after `driver.virtio_gpu0` and before GOP.

## Microkernel contract

```text
  broker / kernel                          capsule driver.bga0
  MkDeviceList, MkDeviceClaim  -------->   discover, setup::run
  PCI command: Memory Space on, Bus Master off
  MkMmioMap BAR2 (DISPI), BAR0 (VRAM) -->  mode kept or set, VRAM cleared
  MkMmap, MkSurfaceRegister, Share ---->   the surface, ordinary memory
  MkServiceRegister, MkIpcRecvFrom/Reply   the wire, below
  compositor -- get_primary ----------->   surface handle and geometry
  compositor -- flush (rectangle) ----->   surface -> VRAM copy
```

## Interface contract

Service 4236, reply 4237 (`endpoint.4294967392`). Only the compositor may
send to it once the kernel's holder table names it (C6, track A's).

## Authority

`CAPSULE_REQUIRED_CAPS` = 0x39018 (IPC, Memory, GraphicsSurfaceCreate,
DeviceEnum, Driver, Mmio), each bit named in Capsule.mk.
`CAPSULE_OPTIONAL_CAPS` = 0x100 (Debug). No Dma, Irq or Pio.

## Privacy and persistence

It keeps nothing but the compositor's frame in the surface. Nothing is
logged per request, and nothing is stored.

## Runtime lifecycle

With no adapter it exits `EXIT_ABSENT` (2) before claiming anything; one
that fails setup is retried on the shared bounded schedule, each try
releasing its claim, then `EXIT_GAVE_UP` (6). Up, it waits in its receive
for at most a second at a time. A refused service registration gives the
adapter back and exits 1.

## Failure model

Each attempt fails by name: a register mapping that ends inside the DISPI
block, a DISPI ID outside 0xB0C0 to 0xB0C5, a mode past VIDEO_MEMORY_64K
or past the BAR0 mapping, a mode that does not read back as set, a surface
the kernel refuses. The DISPI state firmware left is read before any write
and, where it changed, written back at every drop before the mappings and
the claim are given up. A capsule killed outright runs no drop: a kept OVMF
mode stays GOP's, as no DISPI register was written; a mode set stays set.

## Current implemented surface

The mode OVMF left for GOP is kept when it is 32 bits a pixel with its
linear framebuffer and unpanned; else 1024x768x32 is set. VRAM is cleared
once the surface is made. Served: get_primary, transfer, set_scanout, flush.

## Wire format

NVGP, virtio-gpu's ops and statuses. OP_GET_PRIMARY_SURFACE (0x0C) hands
the surface only to the pid `compositor` is registered to, others E_BUSY;
OP_FLUSH (0x0A) copies the rectangle into VRAM; OP_TRANSFER_TO_HOST (0x08)
names where it starts and moves nothing, the surface being the resource;
OP_SET_SCANOUT (0x09) takes only the whole mode, E_DEVICE once the adapter
drops it. A bad frame or rectangle is E_INVAL; an unserved op E_BAD_OP.

## State ownership

The compositor draws in the surface, ordinary memory. The VRAM mapping is
device memory and stays here: as a surface, the kernel's registry would
free its pages to the allocator when this capsule dies, and map them
cacheable in the compositor beside this capsule's uncached mapping.

## Operating rules

No MMIO outside broker grants; every rectangle off the wire is checked
against the mode before a pixel moves.

## Release target

Spawned after virtio-gpu wherever the adapter is present.

## Release evidence

Host proofs (`bga_proofs`); a QEMU `-vga std` OVMF boot showing the
desktop through `driver.bga0`.

## Release checklist

mirror, spawn and `DisplayBga` dispatch (`src/hardware/inventory`). The
embed and spawn wait for A's kernel fix reserving baked service names: a
capsule loaded at runtime can take `compositor` while it is free today,
and with it the primary.

## Explicit non-goals today

No mode set on request, no EDID, one scanout, no hardware cursor.

## Verification

Static gate: `nonos-ci/run-static-checks.sh`.
Proofs: `(cd userland/bga_proofs && cargo test --release)`.
Handbook: [drivers](../../docs/handbook/drivers.md).
