# capsule_driver_bga

Status: wired, not yet signed. The kernel mirror (`src/hardware/bga_capsule`)
embeds and spawns it under `nonos-capsule-driver-bga` (the
`microkernel-driver-bga` build) when the inventory sees `DisplayBga`, and the
compositor finds it after `driver.virtio_gpu0`. It has no owner certificate
yet, so no image carries it until the owner signs it.

## Role

`driver.bga0` drives the Bochs/QEMU standard VGA adapter (1234:1111) and
serves the compositor the display wire it speaks to virtio-gpu. The
compositor looks it up after `driver.virtio_gpu0` and before GOP. The
adapter scans out its VRAM and has no DMA, so the compositor draws in a
surface of ordinary memory the driver shares, and each flush copies the
damaged rectangle into VRAM.

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

Discovery takes only an adapter whose register BAR (BAR2) is MMIO and at
least 0x608 bytes, so it reaches QEMU's extension block; one reached only
through the 0x1CE/0x1CF ports is not served.

## Interface contract

Service 4240, reply 4241 (`endpoint.4294967326`), as the kernel spawns it.
Only the compositor may send to it: the kernel's holder table names it.

## Authority

`CAPSULE_REQUIRED_CAPS` = 0x39018 (IPC, Memory, GraphicsSurfaceCreate,
DeviceEnum, Driver, Mmio), each bit named in Capsule.mk.
`CAPSULE_OPTIONAL_CAPS` = 0x100 (Debug). No Dma, Irq or Pio.

## Privacy and persistence

It keeps nothing but the compositor's frame in the surface. Nothing is
logged per request, and nothing is stored.

## Runtime lifecycle

Spawned once, on `DisplayBga`. With no adapter it exits `EXIT_ABSENT` (2)
before claiming anything; one that fails setup is retried on the shared
bounded schedule, each try releasing its claim, then `EXIT_GAVE_UP` (6).
Up, it waits in its receive for at most a second at a time. A refused
service registration gives the adapter back and exits 1.

## Failure model

Each attempt fails by name: a register mapping that ends inside the DISPI
block, a DISPI ID outside 0xB0C0 to 0xB0C5, a mode past VIDEO_MEMORY_64K
(a report of 0 is no video memory) or past the BAR0 mapping, a mode that
does not read back as set, a surface the kernel refuses. The DISPI state
firmware left is read before any DISPI write and, where it changed,
written back at every drop before the mappings and the claim are given
up; the surface's share is released before its memory is unmapped. A
capsule killed outright runs no drop: a kept OVMF mode stays on the
adapter, as no DISPI register was written; a mode set stays set.

None of this gives the kernel's GOP blit back. Where BAR0 holds the boot
framebuffer, as under OVMF on stdvga, the claim, each attempt's first
step, stops the kernel's console for the rest of the boot (L79), and
MkSurfacePresent answers EBUSY from then on. A driver that fails a later
step, gives up or is killed leaves the last picture on screen and the
compositor nothing to draw it with.

## Current implemented surface

The framebuffer is set little endian where the adapter has QEMU's byte
order register. The mode OVMF left for GOP is kept when it is 32 bits a
pixel with its linear framebuffer and unpanned; else 1024x768x32 is set,
with the screen unblanked through the VGA attribute port first, the
adapter disabled while the mode registers and bank are written, and the
line layout after ENABLE. VRAM is cleared once the surface is made.
Served: get_primary, transfer, set_scanout, flush. One screen, polled by
request; no interrupts.

## Wire format

NVGP, virtio-gpu's ops and statuses, its 20-byte header (`NVGP`, version,
op, flags, request id, payload length), then a 4-byte status and the op's
payload; bga_proofs holds the numbers and the header layout equal to
virtio-gpu's. OP_GET_PRIMARY_SURFACE (0x0C) hands the surface only to the
pid `compositor` is registered to, others E_BUSY; OP_FLUSH (0x0A) copies
the rectangle into VRAM; OP_TRANSFER_TO_HOST (0x08) names where it starts
and moves nothing, the surface being the resource; OP_SET_SCANOUT (0x09)
takes only the whole mode, E_DEVICE once the adapter drops it. A bad frame
or rectangle is E_INVAL; an unserved op E_BAD_OP, HEALTHCHECK among them.

## State ownership

The compositor draws in the surface, ordinary memory. The VRAM mapping is
device memory and stays here: as a surface, the kernel's registry would
free its pages to the allocator when this capsule dies, and map them
cacheable in the compositor beside this capsule's uncached mapping. The
capsule owns the claim and both MMIO grants.

## Operating rules

No MMIO outside broker grants; every rectangle off the wire is checked
against the mode before a pixel moves.

## Release target

The compositor's gfx backend on QEMU stdvga (and Bochs), spawned wherever
the adapter is present, in place of the kernel's GOP blit.

## Release evidence

Host proofs (`bga_proofs`); a QEMU `-vga std` OVMF boot showing the
desktop through `driver.bga0`.

## Release checklist

- Signed by the owner and added to the desktop image's features.
- A QEMU stdvga boot showing the desktop through `driver.bga0`.

## Real hardware bring-up checklist

Bochs and QEMU only; there is no such silicon. Under QEMU `-vga std`:

- the firmware's mode (OVMF's) is kept, with no flicker at the switch;
- the compositor moves from GOP to `driver.bga0` and the desktop keeps
  drawing;
- a machine without the adapter shows `[EXIT] <service> status 2`.

## Explicit non-goals today

No mode set on request, no EDID, one scanout, no hardware cursor, no
VirtualBox (80ee:beef) or Bochs I/O-port-only adapter.

## Verification

Static gate: `nonos-ci/run-static-checks.sh`.
Proofs: `(cd userland/bga_proofs && cargo test --release)`.
Handbook: [drivers](../../docs/handbook/drivers.md).
