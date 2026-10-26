# gpu: one-processor boot of fde2defc9, a starved compositor keeps its frame

## How it ran

- Image: `make dev-image` at fde2defc9 (qemu-dev, throwaway keys).
- Machine: QEMU TCG, no KVM, 1 CPU, 8 GiB, `--lane iommu=intel`, fresh disk.
- Driven over QMP: setup by Enter, then screendumps.

## What it showed

- `serial.log`: `[IOMMU] intel-vt-d present, enforcing=1`, `[SMP] cpu=0`,
  `[DISPLAY] on driver.virtio_gpu0 1920x1080`. While the desktop's capsules
  started, the compositor's turns were slow, `slow turn 2540 ms` and `slow
  turn 5309 ms (drain 5042)`, and no driver cleared the screen: there is no
  `compositor silent 3 s: screen cleared` line and no `output lost` line.
- `serial-before.log`: the boot of 9ad05a662, where only a ready or running
  compositor kept its frame. Its compositor waited asleep on peers in a
  4821 ms turn, and the driver cleared the screen once. fde2defc9 counts
  asleep as alive too.
- `desktop.png`: the desktop after setup, with menu bar, dock and the
  live-session dialog.
