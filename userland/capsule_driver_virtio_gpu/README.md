# capsule_driver_virtio_gpu

## Role

`capsule_driver_virtio_gpu` is the virtio GPU display-controller capsule. It
owns the PCI device claim, BAR mapping, control-queue DMA, the virtio
initialization sequence, and the primary scanout surface for
`driver.virtio_gpu0`.

```text
compositor (the only sender the kernel admits)
        |
        v
driver.virtio_gpu0 -- control queue DMA --> virtio-gpu device
        |
        `-- primary scanout: a DMA grant registered and shared as a surface
```

The capsule is not a compositor, window manager, renderer, toolkit, or input
router. It exposes device configuration and 2D scanout and resource commands;
all desktop policy stays in the compositor.

## Microkernel contract

The manifest grants `IPC`, `Memory`, `GraphicsSurfaceCreate`, `Driver`,
`DeviceEnum`, `Mmio`, `Dma`, and `Pio`:

```text
CAPSULE_REQUIRED_CAPS = 0x1B9018
CAPSULE_OPTIONAL_CAPS = 0x100
```

The optional bit is `Debug`, which only a `capsule-serial-debug` build grants.
`GraphicsSurfaceCreate` admits `MkSurfaceRegister` and `MkSurfaceShare`, which
publish the primary scanout as a surface. `Pio` is no longer used: it served
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
endpoint to it (`src/services/registry/held.rs`), by name and by pid. Within
the driver, the primary resource is bound to the first pid that asks for it,
and transfer, scanout and flush refuse a resource another pid owns.

## Interface contract

| Operation | Input | Output |
|---|---|---|
| `OP_HEALTHCHECK` | none | the control queue's state (0 live, 1 down) and the stalls recovered in place |
| `OP_CONTROLLER_INFO` | none | device id, queue size, feature bits |
| `OP_DISPLAY_INFO` | none | events, scanout count, capset count |
| `OP_CONTROLQ_STATE` | none | control queue DMA metadata |
| `OP_QUERY_CAPS` | none | scanout count, capset count, events, and whether the virgl probe passed |
| `OP_TRANSFER_TO_HOST` | resource, rectangle | TRANSFER_TO_HOST_2D posted |
| `OP_SET_SCANOUT` | resource, scanout | SET_SCANOUT posted |
| `OP_FLUSH` | resource, rectangle | RESOURCE_FLUSH posted |
| `OP_MODE_LIST` | none | the scanout modes |
| `OP_GET_PRIMARY_SURFACE` | none | the primary surface handle and geometry |

Unknown operations reply `E_BAD_OP`. Malformed request bodies reply `E_INVAL`.
Ops `0x0006` and `0x0007` once created a client resource and attached backing
to it. No client sent them, a resource could never be destroyed, and once the
64-entry table was full a create left behind the host resource it had just
made; they are answered as unknown now and their numbers are not reused.

## Authority

The capsule may claim the virtio GPU PCI device, map the virtio 1.0 structures
its capabilities place in a memory BAR, allocate broker-owned DMA for control
queue 0 and for the primary scanout's backing, and register and share that
backing as a surface. It binds
no interrupt. It has no framebuffer MMIO grant, no compositor authority, no
input authority, and no filesystem or network authority.

## Privacy and persistence

The capsule stores no windows, screenshots, pointer paths, keystrokes, or
display history. The primary scanout's backing holds the last frame drawn; it
is a DMA grant, revoked on exit and zeroed by the broker before reuse. Runtime
state is otherwise grant ids, queue addresses, device feature bits, the
resource table, and virtio-gpu config counters.

## Runtime lifecycle

Startup discovers a virtio-gpu PCI function (1af4:1050), claims it, turns
memory decode on with its legacy interrupt line off, maps the common, notify
and device structures through the shared transport (`userland/nonos_virtio`),
resets it (status 0, read back by the clock), negotiates features (VERSION_1,
VIRGL and EDID when offered, and ACCESS_PLATFORM whenever offered), allocates
queue DMA, programs control queue 0, and only then turns bus mastering on and
sets DRIVER_OK: a device left live by a dead predecessor may reach memory only
once it reads back reset and its queue is on this grant, and a device refused
at any step never may. It then reads the scanouts, asks for EDID when offered (logged
only), runs a best-effort virgl render probe that only logs whether 3D works,
and creates the primary scanout: a DMA grant attached to a 2D resource, set on
scanout 0, then registered with `mk_surface_register` and shared with
`mk_surface_share` (`src/setup/primary_surface/create.rs`). A probe command
that stalls resets the device; the queue is restarted on the same grant and
the primary scanout still goes up, without virgl (`src/setup/probes.rs`). It
then serves IPC. Whether the claim is confined to an IOMMU domain is the kernel's to
enforce at claim time; the capsule does not guess.

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
The resource ops post controlq commands; the others read state.

One control command is in flight at most, counted by the driver. A command
the device does not answer within `CTRLQ_MS` (2 s, by the uptime clock), or
answers out of turn, ends in a device reset: its chain named the staging page
the next command would reuse. By then the compositor draws into the primary
surface's frames, so the server never gives the device back or exits (the
kernel would free the frames under it, and nothing restarts a driver). It
recovers in place (`src/server/recover/`): the queue restarted on the same
grant, the primary resource made again under the same id on the same frames
and set on scanout 0, at most three tries, each logged
(`driver.virtio_gpu0: control queue stalled; device reset, recovering (try 1
of 3)`). The reset takes the render context, so `OP_QUERY_CAPS` stops offering
3D. If no try works, or the device stalls again after three recoveries in a
row without serving 30 s after any of them, it stays up and `Down`: the
device left reset, the claim and every grant kept, device ops answering
`E_DEVICE`, and `OP_HEALTHCHECK` naming the state. Only a failed service registration, before
anyone holds the surface, resets the device and gives it back before exit 1.

## Current implemented surface

- Virtio GPU PCI discovery for the modern ID (virtio-gpu has no
  transitional form, and nothing presents 0x1010).
- Brokered device claim, register map, and DMA queue allocation; polled.
- Virtio 1.0 initialization on the shared `userland/nonos_virtio`: the vendor
  capabilities parsed (first usable of each type, lengths and BARs checked,
  MSI-X table pages skipped), the regions mapped and bounds-checked, the
  features negotiated and the queue programmed.
- Config reads for events, scanout count, and capset count.
- IPC health, controller info, display info, control queue state, caps,
  mode list, and the 2D resource commands.
- The primary scanout surface, shared with the compositor.
- Static gates for capability boundary and endpoint ownership.

## Wire format

Requests use the `NVGP` capsule header, version `1`, and the shared 20-byte
driver envelope. Replies begin with a 4-byte signed status word. All multi-byte
integers are little-endian.

## State ownership

`driver.virtio_gpu0` owns only hardware-facing GPU state: PCI claim, register
grant, control queue DMA grant, the primary scanout's DMA grant, the resource
table, queue size, feature mask, and config counters. The compositor owns surfaces, damage, focus, cursor, z-order, and
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
recovery from a stall on the same surface or to `Down` with nothing released,
the recoveries in a row bounded, a probe stall at bring-up still ending in the
2D surface, and the reset-before-release orders
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
