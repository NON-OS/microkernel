# Display

How NONOS puts the desktop on a screen: through the UEFI GOP framebuffer on any machine, through virtio-gpu in a virtual machine, and what it cannot drive.

## Three paths

| Path | Code | Used when |
|---|---|---|
| UEFI GOP framebuffer | the bootloader, `src/kernel_core/init/framebuffer`, the compositor | on every UEFI machine, whenever no virtio GPU driver answers |
| virtio-gpu | `capsule_driver_virtio_gpu`, served as `driver.virtio_gpu0` | a virtio GPU in a virtual machine |
| Bochs BGA | `capsule_driver_bga` | never: the capsule is parked and in no image |

There is no native driver for an Intel, AMD or NVIDIA GPU. On real hardware the desktop is drawn through the framebuffer the firmware set up before boot.

```mermaid
flowchart TD
    L[GOP mode choice] --> K[init_framebuffer]
    K --> C[compositor]
    C -->|no virtio GPU| P[MkSurfacePresent]
    P --> F[GOP framebuffer]
    C -->|driver answers| G[driver.virtio_gpu0]
    G --> S[scanout 0]
```

The loader makes the GOP mode choice, `init_framebuffer` maps it in the kernel, and the compositor either presents its frames through `MkSurfacePresent` to that framebuffer or hands them to `driver.virtio_gpu0`, which shows them on scanout 0.

## The GOP framebuffer

### The mode the loader picks

The bootloader tries the GOP on the handle that carries the firmware console first, as Linux's EFI stub does, so that on a machine with a GOP on each of two GPUs it takes the one the firmware console uses; `console_order` keeps the firmware's order otherwise (`nonos-bootloader/src/display/gop/order.rs:16-31`).

It takes a mode only if it has a linear 32-bit framebuffer: PixelRedGreenBlueReserved8BitPerColor, PixelBlueGreenRedReserved8BitPerColor, or a PixelBitMask with exactly one of those two layouts, as `linear_bgr` and `bitmask_bgr` decide (`nonos-bootloader/src/display/gop/mode.rs:20-33`, `nonos-bootloader/src/display/gop/pick.rs:75-92`). A PixelBltOnly GOP, 16 or 24 bits a pixel, 10-bit channels, or a FrameBufferSize too small for the mode, which `fb_covers_mode` checks, leave no framebuffer (`nonos-bootloader/src/display/gop/mode.rs:35-40`). `init_framebuffer` then has nothing to map, and `log_refused` writes `[FB] not mapped: the loader handed over no framebuffer; the desktop has no display` (`src/kernel_core/init/framebuffer/init.rs:25-29`, `src/kernel_core/init/framebuffer/report.rs:21-26`).

`choose` picks the mode in this order (`nonos-bootloader/src/display/gop/pick.rs:156-211`):

1. A mode pinned by a development build.
2. The panel's native timing, from the first detailed timing in its EDID, which `parse_edid` reads with the header and checksum checks Linux applies (`nonos-bootloader/src/display/gop/pick.rs:99-146`).
3. The mode the firmware left set, when it has at least `KEEP_MIN_HEIGHT` (720) lines.
4. Otherwise the largest offered mode with more pixels than the current one, no wider or taller than the native mode when the EDID names it, and never more pixels than `FALLBACK_MAX_PIXELS`, 3840 times 2400. When no offered mode qualifies, the current mode stays.

Any width or height above `MAX_DIM` (8192) is refused (`nonos-bootloader/src/display/gop/pick.rs:61-73`). The loader logs one line from `report_gop_mode`, `[GOP] WxH <source> pitch=Npx fmt=BGRX or RGBX fb=0x... panel=WxHmm`, or `[GOP] no linear framebuffer:` with the reason (`nonos-bootloader/src/display/gop/report.rs:26-63`).

### The kernel's mapping

`init_framebuffer` takes the framebuffer from the [boot handoff](../overview/glossary.md#boot-handoff) (`src/kernel_core/init/framebuffer/init.rs:25-57`). `frame` maps the pitch times the height and never the firmware's FrameBufferSize, which some firmware reports as the whole GPU aperture. It refuses, with a `Refusal` it logs, a zero width, height, pitch or address, a pitch shorter than a row of 4-byte pixels, an overflow, or a FrameBufferSize smaller than the frame (`src/kernel_core/init/framebuffer/frame.rs:21-75`).

Before the mapping and before any AP starts, `program_boot` makes page attribute table entry 1 write-combining on the boot CPU, `mirror_on_ap` writes the same table on each AP, and on a CPU with no PAT the frame is mapped uncached (`src/arch/x86_64/pat/program.rs:31-61`). The new table comes from `with_wc`, which changes entry 1 and keeps every other entry (`src/arch/x86_64/pat/value.rs:31-40`). The kernel logs `[FB] mapped WxH pitch=N BGRX write-combining scale=S`, with `RGBX` or `uncached` where those apply, or `[FB] not mapped:` with the reason, through `log_mapped` and `log_refused` (`src/kernel_core/init/framebuffer/report.rs:21-41`).

`hidpi_scale` doubles the scale on a panel of 192 DPI or more each way whose half is still at least 1280 by 720; without a physical size from the EDID, it doubles from 2560 by 1440 up (`src/kernel_core/init/framebuffer/hidpi.rs:23-79`). The compositor keeps the same rule in `scale_for_panel` (`userland/compositor/src/setup/canvas_scale.rs:65`), and `setup_layout_proofs` holds the two to the same answer (`userland/compositor/src/setup/canvas_scale.rs:17-21`); it passes 56 tests at this commit.

### The compositor on GOP

When no virtio GPU driver announces itself, `run_gop_once` gives the compositor a surface backed by its own memory and presents through `MkSurfacePresent`; the kernel copies each frame into the firmware framebuffer (`userland/compositor/src/setup/prime_gop.rs:17-22`, `userland/compositor/src/setup/prime_gop.rs:36-88`). The present path refuses a rectangle outside the screen, through `blit` (`src/syscall/dispatch/router/graphics_present/blit.rs:22-67`).

A screen smaller than 1024 by 720, such as an 800 by 600 firmware mode or a 1024 by 600 panel, gets a canvas of its own shape that covers 1024 by 720 and is shrunk onto the screen, from `floor_canvas` (`userland/compositor/src/setup/canvas_floor.rs:16-43`). The picture is soft but every screen fits.

If the virtio GPU driver answers after the compositor fell back to GOP, `upgrade_to_virtio` moves the display over and repaints the whole scene there (`userland/compositor/src/setup/upgrade.rs:17-52`).

## virtio-gpu

The [driver capsule](../overview/glossary.md#driver-capsule) `capsule_driver_virtio_gpu` drives a virtio GPU, PCI 1AF4:1010 (transitional) or 1AF4:1050 (modern), the ids `VIRTIO_GPU_TRANSITIONAL` and `VIRTIO_GPU_MODERN` that `is_match` accepts (`userland/capsule_driver_virtio_gpu/src/constants/pci.rs:16-18`, `userland/capsule_driver_virtio_gpu/src/discover/match_device.rs:23-27`). It polls and binds no interrupt. Its [capability word](../overview/glossary.md#capability-word), `0x1B9018`, adds `GraphicsSurfaceCreate` to the usual driver bits so it can register and share the screen surface (`userland/capsule_driver_virtio_gpu/Capsule.mk`). Only the `compositor` may send to it (`src/services/registry/held_table.rs:34`).

One bring-up attempt, `claimed`, takes the device off its legacy interrupt line, maps the registers, takes the control-queue DMA, runs the virtio handshake, reads the scanouts, asks for EDID, probes for 3D and creates the primary surface (`userland/capsule_driver_virtio_gpu/src/setup/sequence.rs:45-92`). The EDID answer is only logged. The 3D probe is logged, and its result, `virgl_ready`, is reported by `OP_QUERY_CAPS` (`userland/capsule_driver_virtio_gpu/src/server/handlers/query_caps.rs:25-28`). No op of the service uses 3D, so the 2D path is the same either way.

### Modes

`seed` asks the device for its display info and records each enabled scanout at the size the host reports. A scanout smaller than 640 by 480 gets `DEFAULT_SCANOUT_WIDTH` by `DEFAULT_SCANOUT_HEIGHT`, 1280 by 720, and with no enabled scanout scanout 0 is set to that size (`userland/capsule_driver_virtio_gpu/src/setup/scanouts.rs:20-69`). `OP_MODE_LIST` returns the recorded scanouts, written out by `handle` (`userland/capsule_driver_virtio_gpu/src/server/handlers/mode_list.rs:20-42`).

### The primary surface

`create` makes the screen surface on scanout 0 only (`userland/capsule_driver_virtio_gpu/src/setup/create_primary.rs:21-37`). It takes a DMA [grant](../overview/glossary.md#grant) the size of the frame, creates a B8G8R8A8 2D resource on it, sets it on the scanout, and registers and shares it with `mk_surface_register` and `mk_surface_share` so the compositor can draw into it (`userland/capsule_driver_virtio_gpu/src/setup/primary_surface/create.rs:23-73`). A frame of 4 GiB or more is refused by `derive` (`userland/capsule_driver_virtio_gpu/src/setup/primary_surface/geometry.rs:22-33`). The [hardware broker](../overview/glossary.md#hardware-broker) caps one display grant at `DISPLAY_FRAMEBUFFER_PAGES`, 8192 pages or 32 MiB, which holds one 3840 by 2160 frame at 4 bytes a pixel (`src/hardware/broker/dma/limits.rs:28-44`). For a larger scanout 0, such as 3840 by 2400, the primary surface's `mk_dma_map` fails (`userland/capsule_driver_virtio_gpu/src/setup/primary_surface/dma.rs:19-29`). `create_primary::create` hands that error to the bring-up attempt, so every attempt fails the same way (`userland/capsule_driver_virtio_gpu/src/setup/sequence.rs:66-73`).

The service answers twelve ops, from `OP_HEALTHCHECK` to `OP_GET_PRIMARY_SURFACE` (`userland/capsule_driver_virtio_gpu/src/protocol/ops.rs:16-27`). Each resource records the process that created it in `owner_pid`, and the resource ops refuse another process's resource (`userland/capsule_driver_virtio_gpu/src/server/handlers/create_resource.rs:59`, `userland/capsule_driver_virtio_gpu/src/server/handlers/set_scanout.rs:33`).

Without a virtio GPU the capsule exits with `EXIT_ABSENT` (2) at once, and the compositor stays on GOP; a device that fails every bring-up attempt exits with `EXIT_GAVE_UP` (`userland/capsule_driver_virtio_gpu/src/main.rs:40-55`). At this commit `virtio_gpu_proofs` passes 38 host tests. Its crate comment lists what they hold to the specification: the status handshake, the accepted features, the control queue and the wire bytes of each 2D command, with the shipping source mounted by `#[path]` (`userland/virtio_gpu_proofs/src/lib.rs:17-32`).

## Bochs BGA

`capsule_driver_bga` matches the QEMU and Bochs display adapter, PCI 1234:1111 (`VENDOR_QEMU_BOCHS`, `DEVICE_BGA`), with its framebuffer in BAR 0 and registers in BAR 2, as `find_bga` checks (`userland/capsule_driver_bga/src/constants.rs:17-22`, `userland/capsule_driver_bga/src/discover.rs:34-57`). It sets one mode, `MODE_WIDTH` by `MODE_HEIGHT` at 32 bits, 1024 by 768 (`userland/capsule_driver_bga/src/constants.rs:33-35`), then serves nothing and sleeps in turns of `HOLD_MS`, 60 seconds (`userland/capsule_driver_bga/src/main.rs:32-53`).

It is parked. It has no `Capsule.mk`, no Cargo feature and no kernel mirror, and `family_driver` leaves `DisplayBga` without a driver on purpose: re-moding the adapter would destroy the firmware scanout the compositor presents into (`src/hardware/inventory/driver.rs:33-36`). `bga_proofs` passes 9 host tests at this commit.

## What is not supported

- Native GPU drivers. Intel, AMD and NVIDIA display functions are enumerate-only, and `missing_path` names a `native modeset and scanout driver` for them (`src/hardware/inventory/missing.rs:19-27`). There is no mode change after boot, no second display and no hardware acceleration on real machines.
- A GOP without a linear 32-bit framebuffer.
- 3D. The virtio-gpu probe reports whether the host could, and no op uses it.
- Backlight control.

## See also

- [Drivers](README.md)
- [Broker API](broker-api.md)
- [Platform](platform.md)
- [The desktop](../using/desktop.md)
- [Boot handoff and kernel init](../kernel/boot-handoff.md)
- [Support matrix](../hardware/MATRIX.md)
- [UEFI specification](https://uefi.org/specifications)
- [virtio specification](https://docs.oasis-open.org/virtio/virtio/v1.2/virtio-v1.2.html)
