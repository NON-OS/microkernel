# gpu: boot of 508ca4d90 on the Intel IOMMU lane, 25 October 2026

The wallpaper's unanswered call, the honest attestation panel and the

## How it ran

- Image: `make dev-image` at 508ca4d90 (qemu-dev, throwaway keys).
- Machine: QEMU TCG, no KVM, 4 CPUs, 8 GiB, `--lane iommu=intel`, fresh disk.
- Driven over QMP: a screendump once the splash was up, Enter through all 13
  setup steps on their defaults, 105 s, then a screendump.

## What it showed

- `serial.log`:
  - `[IOMMU] intel-vt-d present, enforcing=1, unconfined grants=0`
  - `[SMP] answered=3 online=4`
  - `[DISPLAY] on driver.virtio_gpu0 1920x1080`, `[SPLASH] up 1920x1080`
  - no `ipc.call unanswered` line at all; no `[SHELL] wallpaper` retry line.
- `splash.png`: NONOS's title, the panel saying `secure boot off`,
  `ed25519 verified`, `path only (development)`, `UNVERIFIED`; no firmware
  logo (the white pixels where TianoCore stood are the pointer's, 14 of them).
- `desktop.png`: the desktop with its wallpaper, menu bar and dock, 3356
  colours.

## Open, seen in this run

- `ipc.call late driver.virtio_gpu0 after 100 ms`, a FLUSH as the splash
  steps aside, and `after 213 ms` as the splash is spawned again. Seen on this
  lane in all three boots today and on the no-IOMMU lane in none: gpu's next
  item. The frame is damaged again and goes out on the next one.
- `ipc.call late driver.virtio_net0` (net.core's short polls, hw's B-04).
