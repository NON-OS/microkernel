# gpu: boot of 469dddb58 on the Intel IOMMU lane, 25 October 2026

The same image as `gpu-469dddb58-no-iommu`, on the gate's lane: the no-IOMMU
marking must not show.

## How it ran

- Image: `make dev-image` at 469dddb58 (qemu-dev, throwaway keys).
- Machine: QEMU TCG, no KVM, 4 CPUs, 8 GiB, `--lane iommu=intel`, fresh disk.
- Driven over QMP as in the no-IOMMU run.

## What it showed

- `serial.log`:
  - `[IOMMU] intel-vt-d present, enforcing=1, unconfined grants=0`
  - `[SMP] answered=3 online=4`
  - `[DISPLAY] on driver.virtio_gpu0 1920x1080`, `[SPLASH] up 1920x1080`
- `splash.png`: the splash with no red line; the band's marks with no red.
- `desktop.png`: the desktop, menu bar and dock, 3354 colours; `band.png`: no
  red pixel in the band's bottom rows (0 of 168 sampled).

## Open, seen in this run

- `[DIAG] ipc.call unanswered`: `wallpaper` twice, `attest` once,
  `driver.virtio_net0` four times. Wallpaper is gpu's next item; the others
  are routed to their owners through the maintainers.
