# gpu: boot of 469dddb58 without an IOMMU, 25 October 2026

The no-IOMMU marking and the splash that follows the canvas (letter
not confined by hardware.

## How it ran

- Image: `make dev-image` at 469dddb58 (qemu-dev, throwaway keys).
- Machine: QEMU TCG, no KVM, 4 CPUs, 8 GiB, no IOMMU device, fresh data disk.
- Driven over QMP: a screendump every 5 s through the splash, Enter through
  all 13 setup steps on their defaults, 105 s, then a screendump.

## What it showed

- `serial.log`:
  - `[IOMMU] posture=dma-unconfined reason=no-remapping-hardware-described`
  - `[SMP] answered=3 online=4`
  - `[DISPLAY] on driver.virtio_gpu0 1920x1080`, `[SPLASH] up 1920x1080`
- `splash.png`: the splash, with `device DMA not confined by hardware` in red
  under the attestation panel, and the band at the top right with its red
  bottom edge.
- `desktop.png`: the desktop, menu bar and dock, 3353 colours; `band.png`, the
  band enlarged: dim marks over a red edge.

## Open, seen in this run

- `[DIAG] ipc.call unanswered`: `wallpaper` once, `driver.virtio_net0` seven
  times. The wallpaper one is gpu's next item; virtio-net is not gpu's.
- The canvas did not move from GOP to virtio-gpu in this boot, so the splash's
  redraw on a move ran in proofs only (boot_splash_proofs, `follow_tests`).
