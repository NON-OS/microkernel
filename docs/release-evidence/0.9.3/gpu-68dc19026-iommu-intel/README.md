# gpu: gate boot of 68dc19026, MkBootLogo retired, 25 October 2026

MBGL (MkBootLogo) retired under the stable-ABI rule: the number kept, EINVAL
to every caller before dispatch (`src/syscall/microkernel/retired.rs`), its
handler, the broker's logo record and libc's wrapper removed.

## How it ran

- Image: `make dev-image` at 68dc19026 (qemu-dev, throwaway keys).
- Machine: QEMU TCG, no KVM, 4 CPUs, 8 GiB, `--lane iommu=intel`, fresh disk.
- Driven over QMP: Enter through all 13 setup steps on their defaults, 105 s,
  then a screendump.

## What it showed

- `serial.log`: `[IOMMU] intel-vt-d present, enforcing=1`, `[SMP] answered=3
  online=4`, `[DISPLAY] on driver.virtio_gpu0 1920x1080`, the splash up and
  stepping aside; no `ipc.call unanswered` line.
- `splash.png`: NONOS's mark and the honest panel; `desktop.png`: the desktop,
  menu bar and dock, 3351 colours.

## Open, seen in this run

- `ipc.call late driver.virtio_gpu0 after 100 ms`, the FLUSH as the splash
  steps aside: gpu's item in hand. `late driver.virtio_net0`: net.core's poll.
