# H-06(a): no linear framebuffer, named from the loader to the desktop, booted at a84b6ad8d

hw/gop-refusal a84b6ad8d (on main 03ae0f8e4), built as standard-dev in the
dev copy of the tree and sealed with throwaway keys; no key under `.keys/`
was read. QEMU only, under TCG. **Not hardware-verified.**

## Two boots of the same image

**Normal** (`qemu-normal.txt`, the default lane: q35, no IOMMU, virtio-vga
at 1920x1080, a fresh data disk; `normal.serial.log`):

- The loader latched the firmware's 1280x800 mode (`[GOP] 1280x800 firmware
  current`), the kernel mapped it (`[FB] mapped 1280x800`) and the compositor
  moved to `driver.virtio_gpu0 1920x1080`.
- No `[GOP] no linear framebuffer`, `[GOP] no framebuffer handed over`, `no
  display yet` or `still no display` line.
- `FB_REFUSED` stayed clear: the kernel accepted a handoff with
  `FB_AVAILABLE`, and its framebuffer check refuses one with both bits.
- Setup walked over QMP (every default but Qwen "None for now"), then
  `normal-desktop.png`: the desktop with the menu bar and the dock, 1920x1080,
  3343 colours.

**No display** (`qemu-no-display.txt`: the same command with the virtio-vga
device removed and `-vga none`, so OVMF has no Graphics Output Protocol at
all; `no-display.serial.log`, four minutes):

- Loader: `[GOP] no linear framebuffer: no Graphics Output Protocol on any
  handle`.
- Kernel: `[GOP] no framebuffer handed over: the firmware has no Graphics
  Output Protocol`: the loader's reason, carried by `FB_REFUSED` and the
  refusal word, decoded.
- Compositor: `[DISPLAY] no display yet (no framebuffer: the firmware has no
  Graphics Output Protocol); waiting for a display driver` once, at about 7 s
  of uptime, and `[DISPLAY] still no display 10 s on (...)` once, at about
  16 s (the `[BENCH]` lines before each). It kept waiting: no compositor or
  shell exit in the four minutes.
- Each of the four lines appears exactly once.

## Not booted

The other reasons (PixelBltOnly, 16-bit PixelBitMask with its depth, a
framebuffer smaller than its mode) cannot be made under OVMF, which always
offers a 32-bit linear mode when it has a display. They are host-tested
(`kernel_proofs::handoff_refusal`, every code byte and depth through the
decoder and every word the loader makes round-tripped), and
`HANDOFF/hardware/procedure-gop.md` step 5 asks for them on a real machine.
