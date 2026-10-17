# capsule_driver_virtio_gpu

## Role

`capsule_driver_virtio_gpu` is the virtio GPU display-controller capsule. It
owns the PCI device claim, BAR mapping, control-queue DMA, the virtio
initialization sequence, and the display behind `driver.virtio_gpu0`, which
it serves on the display wire (`nonos_gfx_wire`, through `nonos_gfx_serve`)
like every other G display driver since G-77.

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
`DeviceEnum`, `Mmio`, `Dma`, and `Pio`:

```text
CAPSULE_REQUIRED_CAPS = 0x1B9018
CAPSULE_OPTIONAL_CAPS = 0x100
```

The optional bit is `Debug`, which only a `capsule-serial-debug` build grants.
`GraphicsSurfaceCreate` admits `MkSurfaceRegister` and `MkSurfaceShare`, which
publish each primary as a surface.

The capsule reaches hardware only through `MkDeviceList`, `MkDeviceClaim`,
`MkMmioMap`/`MkPioGrant`, and `MkDmaMap`; it polls and binds no interrupt, so
the manifest holds no `Irq`. The kernel brokers
grants, routes IPC, validates the signed manifest, and tears grants down on
exit. It does not
interpret GPU resources, scanout policy, composition, cursor policy, or window
ownership.

Only the compositor may send to `driver.virtio_gpu0`: the kernel holds the
endpoint to it (`src/services/registry/held.rs`), by name and by pid. The
wire server's owner is the last pid to ask SURFACE: a restarted compositor.

## Interface contract

The display wire (`nonos_gfx_wire`), served by `nonos_gfx_serve`:

| Operation | What virtio-gpu answers |
|---|---|
| `CAPS` | kind 1 (VIRTIO_GPU), one output per enabled scanout (at most 4), `MODE_SET \| MODE_RANGE \| HINTS`, sides to 4096 |
| `MODES` | the host's wanted size (preferred), then output 0's EDID modes, then the size shown |
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

## Authority

The capsule may claim the virtio GPU PCI device, map its register BAR or take a
PIO grant, allocate broker-owned DMA for control queue 0 and for the primary
scanout's backing, and register and share that backing as a surface. It binds
no interrupt. It has no framebuffer MMIO grant, no compositor authority, no
input authority, and no filesystem or network authority.

## Privacy and persistence

The capsule stores no windows, screenshots, pointer paths, keystrokes, or
display history. Each output's surface holds the last frame flushed; it is a
DMA grant, revoked on exit and zeroed by the broker before reuse. When the
owner falls silent, the server zeroes the primaries and flushes them, so the
surface and the host's copy hold none of its frame; a new owner is handed
primaries from zeroed pages. Runtime
state is otherwise grant ids, queue addresses, the heads (each output's
scanout, surface and the host's wanted size), and surfaces in limbo.

## Runtime lifecycle

Startup discovers a virtio-gpu PCI function, claims it, takes it off its
legacy interrupt line, maps its registers, allocates queue DMA, runs
a device reset (status 0, then read back until it reads 0) and
ACK/DRIVER/FEATURES_OK/DRIVER_OK, selects control queue 0, reads the scanouts,
reads output 0's EDID when offered (the wire's panel), runs a best-effort virgl
render probe that only logs whether 3D works, and takes every enabled scanout
(`src/display/take.rs`): a zeroed DMA surface of the host's size, a 2D resource
on it, sent whole, set on the scanout and flushed. It then serves the display
wire, waking at least every 500 ms so the server's watch sees a silent owner.
Shutdown relies on process teardown and broker grant revocation.

## Failure model

Every setup phase rolls back prior broker grants on failure, and a failed
attempt releases the claim, which takes every grant with it, whichever step
failed. Bring-up is tried a bounded number of times with a sleep between tries
(`nonos_libc::bring_up`); the capsule exits 2 when no virtio-gpu is present and
6 when one is present but never comes up, and the compositor falls back to GOP.
Missing BAR0, zero queue size, or FEATURES_OK rejection prevents the capsule
from serving. The resource ops post controlq commands; the others read state.

A silent owner (3 s, `nonos_gfx_serve`'s watch) has its primaries zeroed and
flushed twice, so each surface, the host's copy and the screen hold none of its
frame. A blank a device did not take whole is tried again on the next wake. A
surface the host will not let go of stays in limbo with its pages mapped, never
handed back to the kernel while the host may read them. It is tried again on
every mode set and STATE (`src/display/limbo.rs`).

## Current implemented surface

- Virtio GPU PCI discovery for transitional and modern IDs.
- Brokered device claim, register map, and DMA queue allocation; polled.
- Legacy virtio control-queue initialization.
- Modern (virtio 1.0) initialization for the modern ID, with the vendor
  capabilities parsed by the shared `userland/nonos_virtio` (first usable of
  each type, lengths and BARs checked, MSI-X table pages skipped).
- Config reads for events, scanout count, and capset count.
- The display wire: CAPS, MODES, STATE (with the host's display events),
  SURFACE, FLUSH, MODE_SET and RETIRE, through `nonos_gfx_serve`, for every
  enabled scanout.
- Static gates for capability boundary and endpoint ownership.

## Wire format

The display wire's 20-byte envelope (`NVGP` magic, version `1`) and its bodies,
as `nonos_gfx_wire` defines them. The driver's own private ops (0x02 to 0x0C)
were retired in G-77.

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

Proved on the host, without a device: the capability parse the modern path
uses (`userland/virtio_transport_proofs`), and the legacy and modern
handshakes, the control queue and its wire formats, the device configuration
offsets on both transports and the doorbell bound (`userland/virtio_gpu_proofs`).
Only a boot shows the rest. Under QEMU q35, confirm both ways:

- Without an IOMMU (the default `virtio-vga,disable-modern=on`, legacy): the
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
