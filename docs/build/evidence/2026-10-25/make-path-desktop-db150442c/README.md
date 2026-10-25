# The documented make path boots the desktop: booted at b02/make-path-desktop db150442c

`make nonos-mk-desktop-gui-prod nonos-mk-esp`, the contributor desktop path
in docs/build/make-targets.md, in a fresh copy of the tree with CI's
throwaway keys: `nonos-ci/setup-signing-key.sh`, then
`nonos-ci/scratch-trust-bootstrap.sh` for every capsule (156 slots
enrolled, 2 h 51 min), then the make. No key under the checkout's `.keys/`
was read. The copy was afc75ddcd with db150442c's two changed files,
`mk/20-build.mk` and `nonos-data/trust/policy/image_capability_ceiling.bin`.

## Two faults the path had, and the fix

`make-lines.txt`, from a fresh kernel state each time:

- **Before:** make runs with -j, so nonos-mk-esp's chain reached the
  default kernel rule while the desktop goal built the same ELF. The bare
  microkernel-core landed last; the ESP carried a 3 MB kernel, and
  nonos-mk-esp printed its warning.
- **After:** the desktop kernel builds first, the default rule keeps it
  ("Kernel already built as: microkernel-setup-wizard; kept"), and the ESP
  carries it: 80 MB, signed and attested, no warning.

`serial-before-ceiling-fix.txt`: with the race fixed but the committed
ceiling still 0x7FFFFFFF, the kernel refused input_router ("image forbids
InputConsumer"), attest, app.about, app.linux and app.terminal as they
started, so setup never started. The ceiling is now the union of the
catalogue's caps_ceiling (scripts/check_image_ceiling.py names all eight
capsules the old value refused).

## The boot

`qemu.txt`: OVMF, the make-built ESP as a FAT drive, a blank 2 GiB
virtio-blk, virtio-vga at 1920x1080, qemu-xhci, virtio-rng and virtio-net
on QEMU user networking, all behind the Intel IOMMU; 6 GiB, 4 CPUs, TCG.

- `serial.log`: "[DISPLAY] on driver.virtio_gpu0 1920x1080",
  "[INPUT-ROUTER] capsule spawned", "[SETUP] keyboard held", and the lease
  10.0.2.15/24 gw 10.0.2.2.
- `01-setup.png`: step 01 of 13. Setup was walked by keystroke over QMP,
  every default taken.
- `20-desktop.png`: the desktop after setup, the live-session notice
  dismissed: the menu bar, the desktop icons, the wallpaper and the dock.
  It is not one colour: 975 distinct colours in a 192x108 sample.

## The same image with no IOMMU

`serial-no-iommu.log`, `no-iommu-setup.png`: q35 with no IOMMU, the same
devices without iommu_platform, 10 minutes. The no-IOMMU posture
(0bd052c07) is taken and said ("posture=dma-unconfined"), virtio_gpu0
owns the display from the start ("[DISPLAY] on driver.virtio_gpu0
1920x1080"), no "GOP to driver.virtio_gpu0: CAPS failed" line, the
keyboard is held and the guest leases. Before the posture, these lanes
stayed on GOP: virtio_gpu0 could not map DMA, so it never answered CAPS.
