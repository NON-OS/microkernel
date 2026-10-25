# capsule_driver_virtio_gpu

## Role

`capsule_driver_virtio_gpu` is the virtio GPU display-controller capsule. It
owns the PCI device claim, BAR mapping, control-queue DMA, the virtio
initialization sequence, and the display behind `driver.virtio_gpu0`, which
it serves on the display wire (`nonos_gfx_wire`, through `nonos_gfx_serve`)

```text
compositor (the only sender the kernel admits)
        |
        v
driver.virtio_gpu0 -- control queue DMA --> virtio-gpu device
        |
        |-- primaries: anonymous pages registered and shared as surfaces
        `-- per output: a surface of its own (DMA + a 2D resource) on its scanout
```

The capsule is not a compositor, window manager, renderer, toolkit, or input
router. It answers the wire's CAPS, MODES, STATE, SURFACE, FLUSH, MODE_SET
and RETIRE; all desktop policy stays in the compositor.

## Microkernel contract

The manifest grants `IPC`, `Memory`, `GraphicsSurfaceCreate`, `Driver`,
`DeviceEnum`, `Mmio` and `Dma`, and no `Pio`: both virtio layouts are
reached through a memory BAR (`userland/nonos_virtio`):

```text
CAPSULE_REQUIRED_CAPS = 0xB9018
CAPSULE_OPTIONAL_CAPS = 0x100
```

The optional bit is `Debug`, which only a `capsule-serial-debug` build grants.
`GraphicsSurfaceCreate` admits `MkSurfaceRegister` and `MkSurfaceShare`, which
publish each primary as a surface. `Pio` is no longer used: it served
only the legacy transport, which is gone; the bit can leave the manifest at
its next signing.

The capsule reaches hardware only through `MkDeviceList`, `MkDeviceClaim`,
`MkPciConfigRead`/`MkPciConfigWrite`, `MkMmioMap` and `MkDmaMap`; it polls and
binds no interrupt, so the manifest holds no `Irq`. It never unmaps a DMA
grant itself: `MkDeviceRelease` takes them back once bus mastering is off. The kernel brokers
grants, routes IPC, validates the signed manifest, and tears grants down on
exit. It does not
interpret GPU resources, scanout policy, composition, cursor policy, or window
ownership.

Only the compositor may send to `driver.virtio_gpu0`: the kernel holds the
endpoint to it (`src/services/registry/held.rs`), by name and by pid. The

## Interface contract

The display wire (`nonos_gfx_wire`), served by `nonos_gfx_serve`:

| Operation | What virtio-gpu answers |
|---|---|
| `CAPS` | kind 1 (VIRTIO_GPU), one output per enabled scanout (at most 4), `MODE_SET \| MODE_RANGE \| HINTS`, sides to 4096, surface budget one display grant (32 MiB) |
| `MODES` | the host's wanted size (preferred), then output 0's EDID modes, then the size shown; none whose surface would not fit one display grant |
| `STATE` | the size shown and the host's wanted size: scanning out; dark (no flags) while the host has the scanout off; LOST once it is back, while the wire server shows the primary on it again. A raised display event is read first (and on every turn of the receive loop) |
| `SURFACE`, `FLUSH`, `MODE_SET`, `RETIRE` | as the wire defines. A flush copies into the output's surface, sends the rectangles to the host and flushes them. A mode set makes the new surface, loads it whole, and only then sets it on the scanout |
| `CURSOR` | `ENOSYS`: no cursor plane yet |

While the host has an output's scanout off, its FLUSH sends nothing and
its MODE_SET answers `EAGAIN`; the scanout is let go of and its surface
kept (`src/display/plug.rs`). A display event that changes an output (off,
back, or a new wanted size) is told to the compositor at once with
`OUTPUT_NOTICE` (`OUTPUTS_CHANGED`). Only scanouts taken at bring-up are
outputs: one the host turns on for the first time later is not driven.

A frame from sender 0 is a DGON: one naming this device ends the driver.
The driver's own private ops (`OP_HEALTHCHECK`, `OP_GET_PRIMARY_SURFACE` and
state shows on the wire as each output's STATE (below, Failure model).

## Authority

The capsule may claim the virtio GPU PCI device, map the virtio 1.0 structures
its capabilities place in a memory BAR, allocate broker-owned DMA for control
queue 0 and for each output's surface, and register and share the
primaries' pages as surfaces. It binds
no interrupt. It has no framebuffer MMIO grant, no compositor authority, no
input authority, and no filesystem or network authority.

## Privacy and persistence

The capsule stores no windows, screenshots, pointer paths, keystrokes, or
display history. Each output's surface holds the last frame flushed; it is a
DMA grant, revoked on exit and zeroed by the broker before reuse. When the
primaries from zeroed pages. Runtime
state is otherwise grant ids, queue addresses, the heads (each output's
scanout, surface and the host's wanted size), and surfaces in limbo.

## Runtime lifecycle

Startup discovers a virtio-gpu PCI function (1af4:1050), claims it, turns
memory decode on with its legacy interrupt line off, maps the common, notify
and device structures through the shared transport (`userland/nonos_virtio`),
resets it (status 0, read back by the clock), negotiates features (VERSION_1,
VIRGL and EDID when offered, and ACCESS_PLATFORM whenever offered), allocates
queue DMA, programs control queue 0, and only then turns bus mastering on and
sets DRIVER_OK: a device left live by a dead predecessor may reach memory only
once it reads back reset and its queue is on this grant, and a device refused
at any step never may. It then reads the scanouts, reads output 0's EDID when
offered (the wire's panel), runs a best-effort virgl render probe that only
logs whether 3D works, and takes every enabled scanout
(`src/display/take.rs`): a zeroed DMA surface of the host's size, a 2D resource
on it, sent whole, set on the scanout and flushed. A probe command that stalls
resets the device; the queue is restarted on the same grant and the outputs
are still taken (`src/setup/probes.rs`). It then serves the display wire,
Whether the claim is confined to an IOMMU domain is the kernel's to enforce at
claim time; the capsule does not guess.

## Failure model

A failed attempt resets the device once its window is mapped and then
releases the claim, which takes every grant with it, whichever step failed;
nothing is unmapped before that. Bring-up is tried a bounded number of times with a sleep between tries
(`nonos_libc::bring_up`); the capsule exits 2 when no virtio-gpu is present and
6 when one is present but never comes up, and the compositor falls back to GOP.
A missing common, notify or device configuration (refused by the one it
lacks), no VERSION_1, a zero queue size, or FEATURES_OK rejection prevents the
capsule from serving. A scanout mode under 16x16, or whose frame would not fit
the broker's display grant (32 MiB, one 3840x2160 frame), is recorded as
1280x720 rather than failing the bring-up; any other is taken as the device
gives it.

One control command is in flight at most, counted by the driver. A command
the device does not answer within `CTRLQ_MS` (2 s, by the uptime clock), or
answers out of turn, ends in a device reset: its chain named the staging page
the next command would reuse. By then the compositor draws into the
primaries and every output shows a surface whose pages the driver holds, so
the server never gives the device back or exits (the kernel would free the
pages under the host, and nothing restarts a driver). It recovers in place
on the next turn of the receive loop (`src/server/recover/`): the queue
restarted on the same grant, every output's surface made again under its own
id on its own pages, loaded whole and set on its scanout
(`src/display/reshow.rs`), at most three tries, each logged
(`driver.virtio_gpu0: control queue stalled; device reset, recovering (try 1
of 3)`). Surfaces in limbo go back to the kernel then: a reset host holds
none. If no try works, or the device stalls again after three recoveries in a
row without serving 30 s after any of them, it stays up and `Down`: the
device left reset, the claim and every grant kept, every output's STATE LOST
and dark under a new generation, FLUSH and MODE_SET `ENXIO`, and the
compositor told `OUTPUT_NOTICE` `LEAVING` once, so it drives another display.
Only a failed service registration, before anyone holds a primary, resets the
device and gives it back before exit 1.

flushed twice, so each surface, the host's copy and the screen hold none of its
frame. A blank a device did not take whole is tried again on the next wake. A
surface the host will not let go of stays in limbo with its pages mapped, never
handed back to the kernel while the host may read them. It is tried again on
every mode set and STATE (`src/display/limbo.rs`).

## Current implemented surface

- Virtio GPU PCI discovery for the modern ID (virtio-gpu has no
  transitional form, and nothing presents 0x1010).
- Brokered device claim, register map, and DMA queue allocation; polled.
- Virtio 1.0 initialization on the shared `userland/nonos_virtio`: the vendor
  capabilities parsed (first usable of each type, lengths and BARs checked,
  MSI-X table pages skipped), the regions mapped and bounds-checked, the
  features negotiated and the queue programmed.
- Config reads for events, scanout count, and capset count.
- The display wire: CAPS, MODES, STATE (with the host's display events),
  SURFACE, FLUSH, MODE_SET and RETIRE, through `nonos_gfx_serve`, for every
  enabled scanout.
- Static gates for capability boundary and endpoint ownership.

## Wire format

The display wire's 20-byte envelope (`NVGP` magic, version `1`) and its bodies,
as `nonos_gfx_wire` defines them. The driver's own private ops (0x02 to 0x0C)

## State ownership

`driver.virtio_gpu0` owns only hardware-facing GPU state: PCI claim, register
grant, control queue DMA grant, each output's surface (its DMA and its 2D
resource), the primaries' pages, and surfaces in limbo. The compositor owns surfaces, damage, focus, cursor, z-order, and
presentation policy.

## Operating rules

- Do not draw or composite in the driver capsule.
- Do not expose framebuffer pointers to userland clients.
- Keep scanout/resource policy above the driver.
- Keep queue DMA broker-owned and revoked on capsule exit.
- Add command submission only with bounded command/response buffers.

## Release target

The target chain is:

```text
driver.virtio_gpu0 -> compositor -> wm/toolkit/apps
```

The controlq commands GET_DISPLAY_INFO, RESOURCE_CREATE_2D, ATTACH_BACKING,
SET_SCANOUT, TRANSFER_TO_HOST_2D and RESOURCE_FLUSH are posted. What is left is
a QEMU virtio-gpu scanout validation on a booted image.

## Release evidence

Release evidence requires a QEMU `virtio-gpu-pci` boot, signed capsule spawn,
successful display-info controlq response, a bounded 2D resource flush to
scanout 0, and compositor presentation through the display runtime.

## Release checklist

- Capsule builds with zero warnings.
- Static gates confirm brokered MMIO/PIO/DMA authority and endpoint.
- Kernel profile `microkernel-driver-virtio-gpu` resolves signed artifacts.
- QEMU controlq GET_DISPLAY_INFO validation passes.
- QEMU resource flush validation presents visible pixels.

## Explicit non-goals today

This slice does not implement a compositor, 3D acceleration (the virgl probe
only logs whether the host could), Venus, Wayland, font rendering, app
surfaces, input routing, cursor policy, or window management.

## Verification

- Build: `make -B nonos-mk-driver-virtio-gpu`
- Kernel profile: `cargo check --no-default-features --features
  microkernel-driver-virtio-gpu`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Handbook: [drivers](../../docs/handbook/drivers.md),
  [compositor](../../docs/handbook/desktop/compositor.md).

## Real bring-up checklist

Proved on the host, without a device: the capability parse and the steps of
the shared transport (`userland/virtio_transport_proofs`), and the handshake,
the features taken, the control queue (a timed-out command reset, never run
with the next command's bytes) and its wire formats, the device configuration
offsets, the doorbell bound, Bus Master only after the reset and the queue
and never on a refusal, the scanout mode, the server's answers, the
recovery from a stall on the same surfaces or to `Down` with nothing released,
the recoveries in a row bounded, a probe stall at bring-up still ending in the
outputs taken, and the reset-before-release orders
(`userland/virtio_gpu_proofs`). Only a boot shows
the rest. Under QEMU q35, confirm both ways:

- Without an IOMMU (the default `virtio-vga`, which QEMU builds modern-only,
  1af4:1050): the driver binds modern; the
  mode is set at QEMU_XRES x QEMU_YRES; the display query reports the real
  scanout count; the compositor's requested resolutions are set; with
  `edid=on` the EDID preferred mode is what HiDPI negotiation picks.
- With the IOMMU lane (`-device intel-iommu` with `QEMU_IOMMU_OPTS`, and the
  device with `QEMU_IOMMU_VIRTIO`, i.e. `iommu_platform=on,disable-legacy=on`;
  see mk/10-qemu.mk) (`virtio-vga` made modern-only, 0x1050, structures in a BAR
  beside the framebuffer): the driver binds modern, with no claim and refusal
  loop; the scanout and capset counts are real numbers, not 0xFFFFFFFF; mode
  set, the compositor's resolutions and the EDID preferred mode work as above;
  with `virtio-vga-gl` VirGL comes up; the VT-d log shows no DMA fault.
- A device that cannot come up: at most seven attempts, one
  `driver.virtio_gpu0: device present, bring-up failed` line, exit 6, and
  the compositor falls back to GOP. No virtio-gpu at all: exit 2 at once.
- A device that stalls once the desktop is up: a `recovering (try 1 of 3)`
  line, then `recovered in place, same surface`, and the desktop presents
  again; one that never answers again: three tries, then `down`, the last
  frame stays, the capsule does not exit and the VT-d log shows no fault. One
  that stalls again within 30 s of each recovery: `down` after the third.
