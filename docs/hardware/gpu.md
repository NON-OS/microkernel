# GPUs: what NONOS drives today, and what comes next

What NONOS does today on the GPUs people own, and which vendor documents a native driver
would rest on. Nothing on this page has been booted on real hardware.

## What happens on any GPU today

`make` builds `microkernel-full-gui` (`mk/20-build.mk`, `nonos-mk-zerostate`). That
image carries no native GPU driver: `nonos-capsule-driver-intel-display` and
`nonos-capsule-driver-bga` are held out of it until the release seals them
(`scripts/baselines/display_profiles.txt`), and `capsule_driver_bootfb` is in no image.
So on every real machine the desktop takes one of two paths:

1. **Firmware framebuffer.** The loader picks a linear 32-bit mode from the UEFI GOP,
   the kernel maps it, and the compositor presents through `MkSurfacePresent`
   ([Display](../drivers/display.md#the-gop-framebuffer)). Every GPU whose UEFI
   firmware offers such a mode gets a desktop at that one mode. There is no mode change
   after boot, no second monitor, no hardware cursor, no vblank and no acceleration.
2. **No picture.** A GPU whose option ROM carries no UEFI GOP driver, or whose GOP
   offers only PixelBltOnly or 16, 24 or 30-bit modes, leaves no framebuffer. The kernel
   logs `[FB] not mapped:` and the desktop has no display. NONOS has no legacy BIOS
   path ([Requirements](../install/requirements.md)), so a VBIOS-only card cannot help.

The inventory sorts display functions by vendor (`src/hardware/inventory/classify_display.rs`).
Intel's go to `driver_intel_display` where the image has it; AMD's and NVIDIA's are
enumerate-only, with a `native modeset and scanout driver` missing
(`src/hardware/inventory/support.rs`).

## Summary by family

"Takeover": the native driver serves firmware's mode with flips, a hardware cursor and
hot plug; "mode set": it can change the mode too. The Intel profile is
`microkernel-driver-intel-display`, which no release ships yet.

| Family | Shipped image | Intel profile | Vendor display documents | Page |
|---|---|---|---|---|
| Intel Haswell, Broadwell | GOP | takeover | public PRM | [Intel](gpu-intel.md) |
| Intel Gen9 (Skylake to Comet Lake) | GOP | mode set | public PRM | [Intel](gpu-intel.md) |
| Intel Gen11 (Ice Lake) | GOP | takeover | public PRM | [Intel](gpu-intel.md) |
| Intel Gen12 (Tiger Lake, Rocket Lake) | GOP | takeover | public PRM | [Intel](gpu-intel.md) |
| Intel Alder Lake, Raptor Lake | GOP | takeover | none published | [Intel](gpu-intel.md) |
| Intel Meteor Lake, Arrow Lake, Lunar Lake | GOP | GOP | none published | [Intel](gpu-intel.md) |
| Intel Arc A (DG2), Iris Xe MAX (DG1) | GOP | GOP | public PRM | [Intel](gpu-intel.md) |
| Intel Arc B (Battlemage) | GOP | GOP | none published | [Intel](gpu-intel.md) |
| AMD R5xx, R6xx (2006 to 2008) | GOP or none | GOP or none | public RRG | [AMD](gpu-amd.md) |
| AMD from R7xx, every Ryzen APU | GOP or none | GOP or none | none published | [AMD](gpu-amd.md) |
| NVIDIA, every generation | GOP or none | GOP or none | partly public | [NVIDIA](gpu-nvidia.md) |

"GOP or none": some cards of the family shipped option ROMs without a UEFI GOP driver,
and NONOS shows nothing on those; [Reporting a machine](report.md) is how to tell us.

## Machines with two GPUs

A laptop with an Intel iGPU and an AMD or NVIDIA dGPU usually wires the panel to the
iGPU, and its GOP is then the iGPU's. The loader takes the GOP on the handle that carries
the firmware console first (`nonos-bootloader/src/display/gop/order.rs`), so the
desktop appears where the firmware drew its own screen. The Intel capsule takes the
first Intel function its id files list
(`userland/capsule_driver_intel_display/src/discover.rs`). Outputs wired only to the
dGPU (often HDMI on gaming laptops) stay dark.

On aarch64 the kernel takes a device-tree `simple-framebuffer` node
(`src/arch/fdt/find/framebuffer/find.rs`); no Arm SoC GPU has a native driver.

## Where this goes next

The GPU breadth letters (`the design notes/`) take them as the documents allow:

- Intel Gen11 and Gen12 mode setting from the public Tiger Lake and Ice Lake PRMs;
  published none for current parts;
- NVIDIA on the firmware framebuffer, made solid, with native support costed honestly;
- a design for GPU compute under the IOMMU, Intel Xe first.

None of it is in 0.9.3, whose features freeze on 28 October 2026.
