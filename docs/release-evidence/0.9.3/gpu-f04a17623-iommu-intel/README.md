# gpu: gate boot of f04a17623, the band's glyphs and Settings' address row

Letter HANDOFF/letters/gpu/11-band-and-address.md.

## How it ran

- Image: `make dev-image` at f04a17623 (qemu-dev, throwaway keys).
- Machine: QEMU TCG, no KVM, 4 CPUs, 8 GiB, `--lane iommu=intel`, fresh disk.
- Driven over QMP: setup by Enter, then a screendump. After that the PS/2
  pointer:
  - dismissed the live-session dialog;
  - opened Settings from the dock, then its Network page;
  - scrolled the page with the wheel.

## What it showed

- `serial.log`: `[IOMMU] intel-vt-d present, enforcing=1`, `[SMP] answered=3
  online=4`, `[DISPLAY] on driver.virtio_gpu0 1920x1080`. There is no
  `ipc.call unanswered` line and not one `ipc.call late` line.
- `desktop.png`: the desktop after setup.
- `band-lock-on.png`, `band-at-rest.png`: the band at the top right, 4x.
  - Lock on: the live-session dialog holds the trusted band, so the lock is
    lit.
  - At rest: all four marks dim.
  - These two are from the boot of 12f1f4602 just before, whose band code
    is the same.
- `band-each-mark-host.png`: each mark on in turn, rendered on the host by
  the compositor's own band code. On this build no capsule may hold the
  camera, microphone or screen-capture bit (Decision 9), so a boot cannot
  light them.
- `settings-network.png`: Network status ends at DNS. QEMU's card is not
  Hyper-V's, so the driver gives no station source and no empty "Network
  address" row is drawn. The Hyper-V wording is held by
  capsule_settings_proofs (`station_tests`, `station_words_tests`).
