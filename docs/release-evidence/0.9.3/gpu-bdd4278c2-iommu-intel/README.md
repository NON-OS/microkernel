# gpu: gate boot of bdd4278c2, Settings card notes wrapped

The note wrap is 74f56105e. It was pushed on the owner's order before this
boot, marked "not booted yet"; this is its boot.

## How it ran

- Image: `make dev-image` at bdd4278c2 (qemu-dev, throwaway keys).
- Machine: QEMU TCG, no KVM, 4 CPUs, 8 GiB, `--lane iommu=intel`, fresh disk.
- Driven over QMP: setup by Enter, then a screendump. Then the PS/2 pointer
  dismissed the live-session dialog, opened Settings from the dock, and
  chose its Network page.

## What it showed

- `serial.log`: `[IOMMU] intel-vt-d present, enforcing=1`, `[SMP] answered=3
  online=4`, `[DISPLAY] on driver.virtio_gpu0 1920x1080`. No `ipc.call
  unanswered` line and no `ipc.call late` line.
- `desktop.png`: the desktop after setup, with menu bar, dock and the
  live-session dialog.
- `settings-network.png`, and `settings-network-card.png` cut from it: the
  Default network note, 464 px against the card's 460, now breaks after
  "falls" onto a second line. The card is one line taller and nothing runs
  past its edge.
