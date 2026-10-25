
hw/surface-cap 67074381e (the cap with main merged), built as standard-dev in
the dev copy of the tree and sealed with throwaway keys; no key under `.keys/`
was read. QEMU only, under TCG. **Not hardware-verified.**

## The machine

`qemu.txt`: the default lane, the one `make boot` uses: q35, 8 GiB, 4 vCPUs,
virtio-vga at 1920x1080, no IOMMU, a fresh data disk. The firmware's GOP mode
was 1280x800 (4,096,000 bytes, so the cap is the 64 MiB floor). The compositor
came up on the GOP framebuffer, then moved to driver.virtio_gpu0 at 1920x1080
(`serial.log`: `[DISPLAY] on GOP framebuffer 1280x800`, then `moved from GOP to
driver.virtio_gpu0 1920x1080`). No surface was refused, and no compositor,
shell or GPU driver exited.

## The walk

`qk.py` sends keys over QMP and takes a screendump. Setup was walked with every
default but one: the Qwen model step was set to "None for now", as a model
held in memory says nothing about the display. `setup-00` (keyboard), `-01`
(name), `-04` (network), `-08` (Qwen model) and `-12` (review) are the steps
on the way; `desktop-dialog.png` is the desktop as setup finished, with the
live-session dialog.

## The gate

`desktop-gate.png` (1920x1080, 5945 distinct colours): the desktop after the
dialog was dismissed, with the menu bar and the full dock. `desktop-2min.png`
is the same desktop two minutes on (5944 colours).


the dialog was sent to (1062,620) on the 1920x1080 display. The cursor was
drawn with its body at (570..578, 442..457): x * 1024 / 1920 and y * 768 / 1080
here on a desktop that changed mode under load, on a build without the fix.

## Not booted here

- A firmware mode over 64 MiB: OVMF's mode on this machine is 1280x800, so
  the cap's raised range (64 to 128 MiB) stays host-tested
  (`kernel_proofs::surface_cap`).
- The second canvas at 8K, by arithmetic (design): the GOP backing at
  7680x4320 is 132,710,400 bytes, under the 128 MiB ceiling (134,217,728), so
  it registers. A move to a virtio output builds the new canvas before
  `setup/adopt.rs` releases the old one, so both are held for that moment: at
  1920x1080, 8,294,400 bytes more; at 8K, another 132,710,400 bytes, about
  253 MiB at the peak.
