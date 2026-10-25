# capsule_driver_bga

Status: wired, in its own `microkernel-driver-bga` profile; not yet signed.
The kernel mirror (`src/hardware/bga_capsule`) embeds and spawns it under
`nonos-capsule-driver-bga` when the inventory sees `DisplayBga`, and the
compositor finds it after `driver.virtio_gpu0`. It joins
in the 0.9.3 seal (see the release checklist); until then it is in
`scripts/baselines/display_profiles.txt`.

## Role

Bochs Graphics Adapter (QEMU stdvga 1234:1111, bochs-display) display
driver, serving the display wire (NVGP v1, PLAN-coverage section 12) on
`nonos_gfx_serve` as `driver.bga0`. The compositor looks it up after
`driver.virtio_gpu0` and before GOP. It keeps the mode firmware left until
the compositor asks for another, so it no longer destroys the firmware
scanout that kept `DisplayBga` undispatched
(`src/hardware/inventory/driver.rs`). Its trust artifacts (`Capsule.mk`)
scans out its video memory and has no DMA, so the compositor draws in
primaries of ordinary memory the driver shares, and each flush copies the
damaged rectangles into video memory.

## Microkernel contract

```text
  broker / kernel                          capsule driver.bga0
  MkDeviceList, MkDeviceClaim  -------->   discover, setup::run
  PCI command: Memory Space on, Bus Master off
  MkMmioMap BAR2 (EDID, DISPI), BAR0 -->   ID checked, firmware's DISPI read,
                                           VRAM zeroed, mode kept or set
  MkMmap, MkSurfaceRegister, Share ---->   primaries (nonos_gfx_pool)
  MkServiceRegister, MkIpcRecvFrom/Reply   the display wire, below
```

Discovery takes only an adapter whose register BAR (BAR2) is MMIO and at
least 0x608 bytes, so it reaches QEMU's extension block; one reached only
through the 0x1CE/0x1CF ports is not served.

Service 4240, reply 4241 (`endpoint.4294967326`), as Capsule.mk declares
them and the kernel spawns it (4236 is driver.gpio0's). Only the compositor
may send to it: the kernel's holder table names it (C6). No Dma (the adapter
masters nothing), no Irq (it polls nothing either: every wire request is
answered at once), no Pio (the DISPI block is read at BAR2+0x500).

## Interface contract

The display wire as a single-output DISPI device (`kind` 2): CAPS
(MODE_SET | MODE_RANGE, 640x480 to 4096x4096, widths a multiple of 8, no
hardware cursor), MODES (the EDID's modes that fit, and the one shown),
SURFACE, MODE_SET, FLUSH, STATE, RETIRE; CURSOR answers ENOSYS.
STATE checks DISPI still shows the mode (enabled, linear, 32 bits, its size,
from the start of video memory); if not, the mode is set again before the
answer and STATE says LOST with a new generation, so the compositor
repaints (`display/health.rs`).

## Authority

`CAPSULE_REQUIRED_CAPS` = 0x39018 (IPC, Memory, GraphicsSurfaceCreate,
DeviceEnum, Driver, Mmio), each bit named in Capsule.mk.
`CAPSULE_OPTIONAL_CAPS` = 0x100 (Debug). Spawned, it holds the adapter's
claim, BAR0 (video memory, write-combining) and BAR2 (EDID and DISPI,
uncached).

## Privacy and persistence

It keeps nothing but the compositor's frame in its primaries and video
memory. Nothing is logged per request, and nothing is stored.

## Runtime lifecycle

Spawned once, on `DisplayBga`; it serves until stopped. It sleeps in
`mk_ipc_recv_from` between requests and holds no core. A refused service
registration gives the adapter back, firmware's DISPI state written back,
and exits 1.

## Failure model

Each attempt fails by name: a broker claim or map refused, a register
mapping that ends inside the DISPI block, a DISPI ID outside 0xB0C0 to
0xB0C5, a start-up mode past VIDEO_MEMORY_64K (a report of 0 is no video
memory) or past the BAR0 mapping, a mode that does not read back as set (its
line layout included). The DISPI state firmware left is read before any
write and, where it changed, written back at every drop before the mappings
and the claim are given up; each primary's share is released before its
memory is unmapped (`nonos_gfx_pool`). A capsule killed outright runs no
drop: a kept OVMF mode stays on the adapter, as no DISPI register was
written; a mode set stays set. A MODE_SET the adapter does not take is read
back, refused with EINVAL and the previous mode put back.

None of this gives the kernel's GOP blit back. Where BAR0 holds the boot
framebuffer, as under OVMF on stdvga, the claim, each attempt's first step,
stops the kernel's console for the rest of the boot (L79), and
MkSurfacePresent answers EBUSY from then on. A driver that fails a later
step, gives up or is killed leaves what video memory last held on screen
and the compositor nothing else to draw it with.

With no BGA adapter in the device list the capsule logs one line and exits
`EXIT_ABSENT` (2) before claiming anything. An adapter that is present but
fails setup is retried on the shared bounded schedule
(`nonos_libc::bring_up`: seven tries, sleeping between them, each failed try
having released its claim); running out exits `EXIT_GAVE_UP` (6).

## Current implemented surface

Discovery, claim and maps; the EDID read at BAR2+0 (`edid/`); the DISPI ID
check, video memory, mode set, read-back and firmware write-back (`dispi/`);
the start-up mode (firmware's when it is 32 bits, linear, unpanned, its
lines no narrower than it, and fits both video memory and the mapping,
else the EDID's preferred, else 1024x768: `setup/mode.rs`); the wire's
device half (`display/`) and kernel half (`nonos_gfx_pool`), served by
`serve.rs`. Video memory is zeroed whole before any mode is touched
address source, so the adapter shows a picture (`dispi/unblank.rs`), then
writes the mode with the adapter disabled and the bank back at 0, and the
line layout after ENABLE; where BAR2 holds the byte-order register it is
set little endian at bring-up and not written back at drop
(`dispi/endian.rs`).

## Wire format

```text
compositor -> driver.bga0 (port 4240): NVGP v1 frame -> reply frame
```

## State ownership

Each primary is anonymous memory of this capsule's, registered as an
ARGB8888 surface and shared with the compositor alone, writable: the share
into video memory and fences the copy before the answer. Video memory itself is never shared: as a
surface, the kernel's registry would free its pages to the allocator when
this capsule dies, and map them cacheable in the compositor beside this
capsule's write-combining mapping. The capsule owns the claim and both MMIO
grants.

## Operating rules

No MMIO outside broker grants; every rectangle off the wire is checked
against the mode before a pixel moves.

## Release target

Brokered BGA display driver on the display wire, spawned after virtio-gpu
wherever the adapter is present, in place of the kernel's GOP blit.

## Release evidence

Host proofs (`bga_proofs`); a QEMU `-vga std` OVMF boot showing the
desktop through `driver.bga0`.

## Release checklist

  the desktop image's features.
- Track A's held-table entry, kernel mirror, spawn and `DisplayBga`
  dispatch (`src/hardware/inventory`): in place. A capsule loaded at runtime
  may claim only names it already holds (`src/services/registry/reserved.rs`),
  so none can take `compositor`, and with it the primaries: proved on the
  kernel's own files in `bga_proofs` (`syscall/microkernel/ipc/claim_tests`).
- A QEMU stdvga boot showing the desktop through `driver.bga0`.

## Real hardware bring-up checklist

Bochs and QEMU only; there is no such silicon. What only a boot can
confirm; the host proofs cover the parsers, the bring-up sequence against a
model, and the bounded retry. Under QEMU `-vga std`:

- On a machine without the device: the broker log shows no claim for it and
  the capsule exits at once with `EXIT_ABSENT` (2); the capsule's own
  `no controller present, not started` line needs the Debug capability, so
  without it the kernel prints `[EXIT] <service> status 2: no device
  present, not started` for it (`src/process/exit/end_note.rs`).
- With the device present but failing (disabled in firmware, or a forced
  setup error): at most seven claim and release rounds in the broker log
  over about six seconds, then one `device present, bring-up failed` line
  naming the last cause (with the Debug capability), and exit
  `EXIT_GAVE_UP` (6), which the kernel names as `[EXIT] <service> status 6:
  device present, bring-up failed and was given up`. The capsule holds no
  core while it waits.
- The firmware's mode (OVMF's) is kept, with no flicker at the switch, and
  the compositor moves from GOP to `driver.bga0` and the desktop keeps
  drawing.
- With DISPI left off, the EDID's preferred mode (QEMU's default 1280x800;
  unverified) is set.
- A MODE_SET to each EDID mode changes the picture in one step, with no
  black frame.
- Between requests the capsule is asleep: no CPU time accrues to it.

## Explicit non-goals today

No hardware cursor, no display hints (Decision 15), no second output, no
VirtualBox (80ee:beef) or Bochs I/O-port-only adapter, and no use of the
adapter outside QEMU and Bochs.

## Verification

Static gate: `nonos-ci/run-static-checks.sh`.
Proofs: `(cd userland/bga_proofs && cargo test --release)`.
Handbook: [drivers](../../docs/handbook/drivers.md).
