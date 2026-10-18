# capsule_driver_bga

Status: not in the 0.9.2 image. The capsule builds and its host proofs

## Role

Bochs Graphics Adapter (QEMU stdvga 1234:1111, bochs-display) display
driver, serving the display wire (NVGP v1, PLAN-coverage section 12) on
`nonos_gfx_serve` as `driver.bga0`. The compositor looks it up after
`driver.virtio_gpu0` and before GOP. It keeps the mode firmware left until
the compositor asks for another, so it no longer destroys the firmware
scanout that kept `DisplayBga` undispatched
(`src/hardware/inventory/driver.rs`). Its `Capsule.mk` keeps it out of the

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

Service 4236, reply 4237 (`endpoint.4294967392`), as Capsule.mk declares
them. Only the compositor may send to it once the kernel's holder table
names it (C6, track A's). No Dma (the adapter masters nothing), no Irq (it
polls nothing either: every wire request is answered at once), no Pio (the
DISPI block is read at BAR2+0x500).

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
memory. Nothing is
logged per request, and nothing is stored.

## Runtime lifecycle

Spawned, it serves until stopped; it sleeps in `mk_ipc_recv_from` between
requests and holds no core. A refused service registration gives the
adapter back, firmware's DISPI state written back, and exits 1.

## Failure model

Each attempt fails by name: a broker claim or map refused, a register
mapping that ends inside the DISPI block, a DISPI ID outside 0xB0C0 to
0xB0C5, a start-up mode past VIDEO_MEMORY_64K or past the BAR0 mapping, a
mode that does not read back as set (its line layout included). The DISPI
state firmware left is read before any write and, where it changed, written
back at every drop before the mappings and the claim are given up. A
capsule killed outright runs no drop: a kept OVMF mode stays GOP's, as no
DISPI register was written; a mode set stays set. A MODE_SET the adapter
does not take is read back, refused with EINVAL and the previous mode put
back.

With no BGA adapter in the device list the capsule logs one line and exits
`EXIT_ABSENT` (2) before claiming anything. An adapter that is present but
fails setup is retried on the shared bounded schedule
(`nonos_libc::bring_up`: seven tries, sleeping between them, each failed try
having released its claim); running out exits `EXIT_GAVE_UP` (6).

## Current implemented surface

Discovery, claim and maps; the EDID read at BAR2+0 (`edid/`); the DISPI ID
check, video memory, mode set, read-back and firmware write-back (`dispi/`);
the start-up mode (firmware's when it is 32 bits, linear, unpanned and fits
both video memory and the mapping, else the EDID's preferred, else
1024x768: `setup/mode.rs`); the wire's device half (`display/`) and kernel
half (`nonos_gfx_pool`), served by `serve.rs`. Video memory is zeroed whole

## Wire format

```text
compositor -> driver.bga0 (port 4236): NVGP v1 frame -> reply frame
```

## State ownership

Each primary is anonymous memory of this capsule's, registered as an
ARGB8888 surface; FLUSH copies its rectangles into video memory and fences
the copy before the answer. Video memory itself is never shared: as a
surface, the kernel's registry would free its pages to the allocator when
this capsule dies, and map them cacheable in the compositor beside this
capsule's write-combining mapping.

## Operating rules

No MMIO outside broker grants; every rectangle off the wire is checked
against the mode before a pixel moves.

## Release target

Brokered BGA display driver on the display wire, spawned after virtio-gpu
wherever the adapter is present.

## Release evidence

Host proofs (`bga_proofs`); a QEMU `-vga std` OVMF boot showing the
desktop through `driver.bga0`.

## Release checklist

- Track A's held-table entry, kernel mirror, spawn and `DisplayBga`
  dispatch (`src/hardware/inventory`). The embed and spawn wait for A's
  kernel fix reserving baked service names: a capsule loaded at runtime can
  take `compositor` while it is free today, and with it the primaries.

## Real hardware bring-up checklist

What only a boot on real silicon can confirm. The host proofs cover the parsers, the bring-up sequence against a model, and the bounded retry; these do not.

- On a machine without the device: the broker log shows no claim for it and the capsule exits at once with `EXIT_ABSENT` (2); the capsule's own `no controller present, not started` line needs the Debug capability, so without it the kernel prints `[EXIT] <service> status 2: no device present, not started` for it (`src/process/exit/end_note.rs`).
- With the device present but failing (disabled in firmware, or a forced setup error): at most seven claim and release rounds in the broker log over about six seconds, then one `device present, bring-up failed` line naming the last cause (with the Debug capability), and exit `EXIT_GAVE_UP` (6), which the kernel names as `[EXIT] <service> status 6: device present, bring-up failed and was given up`. The capsule holds no core while it waits.
- (Bochs or QEMU stdvga only.) With DISPI left off, the EDID's preferred mode (QEMU's default 1280x800; unverified) is set.
- A MODE_SET to each EDID mode changes the picture in one step, with no black frame.
- Between requests the capsule is asleep: no CPU time accrues to it.

## Explicit non-goals today

No hardware cursor, no display hints (Decision 15), no second output, and no
use of the adapter outside QEMU and Bochs.

## Verification

Static gate: `nonos-ci/run-static-checks.sh`.
Proofs: `(cd userland/bga_proofs && cargo test --release)`.
Handbook: [drivers](../../docs/handbook/drivers.md).
