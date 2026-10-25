# gpu: boot of 508ca4d90 without an IOMMU, 25 October 2026

The same image as `gpu-508ca4d90-iommu-intel`, where device DMA is not
confined by hardware.

## How it ran

- Image: `make dev-image` at 508ca4d90 (qemu-dev, throwaway keys).
- Machine: QEMU TCG, no KVM, 4 CPUs, 8 GiB, no IOMMU device, fresh disk.
- Driven over QMP as in the Intel IOMMU run.

## What it showed

- `serial.log`:
  - `[IOMMU] posture=dma-unconfined reason=no-remapping-hardware-described`
  - `[SMP] answered=3 online=4`
  - `[DISPLAY] on driver.virtio_gpu0 1920x1080`, `[SPLASH] up 1920x1080`
  - no `ipc.call unanswered` line at all; no `[SHELL] wallpaper` retry line.
- `splash.png`: NONOS's title, the honest panel, `device DMA not confined by
  hardware` in red under it, the band's red edge; no firmware logo.
- `desktop.png`: the desktop with its wallpaper, menu bar and dock, 3351
  colours.

## Open, seen in this run

- `ipc.call late` for `driver.virtio_net0` and `policy`, short caller
  deadlines under spawn load (hw's B-04); not gpu's.
