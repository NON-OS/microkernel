# Broker: The boot framebuffer

Part of [Broker](broker.md).

## The boot framebuffer

The framebuffer the kernel adopted at boot (`init_framebuffer`: the loader's GOP
framebuffer on x86) is one record on the firmware bus: `BUS_KIND_FIRMWARE`,
device `0x0001`, class display (`src/hardware/broker/bootfb/`). Bar 0 is the
window from the page the first pixel lies in to the end of the last row,
`BAR_KIND_MMIO` and `BAR_FLAG_PREFETCH`, and its `aux` is the first pixel's
offset into that page; `MkMmioMap` with `MK_MMIO_MAP_WC` maps it with whatever
type the kernel's own mapping has. Only the process serving `driver.bootfb0` may
claim it (`hold_for_firmware`, `src/hardware/broker/claim/hold_rule.rs`), since
whoever holds it can read the screen. `MkBootFramebuffer` answers the holder
with six little-endian u32s: width, height, row pitch in bytes, the loader's
pixel format (0 RGB, 1 BGR, 2 RGBX, 3 BGRX), the first pixel's offset, and the
panel's size in millimetres (width low 16 bits, height high 16, 0 when unknown).
While the claim is held the kernel draws nothing on the framebuffer:
`MkSurfacePresent` answers `EBUSY`, and the heartbeat and boot log are silent;
the stop and panic screens still draw. Releasing the claim, or the holder's
exit, gives it back to the kernel.

A claim of a function whose memory holds the framebuffer takes its scanout for
the rest of the boot (L79). The function is found once at boot: a record with a
memory BAR sharing a page with the framebuffer, or, when no BAR holds it, the
one display-class PCI function if there is exactly one; with two or more and
none holding it, no claim takes it, and the boot log says which applies. Such a
claim stops the kernel drawing before it is made ready. If `driver.bootfb0`
holds the framebuffer then, its claim is retired, every page of its mapping that
still shows the framebuffer is pointed at one zeroed kernel page (stores land
there unseen, loads read back what was written, nothing faults), and the claim
is released. The GPU's claim is then made ready. From that moment
`MkBootFramebuffer` answers `ENODEV`, `MkMmioUnmap` of the old grant `EINVAL`,
`MkDeviceRelease` `ENODEV`, a new claim of the framebuffer `EPERM` (final; libc
retries only `EAGAIN`), and the kernel never draws on it again
(`MkSurfacePresent` answers `EBUSY`). A covering claim that fails at its last
step gives the console back to the kernel. `MkDmaMap` against the framebuffer's
record always answers `EINVAL`.
